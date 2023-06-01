
use console::style;

use crate::tokenizer::{Token, TokenTypeIdx, get_token_type_name};


pub struct ParsingError<'a> {
    code : &'static str,
    reason : String,
    position : &'a str
}

impl<'a> ParsingError<'a> {
    pub fn pretty_print(&self, line_start_buffer : &[usize], text : &str) {
        let total_lines = line_start_buffer.len();

        let part_start = self.position.as_ptr() as usize - text.as_ptr() as usize;
        let part_end = part_start + self.position.len();

        let err_start = to_file_position(line_start_buffer, part_start);
        let err_end = to_file_position(line_start_buffer, part_end);

        const LINES_BEFORE_MARGIN : usize = 3;
        const LINES_AFTER_MARGIN : usize = 3;

        let before_margin_line = if err_start.line < LINES_BEFORE_MARGIN {0} else {err_start.line - LINES_BEFORE_MARGIN};
        let after_margin_line = if err_end.line > total_lines - LINES_AFTER_MARGIN {total_lines} else {err_start.line + LINES_BEFORE_MARGIN};

        println!("{} ---------", style(self.code).red());
        print!("{}", text.get(line_start_buffer[before_margin_line]..part_start).unwrap());
        print!("{}", style(self.position).red().underlined());
        print!("{}", text.get(part_end..line_start_buffer[err_end.line+1]).unwrap());
        print!("{}{}\n", " ".repeat(err_start.col), style("^ ".to_owned() + &self.reason).red());
        print!("{}", text.get(line_start_buffer[err_end.line+1]..line_start_buffer[after_margin_line+1]).unwrap());
    }

    pub fn new_unowned(code : &'static str, reason : &'static str, position : &'a str) -> ParsingError<'a> {
        ParsingError{code : code, reason : reason.to_owned(), position : position}
    }
    pub fn new_owned(code : &'static str, reason : String, position : &'a str) -> ParsingError<'a> {
        ParsingError{code : code, reason : reason, position : position}
    }
    pub fn new_error_incorrect_token(expected : TokenTypeIdx, found : &Token<'a>, context : &str) -> ParsingError<'a> {
        let reason = "Unexpected Token. Expected ".to_owned() + get_token_type_name(expected) + " but found " + get_token_type_name(found.typ) + " " + context;
        ParsingError::new_owned("P101", reason, found.text)
    }
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
