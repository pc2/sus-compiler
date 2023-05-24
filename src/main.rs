
const ALL_KEYWORDS : [&'static str; 6] = [
    "module",
    "pipeline",
    "state",
    "if",
    "while",
    "for"
];

// ordered by which to prefer
const ALL_SYMBOLS : [&'static str; 31] = [
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

const IDENTIFIER_SYMBOL_TYPE : u8 = (ALL_KEYWORDS.len() + ALL_SYMBOLS.len()) as u8;
const NUMBER_SYMBOL_INDEX : u8 = IDENTIFIER_SYMBOL_TYPE + 1;
const INVALID_SYMBOL_INDEX : u8 = NUMBER_SYMBOL_INDEX + 1;

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
struct ParsingErr<'a> {
    reason : &'static str,
    position : &'a str
}

fn is_valid_identifier_char(c : char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn tokenize<'a>(file_data : &'a str) -> (Vec<LexerPart<'a>>, Vec<ParsingErr>) {
    let mut lexer_result : Vec<LexerPart<'a>> = Vec::new();
    let mut file_char_iter = file_data.char_indices();
    let mut attached_comments : Vec<&'a str> = Vec::new();
    let mut errors : Vec<ParsingErr> = Vec::new();
    loop {
        if let Some((mut char_i, mut cur_char)) = file_char_iter.next() {
            if is_valid_identifier_char(cur_char) {
                // Start of word
                for (word_i, word_char) in &mut file_char_iter {
                    if !is_valid_identifier_char(word_char) {
                        // end of single line comment
                        let word = file_data.get(char_i..word_i).unwrap();
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
                    errors.push(ParsingErr{reason : "Unexpected character", position : file_data.get(char_i..char_i+1).unwrap()});
                }
            }
        } else {
            break;
        }
    }

    return (lexer_result, errors);
}

use std::process::ExitCode;

use console::style;

fn pretty_print(file_text : &str) {
    let (token_vec, _token_errors) = tokenize(file_text);

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
                style(token.text).white()
            } else if token.is_number() {
                style(token.text).green()
            } else {
                style(token.text).red().underlined()
            }
        );
    }

    print!("{}\n", file_text.get(whitespace_start..file_text.len()).unwrap());
}

fn find_line_starts(text : &str) -> Vec<usize> {
    let mut result : Vec<usize> = vec![0];
    for (i, ch) in text.char_indices() {
        if ch == '\n' {
            result.push(i+1);
        }
    }
    result
}

struct FilePosition {
    line : usize,
    col : usize,
}

fn to_file_position(line_start_buffer : &[usize], char_i : usize) -> FilePosition {
    let line = match line_start_buffer.binary_search_by(|probe| probe.cmp(&char_i)) {
        Ok(v) => v,
        Err(v) => v-1
    };
    let col = char_i - line_start_buffer[line];
    FilePosition{line : line, col : col}
}

fn pretty_print_error(line_start_buffer : &[usize], text : &str, err : &ParsingErr) {
    let total_lines = line_start_buffer.len();

    let part_start = err.position.as_ptr() as usize - text.as_ptr() as usize;
    let part_end = part_start + err.position.len();

    let err_start = to_file_position(line_start_buffer, part_start);
    let err_end = to_file_position(line_start_buffer, part_end);

    const LINES_BEFORE_MARGIN : usize = 3;
    const LINES_AFTER_MARGIN : usize = 3;

    let before_margin_line = if err_start.line < LINES_BEFORE_MARGIN {0} else {err_start.line - LINES_BEFORE_MARGIN};
    let after_margin_line = if err_end.line > total_lines - LINES_AFTER_MARGIN {total_lines} else {err_start.line + LINES_BEFORE_MARGIN};

    print!("{}", text.get(line_start_buffer[before_margin_line]..part_start).unwrap());
    print!("{}", style(err.position).red().underlined());
    print!("{}", text.get(part_end..line_start_buffer[err_end.line+1]).unwrap());
    print!("{}{}\n", " ".repeat(err_start.col), style("^ ".to_owned() + err.reason).red());
    print!("{}", text.get(line_start_buffer[err_end.line+1]..line_start_buffer[after_margin_line+1]).unwrap());
}

fn main() -> ExitCode {
    let file_path = "multiply_add.vpp";
    
    match std::fs::read_to_string(file_path) {
        Ok(file_text) => {
            let (_token_vec, token_errors) = tokenize(&file_text);
        
            if !token_errors.is_empty() {
                let line_start_buffer = find_line_starts(&file_text);
                for err in token_errors {
                    pretty_print_error(&line_start_buffer, &file_text, &err);
                }
                return ExitCode::FAILURE;
            } else {
                pretty_print(&file_text);
                return ExitCode::SUCCESS;
            }
        },
        Err(err) => {
            println!("Could not open file {}: {}", style(file_path).yellow(), style(err.to_string()));
            return ExitCode::FAILURE;
        }
    }
}
