use std::str::CharIndices;

use crate::errors::*;

pub type TokenTypeIdx = u8;

use crate::ast::FilePos;
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

pub fn tokenize(file_data : &str) -> (Vec<TokenTypeIdx>, Vec<CharSpan>, Vec<ParsingError<CharSpan>>) {
    let mut token_spans : Vec<CharSpan> = Vec::new();
    let mut token_types : Vec<TokenTypeIdx> = Vec::new();
    let mut file_char_iter = FileIter::new(file_data);
    let mut errors : Vec<ParsingError<CharSpan>> = Vec::new();
    
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
                token_spans.push(CharSpan{file_pos, length: comment_length + 2}); // Add 2 because comment starts with "//"
                token_types.push(TOKEN_COMMENT);

            } else if symbol_tok_id == kw("/*") {
                // Open single multi-line comment
                let comment_length = file_char_iter.iter_until_comment_end();
                token_spans.push(CharSpan{file_pos, length: comment_length+2}); // Add 2 because comment starts with "/*"
                token_types.push(TOKEN_COMMENT);
            } else if symbol_tok_id == kw("*/") {
                // Unexpected close comment
                errors.push(error_basic_str(CharSpan{file_pos, length: 2}, "Unexpected comment closer when not in comment"));
            } else {
                let symbol_text_span = CharSpan{file_pos, length: ALL_SYMBOLS[symbol_idx].0.len()};
                
                token_types.push(symbol_tok_id);
                token_spans.push(symbol_text_span);
            }
        } else { // Symbol not found!
            errors.push(error_basic_str(CharSpan{file_pos, length: 1}, "Unexpected character"));
        }
    }

    return (token_types, token_spans, errors);
}
