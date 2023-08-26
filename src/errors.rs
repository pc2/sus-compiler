

use std::ops::Range;

use crate::ast::Span;
use ariadne::*;

use crate::tokenizer::{TokenTypeIdx, get_token_type_name};

pub struct ErrorInfo<T> {
    pub position : T,
    pub reason : String
}

pub struct ParsingError<T> {
    pub error : ErrorInfo<T>,
    pub infos : Vec<ErrorInfo<T>>
}

impl<'a> ParsingError<Span> {
    pub fn pretty_print_error(&self, file_name : &str, file_text : &str, character_ranges : &[Range<usize>]) {
        // Generate & choose some colours for each of our elements
        let err_color = Color::Red;
        let info_color = Color::Blue;

        let error_span = self.error.position.to_range(character_ranges);
        let mut report = Report::build(ReportKind::Error, file_name, error_span.start)
            .with_message(&self.error.reason)
            .with_label(
                Label::new((file_name, error_span))
                    .with_message(&self.error.reason)
                    .with_color(err_color)
            );

        for info in &self.infos {
            let info_span = info.position.to_range(character_ranges);
            report = report.with_label(
                Label::new((file_name, info_span))
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

pub fn error_info<T, S : Into<String>>(position : T, reason : S) -> ErrorInfo<T> {
    ErrorInfo{position : position, reason : reason.into()}
}

pub fn error_basic<T, S : Into<String>>(position : T, reason : S) -> ParsingError<T> {
    ParsingError{error : error_info(position, reason), infos : Vec::new()}
}

pub fn error_with_info<T, S : Into<String>, V : Into<Vec<ErrorInfo<T>>>>(position : T, reason : S, infos : V) -> ParsingError<T> {
    ParsingError{error : error_info(position, reason), infos : infos.into()}
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

