use std::str::CharIndices;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::hash_map::*;
use std::borrow::Cow;

use crate::errors::*;

use num_bigint::*;

use core::convert::TryFrom;

pub type TokenTypeIdx = u8;
pub type TokenExtraInfo = u64;
const NO_TOKEN_INFO : TokenExtraInfo = 0;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token {
    typ : TokenTypeIdx,
    info : TokenExtraInfo
}

impl Token {
    pub fn get_type(&self) -> TokenTypeIdx {
        self.typ
    }
    pub fn get_info(&self) -> TokenExtraInfo {
        self.info
    }
}


use crate::ast::FilePos;
use crate::ast::CharSpan;


pub const ALL_KEYWORDS : [(&'static str, u8); 16] = [
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
    ("enum", 0)
];

// Extra data is opreator prescedence. Lower number is higher prescedence of operators
// ordered by which to prefer when parsing
pub const ALL_SYMBOLS : [(&'static str, u8); 34] = [
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
    typ >= kw("<=") && typ <= kw("@")
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
    row : usize,
    col_starts_at : usize
}

impl<'iter> FileIter<'iter> {
    fn new(text : &'iter str) -> Self {
        Self{char_iter : text.char_indices(), row : 0, col_starts_at : 0}
    }

    // Returns number of parsed chars
    fn iter_until_end_of_identifier(&mut self) -> (usize, Option<(FilePos, char)>) {
        let mut parsed_chars = 0; // already include the first parsed character
        for (word_i, word_char) in self {
            if !is_valid_identifier_char(word_char) {
                return (parsed_chars, Some((word_i, word_char)));
            }
            parsed_chars += 1;
        }
        (parsed_chars, None)
    }

    // Returns number of characters parsed
    fn iter_until_end(&mut self, end : char) -> usize {
        let mut parsed_chars = 0; // already include the first parsed character
        for (_, word_char) in self {
            parsed_chars += 1;
            if word_char == end {
                return parsed_chars;
            }
        }
        parsed_chars
    }

    // Returns number of characters parsed
    fn iter_until_comment_end(&mut self) -> usize {
        let mut parsed_chars = 0;
        while let Some((_, comment_char)) = self.next() {
            parsed_chars += 1;
            if comment_char == '*' {
                // end of single line comment
                while let Some((_, comment_char_2)) = self.next() {
                    parsed_chars += 1;
                    if comment_char_2 == '/' {
                        // End of comment
                        return parsed_chars;
                    } else if comment_char_2 == '*' {
                        continue;
                    } else {
                        break;
                    }
                }
            }
        }
        return parsed_chars
    }
}

impl<'iter> Iterator for FileIter<'iter> {
    type Item = (FilePos, char);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((pos, ch)) = self.char_iter.next() {
            let cur_pos = FilePos{char_idx : pos, row : self.row, col : pos - self.col_starts_at};
            if ch == '\n' {
                self.row += 1;
                self.col_starts_at = pos+1;
            }
            Some((cur_pos, ch))
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
pub struct TokenizerResult<'a> {
    pub tokens : Vec<Token>,
    pub token_spans : Vec<CharSpan>,
    pub unique_identifiers : Vec<&'a str>,
    pub numbers : Vec<BigUint>
}

const SMALL_NUMBER_CUTOFF : TokenExtraInfo = 1 << (TokenExtraInfo::BITS - 1);
const KEYWORD_CUTOFF : TokenExtraInfo = TokenExtraInfo::MAX - ALL_KEYWORDS.len() as TokenExtraInfo;

impl<'a> TokenizerResult<'a> {
    pub fn get_number<'s>(&'s self, info : TokenExtraInfo) -> Cow<'s, BigUint> {
        if info < SMALL_NUMBER_CUTOFF {
            Cow::Owned(info.to_biguint().unwrap())
        } else {
            Cow::Borrowed(&self.numbers[(info - SMALL_NUMBER_CUTOFF) as usize])
        }
    }
}

impl<'a> TokenizerResult<'a> {
    pub fn push(&mut self, typ : TokenTypeIdx, span : CharSpan) {
        self.tokens.push(Token{typ, info : NO_TOKEN_INFO});
        self.token_spans.push(span);
    }
    pub fn push_word(&mut self, id : &'a str, span : CharSpan, map : &mut HashMap<&'a str, TokenExtraInfo>) {
        let entry = map.entry(id);
        match entry {
            Entry::Occupied(occ) => {
                let info = *occ.get();
                if info >= KEYWORD_CUTOFF {
                    self.tokens.push(Token{typ : (info - KEYWORD_CUTOFF) as TokenTypeIdx, info : NO_TOKEN_INFO});
                } else {
                    self.tokens.push(Token{typ : TOKEN_IDENTIFIER, info});
                }
            },
            Entry::Vacant(vacant) => {
                let info = self.unique_identifiers.len() as TokenExtraInfo;
                vacant.insert(info);
                self.unique_identifiers.push(id);
                self.tokens.push(Token{typ : TOKEN_IDENTIFIER, info});
            }
        }
        self.token_spans.push(span);
    }
    pub fn push_number(&mut self, num_text : &str, span : CharSpan) {
        let num_bigint = BigUint::from_str(num_text).unwrap();
        let info = if let Ok(as_small_int) = TokenExtraInfo::try_from(&num_bigint) {
            if as_small_int < SMALL_NUMBER_CUTOFF {
                as_small_int
            } else {
                let num_idx = self.numbers.len();
                self.numbers.push(num_bigint);
                num_idx as TokenExtraInfo
            }
        } else {
            let num_idx = self.numbers.len();
            self.numbers.push(num_bigint);
            num_idx as TokenExtraInfo
        };
        self.tokens.push(Token{typ : TOKEN_NUMBER, info : info as TokenExtraInfo});
        self.token_spans.push(span);
    }
    pub fn len(&self) -> usize {
        self.tokens.len()
    }
    fn init_unique_id_map(&self) -> HashMap<&'a str, TokenExtraInfo> {
        let mut map : HashMap<&'a str, TokenExtraInfo> = HashMap::new();

        for (idx, id) in ALL_KEYWORDS.iter().enumerate() {
            map.insert(id.0, idx as TokenExtraInfo + KEYWORD_CUTOFF);
        }

        map
    }
}

pub fn tokenize<'txt>(file_data : &'txt str) -> (TokenizerResult<'txt>, Vec<ParsingError<CharSpan>>) {
    let mut result : TokenizerResult<'txt> = Default::default();
    let mut file_char_iter = FileIter::new(file_data);
    let mut errors : Vec<ParsingError<CharSpan>> = Vec::new();
    let mut unique_id_map : HashMap<&'txt str, TokenExtraInfo> = result.init_unique_id_map();
    
    while let Some((mut file_pos, cur_char)) = file_char_iter.next() {
        if cur_char.is_whitespace() {
            // Whitespace, ignore
            continue;
        }
        if is_valid_identifier_char(cur_char) {
            // Start of word
            let (num_chars_parsed, new_cur_char) = file_char_iter.iter_until_end_of_identifier();
            let was_end_of_file = new_cur_char.is_none();
            let word_span = CharSpan{file_pos, length: num_chars_parsed+1}; // Already parsed the first char beforehand
            
            let word = file_data.get(word_span.as_range()).unwrap();
            let mut word_chars = word.chars();

            if word_chars.next().unwrap().is_digit(10) {
                // It's a number
                if word_chars.find(|v| !v.is_digit(10)).is_some() {
                    errors.push(error_basic_str(word_span, "Unexpected letter within number"));
                    result.push(TOKEN_INVALID, word_span);
                } else {
                    result.push_number(word, word_span);
                }
            } else {
                result.push_word(word, word_span, &mut unique_id_map);
            };

            if was_end_of_file {
                break;
            }

            if let Some((next_pos_i, next_char)) = new_cur_char {
                if next_char.is_whitespace() {
                    continue;
                }
                file_pos = next_pos_i;
            }
        } // no else! Continue next character
        
        let char_file_pos = file_pos.char_idx;
        if let Some(symbol_idx) = ALL_SYMBOLS.iter().position(|&symb| Some(symb.0) == file_data.get(char_file_pos..char_file_pos+symb.0.len())) {
            if ALL_SYMBOLS[symbol_idx].0.len() > 1 {
                file_char_iter.nth(ALL_SYMBOLS[symbol_idx].0.len() - 2); // Advance iterator properly
            }
            let symbol_tok_id = (symbol_idx + ALL_KEYWORDS.len()) as TokenTypeIdx;
            if symbol_tok_id == kw("//") {
                // Open single line comment
                let comment_length = file_char_iter.iter_until_end('\n');
                result.push(TOKEN_COMMENT, CharSpan{file_pos, length: comment_length + 2});// Add 2 because comment starts with "//"

            } else if symbol_tok_id == kw("/*") {
                // Open single multi-line comment
                let comment_length = file_char_iter.iter_until_comment_end();
                result.push(TOKEN_COMMENT, CharSpan{file_pos, length: comment_length+2}); // Add 2 because comment starts with "/*"
                
            } else if symbol_tok_id == kw("*/") {
                // Unexpected close comment
                errors.push(error_basic_str(CharSpan{file_pos, length: 2}, "Unexpected comment closer when not in comment"));
            } else {
                let symbol_text_span = CharSpan{file_pos, length: ALL_SYMBOLS[symbol_idx].0.len()};
                
                result.push(symbol_tok_id, symbol_text_span);
            }
        } else { // Symbol not found!
            errors.push(error_basic_str(CharSpan{file_pos, length: 1}, "Unexpected character"));
        }
    }

    (result, errors)
}
