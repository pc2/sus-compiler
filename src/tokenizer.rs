use std::ops::Range;
use std::str::CharIndices;

use crate::ast::{RowCol, Span};
use crate::errors::*;

pub type TokenTypeIdx = u8;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token {
    typ : TokenTypeIdx,
    from : usize,
    to : usize
}

impl Token {
    pub fn get_type(&self) -> TokenTypeIdx {
        self.typ
    }
    pub fn get_range(&self) -> Range<usize> {
        self.from..self.to
    }
}


use crate::ast::FilePos;


pub const ALL_KEYWORDS : [(&'static str, u8); 17] = [
    ("template", 0),
    ("module", 0),
    ("pipeline", 0),
    ("interface", 0),
    ("timeline", 0),
    ("loop", 0),
    ("assume", 0),
    ("state", 0),
    ("if", 0),
    ("else", 0),
    ("true", 0),
    ("false", 0),
    ("while", 0),
    ("for", 0),
    ("struct", 0),
    ("enum", 0),
    ("reg", 0)
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
    ("<=", 2), // Start of operators (see is_operator())
    (">=", 2),
    ("==", 2),
    ("!=", 2),
    ("<<", 4),
    (">>", 4),
    ("..", 1),
    // small Symbols
    ("+", 6),
    ("-", 6),
    ("*", 5),
    ("/", 5),
    ("%", 5),
    ("&", 3),
    ("|", 3),
    ("^", 3),
    ("<", 2),
    (">", 2),
    ("!", 0),// End of operators (see is_operator())
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

pub const MISC_TOKENS : [&'static str; 5] = [
    "IDENTIFIER",
    "NUMBER",
    "BIG_INTEGER",
    "COMMENT",
    "INVALID"
];

pub const TOKEN_IDENTIFIER : TokenTypeIdx = (ALL_KEYWORDS.len() + ALL_SYMBOLS.len()) as TokenTypeIdx;
pub const TOKEN_NUMBER : TokenTypeIdx = TOKEN_IDENTIFIER + 1;
pub const TOKEN_COMMENT : TokenTypeIdx = TOKEN_IDENTIFIER + 2;
pub const TOKEN_INVALID : TokenTypeIdx = TOKEN_IDENTIFIER + 3;

pub const fn const_eq_str(a: &str, b: &str) -> bool {
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
    typ >= kw("<=") && typ <= kw("!")
}
pub fn is_unary_operator(typ : TokenTypeIdx) -> bool {
    typ == kw("|") || typ == kw("&") || typ == kw("^") || typ == kw("+") || typ == kw("*") || typ == kw("!")
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

struct FileIter<'iter> {
    char_iter : CharIndices<'iter>,
    row_col : RowCol
}

impl<'iter> FileIter<'iter> {
    fn new(text : &'iter str) -> Self {
        Self{char_iter : text.char_indices(), row_col : RowCol{row : 0, col : 0}}
    }

    // Returns index of last char
    fn iter_until_end_of_identifier<'a>(&mut self, start_char_idx : usize, file_text : &'a str) -> (Range<usize>, Option<(FilePos, char)>) {
        for (word_i, word_char) in self {
            if !is_valid_identifier_char(word_char) {
                return (start_char_idx..word_i.char_idx, Some((word_i, word_char)));
            }
        }
        (start_char_idx..file_text.len(), None)
    }

    // Returns number of characters parsed
    fn iter_until_end(&mut self, end : char) -> Option<usize> {
        for (newline_pos, newline_char) in self {
            if newline_char == end {
                return Some(newline_pos.char_idx);
            }
        }
        None
    }

    // Returns number of characters parsed
    fn iter_until_comment_end(&mut self) -> Option<usize> {
        while let Some((_, comment_char)) = self.next() {
            if comment_char == '*' {
                // end of single line comment
                while let Some((end_pos, comment_char_2)) = self.next() {
                    if comment_char_2 == '/' {
                        // End of comment
                        return Some(end_pos.char_idx + '/'.len_utf8());
                    } else if comment_char_2 == '*' {
                        continue;
                    } else {
                        break;
                    }
                }
            }
        }
        None
    }
}

impl<'iter> Iterator for FileIter<'iter> {
    type Item = (FilePos, char);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((pos, ch)) = self.char_iter.next() {
            let cur_pos = FilePos{char_idx : pos, row_col : self.row_col};
            self.row_col.advance_char(ch);
            Some((cur_pos, ch))
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
pub struct TokenizerResult {
    pub tokens : Vec<Token>,
    pub token_row_cols : Vec<RowCol>
}

impl TokenizerResult {
    pub fn push(&mut self, typ : TokenTypeIdx, span : Range<usize>, row_col : RowCol) {
        self.tokens.push(Token{typ, from : span.start, to : span.end});
        self.token_row_cols.push(row_col);
    }
    pub fn len(&self) -> usize {
        self.tokens.len()
    }
}

pub fn tokenize<'txt>(file_data : &'txt str) -> (TokenizerResult, Vec<ParsingError<Span>>) {
    let mut result : TokenizerResult = Default::default();
    let mut file_char_iter = FileIter::new(file_data);
    let mut errors : Vec<ParsingError<Span>> = Vec::new();
    
    while let Some((mut file_pos, cur_char)) = file_char_iter.next() {
        if cur_char.is_whitespace() {
            // Whitespace, ignore
            continue;
        }
        if is_valid_identifier_char(cur_char) {
            // Start of word
            let (word, new_cur_char) = file_char_iter.iter_until_end_of_identifier(file_pos.char_idx, file_data);
            
            let word_str = &file_data[word.clone()];
            let mut word_chars = word_str.chars();
            let tok_typ = if word_chars.next().unwrap().is_digit(10) {
                // It's a number
                if word_chars.find(|v| !v.is_digit(10)).is_some() {
                    errors.push(error_basic(Span::from(result.len()), "Unexpected letter within number"));
                    TOKEN_INVALID
                } else {
                    TOKEN_NUMBER
                }
            } else {
                if let Some(found) = const_str_position(word_str, &ALL_KEYWORDS) {
                    found as TokenTypeIdx
                } else {
                    TOKEN_IDENTIFIER
                }
            };
            result.push(tok_typ, word, file_pos.row_col);

            if let Some((next_pos_i, next_char)) = new_cur_char {
                if next_char.is_whitespace() {
                    continue;
                }
                file_pos = next_pos_i;
            } else {
                break;
            }
        } // no else! Continue next character
        
        let char_file_pos = file_pos.char_idx;
        if let Some(symbol_idx) = ALL_SYMBOLS.iter().position(
            // Have to do .as_bytes here so we don't get the exception that we're cutting a character in half
            |&symb| *symb.0.as_bytes() == file_data.as_bytes()[char_file_pos..char_file_pos+symb.0.len()]
        ) {
            let symbol_text : &'static str = ALL_SYMBOLS[symbol_idx].0;
            if symbol_text.len() > 1 {
                file_char_iter.nth(symbol_text.len() - 2); // Advance iterator properly
            }
            let symbol_tok_id = (symbol_idx + ALL_KEYWORDS.len()) as TokenTypeIdx;
            if symbol_tok_id == kw("//") {
                // Open single line comment
                let end_pos = if let Some(comment_end_idx) = file_char_iter.iter_until_end('\n') {
                    comment_end_idx
                } else {
                    file_data.len()
                };
                let comment_span = file_pos.char_idx..end_pos;
                result.push(TOKEN_COMMENT, comment_span, file_pos.row_col);

            } else if symbol_tok_id == kw("/*") {
                // Open single multi-line comment
                let end_pos = if let Some(comment_end_idx) = file_char_iter.iter_until_comment_end() {
                    comment_end_idx
                } else {
                    file_data.len()
                };
                let comment_span = file_pos.char_idx..end_pos;
                result.push(TOKEN_COMMENT, comment_span, file_pos.row_col);
                
            } else if symbol_tok_id == kw("*/") {
                // Unexpected close comment
                errors.push(error_basic(Span::from(result.len()), "Unexpected comment closer when not in comment"));
                result.push(TOKEN_INVALID, file_pos.char_idx..file_pos.char_idx + 2, file_pos.row_col);
            } else {
                result.push(symbol_tok_id, file_pos.char_idx..file_pos.char_idx + 2, file_pos.row_col);
            }
        } else { // Symbol not found!
            errors.push(error_basic(Span::from(result.len()), "Unexpected character"));
            result.push(TOKEN_INVALID, file_pos.char_idx..file_pos.char_idx + cur_char.len_utf8(), file_pos.row_col);
        }
    }

    (result, errors)
}
