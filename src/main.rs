

struct LexerPart<'a> {
    typ : u8,
    text : &'a str,
    attached_comment : Vec<&'a str>
}

struct ParsingErr {
    reason : &'static str,
    position : usize
}

static ALL_KEYWORDS : &'static [&'static str] = &[
    "module",
    "pipeline",
    "state",
    "reg"
];

// ordered by which to prefer
static ALL_SYMBOLS : &'static [&'static str] = &[
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

static WORD_INDEX : u8 = (ALL_KEYWORDS.len() + ALL_SYMBOLS.len()) as u8;

fn lexer<'a>(file_data : &'a str) -> Result<Vec<LexerPart<'a>>, ParsingErr> {
    let mut lexer_result : Vec<LexerPart<'a>> = Vec::new();
    let mut word_start : usize = usize::MAX;
    let mut file_char_iter = file_data.char_indices();
    let mut attached_comments : Vec<&'a str> = Vec::new();
    loop {
        if let Some((char_i, cur_char)) = file_char_iter.next() {
            if cur_char.is_alphanumeric() {
                // Start of word
                if word_start == usize::MAX {
                    word_start = char_i;
                }
            } else {
                if word_start != usize::MAX {
                    // End of word
                    let word = file_data.get(word_start..char_i).unwrap();

                    let sym_type = if let Some(keyword_id) = ALL_KEYWORDS.iter().position(|&kw| kw == word) {
                        keyword_id as u8
                    } else {
                        WORD_INDEX
                    };
                    lexer_result.push(LexerPart{typ : sym_type, text : word, attached_comment : attached_comments});
                    attached_comments = Vec::new();
                }
                if cur_char.is_whitespace() {
                    // Whitespace, ignore
                    continue;
                } else {
                    if file_data.get(char_i..char_i+2) == Some("//") {
                        file_char_iter.next();
                        for (comment_i, comment_char) in &mut file_char_iter {
                            if comment_char == '\n' {
                                // end of single line comment
                                attached_comments.push(file_data.get(char_i..comment_i).unwrap());
                                break;
                            }
                        }
                    } else if let Some(symbol_id) = ALL_SYMBOLS.iter().position(|&symb| Some(symb) == file_data.get(char_i..char_i+symb.len())) {
                        file_char_iter.nth(ALL_SYMBOLS[symbol_id].len() - 1);

                    } else { // Symbol not found!
                        return Err(ParsingErr{reason : "Unexpected character", position : char_i});
                    }
                }
            }
        } else {
            break;
        }
    }

    return Ok(lexer_result);
}

fn main() {

}
