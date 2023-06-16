

use crate::ast::Span;
use ariadne::*;
use std::ops::Range;

use crate::tokenizer::{Token, TokenTypeIdx, get_token_type_name};

pub struct ErrorInfo<T> {
    position : T,
    reason : String
}

pub struct ParsingError<T> {
    error : ErrorInfo<T>,
    infos : Vec<ErrorInfo<T>>
}

fn as_char_range(file_text : &str, position : &str) -> Range<usize> {
    let part_start = position.as_ptr() as usize - file_text.as_ptr() as usize;
    let part_end = part_start + position.len();

    part_start..part_end
}

impl<'a> ParsingError<&'a str> {
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

impl ParsingError<Span> {
    pub fn pretty_print_error(self, file_name : &str, file_text : &str, token_vec : &[Token]) {
        cvt_token_error_to_str_error(self, file_text, token_vec).pretty_print_error(file_name, file_text);
    }
}

fn get_token_text_or_eof<'a>(idx : usize, file_text : &'a str, tokens : &[Token<'a>]) -> &'a str {
    if idx < tokens.len() {
        tokens[idx].text
    } else {
        file_text.get(file_text.len() - 1..file_text.len()).unwrap()
    }
}

fn get_file_error_span<'a>(span : Span, file_text : &'a str, tokens : &[Token<'a>]) -> &'a str {
    let start_str = get_token_text_or_eof(span.0, file_text, tokens);
    let end_str = get_token_text_or_eof(span.1, file_text, tokens);

    let start = start_str.as_ptr() as usize - file_text.as_ptr() as usize;
    let end = end_str.as_ptr() as usize - file_text.as_ptr() as usize + end_str.len();

    file_text.get(start..end).unwrap()
}

pub fn cvt_token_err_info_to_str<'a>(err : ErrorInfo<Span>, file_text : &'a str, tokens : &[Token<'a>]) -> ErrorInfo<&'a str> {
    ErrorInfo{position : get_file_error_span(err.position, file_text, tokens), reason : err.reason}
}

pub fn cvt_token_error_to_str_error<'a>(err : ParsingError<Span>, file_text : &'a str, tokens : &[Token<'a>]) -> ParsingError<&'a str> {
    let mut info_vec : Vec<ErrorInfo<&'a str>> = Vec::new();
    info_vec.reserve(err.infos.len());

    for i in err.infos {
        info_vec.push(cvt_token_err_info_to_str(i, file_text, tokens));
    }

    ParsingError{error : cvt_token_err_info_to_str(err.error, file_text, tokens), infos : info_vec}
}

pub fn error_info<T>(position : T, reason : String) -> ErrorInfo<T> {
    ErrorInfo{position : position, reason : reason}
}
pub fn error_info_str<T>(position : T, reason : &str) -> ErrorInfo<T> {
    ErrorInfo{position : position, reason : reason.to_owned()}
}

pub fn error_basic<T>(position : T, reason : String) -> ParsingError<T> {
    ParsingError{error : error_info(position, reason), infos : Vec::new()}
}

pub fn error_basic_str<T>(position : T, reason : &str) -> ParsingError<T> {
    ParsingError{error : error_info(position, reason.to_owned()), infos : Vec::new()}
}

pub fn error_with_info<T>(position : T, reason : String, infos : Vec<ErrorInfo<T>>) -> ParsingError<T> {
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

pub fn error_incorrect_token(expected : &[TokenTypeIdx], position : usize, typ : TokenTypeIdx, context : &str) -> ParsingError<Span> {
    let expected_str = join_expected_list(expected);
    let token_name = get_token_type_name(typ);
    let reason = format!("Unexpected Token. Expected {expected_str} but found '{token_name}' while parsing {context}");

    error_basic(Span::from(position), reason)
}
pub fn error_unclosed_bracket(open_pos : usize, open_typ : TokenTypeIdx, close_before_pos : usize) -> ParsingError<Span> {
    let open_name = get_token_type_name(open_typ);
    let reason = format!("Unclosed bracket {open_name}");
    error_with_info(Span::from(open_pos), reason, vec![error_info_str(Span(close_before_pos, close_before_pos), "must be closed before this")])
}
pub fn error_unopened_bracket(close_pos : usize, close_typ : TokenTypeIdx, open_after_pos : usize) -> ParsingError<Span> {
    let close_name = get_token_type_name(close_typ);
    let reason = format!("Unopened bracket. Closing bracket {close_name} found but was not opened.");
    error_with_info(Span::from(close_pos), reason, vec![error_info_str(Span(open_after_pos, open_after_pos), "must be opened in scope after this")])
}
