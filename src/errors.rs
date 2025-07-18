use crate::append_only_vec::AppendOnlyVec;
use crate::prelude::*;

use std::cell::Cell;
use std::fmt::Display;

use crate::{alloc::ArenaAllocator, typing::template::Parameter};

use crate::flattening::{
    Declaration, DomainInfo, Instruction, Interface, InterfaceDeclaration, Module, Port,
    SubModuleInstance,
};
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

impl Default for ErrorStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorStore {
    pub fn new() -> ErrorStore {
        ErrorStore {
            errors: Vec::new(),
            did_error: false,
        }
    }
    pub fn new_did_error() -> ErrorStore {
        ErrorStore {
            errors: Vec::new(),
            did_error: true,
        }
    }

    pub fn take(&mut self) -> Self {
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

    pub fn push(&mut self, err: CompileError) {
        self.did_error |= err.level == ErrorLevel::Error;
        self.errors.push(err);
    }

    pub fn append(&mut self, errs: &ErrorStore) {
        self.did_error |= errs.did_error;
        self.errors.extend_from_slice(&errs.errors);
    }

    pub fn sort(&mut self) {
        self.errors.sort_by(|a, b| {
            a.position
                .as_range()
                .start
                .cmp(&b.position.as_range().start)
                .then_with(|| a.reason.cmp(&b.reason))
        });
    }
}

impl IntoIterator for ErrorStore {
    type Item = CompileError;

    type IntoIter = std::vec::IntoIter<CompileError>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
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
pub struct ErrorCollector<'linker> {
    errors: AppendOnlyVec<CompileError>,
    did_error: Cell<bool>,
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
        Self::from_storage(ErrorStore::new(), file, files)
    }

    /// Turn the collector back into a [ErrorStore]
    pub fn into_storage(mut self) -> ErrorStore {
        ErrorStore {
            errors: Vec::from(std::mem::take(&mut self.errors)),
            did_error: self.did_error.get(),
        }
    }
    /// Turn an [ErrorStore] into [ErrorCollector]
    pub fn from_storage(
        error_store: ErrorStore,
        file: FileUUID,
        files: &'linker ArenaAllocator<FileData, FileUUIDMarker>,
    ) -> Self {
        Self {
            errors: AppendOnlyVec::from(error_store.errors),
            did_error: Cell::new(error_store.did_error),
            file,
            file_len: files[file].file_text.len(),
            files,
        }
    }

    fn assert_span_good(&self, span: Span) {
        span.debug();
        let rng = span.as_range();
        assert!(rng.end <= self.file_len); // Don't need to verify start, since Span already enforces start <= end
    }
    fn push_diagnostic(
        &self,
        position: Span,
        reason: String,
        level: ErrorLevel,
    ) -> ErrorReference<'_> {
        self.assert_span_good(position);

        ErrorReference {
            err_collector: self,
            built_error: Some(CompileError {
                position,
                reason,
                infos: Vec::new(),
                level,
            }),
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

    pub fn type_error(
        &self,
        position: Span,
        found: impl Display,
        expected: impl Display,
    ) -> ErrorReference<'_> {
        self.error(
            position,
            format!("Typecheck error: Found {found}, but expected {expected}"),
        )
    }
    pub fn subtype_error(
        &self,
        span: Span,
        found: impl Display,
        expected: impl Display,
    ) -> ErrorReference<'_> {
        self.error(
            span,
            format!(
                "Typecheck error: Found {found}, which is not a subtype of the expected type {expected}"
            ),
        )
    }

    pub fn did_error(&self) -> bool {
        self.did_error.get()
    }
    pub fn set_did_error(&mut self) {
        self.did_error.set(true);
    }
}

impl Drop for ErrorCollector<'_> {
    fn drop(&mut self) {
        if !self.errors.is_empty() && !std::thread::panicking() {
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
    built_error: Option<CompileError>,
}

// This is the trick, the error is only added to the ErrorCollector when the ErrorReference is dropped.
impl<'ec> Drop for ErrorReference<'ec> {
    fn drop(&mut self) {
        let built_error = self.built_error.take().unwrap();
        self.err_collector
            .did_error
            .set(self.err_collector.did_error.get() | (built_error.level == ErrorLevel::Error));

        self.err_collector.errors.push(built_error);
    }
}

impl ErrorReference<'_> {
    pub fn existing_info(mut self, error_info: ErrorInfo) -> Self {
        assert!(
            error_info.position.debug().as_range().end
                <= self.err_collector.files[error_info.file].file_text.len()
        );
        self.built_error.as_mut().unwrap().infos.push(error_info);
        self
    }
    pub fn info<S: Into<String>>(self, (span, file): SpanFile, reason: S) -> Self {
        self.existing_info(ErrorInfo {
            position: span,
            file,
            info: reason.into(),
        })
    }
    pub fn info_same_file<S: Into<String>>(self, span: Span, reason: S) -> Self {
        let span_file = (span, self.err_collector.file);
        self.info(span_file, reason)
    }
    pub fn info_obj<Obj: FileKnowingErrorInfoObject>(self, obj: &Obj) -> Self {
        let info = obj.make_global_info(self.err_collector.files);
        self.existing_info(info)
    }
    pub fn info_obj_same_file<Obj: ErrorInfoObject>(self, obj: &Obj) -> Self {
        if let Some(info) = obj.make_info(self.err_collector.file) {
            self.existing_info(info)
        } else {
            self
        }
    }
    pub fn info_obj_different_file<Obj: ErrorInfoObject>(self, obj: &Obj, file: FileUUID) -> Self {
        if let Some(info) = obj.make_info(file) {
            self.existing_info(info)
        } else {
            self
        }
    }
    pub fn add_info_list(&mut self, mut info_list: Vec<ErrorInfo>) {
        self.built_error
            .as_mut()
            .unwrap()
            .infos
            .append(&mut info_list);
    }
    pub fn suggest_replace<S: Into<String>>(self, replace_span: Span, replace_with: S) -> Self {
        self.info_same_file(
            replace_span,
            format!("SUGGEST: Replace this with \"{}\"", replace_with.into()),
        )
    }
    pub fn suggest_remove(self, remove_span: Span) -> Self {
        self.info_same_file(remove_span, "SUGGEST: Remove this")
    }
}

/// This represents objects that can be given as info to an error in a straight-forward way.
pub trait ErrorInfoObject {
    fn make_info(&self, file: FileUUID) -> Option<ErrorInfo>;
}

/// This represents objects that can be given as info to an error in a straight-forward way.
pub trait FileKnowingErrorInfoObject {
    fn make_global_info(&self, files: &ArenaAllocator<FileData, FileUUIDMarker>) -> ErrorInfo;
}

// Trait implementations in the compiler

impl ErrorInfoObject for Declaration {
    fn make_info(&self, file: FileUUID) -> Option<ErrorInfo> {
        Some(ErrorInfo {
            position: self.name_span,
            file,
            info: format!("'{}' declared here", &self.name),
        })
    }
}

impl ErrorInfoObject for SubModuleInstance {
    fn make_info(&self, file: FileUUID) -> Option<ErrorInfo> {
        Some(ErrorInfo {
            position: self.name_span,
            file,
            info: format!("{} declared here", self.name),
        })
    }
}

impl ErrorInfoObject for InterfaceDeclaration {
    fn make_info(&self, file: FileUUID) -> Option<ErrorInfo> {
        Some(ErrorInfo {
            position: self.name_span,
            file,
            info: format!("'{}' declared here", &self.name),
        })
    }
}

impl ErrorInfoObject for Instruction {
    fn make_info(&self, file: FileUUID) -> Option<ErrorInfo> {
        match self {
            Instruction::SubModule(decl) => decl.make_info(file),
            Instruction::Declaration(decl) => decl.make_info(file),
            Instruction::Interface(decl) => decl.make_info(file),
            Instruction::Expression(_) => None,
            _ => unreachable!("At least there shouldn't be cases where we're referring to something other than SubModule or Declaration")
        }
    }
}

impl ErrorInfoObject for DomainInfo {
    fn make_info(&self, file: FileUUID) -> Option<ErrorInfo> {
        Some(ErrorInfo {
            position: self.name_span?,
            file,
            info: format!("Domain '{}' declared here", self.name),
        })
    }
}

impl ErrorInfoObject for Parameter {
    fn make_info(&self, file: FileUUID) -> Option<ErrorInfo> {
        Some(ErrorInfo {
            position: self.name_span,
            file,
            info: format!("Parameter '{}' declared here", self.name),
        })
    }
}

impl ErrorInfoObject for Port {
    fn make_info(&self, file: FileUUID) -> Option<ErrorInfo> {
        Some(ErrorInfo {
            position: self.name_span,
            file,
            info: format!("Port '{}' declared here", &self.name),
        })
    }
}
impl ErrorInfoObject for Interface {
    fn make_info(&self, file: FileUUID) -> Option<ErrorInfo> {
        Some(ErrorInfo {
            position: self.name_span,
            file,
            info: format!("Interface '{}' declared here", &self.name),
        })
    }
}

impl FileKnowingErrorInfoObject for LinkInfo {
    fn make_global_info(&self, _files: &ArenaAllocator<FileData, FileUUIDMarker>) -> ErrorInfo {
        ErrorInfo {
            position: self.name_span,
            file: self.file,
            info: format!("'{}' defined here", &self.name),
        }
    }
}

impl FileKnowingErrorInfoObject for Module {
    fn make_global_info(&self, files: &ArenaAllocator<FileData, FileUUIDMarker>) -> ErrorInfo {
        let ports_str =
            self.make_all_ports_info_string(&files[self.link_info.file].file_text, None);

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
