

use std::cell::RefCell;

use crate::{arena_alloc::ArenaAllocator, file_position::Span, linker::{checkpoint::ErrorCheckpoint, FileData, FileUUID, FileUUIDMarker}};

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

/// Stores all errors gathered within a context for reporting to the user. 
/// 
/// Only editable by converting to a ErrorCollector using [ErrorStore::take_for_editing]
#[derive(Debug,Clone)]
pub struct ErrorStore {
    errors : Vec<CompileError>,
    pub did_error : bool
}

impl ErrorStore {
    pub fn new() -> ErrorStore {
        ErrorStore{
            errors : Vec::new(),
            did_error : false
        }
    }

    pub fn take_for_editing<'linker>(&mut self, file : FileUUID, files : &'linker ArenaAllocator<FileData, FileUUIDMarker>) -> ErrorCollector<'linker> {
        let error_store = RefCell::new(std::mem::replace(self, ErrorStore::new()));
        ErrorCollector { error_store, file, file_len : files[file].file_text.len(), files }
    }

    pub fn checkpoint(&self) -> ErrorCheckpoint {
        ErrorCheckpoint(self.errors.len(), self.did_error)
    }

    pub fn reset_to(&mut self, checkpoint : ErrorCheckpoint) {
        self.errors.truncate(checkpoint.0);
        self.did_error = checkpoint.1;
    }

    pub fn is_untouched(&self) -> bool {
        self.errors.is_empty()
    }
}

impl<'e> IntoIterator for &'e ErrorStore {
    type Item = &'e CompileError;

    type IntoIter = std::slice::Iter<'e, CompileError>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.iter()
    }
}



/// Class that collects and manages errors and warnings
/// 
/// Implemented such that it can be shared immutably. 
/// This allows use in immutable contexts, because reporting errors isn't really changing the context
#[derive(Clone)]
pub struct ErrorCollector<'linker> {
    error_store : RefCell<ErrorStore>,
    /// Main file of this collector. Makes creating errors easier
    pub file : FileUUID,
    /// Only used for debugging, to see no invalid errors are produced
    file_len : usize,
    files : &'linker ArenaAllocator<FileData, FileUUIDMarker>
}

impl<'linker> ErrorCollector<'linker> {
    pub fn new_empty(file : FileUUID, files : &'linker ArenaAllocator<FileData, FileUUIDMarker>) -> Self {
        Self{error_store : RefCell::new(ErrorStore::new()), file, file_len : files[file].file_text.len(), files}
    }

    /// Turn the collector back into a [ErrorStore]
    pub fn into_storage(self) -> ErrorStore {
        self.error_store.into_inner()
    }

    fn assert_span_good(&self, span : Span) {
        span.debug();
        let rng = span.into_range();
        assert!(rng.end <= self.file_len); // Don't need to verify start, since Span already enforces start <= end
    }
    fn push_diagnostic(&self, diagnostic : CompileError) {
        self.assert_span_good(diagnostic.position);
        for info in &diagnostic.infos {
            assert!(info.position.into_range().end <= self.files[info.file].file_text.len());
        }
        let mut store = self.error_store.borrow_mut();
        store.did_error |= diagnostic.level == ErrorLevel::Error;
        store.errors.push(diagnostic);
    }

    pub fn error_basic<S : Into<String>>(&self, position : Span, reason : S) {
        self.push_diagnostic(CompileError{position, reason : reason.into(), infos : Vec::new(), level : ErrorLevel::Error});
    }
    
    pub fn error_with_info<S : Into<String>>(&self, position : Span, reason : S, infos : Vec<ErrorInfo>) {
        self.push_diagnostic(CompileError{position, reason : reason.into(), infos : infos, level : ErrorLevel::Error});
    }
    
    pub fn warn_basic<S : Into<String>>(&self, position : Span, reason : S) {
        self.push_diagnostic(CompileError{position, reason : reason.into(), infos : Vec::new(), level : ErrorLevel::Warning});
    }
    
    pub fn warn_with_info<S : Into<String>>(&self, position : Span, reason : S, infos : Vec<ErrorInfo>) {
        self.push_diagnostic(CompileError{position, reason : reason.into(), infos : infos, level : ErrorLevel::Warning});
    }

    pub fn did_error(&self) -> bool {
        self.error_store.borrow().did_error
    }
}
