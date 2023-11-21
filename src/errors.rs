

use std::{ops::Range, path::{Path, PathBuf}, cell::RefCell};

use crate::{ast::Span, linker::{FileUUID, FileUUIDMarker}, arena_alloc::ArenaVector};
use ariadne::*;


use crate::tokenizer::{TokenTypeIdx, get_token_type_name};

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum ErrorLevel {
    Error,
    Warning
}

#[derive(Debug,Clone)]
pub struct ErrorInfo {
    pub position : Span,
    pub file : FileUUID,
    pub info : String
}

#[derive(Debug,Clone)]
pub struct CompileError {
    pub position : Span,
    pub reason : String,
    pub infos : Vec<ErrorInfo>,
    pub level : ErrorLevel
}

struct CustomSpan<'a> {
    file : &'a Path,
    span : Range<usize>
}
impl<'a> ariadne::Span for CustomSpan<'a> {
    type SourceId = Path;

    fn source(&self) -> &Self::SourceId { &self.file }
    fn start(&self) -> usize { self.span.start }
    fn end(&self) -> usize { self.span.end }
}

impl CompileError {
    // Requires that character_ranges.len() == tokens.len() + 1 to include EOF token
    pub fn pretty_print_error(&self, file : FileUUID, character_ranges : &[Range<usize>], paths : &ArenaVector<PathBuf, FileUUIDMarker>, file_cache : &mut FileCache) {
        // Generate & choose some colours for each of our elements
        let (err_color, report_kind) = match self.level {
            ErrorLevel::Error => (Color::Red, ReportKind::Error),
            ErrorLevel::Warning => (Color::Yellow, ReportKind::Warning),
        };
        let info_color = Color::Blue;

        let error_span = self.position.to_range(character_ranges);

        let file_path = &paths[file];

        let mut report: ReportBuilder<'_, CustomSpan> = Report::build(report_kind, file_path, error_span.start);
        report = report
            .with_message(&self.reason)
            .with_label(
                Label::new(CustomSpan{file : file_path, span : error_span})
                    .with_message(&self.reason)
                    .with_color(err_color)
            );

        for info in &self.infos {
            let info_span = info.position.to_range(character_ranges);
            report = report.with_label(
                Label::new(CustomSpan{file : &paths[info.file], span : info_span})
                    .with_message(&info.info)
                    .with_color(info_color)
            )
        }
            /*.with_note(format!(
                "Outputs of {} expressions must coerce to the same type",
                "match".fg(out)
            ))*/
        report.finish()
        .eprint(file_cache)
        .unwrap();
    }
}

pub fn error_info<S : Into<String>>(position : Span, file : FileUUID, reason : S) -> ErrorInfo {
    ErrorInfo{position, file, info : reason.into()}
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
// Implemented such that it can be shared immutably. This makes many operations to do with parsing easier
#[derive(Debug,Clone)]
pub struct ErrorCollector {
    errors : RefCell<Vec<CompileError>>,
    pub file : FileUUID
}

impl ErrorCollector {
    pub fn new(file : FileUUID) -> Self {
        Self{errors : RefCell::new(Vec::new()), file}
    }

    pub fn error_basic<S : Into<String>>(&self, position : Span, reason : S) {
        self.errors.borrow_mut().push(CompileError{position, reason : reason.into(), infos : Vec::new(), level : ErrorLevel::Error});
    }
    
    pub fn error_with_info<S : Into<String>>(&self, position : Span, reason : S, infos : Vec<ErrorInfo>) {
        self.errors.borrow_mut().push(CompileError{position, reason : reason.into(), infos : infos, level : ErrorLevel::Error});
    }
    
    pub fn warn_basic<S : Into<String>>(&self, position : Span, reason : S) {
        self.errors.borrow_mut().push(CompileError{position, reason : reason.into(), infos : Vec::new(), level : ErrorLevel::Warning});
    }
    
    pub fn warn_with_info<S : Into<String>>(&self, position : Span, reason : S, infos : Vec<ErrorInfo>) {
        self.errors.borrow_mut().push(CompileError{position, reason : reason.into(), infos : infos, level : ErrorLevel::Warning});
    }

    pub fn get(self) -> (Vec<CompileError>, FileUUID) {
        (self.errors.into_inner(), self.file)
    }

    pub fn ingest(&self, source : &Self) {
        assert!(self.file == source.file);
        self.errors.borrow_mut().extend_from_slice(&source.errors.borrow());
    }
}
