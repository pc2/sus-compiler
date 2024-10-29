//! This module provides a safe interface to edit both the current module, and access other modules in the linker.

use std::ops::Index;

use self::checkpoint::ResolvedGlobalsCheckpoint;

use super::*;

#[derive(Debug)]
pub struct ResolvedGlobals {
    referenced_globals: Vec<NameElem>,
    all_resolved: bool,
}

impl ResolvedGlobals {
    pub fn empty() -> ResolvedGlobals {
        ResolvedGlobals {
            referenced_globals: Vec::new(),
            all_resolved: true,
        }
    }
    pub fn take(&mut self) -> ResolvedGlobals {
        std::mem::replace(self, ResolvedGlobals::empty())
    }
    pub fn is_untouched(&self) -> bool {
        self.referenced_globals.is_empty() && self.all_resolved
    }
    pub fn reset_to(&mut self, checkpoint: ResolvedGlobalsCheckpoint) {
        self.referenced_globals.truncate(checkpoint.0);
        self.all_resolved = checkpoint.1;
    }
    pub fn checkpoint(&self) -> ResolvedGlobalsCheckpoint {
        ResolvedGlobalsCheckpoint(self.referenced_globals.len(), self.all_resolved)
    }
}

pub struct GlobalResolver<'linker> {
    linker: &'linker Linker,
    pub file_data: &'linker FileData,
    pub obj_link_info: &'linker LinkInfo,

    pub errors: ErrorCollector<'linker>,
    pub resolved_globals: RefCell<ResolvedGlobals>
}

impl<'linker> GlobalResolver<'linker> {
    pub fn take_errors_globals(linker: &mut Linker, global_obj: NameElem) -> (ErrorStore, ResolvedGlobals) {
        let obj_link_info = Linker::get_link_info_mut(&mut linker.modules, &mut linker.types, global_obj).unwrap();

        let errors = obj_link_info.errors.take();
        let resolved_globals = obj_link_info.resolved_globals.take();

        (errors, resolved_globals)
    }
    pub fn new(linker: &'linker Linker, global_obj: NameElem, errors_globals: (ErrorStore, ResolvedGlobals)) -> Self {
        let obj_link_info = linker.get_link_info(global_obj).unwrap();

        let file_data = &linker.files[obj_link_info.file];

        GlobalResolver {
            linker,
            file_data,
            obj_link_info,
            errors: ErrorCollector::from_storage(errors_globals.0, obj_link_info.file, &linker.files),
            resolved_globals: RefCell::new(errors_globals.1),
        }
    }
    /// Get the [ErrorCollector] and [ResolvedGlobals] out of this
    pub fn decommission<'linker_files>(self, linker_files: &'linker_files ArenaAllocator<FileData, FileUUIDMarker>) -> (ErrorCollector<'linker_files>, ResolvedGlobals) {
        let errors = self.errors.re_attach(linker_files);
        let resolved_globals = self.resolved_globals.into_inner();
        (errors, resolved_globals)
    }

    /// SAFETY: Files are never touched, and as long as this object is managed properly linker will also exist long enough.
    pub fn resolve_global<'slf>(&'slf self, name_span: Span) -> Option<(NameElem, Span)> {
        let name = &self.file_data.file_text[name_span];

        let mut resolved_globals = self.resolved_globals.borrow_mut();
        match self.linker.global_namespace.get(name) {
            Some(NamespaceElement::Global(found)) => {
                resolved_globals.referenced_globals.push(*found);
                Some((*found, name_span))
            }
            Some(NamespaceElement::Colission(coll)) => {
                resolved_globals.all_resolved = false;

                let err_ref = self.errors.error(name_span, format!("There were colliding imports for the name '{name}'. Pick one and import it by name."));

                for collider_global in coll.iter() {
                    let err_loc = self.linker.get_linking_error_location(*collider_global);
                    if let Some(span_file) = err_loc.location {
                        err_ref.info(
                            span_file,
                            format!("{} {} declared here", err_loc.named_type, err_loc.full_name),
                        );
                    } else {
                        // Kinda hacky, point the 'builtin' back to the declaration location because builtins don't have a location
                        err_ref.info_same_file(
                            name_span,
                            format!("{} {}", err_loc.named_type, err_loc.full_name),
                        );
                    }
                }

                None
            }
            None => {
                resolved_globals.all_resolved = false;

                self.errors.error(
                    name_span,
                    format!(
                        "No Global of the name '{name}' was found. Did you forget to import it?"
                    ),
                );

                None
            }
        }
    }

    pub fn get_linking_error_location(&self, name_elem: NameElem) -> LinkingErrorLocation {
        self.linker.get_linking_error_location(name_elem)
    }
    pub fn not_expected_global_error(&self, name_elem: NameElem, span: Span, expected: &str) {
        // SAFETY: The allocated linker objects aren't going to change.
        let info = self.get_linking_error_location(name_elem);
        let name = &info.full_name;
        let global_type = info.named_type;
        let err_ref = self.errors.error(
            span,
            format!("{name} is not a {expected}, it is a {global_type} instead!"),
        );
        if let Some(span_file) = info.location {
            err_ref.info(span_file, "Defined here");
        }
    }
}

impl<'l> Index<ModuleUUID> for GlobalResolver<'l> {
    type Output = Module;

    fn index(&self, index: ModuleUUID) -> &Self::Output {
        self.resolved_globals.borrow_mut()
            .referenced_globals
            .push(NameElem::Module(index));

        &self.linker.modules[index]
    }
}
impl<'l> Index<TypeUUID> for GlobalResolver<'l> {
    type Output = StructType;

    fn index(&self, index: TypeUUID) -> &Self::Output {
        self.resolved_globals.borrow_mut()
            .referenced_globals
            .push(NameElem::Type(index));

        &self.linker.types[index]
    }
}
impl<'l> Index<ConstantUUID> for GlobalResolver<'l> {
    type Output = NamedConstant;

    fn index(&self, index: ConstantUUID) -> &Self::Output {
        self.resolved_globals.borrow_mut()
            .referenced_globals
            .push(NameElem::Constant(index));

        &self.linker.constants[index]
    }
}

impl LinkInfo {
    pub fn reabsorb_errors_globals(&mut self, (errors, resolved_globals): (ErrorCollector, ResolvedGlobals), checkpoint_id: usize) {
        // Store errors and resolved_globals back into module
        assert!(self.resolved_globals.is_untouched());
        assert!(self.errors.is_untouched());
        let expected_checkpoint = self.checkpoints.len();
        assert_eq!(expected_checkpoint, checkpoint_id, "The new checkpoint is not what was expected. The new checkpoint was {checkpoint_id}, whereas the expected next checkpoint is {expected_checkpoint}");
        
        self.resolved_globals = resolved_globals;
        self.errors = errors.into_storage();
        self.checkpoints.push(CheckPoint::checkpoint(
            &self.errors,
            &self.resolved_globals,
        ));
    }
}
