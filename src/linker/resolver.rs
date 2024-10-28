//! This module provides a safe interface to edit both the current module, and access other modules in the linker.

use std::ops::Index;

use crate::alloc::{UUIDMarker, UUID};

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

pub struct Resolver<'linker, 'err_and_globals, IDM: UUIDMarker, T> {
    arr: &'linker ArenaAllocator<T, IDM>,
    resolved_globals: &'err_and_globals RefCell<ResolvedGlobals>,
}

impl<'linker, 'err_and_globals, IDM: UUIDMarker, T> Index<UUID<IDM>>
    for Resolver<'linker, 'err_and_globals, IDM, T>
where
    NameElem: From<UUID<IDM>>,
{
    type Output = T;

    fn index(&self, index: UUID<IDM>) -> &'linker Self::Output {
        self.resolved_globals
            .borrow_mut()
            .referenced_globals
            .push(NameElem::from(index));
        &self.arr[index]
    }
}

pub struct NameResolver<'linker, 'err_and_globals> {
    pub file_text: &'linker FileText,
    pub errors: &'err_and_globals ErrorCollector<'linker>,
    linker: &'linker Linker,
    resolved_globals: &'err_and_globals RefCell<ResolvedGlobals>,
}

impl<'linker, 'err_and_globals> NameResolver<'linker, 'err_and_globals> {
    /// SAFETY: Files are never touched, and as long as this object is managed properly linker will also exist long enough.
    pub fn resolve_global<'slf>(&'slf self, name_span: Span) -> Option<(NameElem, Span)> {
        let name = &self.file_text[name_span];

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

pub fn make_resolvers<'linker, 'errors>(linker: &'linker Linker, file_text: &'linker FileText, (errors, resolved_globals): &'errors (ErrorCollector<'linker>, RefCell<ResolvedGlobals>)) -> (
    Resolver<'linker, 'errors, ModuleUUIDMarker, Module>,
    Resolver<'linker, 'errors, TypeUUIDMarker, StructType>,
    Resolver<'linker, 'errors, ConstantUUIDMarker, NamedConstant>,
    NameResolver<'linker, 'errors>
) {
    (Resolver {
        arr: &linker.modules,
        resolved_globals,
    },
    Resolver {
        arr: &linker.types,
        resolved_globals,
    },
    Resolver {
        arr: &linker.constants,
        resolved_globals,
    },
    NameResolver {
        file_text,
        linker,
        errors,
        resolved_globals,
    })
}

impl LinkInfo {
    pub fn take_errors_globals_for_editing<'linker>(&mut self, files: &'linker ArenaAllocator<FileData, FileUUIDMarker>) -> (ErrorCollector<'linker>, RefCell<ResolvedGlobals>) {
        let errors = self.errors.take_for_editing(self.file, files);
        let resolved_globals = RefCell::new(self.resolved_globals.take());

        (errors, resolved_globals)
    }

    pub fn reabsorb_errors_globals(&mut self, (errors, resolved_globals): (ErrorCollector, RefCell<ResolvedGlobals>)) {
        // Store errors and resolved_globals back into module
        assert!(self.resolved_globals.is_untouched());
        assert!(self.errors.is_untouched());
        self.resolved_globals = resolved_globals.into_inner();

        self.errors = errors.into_storage();
        self.after_flatten_cp = Some(CheckPoint::checkpoint(
            &self.errors,
            &self.resolved_globals,
        ));
    }
}
