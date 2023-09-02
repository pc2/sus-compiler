

use std::{ops::Range, rc::Rc};

use crate::ast::Span;
use ariadne::*;

use crate::tokenizer::{TokenTypeIdx, get_token_type_name};

pub type FileName = Rc<String>;

pub struct ErrorInfo {
    pub position : Span,
    pub file_name : FileName,
    pub info : String
}

pub struct ParsingError {
    pub position : Span,
    pub reason : String,
    pub infos : Vec<ErrorInfo>
}

impl ParsingError {
    pub fn pretty_print_error(&self, main_file_name : &str, file_text : &str, character_ranges : &[Range<usize>]) {
        // Generate & choose some colours for each of our elements
        let err_color = Color::Red;
        let info_color = Color::Blue;

        let error_span = self.position.to_range(character_ranges);
        let mut report = Report::build(ReportKind::Error, main_file_name, error_span.start)
            .with_message(&self.reason)
            .with_label(
                Label::new((main_file_name, error_span))
                    .with_message(&self.reason)
                    .with_color(err_color)
            );

        for info in &self.infos {
            let info_span = info.position.to_range(character_ranges);
            let info_file : &str = &info.file_name;
            report = report.with_label(
                Label::new((info_file, info_span))
                    .with_message(&info.info)
                    .with_color(info_color)
            )
        }
            /*.with_note(format!(
                "Outputs of {} expressions must coerce to the same type",
                "match".fg(out)
            ))*/
        report.finish()
        .print((main_file_name, Source::from(file_text)))
        .unwrap();
    }
}

pub fn error_info<S : Into<String>>(position : Span, file_name : FileName, reason : S) -> ErrorInfo {
    ErrorInfo{position, file_name, info : reason.into()}
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

// Class that collects and manages errors and warnings
pub struct ErrorCollector {
    pub errors : Vec<ParsingError>,
    pub main_file : FileName
}

impl ErrorCollector {
    pub fn new(main_file : FileName) -> Self {
        Self{errors : Vec::new(), main_file}
    }
    
    pub fn error_basic<S : Into<String>>(&mut self, position : Span, reason : S) {
        self.errors.push(ParsingError{position, reason : reason.into(), infos : Vec::new()});
    }
    
    pub fn error_with_info<S : Into<String>>(&mut self, position : Span, reason : S, infos : Vec<ErrorInfo>) {
        self.errors.push(ParsingError{position, reason : reason.into(), infos : infos});
    }
}
