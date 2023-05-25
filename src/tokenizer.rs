use std::str::CharIndices;


pub const ALL_KEYWORDS : [&'static str; 6] = [
    "module",
    "pipeline",
    "state",
    "if",
    "while",
    "for"
];

// ordered by which to prefer
pub const ALL_SYMBOLS : [&'static str; 31] = [
    // Big symbols
    "<=",
    ">=",
    "==",
    "!=",
    "<<",
    ">>",
    "->",
    "&&",
    "||",
    // small Symbols
    "+",
    "-",
    "*",
    "/",
    "!",
    "%",
    "&",
    "|",
    "^",
    "<",
    ">",
    "=",
    "[",
    "]",
    "(",
    ")",
    "{",
    "}",
    ",",
    ";",
    ":",
    "@"
];

pub const IDENTIFIER_SYMBOL_TYPE : u8 = (ALL_KEYWORDS.len() + ALL_SYMBOLS.len()) as u8;
pub const NUMBER_SYMBOL_INDEX : u8 = IDENTIFIER_SYMBOL_TYPE + 1;
pub const COMENT_SYMBOL_INDEX : u8 = NUMBER_SYMBOL_INDEX + 1;
pub const INVALID_SYMBOL_INDEX : u8 = COMENT_SYMBOL_INDEX + 1;

#[derive(Debug,Clone,PartialEq)]
pub struct LexerPart<'a> {
    pub typ : u8,
    pub text : &'a str
}

impl<'a> LexerPart<'a> {
    pub fn is_keyword(&self) -> bool {
        self.typ < ALL_KEYWORDS.len() as u8
    }
    pub fn is_symbol(&self) -> bool {
        self.typ < IDENTIFIER_SYMBOL_TYPE
    }
    pub fn is_identifier(&self) -> bool {
        self.typ == IDENTIFIER_SYMBOL_TYPE
    }
    pub fn is_number(&self) -> bool {
        self.typ == NUMBER_SYMBOL_INDEX
    }
    pub fn is_comment(&self) -> bool {
        self.typ == COMENT_SYMBOL_INDEX
    }
}

#[derive(Debug,Clone,PartialEq)]
pub struct ParsingErr<'a> {
    pub reason : &'static str,
    pub position : &'a str
}

fn is_valid_identifier_char(c : char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn iter_until_comment_end(mut file_char_iter : &mut CharIndices) -> Option<usize> {
	loop {
		if let Some((_, comment_char)) = file_char_iter.next() {
			if comment_char == '*' {
				// end of single line comment
				for (comment_i_2, comment_char_2) in &mut file_char_iter {
					if comment_char_2 == '/' {
						// End of comment
						return Some(comment_i_2);
					} else if comment_char_2 == '*' {
						continue;
					} else {
						break;
					}
				}
			}
		} else {
			return None
		}
	}
}

fn find_end_of_identifier(mut file_char_iter : &mut CharIndices) -> Option<(usize, char)> {
    for (word_i, word_char) in &mut file_char_iter {
        if !is_valid_identifier_char(word_char) {
            return Some((word_i, word_char));
        }
    }
    None // End of file
}

pub fn tokenize<'a>(file_data : &'a str) -> (Vec<LexerPart<'a>>, Vec<ParsingErr>) {
    let mut lexer_result : Vec<LexerPart<'a>> = Vec::new();
    let mut file_char_iter = file_data.char_indices();
    let mut errors : Vec<ParsingErr> = Vec::new();
    loop {
        if let Some((mut char_i, mut cur_char)) = file_char_iter.next() {
            if is_valid_identifier_char(cur_char) {
                // Start of word
                let end_of_identifier =  find_end_of_identifier(&mut file_char_iter);
                let was_end_of_file = end_of_identifier.is_none();
                let word = if let Some((word_end, next_char)) = end_of_identifier {
                    // no looping back the iterator, just continue from non-alphanumeric character
                    let result = file_data.get(char_i..word_end).unwrap();
                    char_i = word_end;
                    cur_char = next_char;
                    result
                } else {
                    file_data.get(char_i..).unwrap()
                };

                let mut word_chars = word.chars();

                let sym_type = if word_chars.next().unwrap().is_digit(10) {
                    // It's a number
                    if word_chars.find(|v| !v.is_digit(10)).is_some() {
                        errors.push(ParsingErr{reason : "Unexpected letter within number", position : word});
                        INVALID_SYMBOL_INDEX
                    } else {
                        NUMBER_SYMBOL_INDEX
                    }
                } else if let Some(keyword_id) = ALL_KEYWORDS.iter().position(|&kw| kw == word) {
                    keyword_id as u8
                } else {
                    IDENTIFIER_SYMBOL_TYPE
                };
                lexer_result.push(LexerPart{typ : sym_type, text : word});

                if was_end_of_file {
                    break;
                }
            } // no else! Continue next character
            if cur_char.is_whitespace() {
                // Whitespace, ignore
                continue;
            } else {
                if file_data.get(char_i..char_i+2) == Some("//") {
                    file_char_iter.next();
					let comment_text = if let Some((comment_i, _)) = file_char_iter.find(|&(_comment_i, comment_char)| comment_char == '\n') {
						file_data.get(char_i..comment_i).unwrap()
					} else {
						file_data.get(char_i..).unwrap()
					};
                    lexer_result.push(LexerPart{typ : COMENT_SYMBOL_INDEX, text : comment_text});
                } else if file_data.get(char_i..char_i+2) == Some("/*") {
                    file_char_iter.next();
                    let comment_text = if let Some(comment_i) = iter_until_comment_end(&mut file_char_iter) {
						file_data.get(char_i..comment_i+1).unwrap()
					} else {
						file_data.get(char_i..).unwrap()
					};
                    lexer_result.push(LexerPart{typ : COMENT_SYMBOL_INDEX, text : comment_text});
                } else if let Some(symbol_id) = ALL_SYMBOLS.iter().position(|&symb| Some(symb) == file_data.get(char_i..char_i+symb.len())) {
                    let symbol_text = file_data.get(char_i..char_i+ALL_SYMBOLS[symbol_id].len()).unwrap();
                    file_char_iter.nth(symbol_text.len() - 1);
                    lexer_result.push(LexerPart{typ : (symbol_id + ALL_KEYWORDS.len()) as u8, text : symbol_text});
                } else { // Symbol not found!
                    errors.push(ParsingErr{reason : "Unexpected character", position : file_data.get(char_i..char_i+1).unwrap()});
                }
            }
        } else {
            break;
        }
    }

    return (lexer_result, errors);
}
