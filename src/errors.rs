

use std::cell::{RefCell, Cell};

use crate::{linker::FileUUID, file_position::Span};

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

pub fn error_info<S : Into<String>>(position : Span, file : FileUUID, reason : S) -> ErrorInfo {
    ErrorInfo{position, file, info : reason.into()}
}

pub fn join_expected_list(expected : &[TokenTypeIdx]) -> String {
    use std::fmt::Write;
    
    assert!(!expected.is_empty());
    let mut result = String::new();
    for exp in expected.get(..expected.len() - 1).unwrap() {
        let tok_typ_name = get_token_type_name(*exp);
        writeln!(&mut result, "'{tok_typ_name}',").unwrap();
    }
    if expected.len() >= 2 {
        result += " or ";
    }
    let tok_typ_name = get_token_type_name(expected[expected.len() - 1]);
    writeln!(&mut result, "'{tok_typ_name}'").unwrap();
    result
}

// Class that collects and manages errors and warnings
// Implemented such that it can be shared immutably. This makes many operations to do with parsing easier
// It doesn't allow indexing, so no immutable references to contents can exist
#[derive(Debug,Clone)]
pub struct ErrorCollector {
    errors : RefCell<Vec<CompileError>>,
    pub did_error : Cell<bool>,
    pub file : FileUUID,
}

impl ErrorCollector {
    pub fn new(file : FileUUID) -> Self {
        Self{errors : RefCell::new(Vec::new()), file, did_error : Cell::new(false)}
    }

    pub fn error_basic<S : Into<String>>(&self, position : Span, reason : S) {
        self.errors.borrow_mut().push(CompileError{position, reason : reason.into(), infos : Vec::new(), level : ErrorLevel::Error});
        self.did_error.set(true);
    }
    
    pub fn error_with_info<S : Into<String>>(&self, position : Span, reason : S, infos : Vec<ErrorInfo>) {
        self.errors.borrow_mut().push(CompileError{position, reason : reason.into(), infos : infos, level : ErrorLevel::Error});
        self.did_error.set(true);
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
