

use ariadne::*;
use std::ops::Range;

use crate::tokenizer::{Token, TokenTypeIdx, get_token_type_name};

pub struct ErrorInfo<'a> {
    position : &'a str,
    reason : String
}

pub struct ParsingError<'a> {
    error : ErrorInfo<'a>,
    infos : Vec<ErrorInfo<'a>>
}

fn as_char_range(file_text : &str, position : &str) -> Range<usize> {
    let part_start = position.as_ptr() as usize - file_text.as_ptr() as usize;
    let part_end = part_start + position.len();

    part_start..part_end
}

impl<'a> ParsingError<'a> {
    pub fn pretty_print_error(&self, file_name : &str, file_text : &str) {
        let mut colors = ColorGenerator::new();

        // Generate & choose some colours for each of our elements
        let err_color = Color::Red;
        let info_color = Color::Blue;

        let mut report = Report::build(ReportKind::Error, file_name, 12)
            .with_message(&self.error.reason)
            .with_label(
                Label::new((file_name, as_char_range(file_text, self.error.position)))
                    .with_message(&self.error.reason)
                    .with_color(err_color)
            );

        for info in &self.infos {
            report = report.with_label(
                Label::new((file_name, as_char_range(file_text, info.position)))
                    .with_message(&info.reason)
                    .with_color(info_color)
            )
        }
            /*.with_note(format!(
                "Outputs of {} expressions must coerce to the same type",
                "match".fg(out)
            ))*/
        report.finish()
        .print((file_name, Source::from(file_text)))
        .unwrap();
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

pub fn join_expected_list(expected : &[TokenTypeIdx]) -> String {
    assert!(!expected.is_empty());
    let mut result = String::new();
    for exp in expected.get(..expected.len() - 1).unwrap() {
        result += "'";
        result += get_token_type_name(*exp);
        result += "',";
    }
    if expected.len() >= 2 {
        result += " or ";
    }
    result += "'";
    result += get_token_type_name(expected[expected.len() - 1]);
    result += "'";
    result
}

pub fn error_incorrect_token<'a>(expected : &[TokenTypeIdx], found : &Token<'a>, context : &str) -> ParsingError<'a> {
    let reason = "Unexpected Token. Expected ".to_owned() + &join_expected_list(expected) + " but found '" + get_token_type_name(found.typ) + "' " + context;

    error_basic(found.text, reason)
}
pub fn error_unclosed_bracket<'a>(open : &Token<'a>, close_before : &Token<'a>) -> ParsingError<'a> {
    let reason = "Unclosed bracket. ".to_owned() + open.text + " must be closed before " + close_before.text;
    error_with_info(open.text, reason, vec![error_info_str(close_before.text, "must be closed before this")])
}
pub fn error_unopened_bracket<'a>(close : &Token<'a>, open_after : &Token<'a>) -> ParsingError<'a> {
    let reason = "Unopened bracket. Closing bracket ".to_owned() + close.text + " found but was not opened. Must be opened in scope of " + open_after.text;
    error_with_info(close.text, reason, vec![error_info_str(open_after.text, "must be opened in scope after this")])
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
