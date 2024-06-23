

use std::cell::RefCell;

use crate::{arena_alloc::ArenaAllocator, file_position::{Span, SpanFile}, flattening::{Declaration, Interface, Module, Port, SubModuleInstance}, linker::{checkpoint::ErrorCheckpoint, FileData, FileUUID, FileUUIDMarker, LinkInfo}};

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
    fn push_diagnostic(&self, position : Span, reason : String, level : ErrorLevel) -> ErrorReference<'_> {
        self.assert_span_good(position);
        
        let mut store = self.error_store.borrow_mut();
        store.did_error |= level == ErrorLevel::Error;
        let pos = store.errors.len();
        store.errors.push(CompileError{ position, reason, infos: Vec::new(), level });
        ErrorReference{ err_collector: self, pos }
    }

    pub fn error<S : Into<String>>(&self, position : Span, reason : S)-> ErrorReference<'_> {
        self.push_diagnostic(position, reason.into(), ErrorLevel::Error)
    }
    
    pub fn warn<S : Into<String>>(&self, position : Span, reason : S)-> ErrorReference<'_> {
        self.push_diagnostic(position, reason.into(), ErrorLevel::Warning)
    }
    
    pub fn todo<S : Into<String>>(&self, position : Span, reason : S)-> ErrorReference<'_> {
        self.push_diagnostic(position, format!("TODO: {}", reason.into()), ErrorLevel::Error)
    }
    
    pub fn did_error(&self) -> bool {
        self.error_store.borrow().did_error
    }
    pub fn set_did_error(&mut self) {
        self.error_store.get_mut().did_error = true;
    }
}

pub struct ErrorReference<'ec> {
    err_collector : &'ec ErrorCollector<'ec>,
    pos : usize
}

impl<'ec> ErrorReference<'ec> {
    pub fn existing_info(&self, error_info : ErrorInfo) -> &Self {
        assert!(error_info.position.debug().into_range().end <= self.err_collector.files[error_info.file].file_text.len());
        self.err_collector.error_store.borrow_mut().errors[self.pos].infos.push(error_info);
        self
    }
    pub fn info<S : Into<String>>(&self, (span, file) : SpanFile, reason : S) -> &Self {
        self.existing_info(ErrorInfo{position : span, file, info : reason.into()})
    }
    pub fn info_same_file<S : Into<String>>(&self, span : Span, reason : S) -> &Self {
        self.info((span, self.err_collector.file), reason)
    }
    pub fn info_obj<Obj : FileKnowingErrorInfoObject>(&self, obj : &Obj) -> &Self {
        let ((position, file), info) = obj.make_global_info(&self.err_collector.files);
        self.existing_info(ErrorInfo{ position, file, info })
    }
    pub fn info_obj_same_file<Obj : ErrorInfoObject>(&self, obj : &Obj) -> &Self {
        let (position, info) = obj.make_info(&self.err_collector.files[self.err_collector.file]);
        self.existing_info(ErrorInfo{ position, file : self.err_collector.file, info })
    }
    pub fn info_obj_different_file<Obj : ErrorInfoObject>(&self, obj : &Obj, file : FileUUID) -> &Self {
        let (position, info) = obj.make_info(&self.err_collector.files[file]);
        self.existing_info(ErrorInfo{ position, file, info })
    }
    pub fn suggest_replace<S : Into<String>>(&self, replace_span : Span, replace_with : S) -> &Self {
        self.info_same_file(replace_span, format!("SUGGEST: Replace this with \"{}\"", replace_with.into()))
    }
    pub fn suggest_remove(&self, remove_span : Span) -> &Self {
        self.info_same_file(remove_span, "SUGGEST: Remove this")
    }
}

/// This represents objects that can be given as info to an error in a straight-forward way. 
pub trait ErrorInfoObject {
    fn make_info(&self, file_data : &FileData) -> (Span, String);
}

pub trait FileKnowingErrorInfoObject {
    fn make_global_info(&self, files : &ArenaAllocator<FileData, FileUUIDMarker>) -> ((Span, FileUUID), String);
}

// Trait implementations in the compiler

impl ErrorInfoObject for Declaration {
    fn make_info(&self, _file_data : &FileData) -> (Span, String) {
        (self.decl_span, format!("'{}' declared here", &self.name))
    }
}

impl ErrorInfoObject for SubModuleInstance {
    fn make_info(&self, _file_data : &FileData) -> (Span, String) {
        if let Some((name, span)) = &self.name {
            (*span, format!("{name} declared here"))
        } else {
            (self.module_name_span, "Used here".to_owned())
        }
    }
}

impl FileKnowingErrorInfoObject for LinkInfo {
    fn make_global_info(&self, _files : &ArenaAllocator<FileData, FileUUIDMarker>) -> ((Span, FileUUID), String) {
        ((self.name_span, self.file), format!("'{}' defined here", &self.name))
    }
}

/// For interfaces of this module
impl FileKnowingErrorInfoObject for (&'_ Module, &'_ Interface) {
    fn make_global_info(&self, _files : &ArenaAllocator<FileData, FileUUIDMarker>) -> ((Span, FileUUID), String) {
        let (md, interface) = *self;
        ((interface.name_span, md.link_info.file), format!("Interface '{}' defined here", &interface.name))
    }
}

impl ErrorInfoObject for Port {
    fn make_info(&self, _file_data : &FileData) -> (Span, String) {
        (self.name_span, format!("Port '{}' declared here", &self.name))
    }
}

impl FileKnowingErrorInfoObject for Module {
    fn make_global_info(&self, files : &ArenaAllocator<FileData, FileUUIDMarker>) -> ((Span, FileUUID), String) {
        let ports_str = self.make_all_ports_info_string(&files[self.link_info.file].file_text, None);
        ((self.link_info.name_span, self.link_info.file), format!("Module '{}' defined here. {}", &self.link_info.name, ports_str))
    }
}
