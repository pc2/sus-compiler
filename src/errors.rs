

use crate::ast::Span;
use crate::ast::CharSpan;
use crate::ast::cvt_span_to_char_span;
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

impl<'a> ParsingError<CharSpan> {
    pub fn pretty_print_error(&self, file_name : &str, file_text : &str) {
        // Generate & choose some colours for each of our elements
        let err_color = Color::Red;
        let info_color = Color::Blue;

        let mut report = Report::build(ReportKind::Error, file_name, self.error.position.file_pos.char_idx)
            .with_message(&self.error.reason)
            .with_label(
                Label::new((file_name, self.error.position.as_range()))
                    .with_message(&self.error.reason)
                    .with_color(err_color)
            );

        for info in &self.infos {
            report = report.with_label(
                Label::new((file_name, info.position.as_range()))
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

pub fn cvt_token_err_info_to_str(err : ErrorInfo<Span>, token_spans : &[CharSpan]) -> ErrorInfo<CharSpan> {
    ErrorInfo{position : cvt_span_to_char_span(err.position, token_spans), reason : err.reason}
}

pub fn cvt_token_error_to_str_error(err : ParsingError<Span>, token_spans : &[CharSpan]) -> ParsingError<CharSpan> {
    let mut info_vec : Vec<ErrorInfo<CharSpan>> = Vec::new();
    info_vec.reserve(err.infos.len());

    for i in err.infos {
        info_vec.push(cvt_token_err_info_to_str(i, token_spans));
    }

    ParsingError{error : cvt_token_err_info_to_str(err.error, token_spans), infos : info_vec}
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

