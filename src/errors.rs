
use console::style;

use crate::tokenizer::{Token, TokenTypeIdx, get_token_type_name};

pub struct ErrorInfo<'a> {
    position : &'a str,
    reason : String
}

pub struct ParsingError<'a> {
    error : ErrorInfo<'a>,
    infos : Vec<ErrorInfo<'a>>
}

impl<'a> ParsingError<'a> {
    pub fn pretty_print(&self, line_start_buffer : &[usize], text : &str) {
        let total_lines = line_start_buffer.len();

        let part_start = self.error.position.as_ptr() as usize - text.as_ptr() as usize;
        let part_end = part_start + self.error.position.len();

        let err_start = to_file_position(line_start_buffer, part_start);
        let err_end = to_file_position(line_start_buffer, part_end);

        const LINES_BEFORE_MARGIN : usize = 3;
        const LINES_AFTER_MARGIN : usize = 3;

        let before_margin_line = if err_start.line < LINES_BEFORE_MARGIN {0} else {err_start.line - LINES_BEFORE_MARGIN};
        let after_margin_line = if err_end.line > total_lines - LINES_AFTER_MARGIN {total_lines} else {err_start.line + LINES_BEFORE_MARGIN};

        print!("{}", text.get(line_start_buffer[before_margin_line]..part_start).unwrap());
        print!("{}", style(self.error.position).red().underlined());
        print!("{}", text.get(part_end..line_start_buffer[err_end.line+1]).unwrap());
        print!("{}{}\n", " ".repeat(err_start.col), style("^ ".to_owned() + &self.error.reason).red());
        print!("{}", text.get(line_start_buffer[err_end.line+1]..line_start_buffer[after_margin_line+1]).unwrap());
    }
}

pub fn error_info<'a>(position : &'a str, reason : String) -> ErrorInfo<'a> {
    ErrorInfo{position : position, reason : reason}
}
pub fn error_info_str<'a>(position : &'a str, reason : &str) -> ErrorInfo<'a> {
    ErrorInfo{position : position, reason : reason.to_owned()}
}

pub fn error_basic<'a>(position : &'a str, reason : String) -> ParsingError<'a> {
    ParsingError{error : error_info(position, reason), infos : Vec::new()}
}

pub fn error_basic_str<'a>(position : &'a str, reason : &str) -> ParsingError<'a> {
    ParsingError{error : error_info(position, reason.to_owned()), infos : Vec::new()}
}

pub fn error_with_info<'a>(position : &'a str, reason : String, infos : Vec<ErrorInfo<'a>>) -> ParsingError<'a> {
    ParsingError{error : error_info(position, reason), infos : infos}
}

pub fn error_incorrect_token<'a>(expected : TokenTypeIdx, found : &Token<'a>, context : &str) -> ParsingError<'a> {
    let reason = "Unexpected Token. Expected ".to_owned() + get_token_type_name(expected) + " but found " + get_token_type_name(found.typ) + " " + context;
    error_basic(found.text, reason)
}
pub fn error_unclosed_bracket<'a>(open : &Token<'a>, close_before : &Token<'a>) -> ParsingError<'a> {
    let reason = "Unclosed bracket. ".to_owned() + open.text + " must be closed before " + close_before.text;
    error_with_info(open.text, reason, vec![error_info_str(close_before.text, "must be closed before this token")])
}

struct FilePosition {
    pub line : usize,
    pub col : usize,
}

fn to_file_position(line_start_buffer : &[usize], char_i : usize) -> FilePosition {
    let line = match line_start_buffer.binary_search_by(|probe| probe.cmp(&char_i)) {
        Ok(v) => v,
        Err(v) => v-1
    };
    let col = char_i - line_start_buffer[line];
    FilePosition{line : line, col : col}
}
