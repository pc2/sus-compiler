use std::str::CharIndices;

use crate::errors::*;

pub type TokenTypeIdx = u8;

pub const ALL_KEYWORDS : [&'static str; 10] = [
    "module",
    "pipeline",
    "interface",
    "assume",
    "state",
    "if",
    "while",
    "for",
    "struct",
    "enum"
];

// ordered by which to prefer
pub const ALL_SYMBOLS : [&'static str; 29] = [
    // Big symbols
    "<=",
    ">=",
    "==",
    "!=",
    "<<",
    ">>",
    "->",
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
    "(", // Close parens are always 1 larger than their open variant, (see closes())
    ")",
    "{",
    "}",
    "[",
    "]",
    ",",
    ";",
    ":",
    "@"
];

pub const MISC_TOKENS : [&'static str; 3] = [
    "IDENTIFIER",
    "NUMBER",
    "INVALID"
];

pub const TOKEN_IDENTIFIER : TokenTypeIdx = (ALL_KEYWORDS.len() + ALL_SYMBOLS.len()) as TokenTypeIdx;
pub const TOKEN_NUMBER : TokenTypeIdx = TOKEN_IDENTIFIER + 1;
pub const TOKEN_INVALID : TokenTypeIdx = TOKEN_IDENTIFIER + 2;

const fn const_eq_str(a: &str, b: &str) -> bool {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    if a_bytes.len() != b_bytes.len() {
        return false;
    }

    let mut i: usize = 0;
    while i < a_bytes.len() {
        if a_bytes[i] != b_bytes[i] {
            return false;
        }
        i += 1;
    }

    true
}

const fn const_str_position(v : &str, list : &[&str]) -> Option<usize> {
    let mut i : usize = 0;

    while i < list.len() {
        if const_eq_str(v, list[i]) {
            return Some(i);
        }
        i += 1;
    }
    None
}

pub const fn kw(name : &str) -> TokenTypeIdx {
    if let Some(found) = const_str_position(name, &ALL_KEYWORDS) {
        found as TokenTypeIdx
    } else if let Some(found) = const_str_position(name, &ALL_SYMBOLS) {
        (found + ALL_KEYWORDS.len()) as TokenTypeIdx
    } else {
        unreachable!();
    }
}

pub fn is_keyword(typ : TokenTypeIdx) -> bool {
    typ < ALL_KEYWORDS.len() as TokenTypeIdx
}
pub fn is_symbol(typ : TokenTypeIdx) -> bool {
    typ < TOKEN_IDENTIFIER
}
pub fn is_identifier(typ : TokenTypeIdx) -> bool {
    typ == TOKEN_IDENTIFIER
}
pub fn is_number(typ : TokenTypeIdx) -> bool {
    typ == TOKEN_NUMBER
}
pub fn get_token_type_name(typ : TokenTypeIdx) -> &'static str {
    if is_keyword(typ) {
        ALL_KEYWORDS[typ as usize]
    } else if is_symbol(typ) {
        ALL_SYMBOLS[typ as usize - ALL_KEYWORDS.len()]
    } else {
        MISC_TOKENS[typ as usize - ALL_KEYWORDS.len() - ALL_SYMBOLS.len()]
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum IsBracket {
    Open,
    Close,
    NotABracket
}
pub fn is_bracket(typ : TokenTypeIdx) -> IsBracket {
    if typ == kw("(") || typ == kw("{") || typ == kw("[") {
        IsBracket::Open
    } else if typ == kw(")") || typ == kw("}") || typ == kw("]") {
        IsBracket::Close
    } else {
        IsBracket::NotABracket
    }
}
pub fn closes(open : TokenTypeIdx, close : TokenTypeIdx) -> bool {
    assert!(is_bracket(open) == IsBracket::Open, "Open is not an open paren!");
    assert!(is_bracket(close) == IsBracket::Close, "Close is not a close paren!");

    close == open + 1
}

#[derive(Debug,Clone)]
pub struct Token<'a> {
    pub typ : TokenTypeIdx,
    pub text : &'a str
}

#[derive(Debug,Clone)]
pub struct CommentToken<'a> {
    pub text : &'a str,
    pub token_idx : usize
}

fn is_valid_identifier_char(c : char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn iter_until_comment_end(mut file_char_iter : &mut CharIndices) -> Option<usize> {
	while let Some((_, comment_char)) = file_char_iter.next() {
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
    }
    return None
}

fn find_end_of_identifier(mut file_char_iter : &mut CharIndices) -> Option<(usize, char)> {
    for (word_i, word_char) in &mut file_char_iter {
        if !is_valid_identifier_char(word_char) {
            return Some((word_i, word_char));
        }
    }
    None // End of file
}

pub fn tokenize<'a>(file_data : &'a str) -> (Vec<Token<'a>>, Vec<CommentToken<'a>>, Vec<ParsingError<'a>>) {
    let mut tokens : Vec<Token<'a>> = Vec::new();
    let mut file_char_iter = file_data.char_indices();
    let mut errors : Vec<ParsingError<'a>> = Vec::new();
    let mut comments : Vec<CommentToken<'a>> = Vec::new();
    while let Some((mut char_i, mut cur_char)) = file_char_iter.next() {
        if is_valid_identifier_char(cur_char) {
            // Start of word
            let end_of_identifier = find_end_of_identifier(&mut file_char_iter);
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
                    errors.push(error_basic_str(word, "Unexpected letter within number"));
                    TOKEN_INVALID
                } else {
                    TOKEN_NUMBER
                }
            } else if let Some(keyword_id) = ALL_KEYWORDS.iter().position(|&kw| kw == word) {
                keyword_id as TokenTypeIdx
            } else {
                TOKEN_IDENTIFIER
            };
            tokens.push(Token{typ : sym_type, text : word});

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
                comments.push(CommentToken{text : comment_text, token_idx : tokens.len()});
            } else if file_data.get(char_i..char_i+2) == Some("/*") {
                file_char_iter.next();
                let comment_text = if let Some(comment_i) = iter_until_comment_end(&mut file_char_iter) {
                    file_data.get(char_i..comment_i+1).unwrap()
                } else {
                    file_data.get(char_i..).unwrap()
                };
                comments.push(CommentToken{text : comment_text, token_idx : tokens.len()});
            } else if let Some(symbol_id) = ALL_SYMBOLS.iter().position(|&symb| Some(symb) == file_data.get(char_i..char_i+symb.len())) {
                let symbol_text = file_data.get(char_i..char_i+ALL_SYMBOLS[symbol_id].len()).unwrap();
                file_char_iter.nth(symbol_text.len() - 1);
                tokens.push(Token{typ : (symbol_id + ALL_KEYWORDS.len()) as TokenTypeIdx, text : symbol_text});
            } else { // Symbol not found!
                errors.push(error_basic_str(file_data.get(char_i..char_i+1).unwrap(), "Unexpected character"));
            }
        }
    }

    return (tokens, comments, errors);
}
