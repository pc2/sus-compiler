use std::ops::Range;
use std::str::CharIndices;

use crate::ast::Span;
use crate::errors::*;
use crate::util::const_str_position_in_tuples;

pub type TokenTypeIdx = u8;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token {
    typ : TokenTypeIdx,
    from : usize,
    to : usize
}

impl Token {
    pub fn new(typ : TokenTypeIdx, range : Range<usize>) -> Self {
        Self{typ, from : range.start, to : range.end}
    }
    pub fn get_type(&self) -> TokenTypeIdx {
        self.typ
    }
    pub fn get_range(&self) -> Range<usize> {
        self.from..self.to
    }
}

pub const ALL_KEYWORDS : [(&'static str, u8); 20] = [
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
    ("while", 0),
    ("for", 0),
    ("in", 0),
    ("out", 0),
    ("struct", 0),
    ("enum", 0),
    ("reg", 0),
    ("finish", 0),
    ("gen", 0),
    ("initial", 0)
];

// Extra data is opreator prescedence. Lower number is higher prescedence of operators
// ordered by which to prefer when parsing
pub const ALL_SYMBOLS : [(&'static str, u8); 35] = [
    // 'Meta symbols', for comments. Not actually used in further parsing
    ("/*", 0),
    ("//", 0),
    ("*/", 0),
    // Big symbols
    ("::", 0),
    ("->", 0),
    ("..", 1),
    ("<=", 2), // Start of operators (see is_operator())
    (">=", 2),
    ("==", 2),
    ("!=", 2),
    ("<<", 4),
    (">>", 4),
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
    ("!", 0),// End of operators (see is_operator()), ! is not a binary operator
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
    (":", 0),
    ("'", 0)
];

pub const TOKEN_IDENTIFIER : TokenTypeIdx = (ALL_KEYWORDS.len() + ALL_SYMBOLS.len()) as TokenTypeIdx;
pub const TOKEN_NUMBER : TokenTypeIdx = TOKEN_IDENTIFIER + 1;
pub const TOKEN_COMMENT : TokenTypeIdx = TOKEN_IDENTIFIER + 2;
pub const TOKEN_INVALID : TokenTypeIdx = TOKEN_IDENTIFIER + 3;

pub const MISC_TOKENS : [&'static str; (TOKEN_INVALID - TOKEN_IDENTIFIER + 1) as usize] = [
    "IDENTIFIER",
    "NUMBER",
    "COMMENT",
    "INVALID"
];

pub const fn kw(name : &str) -> TokenTypeIdx {
    if let Some(found) = const_str_position_in_tuples(name, &ALL_KEYWORDS) {
        found as TokenTypeIdx
    } else if let Some(found) = const_str_position_in_tuples(name, &ALL_SYMBOLS) {
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
    typ == kw("|") || typ == kw("&") || typ == kw("^") || typ == kw("+") || typ == kw("-") || typ == kw("*") || typ == kw("!")
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
    char_iter : CharIndices<'iter>
}

impl<'iter> FileIter<'iter> {
    fn new(text : &'iter str) -> Self {
        Self{char_iter : text.char_indices()}
    }

    // Returns index of last char
    fn iter_until_end_of_identifier<'a>(&mut self, start_char_idx : usize, file_text : &'a str) -> (Range<usize>, Option<(usize, char)>) {
        for (word_i, word_char) in self {
            if !is_valid_identifier_char(word_char) {
                return (start_char_idx..word_i, Some((word_i, word_char)));
            }
        }
        (start_char_idx..file_text.len(), None)
    }

    // Returns number of characters parsed
    fn iter_until_end(&mut self, end : char) -> Option<usize> {
        for (newline_pos, newline_char) in self {
            if newline_char == end {
                return Some(newline_pos);
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
                        return Some(end_pos + '/'.len_utf8());
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
    type Item = (usize, char);
    fn next(&mut self) -> Option<Self::Item> {
        self.char_iter.next()
    }
}

pub fn tokenize<'txt>(file_text : &'txt str, errors : &ErrorCollector) -> Vec<Token> {
    let mut result : Vec<Token> = Vec::new();
    let mut file_char_iter = FileIter::new(file_text);
    
    while let Some((mut file_pos, cur_char)) = file_char_iter.next() {
        if cur_char.is_whitespace() {
            // Whitespace, ignore
            continue;
        }
        if is_valid_identifier_char(cur_char) {
            // Start of word
            let (word, new_cur_char) = file_char_iter.iter_until_end_of_identifier(file_pos, file_text);
            
            let word_str = &file_text[word.clone()];
            let mut word_chars = word_str.chars();
            let tok_typ = if word_chars.next().unwrap().is_digit(10) {
                // It's a number
                if word_chars.find(|v| !v.is_digit(10)).is_some() {
                    errors.error_basic(Span::from(result.len()), "Unexpected letter within number");
                    TOKEN_INVALID
                } else {
                    TOKEN_NUMBER
                }
            } else {
                if let Some(found) = const_str_position_in_tuples(word_str, &ALL_KEYWORDS) {
                    found as TokenTypeIdx
                } else {
                    TOKEN_IDENTIFIER
                }
            };
            result.push(Token::new(tok_typ, word));

            if let Some((next_pos_i, next_char)) = new_cur_char {
                if next_char.is_whitespace() {
                    continue;
                }
                file_pos = next_pos_i;
            } else {
                break;
            }
        } // no else! Continue next character
        
        let char_file_pos = file_pos;
        if let Some(symbol_idx) = ALL_SYMBOLS.iter().position(
            // Have to do .as_bytes here so we don't get the exception that we're cutting a character in half
            |&symb| file_text[char_file_pos..].starts_with(symb.0)
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
                    file_text.len()
                };
                let comment_span = file_pos..end_pos;
                result.push(Token::new(TOKEN_COMMENT, comment_span));

            } else if symbol_tok_id == kw("/*") {
                // Open single multi-line comment
                let end_pos = if let Some(comment_end_idx) = file_char_iter.iter_until_comment_end() {
                    comment_end_idx
                } else {
                    file_text.len()
                };
                let comment_span = file_pos..end_pos;
                result.push(Token::new(TOKEN_COMMENT, comment_span));
                
            } else if symbol_tok_id == kw("*/") {
                // Unexpected close comment
                errors.error_basic(Span::from(result.len()), "Unexpected comment closer when not in comment");
                result.push(Token::new(TOKEN_INVALID, file_pos..file_pos + 2));
            } else {
                result.push(Token::new(symbol_tok_id, file_pos..file_pos + symbol_text.len()));
            }
        } else { // Symbol not found!
            errors.error_basic(Span::from(result.len()), "Unexpected character");
            result.push(Token::new(TOKEN_INVALID, file_pos..file_pos + cur_char.len_utf8()));
        }
    }

    result
}
