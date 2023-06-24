use std::str::CharIndices;

use crate::errors::*;

pub type TokenTypeIdx = u8;

use crate::ast::CharSpan;

pub const ALL_KEYWORDS : [(&'static str, u8); 12] = [
    ("module", 0),
    ("pipeline", 0),
    ("interface", 0),
    ("timeline", 0),
    ("loop", 0),
    ("assume", 0),
    ("state", 0),
    ("if", 0),
    ("while", 0),
    ("for", 0),
    ("struct", 0),
    ("enum", 0)
];

// Extra data is opreator prescedence. Lower number is higher prescedence of operators
// ordered by which to prefer when parsing
pub const ALL_SYMBOLS : [(&'static str, u8); 33] = [
    // 'Meta symbols', for comments. Not actually used in further parsing
    ("/*", 0),
    ("//", 0),
    ("*/", 0),
    // Big symbols
    ("->", 0),
    ("<=", 1), // Start of operators (see is_operator())
    (">=", 1),
    ("==", 1),
    ("!=", 1),
    ("<<", 3),
    (">>", 3),
    // small Symbols
    ("+", 5),
    ("-", 5),
    ("*", 4),
    ("/", 4),
    ("%", 4),
    ("&", 2),
    ("|", 2),
    ("^", 2),
    ("<", 1),
    (">", 1),
    ("!", 0),
    ("@", 0), // End of operators (see is_operator())
    ("#", 0),
    ("=", 0),
    ("(", 0), // Close parens are always 1 larger than their open variant, (see closes())
    (")", 0),
    ("{", 0),
    ("}", 0),
    ("[", 0),
    ("]", 0),
    (",", 0),
    (";", 0),
    (":", 0)
];

pub const MISC_TOKENS : [&'static str; 4] = [
    "IDENTIFIER",
    "NUMBER",
    "COMMENT",
    "INVALID"
];

pub const TOKEN_IDENTIFIER : TokenTypeIdx = (ALL_KEYWORDS.len() + ALL_SYMBOLS.len()) as TokenTypeIdx;
pub const TOKEN_NUMBER : TokenTypeIdx = TOKEN_IDENTIFIER + 1;
pub const TOKEN_COMMENT : TokenTypeIdx = TOKEN_IDENTIFIER + 2;
pub const TOKEN_INVALID : TokenTypeIdx = TOKEN_IDENTIFIER + 3;

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

const fn const_str_position(v : &str, list : &[(&str, u8)]) -> Option<usize> {
    let mut i : usize = 0;

    while i < list.len() {
        if const_eq_str(v, list[i].0) {
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
        panic!();
    }
}

pub fn is_keyword(typ : TokenTypeIdx) -> bool {
    typ < ALL_KEYWORDS.len() as TokenTypeIdx
}
pub fn is_symbol(typ : TokenTypeIdx) -> bool {
    typ < TOKEN_IDENTIFIER
}
pub fn is_operator(typ : TokenTypeIdx) -> bool {
    typ >= kw("<=") && typ <= kw("@")
}
pub fn is_identifier(typ : TokenTypeIdx) -> bool {
    typ == TOKEN_IDENTIFIER
}
pub fn is_number(typ : TokenTypeIdx) -> bool {
    typ == TOKEN_NUMBER
}
pub fn is_comment(typ : TokenTypeIdx) -> bool {
    typ == TOKEN_COMMENT
}
pub fn get_token_type_name(typ : TokenTypeIdx) -> &'static str {
    if is_keyword(typ) {
        ALL_KEYWORDS[typ as usize].0
    } else if is_symbol(typ) {
        ALL_SYMBOLS[typ as usize - ALL_KEYWORDS.len()].0
    } else {
        MISC_TOKENS[typ as usize - ALL_KEYWORDS.len() - ALL_SYMBOLS.len()]
    }
}
pub fn get_binary_operator_prescedence(typ : TokenTypeIdx) -> u8 {
    let result = ALL_SYMBOLS[typ as usize - ALL_KEYWORDS.len()].1;
    assert!(result != 0);
    result
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

pub fn tokenize(file_data : &str) -> (Vec<TokenTypeIdx>, Vec<CharSpan>, Vec<ParsingError<CharSpan>>) {
    let mut token_spans : Vec<CharSpan> = Vec::new();
    let mut token_types : Vec<TokenTypeIdx> = Vec::new();
    let mut file_char_iter = file_data.char_indices();
    let mut errors : Vec<ParsingError<CharSpan>> = Vec::new();
    let file_length = file_data.len();
    
    while let Some((mut char_i, mut cur_char)) = file_char_iter.next() {
        if is_valid_identifier_char(cur_char) {
            // Start of word
            let end_of_identifier = find_end_of_identifier(&mut file_char_iter);
            let was_end_of_file = end_of_identifier.is_none();
            let word_span = if let Some((word_end, next_char)) = end_of_identifier {
                // no looping back the iterator, just continue from non-alphanumeric character
                let result = CharSpan(char_i,word_end);
                char_i = word_end;
                cur_char = next_char;
                result
            } else {
                CharSpan(char_i,file_length)
            };

            let word = file_data.get(word_span.as_range()).unwrap();
            let mut word_chars = word.chars();

            let sym_type = if word_chars.next().unwrap().is_digit(10) {
                // It's a number
                if word_chars.find(|v| !v.is_digit(10)).is_some() {
                    errors.push(error_basic_str(word_span, "Unexpected letter within number"));
                    TOKEN_INVALID
                } else {
                    TOKEN_NUMBER
                }
            } else if let Some(keyword_id) = const_str_position(word, &ALL_KEYWORDS) {
                keyword_id as TokenTypeIdx
            } else {
                TOKEN_IDENTIFIER
            };
            token_types.push(sym_type);
            token_spans.push(word_span);

            if was_end_of_file {
                break;
            }
        } // no else! Continue next character
        if cur_char.is_whitespace() {
            // Whitespace, ignore
            continue;
        } else {
            if let Some(symbol_idx) = ALL_SYMBOLS.iter().position(|&symb| Some(symb.0) == file_data.get(char_i..char_i+symb.0.len())) {
                let symbol_tok_id = (symbol_idx + ALL_KEYWORDS.len()) as TokenTypeIdx;
                if symbol_tok_id == kw("//") {
                    // Open single line comment
                    file_char_iter.next();
                    let comment_text_span = if let Some((comment_i, _)) = file_char_iter.find(|&(_comment_i, comment_char)| comment_char == '\n') {
                        CharSpan(char_i,comment_i)
                    } else {
                        CharSpan(char_i,file_length)
                    };
                    token_spans.push(comment_text_span);
                    token_types.push(TOKEN_COMMENT);

                } else if symbol_tok_id == kw("/*") {
                    // Open single multi-line comment
                    file_char_iter.next();
                    let comment_text_span = if let Some(comment_i) = iter_until_comment_end(&mut file_char_iter) {
                        CharSpan(char_i,comment_i+1)
                    } else {
                        CharSpan(char_i,file_length)
                    };
                    token_spans.push(comment_text_span);
                    token_types.push(TOKEN_COMMENT);
                } else if symbol_tok_id == kw("*/") {
                    // Unexpected close comment
                    errors.push(error_basic_str(CharSpan(char_i,char_i+2), "Unexpected close comment"));
                    file_char_iter.next(); // symbol is 2 chars large, so one additional skip is needed
                } else {
                    let symbol_text_span = CharSpan(char_i, char_i+ALL_SYMBOLS[symbol_idx].0.len());
                    if symbol_text_span.len() > 1 {
                        file_char_iter.nth(symbol_text_span.len() - 2);
                    }
                    token_types.push(symbol_tok_id);
                    token_spans.push(symbol_text_span);
                }
            } else { // Symbol not found!
                errors.push(error_basic_str(CharSpan(char_i,char_i+1), "Unexpected character"));
            }
        }
    }

    return (token_types, token_spans, errors);
}
