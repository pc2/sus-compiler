use crate::prelude::*;

use std::cell::RefCell;

use crate::{alloc::ArenaAllocator, typing::template::TemplateInput};

use crate::flattening::{Declaration, Instruction, Interface, Module, Port, SubModuleInstance};
use crate::linker::{checkpoint::ErrorCheckpoint, FileData, LinkInfo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorLevel {
    Error,
    Warning,
}

/// Represents a comment about a location in the source code.
/// 
/// Multiple infos can be attached to a single [CompileError]
#[derive(Debug, Clone)]
pub struct ErrorInfo {
    pub position: Span,
    pub file: FileUUID,
    pub info: String,
}

/// Represents an error or warning that the compiler produced. They can be shown in the IDE, or on the CLI
/// 
/// All errors for a single file are stored together, which is why this struct does not contain a FileUUID
#[derive(Debug, Clone)]
pub struct CompileError {
    pub position: Span,
    pub reason: String,
    pub infos: Vec<ErrorInfo>,
    pub level: ErrorLevel,
}

/// Stores all errors gathered within a context for reporting to the user.
///
/// Only editable by converting to a ErrorCollector using [ErrorCollector::from_storage]
#[derive(Debug, Clone)]
pub struct ErrorStore {
    errors: Vec<CompileError>,
    pub did_error: bool,
}

impl ErrorStore {
    pub fn new() -> ErrorStore {
        ErrorStore {
            errors: Vec::new(),
            did_error: false,
        }
    }

    pub fn take<'linker>(&mut self) -> Self {
        std::mem::replace(self, ErrorStore::new())
    }

    pub fn checkpoint(&self) -> ErrorCheckpoint {
        ErrorCheckpoint(self.errors.len(), self.did_error)
    }

    pub fn reset_to(&mut self, checkpoint: ErrorCheckpoint) {
        self.errors.truncate(checkpoint.0);
        self.did_error = checkpoint.1;
    }

    /// Returns true if no errors have been reported
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
    error_store: RefCell<ErrorStore>,
    /// Main file of this collector. Makes creating errors easier
    pub file: FileUUID,
    /// Only used for debugging, to see no invalid errors are produced
    file_len: usize,
    pub files: &'linker ArenaAllocator<FileData, FileUUIDMarker>,
}

impl<'linker> ErrorCollector<'linker> {
    pub fn new_empty(
        file: FileUUID,
        files: &'linker ArenaAllocator<FileData, FileUUIDMarker>,
    ) -> Self {
        Self {
            error_store: RefCell::new(ErrorStore::new()),
            file,
            file_len: files[file].file_text.len(),
            files,
        }
    }

    /// Turn the collector back into a [ErrorStore]
    pub fn into_storage(self) -> ErrorStore {
        self.error_store.replace(ErrorStore::new())
    }
    /// Turn an [ErrorStore] into [ErrorCollector]
    pub fn from_storage(error_store: ErrorStore, file: FileUUID, files: &'linker ArenaAllocator<FileData, FileUUIDMarker>) -> Self {
        Self {
            error_store: RefCell::new(error_store),
            file,
            file_len: files[file].file_text.len(),
            files,
        }
    }
    /// To re-attach this [ErrorCollector] to a new [Linker]. Mostly to get around the borrow checker
    pub fn re_attach<'new_linker>(self, files: &'new_linker ArenaAllocator<FileData, FileUUIDMarker>) -> ErrorCollector<'new_linker> {
        ErrorCollector {
            error_store: RefCell::new(self.error_store.replace(ErrorStore::new())),
            file: self.file,
            file_len: self.file_len,
            files,
        }
    }

    fn assert_span_good(&self, span: Span) {
        span.debug();
        let rng = span.into_range();
        assert!(rng.end <= self.file_len); // Don't need to verify start, since Span already enforces start <= end
    }
    fn push_diagnostic(
        &self,
        position: Span,
        reason: String,
        level: ErrorLevel,
    ) -> ErrorReference<'_> {
        self.assert_span_good(position);

        let mut store = self.error_store.borrow_mut();
        store.did_error |= level == ErrorLevel::Error;
        let pos = store.errors.len();
        store.errors.push(CompileError {
            position,
            reason,
            infos: Vec::new(),
            level,
        });
        ErrorReference {
            err_collector: self,
            pos,
        }
    }

    pub fn error<S: Into<String>>(&self, position: Span, reason: S) -> ErrorReference<'_> {
        self.push_diagnostic(position, reason.into(), ErrorLevel::Error)
    }

    pub fn warn<S: Into<String>>(&self, position: Span, reason: S) -> ErrorReference<'_> {
        self.push_diagnostic(position, reason.into(), ErrorLevel::Warning)
    }

    pub fn todo<S: Into<String>>(&self, position: Span, reason: S) -> ErrorReference<'_> {
        self.push_diagnostic(
            position,
            format!("TODO: {}", reason.into()),
            ErrorLevel::Error,
        )
    }

    pub fn did_error(&self) -> bool {
        self.error_store.borrow().did_error
    }
    pub fn set_did_error(&mut self) {
        self.error_store.get_mut().did_error = true;
    }
}

impl<'l> Drop for ErrorCollector<'l> {
    fn drop(&mut self) {
        if !self.error_store.borrow().is_untouched() {
            panic!("ErrorCollector should have been emptied!");
        }
    }
}

/// Intermediary struct to make adding infos far easier. 
/// 
/// Use as:
/// 
///     errors.warn(span, "Unused Variable").info(span2, file2, "In module").info(blablabla)
pub struct ErrorReference<'ec> {
    err_collector: &'ec ErrorCollector<'ec>,
    pos: usize,
}

impl<'ec> ErrorReference<'ec> {
    pub fn existing_info(&self, error_info: ErrorInfo) -> &Self {
        assert!(
            error_info.position.debug().into_range().end
                <= self.err_collector.files[error_info.file].file_text.len()
        );
        self.err_collector.error_store.borrow_mut().errors[self.pos]
            .infos
            .push(error_info);
        self
    }
    pub fn info<S: Into<String>>(&self, (span, file): SpanFile, reason: S) -> &Self {
        self.existing_info(ErrorInfo {
            position: span,
            file,
            info: reason.into(),
        })
    }
    pub fn info_same_file<S: Into<String>>(&self, span: Span, reason: S) -> &Self {
        self.info((span, self.err_collector.file), reason)
    }
    pub fn info_obj<Obj: FileKnowingErrorInfoObject>(&self, obj: &Obj) -> &Self {
        self.existing_info(obj.make_global_info(&self.err_collector.files))
    }
    pub fn info_obj_same_file<Obj: ErrorInfoObject>(&self, obj: &Obj) -> &Self {
        self.existing_info(obj.make_info(self.err_collector.file))
    }
    pub fn info_obj_different_file<Obj: ErrorInfoObject>(
        &self,
        obj: &Obj,
        file: FileUUID,
    ) -> &Self {
        self.existing_info(obj.make_info(file))
    }
    pub fn add_info_list(&self, mut info_list: Vec<ErrorInfo>) {
        self.err_collector.error_store.borrow_mut().errors[self.pos].infos.append(&mut info_list);
    }
    pub fn suggest_replace<S: Into<String>>(&self, replace_span: Span, replace_with: S) -> &Self {
        self.info_same_file(
            replace_span,
            format!("SUGGEST: Replace this with \"{}\"", replace_with.into()),
        )
    }
    pub fn suggest_remove(&self, remove_span: Span) -> &Self {
        self.info_same_file(remove_span, "SUGGEST: Remove this")
    }
}

/// This represents objects that can be given as info to an error in a straight-forward way.
pub trait ErrorInfoObject {
    fn make_info(&self, file: FileUUID) -> ErrorInfo;
}

pub trait FileKnowingErrorInfoObject {
    fn make_global_info(
        &self,
        files: &ArenaAllocator<FileData, FileUUIDMarker>,
    ) -> ErrorInfo;
}

// Trait implementations in the compiler

impl ErrorInfoObject for Declaration {
    fn make_info(&self, file: FileUUID) -> ErrorInfo {
        ErrorInfo {
            position: self.name_span,
            file,
            info: format!("'{}' declared here", &self.name),
        }
    }
}

impl ErrorInfoObject for SubModuleInstance {
    fn make_info(&self, file: FileUUID) -> ErrorInfo {
        let (position, info) = if let Some((name, span)) = &self.name {
            (*span, format!("{name} declared here"))
        } else {
            (self.module_ref.name_span, "Used here".to_owned())
        };
        ErrorInfo {
            position,
            file,
            info,
        }
    }
}

impl ErrorInfoObject for Instruction {
    fn make_info(&self, file: FileUUID) -> ErrorInfo {
        match self {
            Instruction::SubModule(sm) => sm.make_info(file),
            Instruction::Declaration(decl) => decl.make_info(file),
            _ => unreachable!("At least there shouldn't be cases where we're referring to something other than SubModule or Declaration")
        }
    }
}

impl ErrorInfoObject for TemplateInput {
    fn make_info(&self, file: FileUUID) -> ErrorInfo {
        ErrorInfo {
            position: self.name_span,
            file,
            info: format!("Parameter '{}' declared here", self.name),
        }
    }
}

impl ErrorInfoObject for Port {
    fn make_info(&self, file: FileUUID) -> ErrorInfo {
        ErrorInfo {
            position: self.name_span,
            file,
            info: format!("Port '{}' declared here", &self.name),
        }
    }
}

impl FileKnowingErrorInfoObject for LinkInfo {
    fn make_global_info(
        &self,
        _files: &ArenaAllocator<FileData, FileUUIDMarker>,
    ) -> ErrorInfo {
        ErrorInfo {
            position: self.name_span,
            file: self.file,
            info: format!("'{}' defined here", &self.name),
        }
    }
}

/// For interfaces of this module
impl FileKnowingErrorInfoObject for (&'_ Module, &'_ Interface) {
    fn make_global_info(
        &self,
        _files: &ArenaAllocator<FileData, FileUUIDMarker>,
    ) -> ErrorInfo {
        let (md, interface) = *self;
        ErrorInfo {
            position: interface.name_span,
            file: md.link_info.file,
            info: format!("Interface '{}' defined here", &interface.name),
        }
    }
}

impl FileKnowingErrorInfoObject for Module {
    fn make_global_info(
        &self,
        files: &ArenaAllocator<FileData, FileUUIDMarker>,
    ) -> ErrorInfo {
        let ports_str = self.make_all_ports_info_string(&files[self.link_info.file].file_text, None);

        ErrorInfo {
            position: self.link_info.name_span,
            file: self.link_info.file,
            info: format!(
                "Module '{}' defined here. {}",
                &self.link_info.name, ports_str
            ),
        }
    }
}
