//! This module provides a safe interface to edit both the current module, and access other modules in the linker.

use std::{ops::Index, path};

use crate::typing::template::GlobalReference;

use self::checkpoint::ResolvedGlobalsCheckpoint;

use super::*;

/// See [GlobalResolver]
#[derive(Debug)]
pub struct ResolvedGlobals {
    referenced_globals: Vec<GlobalUUID>,
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

struct LinkingErrorLocation {
    pub named_type: &'static str,
    pub full_name: String,
    pub location: SpanFile,
}

/// This struct encapsulates the concept of name resolution. It reports name-not-found errors,
/// and remembers all of the requested globals in preparation for #49
pub struct GlobalResolver<'linker> {
    linker: &'linker Linker,
    pub file_data: &'linker FileData,

    pub errors: ErrorCollector<'linker>,
    resolved_globals: RefCell<ResolvedGlobals>,
}

impl<'linker> GlobalResolver<'linker> {
    pub fn take_errors_globals(
        linker: &mut Linker,
        global_obj: GlobalUUID,
    ) -> (ErrorStore, ResolvedGlobals) {
        let obj_link_info = Linker::get_link_info_mut(
            &mut linker.modules,
            &mut linker.types,
            &mut linker.constants,
            global_obj,
        );

        let errors = obj_link_info.errors.take();
        let resolved_globals = obj_link_info.resolved_globals.take();

        (errors, resolved_globals)
    }
    pub fn new(
        linker: &'linker Linker,
        obj_link_info: &'linker LinkInfo,
        errors_globals: (ErrorStore, ResolvedGlobals),
    ) -> Self {
        let file_data = &linker.files[obj_link_info.file];

        GlobalResolver {
            linker,
            file_data,
            errors: ErrorCollector::from_storage(
                errors_globals.0,
                obj_link_info.file,
                &linker.files,
            ),
            resolved_globals: RefCell::new(errors_globals.1),
        }
    }
    /// Get the [ErrorCollector] and [ResolvedGlobals] out of this
    pub fn decommission(
        self,
        linker_files: &ArenaAllocator<FileData, FileUUIDMarker>,
    ) -> (ErrorCollector<'_>, ResolvedGlobals) {
        let errors = self.errors.re_attach(linker_files);
        let resolved_globals = self.resolved_globals.into_inner();
        (errors, resolved_globals)
    }

    fn get_linking_error_location(&self, global: GlobalUUID) -> LinkingErrorLocation {
        let named_type = match global {
            GlobalUUID::Module(_) => "Module",
            GlobalUUID::Type(_) => "Struct",
            GlobalUUID::Constant(_) => "Constant",
        };
        let link_info = self.linker.get_link_info(global);
        LinkingErrorLocation {
            named_type,
            full_name: link_info.get_full_name(),
            location: link_info.get_span_file(),
        }
    }

    // generate vec path for namespace ot of vec name span
    pub fn namespace_path_from_span_vec<'slf>(&'slf self, name_path: Vec<Span>) -> Vec<String> {
        let mut path_vec: Vec<String> = vec![];
        for span in name_path {
            path_vec.push(self.file_data.file_text[span].to_owned());
        }
        path_vec.pop();
        path_vec
    }

    // attempts to resolve the name in the namespaces that where imported in the file of the module
    // includes standard imports as well as custom imports specified at the top of a file
    pub fn resolve_imported_namespace<'slf>(&'slf self, name_span: Span) -> Option<GlobalUUID> {
        
        // Program logic
        let mut global: Vec<GlobalUUID> = Vec::new();
        // loop through all imported namespaces for file including default imports
        for possible_namespace in &self.file_data.associated_namespaces{
            if let Some(name_found) = self.try_resolve_possible_namespace(possible_namespace.clone(), name_span) {
                global.push(name_found);
            }
        }

        // Error handeling and returning
        // returns GlobalUUID when one possible candidate found, else error and return none
        let name = self.file_data.file_text[name_span].to_owned();
        let mut resolved_globals = self.resolved_globals.borrow_mut();
        if global.len() == 1 {
            resolved_globals.referenced_globals.push(global[0]);
            Some(global[0])
        } else if global.len() > 1 {
            resolved_globals.all_resolved = false;

            let err_ref = self.errors.error(name_span, format!("There were colliding imports for the name '{name}'. Pick one and import it by full name."));

            for collider_global in global.iter() {
                let err_loc = self.get_linking_error_location(*collider_global);
                err_ref.info(
                    err_loc.location,
                    format!("{} {} declared here", err_loc.named_type, err_loc.full_name),
                );
            }

            None
        } else {
            resolved_globals.all_resolved = false;

            self.errors.error(
                name_span,
                format!(
                    "'{name}' was not found. Did you forget to import the corresponding module?"
                ),
            );

            None
        }
    }

    // Attempts to resolve Name for given Namespace path without Errors for None and Subnamespace
    fn try_resolve_possible_namespace<'slf>(&'slf self, mut path_vec: Vec<String>, name_span: Span) -> Option<GlobalUUID> {
        let name = self.file_data.file_text[name_span].to_owned(); // convert span to actual name, required for nice error messages
        let current_namespace = self.linker.get_subnamespace(&mut path_vec);
        let mut resolved_globals = self.resolved_globals.borrow_mut();
        match current_namespace.get(&name) {
            Some(NamespaceElement::Global(found)) => {
                Some(*found)
            }
            Some(NamespaceElement::Colission(coll)) => {
                resolved_globals.all_resolved = false;

                let err_ref = self.errors.error(name_span, format!("There were colliding imports for the name '{name}'. Pick one and import it by name."));

                for collider_global in coll.iter() {
                    let err_loc = self.get_linking_error_location(*collider_global);
                    err_ref.info(
                        err_loc.location,
                        format!("{} {} declared here", err_loc.named_type, err_loc.full_name),
                    );
                }
                None
            }
            Some(_) => None,
            None => None,
        }
    }

    /// SAFETY: Files are never touched, and as long as this object is managed properly linker will also exist long enough.
    pub fn resolve_full_path<'slf>(&'slf self, mut path_vec: Vec<String>, name_span: Span) -> Option<GlobalUUID> {
        let name = self.file_data.file_text[name_span].to_owned();
        let current_namespace = self.linker.get_subnamespace(&mut path_vec);
        let mut resolved_globals = self.resolved_globals.borrow_mut();

        match current_namespace.get(&name) {
            Some(NamespaceElement::Global(found)) => {
                resolved_globals.referenced_globals.push(*found);
                Some(*found)
            }
            Some(NamespaceElement::Colission(coll)) => {
                resolved_globals.all_resolved = false;

                let err_ref = self.errors.error(name_span, format!("There were colliding imports for the name '{name}'. Pick one and import it by name."));

                for collider_global in coll.iter() {
                    let err_loc = self.get_linking_error_location(*collider_global);
                    err_ref.info(
                        err_loc.location,
                        format!("{} {} declared here", err_loc.named_type, err_loc.full_name),
                    );
                }

                None
            }
            Some(NamespaceElement::Subnamespace(sub)) => {
                resolved_globals.all_resolved = false;
                self.errors.error(
                    name_span,
                    format!(
                        "Subnamespace hit when trying to resolve Name '{name}'"
                    ),
                );
                None
            }
            None => {
                
                resolved_globals.all_resolved = false;

                self.errors.error(
                    name_span,
                    format!(
                        "'{name}' was not found. Did you forget to import the corresponding module?"
                    ),
                );

                None
            }
        }
    }

    pub fn not_expected_global_error<ID: Copy>(
        &self,
        global_ref: &GlobalReference<ID>,
        expected: &str,
    ) where
        GlobalUUID: From<ID>,
    {
        // SAFETY: The allocated linker objects aren't going to change.
        let info = self.get_linking_error_location(GlobalUUID::from(global_ref.id));
        let name = &info.full_name;
        let global_type = info.named_type;
        let err_ref = self.errors.error(
            global_ref.name_span,
            format!("{name} is not a {expected}, it is a {global_type} instead!"),
        );
        err_ref.info(info.location, "Defined here");
    }

    pub fn get_link_info(&self, id: GlobalUUID) -> &LinkInfo {
        self.resolved_globals
            .borrow_mut()
            .referenced_globals
            .push(id);
        self.linker.get_link_info(id)
    }
}

impl Index<ModuleUUID> for GlobalResolver<'_> {
    type Output = Module;

    fn index(&self, index: ModuleUUID) -> &Self::Output {
        self.resolved_globals
            .borrow_mut()
            .referenced_globals
            .push(GlobalUUID::Module(index));

        &self.linker.modules[index]
    }
}
impl Index<TypeUUID> for GlobalResolver<'_> {
    type Output = StructType;

    fn index(&self, index: TypeUUID) -> &Self::Output {
        self.resolved_globals
            .borrow_mut()
            .referenced_globals
            .push(GlobalUUID::Type(index));

        &self.linker.types[index]
    }
}
impl Index<ConstantUUID> for GlobalResolver<'_> {
    type Output = NamedConstant;

    fn index(&self, index: ConstantUUID) -> &Self::Output {
        self.resolved_globals
            .borrow_mut()
            .referenced_globals
            .push(GlobalUUID::Constant(index));

        &self.linker.constants[index]
    }
}

impl LinkInfo {
    pub fn reabsorb_errors_globals(
        &mut self,
        (errors, resolved_globals): (ErrorCollector, ResolvedGlobals),
        checkpoint_id: usize,
    ) {
        // Store errors and resolved_globals back into module
        assert!(self.resolved_globals.is_untouched());
        assert!(self.errors.is_untouched());
        let expected_checkpoint = self.checkpoints.len();
        assert!(expected_checkpoint == checkpoint_id, "In {}: The new checkpoint is not what was expected. The new checkpoint was {checkpoint_id}, whereas the expected next checkpoint is {expected_checkpoint}", self.get_full_name());

        self.resolved_globals = resolved_globals;
        self.errors = errors.into_storage();
        self.checkpoints
            .push(CheckPoint::new(&self.errors, &self.resolved_globals));
    }
}
