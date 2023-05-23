
const ALL_KEYWORDS : [&'static str; 4] = [
    "module",
    "pipeline",
    "state",
    "reg"
];

// ordered by which to prefer
const ALL_SYMBOLS : [&'static str; 30] = [
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

const IDENTIFIER_SYMBOL_TYPE : u8 = (ALL_KEYWORDS.len() + ALL_SYMBOLS.len()) as u8;
const NUMBER_SYMBOL_INDEX : u8 = (IDENTIFIER_SYMBOL_TYPE + 1) as u8;

#[derive(Debug,Clone,PartialEq)]
struct LexerPart<'a> {
    typ : u8,
    text : &'a str,
    attached_comment : Vec<&'a str>
}

impl<'a> LexerPart<'a> {
    fn is_keyword(&self) -> bool {
        self.typ < ALL_KEYWORDS.len() as u8
    }
    fn is_symbol(&self) -> bool {
        self.typ < IDENTIFIER_SYMBOL_TYPE
    }
    fn is_identifier(&self) -> bool {
        self.typ == IDENTIFIER_SYMBOL_TYPE
    }
    fn is_number(&self) -> bool {
        self.typ == NUMBER_SYMBOL_INDEX
    }
}

#[derive(Debug,Clone,PartialEq)]
struct ParsingErr {
    reason : &'static str,
    position : usize
}

fn tokenize<'a>(file_data : &'a str) -> (Vec<LexerPart<'a>>, Vec<ParsingErr>) {
    let mut lexer_result : Vec<LexerPart<'a>> = Vec::new();
    let mut file_char_iter = file_data.char_indices();
    let mut attached_comments : Vec<&'a str> = Vec::new();
    let mut errors : Vec<ParsingErr> = Vec::new();
    loop {
        if let Some((mut char_i, mut cur_char)) = file_char_iter.next() {
            if cur_char.is_digit(10) {
                // Start of number
                for (num_text_i, num_char) in &mut file_char_iter {
                    if !num_char.is_numeric() {
                        if num_char.is_alphabetic() {
                            errors.push(ParsingErr{reason : "Unexpected letter within number", position : char_i});
                        }
                        let number_text = file_data.get(char_i..num_text_i).unwrap();
                        lexer_result.push(LexerPart{typ : NUMBER_SYMBOL_INDEX, text : number_text, attached_comment : attached_comments});
                        attached_comments = Vec::new();

                        // no looping back the iterator, just continue from non-alphanumeric character
                        char_i = num_text_i;
                        cur_char = num_char;
                        break;
                    }
                }
            } else if cur_char.is_alphabetic() {
                // Start of word
                for (word_i, word_char) in &mut file_char_iter {
                    if !word_char.is_alphanumeric() {
                        // end of single line comment
                        let word = file_data.get(char_i..word_i).unwrap();
                        let sym_type = if let Some(keyword_id) = ALL_KEYWORDS.iter().position(|&kw| kw == word) {
                            keyword_id as u8
                        } else {
                            IDENTIFIER_SYMBOL_TYPE
                        };
                        lexer_result.push(LexerPart{typ : sym_type, text : word, attached_comment : attached_comments});
                        attached_comments = Vec::new();

                        // no looping back the iterator, just continue from non-alphanumeric character
                        char_i = word_i;
                        cur_char = word_char;
                        break;
                    }
                }
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
                    let symbol_text = file_data.get(char_i..char_i+ALL_SYMBOLS[symbol_id].len()).unwrap();
                    file_char_iter.nth(symbol_text.len() - 1);
                    lexer_result.push(LexerPart{typ : (symbol_id + ALL_KEYWORDS.len()) as u8, text : symbol_text, attached_comment : attached_comments});
                    attached_comments = Vec::new();
                } else { // Symbol not found!
                    errors.push(ParsingErr{reason : "Unexpected character", position : char_i});
                }
            }
        } else {
            break;
        }
    }

    return (lexer_result, errors);
}

use console::style;

fn pretty_print(file_text : &str) {
    let (token_vec, token_errors) = tokenize(file_text);

    let mut whitespace_start : usize = 0;

    for token in token_vec {
        let whitespace_end = token.text.as_ptr() as usize - file_text.as_ptr() as usize;
        let whitespace_text = file_text.get(whitespace_start..whitespace_end).unwrap();
        whitespace_start = token.text.as_ptr() as usize + token.text.len() - file_text.as_ptr() as usize;
        print!("{}{}", whitespace_text, 
            if token.is_keyword() {
                style(token.text).blue()
            } else if token.is_symbol() {
                style(token.text).cyan()
            } else if token.is_identifier() {
                style(token.text).green()
            } else if token.is_number() {
                style(token.text).yellow()
            } else {
                style(token.text).red().bold()
            }
        );
    }
}

fn main() {
    let file_text = "module test: a, b -> c { a = b + 5; }";

    let (token_vec, token_errors) = tokenize(file_text);

    println!("\n{}\n", file_text);
    //println!("{:?}\n", token_vec);
    /*for tok in token_vec {
        println!("{:?}", tok);
    }*/
    for err in token_errors {
        println!("{}", err.reason);
    }

    pretty_print(file_text);
}
