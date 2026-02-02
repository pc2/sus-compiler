use crate::append_only_vec::AppendOnlyVec;
use crate::linker::LinkerFiles;
use crate::prelude::*;

use std::cell::Cell;
use std::fmt::Display;

use crate::typing::template::Parameter;

use crate::flattening::{
    ClockInfo, Declaration, Instruction, Interface, InterfaceDeclaration, LatencyDomainInfo,
    Module, Port, SubModuleInstance,
};
use crate::linker::{LinkInfo, checkpoint::ErrorCheckpoint};

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
    pub span: Span,
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

    pub fn take(&mut self) -> Self {
        std::mem::take(self)
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
                .start
                .cmp(&b.position.start)
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
    /// All errors & warnings must be within this span. Of course their infos can point outside it.
    pub context_span: Span,
    pub files: &'linker LinkerFiles,
}

impl<'linker> ErrorCollector<'linker> {
    pub fn new_empty(context_span: Span, files: &'linker LinkerFiles) -> Self {
        Self::from_storage(ErrorStore::new(), context_span, files)
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
        context_span: Span,
        files: &'linker LinkerFiles,
    ) -> Self {
        Self {
            errors: AppendOnlyVec::from(error_store.errors),
            did_error: Cell::new(error_store.did_error),
            context_span,
            files,
        }
    }

    fn push_diagnostic(
        &self,
        position: Span,
        reason: String,
        level: ErrorLevel,
    ) -> ErrorReference<'_> {
        assert!(
            self.context_span.contains(position),
            "Base error span must be within the context span. Error: {position:?}, Context: {:?}",
            self.context_span
        );

        ErrorReference {
            err_collector: self,
            built_error: CompileError {
                position,
                reason,
                infos: Vec::new(),
                level,
            },
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
        context: &'static str,
        position: Span,
        found: impl Display,
        expected: impl Display,
    ) -> ErrorReference<'_> {
        self.error(
            position,
            format!("Typecheck error: In {context}, found {found}, but expected {expected}"),
        )
    }
    pub fn subtype_error(
        &self,
        context: &'static str,
        span: Span,
        found: impl Display,
        expected: impl Display,
    ) -> ErrorReference<'_> {
        self.error(
            span,
            format!(
                "Typecheck error: In {context}, found {found}, which is not a subtype of the expected type {expected}"
            ),
        )
    }

    pub fn did_error(&self) -> bool {
        self.did_error.get()
    }
    pub fn set_did_error(&self) {
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
///     errors.warn(span, "Unused Variable").info(span2, "In module").info(blablabla)
pub struct ErrorReference<'ec> {
    err_collector: &'ec ErrorCollector<'ec>,
    built_error: CompileError,
}

// This is the trick, the error is only added to the ErrorCollector when the ErrorReference is dropped.
impl<'ec> Drop for ErrorReference<'ec> {
    fn drop(&mut self) {
        let default_err = CompileError {
            position: Span::PLACEHOLDER,
            reason: String::new(),
            infos: Vec::new(),
            level: ErrorLevel::Error,
        };
        let built_error = std::mem::replace(&mut self.built_error, default_err);
        self.err_collector
            .did_error
            .set(self.err_collector.did_error.get() | (built_error.level == ErrorLevel::Error));

        self.err_collector.errors.push(built_error);
    }
}

impl ErrorReference<'_> {
    pub fn info<S: Into<String>>(&mut self, span: Span, reason: S) -> &mut Self {
        self.built_error.infos.push(ErrorInfo {
            span,
            info: reason.into(),
        });
        self
    }
    pub fn info_obj<Obj: ErrorInfoObject>(&mut self, obj: Obj) -> &mut Self {
        if let Some(info) = obj.make_info() {
            self.built_error.infos.push(info);
        }

        self
    }
    pub fn add_info_list(&mut self, mut info_list: Vec<ErrorInfo>) {
        self.built_error.infos.append(&mut info_list);
    }
    pub fn suggest_replace<S: Into<String>>(
        &mut self,
        replace_span: Span,
        replace_with: S,
    ) -> &mut Self {
        self.info(
            replace_span,
            format!("SUGGEST: Replace this with \"{}\"", replace_with.into()),
        )
    }
    pub fn suggest_remove(&mut self, remove_span: Span) -> &mut Self {
        self.info(remove_span, "SUGGEST: Remove this")
    }
}

/// This represents objects that can be given as info to an error in a straight-forward way.
pub trait ErrorInfoObject {
    fn make_info(self) -> Option<ErrorInfo>;
}

/// Simplify [ErrorReference::info_obj]
impl ErrorInfoObject for ErrorInfo {
    fn make_info(self) -> Option<ErrorInfo> {
        Some(self)
    }
}

impl ErrorInfoObject for &Declaration {
    fn make_info(self) -> Option<ErrorInfo> {
        Some(ErrorInfo {
            span: self.name_span,
            info: format!("'{}' declared here", &self.name),
        })
    }
}

impl ErrorInfoObject for &SubModuleInstance {
    fn make_info(self) -> Option<ErrorInfo> {
        Some(ErrorInfo {
            span: self.name_span,
            info: format!("{} declared here", self.name),
        })
    }
}

impl ErrorInfoObject for &InterfaceDeclaration {
    fn make_info(self) -> Option<ErrorInfo> {
        Some(ErrorInfo {
            span: self.name_span,
            info: format!("'{}' declared here", &self.name),
        })
    }
}

impl ErrorInfoObject for &Instruction {
    fn make_info(self) -> Option<ErrorInfo> {
        match self {
            Instruction::SubModule(decl) => decl.make_info(),
            Instruction::Declaration(decl) => decl.make_info(),
            Instruction::Interface(decl) => decl.make_info(),
            Instruction::Expression(_) => None,
            _ => unreachable!(
                "At least there shouldn't be cases where we're referring to something other than SubModule or Declaration"
            ),
        }
    }
}

impl ErrorInfoObject for &ClockInfo {
    fn make_info(self) -> Option<ErrorInfo> {
        Some(ErrorInfo {
            span: self.name_span?,
            info: format!("Clock '{}' declared here", self.name),
        })
    }
}

impl ErrorInfoObject for &LatencyDomainInfo {
    fn make_info(self) -> Option<ErrorInfo> {
        Some(ErrorInfo {
            span: self.name_span?,
            info: format!("Latency Domain '{}' declared here", self.name),
        })
    }
}

impl ErrorInfoObject for &Parameter {
    fn make_info(self) -> Option<ErrorInfo> {
        Some(ErrorInfo {
            span: self.name_span,
            info: format!("Parameter '{}' declared here", self.name),
        })
    }
}

impl ErrorInfoObject for &Port {
    fn make_info(self) -> Option<ErrorInfo> {
        Some(ErrorInfo {
            span: self.name_span,
            info: format!("Port '{}' declared here", &self.name),
        })
    }
}
impl ErrorInfoObject for &Interface {
    fn make_info(self) -> Option<ErrorInfo> {
        Some(ErrorInfo {
            span: self.name_span,
            info: format!("Interface '{}' declared here", &self.name),
        })
    }
}

impl ErrorInfoObject for &LinkInfo {
    fn make_info(self) -> Option<ErrorInfo> {
        Some(ErrorInfo {
            span: self.name_span,
            info: format!("'{}' defined here", &self.name),
        })
    }
}

impl ErrorInfoObject for (&Module, &LinkerFiles) {
    fn make_info(self) -> Option<ErrorInfo> {
        let (md, files) = self;
        let ports_str = md.display_all_ports_info(&files[md.link_info.span.file].file_text);

        Some(ErrorInfo {
            span: md.link_info.name_span,
            info: format!(
                "Module '{}' defined here. {}",
                &md.link_info.name, ports_str
            ),
        })
    }
}
