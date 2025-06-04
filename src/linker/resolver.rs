//! This module provides a safe interface to edit both the current module, and access other modules in the linker.

use std::ops::Index;

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

    pub errors: ErrorCollector<'linker>,
    resolved_globals: RefCell<ResolvedGlobals>,
}

impl<'linker> GlobalResolver<'linker> {
    pub fn new(
        linker: &'linker Linker,
        obj_link_info: &'linker LinkInfo,
        errors_globals: (ErrorStore, ResolvedGlobals),
    ) -> Self {
        GlobalResolver {
            linker,
            errors: ErrorCollector::from_storage(
                errors_globals.0,
                obj_link_info.file,
                &linker.files,
            ),
            resolved_globals: RefCell::new(errors_globals.1),
        }
    }
    /// Get the [ErrorCollector] and [ResolvedGlobals] out of this
    pub fn decommission(self) -> (ErrorStore, ResolvedGlobals) {
        (
            self.errors.into_storage(),
            self.resolved_globals.into_inner(),
        )
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

    /// SAFETY: Files are never touched, and as long as this object is managed properly linker will also exist long enough.
    pub fn resolve_global(&self, name_span: Span, name: &str) -> Option<GlobalUUID> {
        let mut resolved_globals = self.resolved_globals.borrow_mut();
        match self.linker.global_namespace.get(name) {
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
    pub fn take_errors_globals(&mut self) -> (ErrorStore, ResolvedGlobals) {
        let errors = self.errors.take();
        let resolved_globals = self.resolved_globals.take();

        (errors, resolved_globals)
    }
    pub fn reabsorb_errors_globals(
        &mut self,
        (errors, resolved_globals): (ErrorStore, ResolvedGlobals),
        checkpoint_id: usize,
    ) {
        // Store errors and resolved_globals back into module
        assert!(self.resolved_globals.is_untouched());
        assert!(self.errors.is_untouched());
        let expected_checkpoint = self.checkpoints.len();
        assert!(expected_checkpoint == checkpoint_id, "In {}: The new checkpoint is not what was expected. The new checkpoint was {checkpoint_id}, whereas the expected next checkpoint is {expected_checkpoint}", self.get_full_name());

        self.resolved_globals = resolved_globals;
        self.errors = errors;
        self.checkpoints
            .push(CheckPoint::new(&self.errors, &self.resolved_globals));
    }
}
