
//! This module provides a safe interface to edit both the current module, and access other modules in the linker. 

use std::ops::Index;

use self::checkpoint::ResolvedGlobalsCheckpoint;

use super::*;


#[derive(Debug)]
pub struct ResolvedGlobals {
    referenced_globals : Vec<NameElem>,
    all_resolved : bool
}

impl ResolvedGlobals {
    pub fn empty() -> ResolvedGlobals {
        ResolvedGlobals{referenced_globals : Vec::new(), all_resolved : true}
    }
    pub fn take(&mut self) -> ResolvedGlobals {
        std::mem::replace(self, ResolvedGlobals::empty())
    }
    pub fn is_untouched(&self) -> bool {
        self.referenced_globals.is_empty() && self.all_resolved
    }
    pub fn reset_to(&mut self, checkpoint : ResolvedGlobalsCheckpoint) {
        self.referenced_globals.truncate(checkpoint.0);
        self.all_resolved = checkpoint.1;
    }
    pub fn checkpoint(&self) -> ResolvedGlobalsCheckpoint {
        ResolvedGlobalsCheckpoint(self.referenced_globals.len(), self.all_resolved)
    }
}


pub struct Resolver<'linker, 'err_and_globals, IDM : UUIDMarker, T> {
    arr : &'linker ArenaAllocator<T, IDM>,
    resolved_globals : &'err_and_globals RefCell<ResolvedGlobals>
}

impl<'linker, 'err_and_globals, IDM : UUIDMarker, T> Index<UUID<IDM>> for Resolver<'linker, 'err_and_globals, IDM, T> where NameElem : From<UUID<IDM>> {
    type Output = T;

    fn index(&self, index: UUID<IDM>) -> &'linker Self::Output {
        self.resolved_globals.borrow_mut().referenced_globals.push(NameElem::from(index));
        &self.arr[index]
    }
}

pub struct InternalResolver<'linker, 'err_and_globals, IDM : UUIDMarker, T> {
    pub working_on : &'linker mut T,
    arr : *const ArenaAllocator<T, IDM>,
    resolved_globals : &'err_and_globals RefCell<ResolvedGlobals>
}

impl<'linker, 'err_and_globals, IDM : UUIDMarker, T> Index<UUID<IDM>> for InternalResolver<'linker, 'err_and_globals, IDM, T> where NameElem : From<UUID<IDM>> {
    type Output = T;

    fn index<'slf>(&'slf self, index: UUID<IDM>) -> &'slf Self::Output {
        self.resolved_globals.borrow_mut().referenced_globals.push(NameElem::from(index));
        unsafe{&(*self.arr)[index]}
    }
}

pub struct NameResolver<'linker, 'err_and_globals> {
    pub file_text : &'linker FileText,
    pub errors : &'err_and_globals ErrorCollector,
    linker : *const Linker,
    resolved_globals : &'err_and_globals RefCell<ResolvedGlobals>
}

impl<'linker, 'err_and_globals> NameResolver<'linker, 'err_and_globals> {
    /// SAFETY: Files are never touched, and as long as this object is managed properly linker will also exist long enough. 
    pub fn resolve_global<'slf>(&'slf self, name_span : Span) -> ResolvedName<'slf> {
        let name = &self.file_text[name_span];
        let linker = unsafe{&*self.linker};

        let mut resolved_globals = self.resolved_globals.borrow_mut();
        match linker.global_namespace.get(name) {
            Some(NamespaceElement::Global(found)) => {
                resolved_globals.referenced_globals.push(*found);
                ResolvedName{name_elem: Some(*found), linker : self.linker, errors: &self.errors, span: name_span}
            }
            Some(NamespaceElement::Colission(coll)) => {
                resolved_globals.all_resolved = false;

                let decl_infos = coll.iter().map(|collider_global| {
                    let err_loc = linker.get_linking_error_location(*collider_global);
                    if let Some((file, span)) = err_loc.location {
                        error_info(span, file, format!("{} {} declared here", err_loc.named_type, err_loc.full_name))
                    } else {
                        // Kinda hacky, point the 'builtin' back to the declaration location because builtins don't have a location
                        error_info(name_span, self.errors.file, format!("{} {}", err_loc.named_type, err_loc.full_name))
                    }
                }).collect();

                self.errors.error_with_info(name_span, format!("There were colliding imports for the name '{name}'. Pick one and import it by name."), decl_infos);

                ResolvedName{name_elem: None, linker : self.linker, errors: &self.errors, span: name_span}
            }
            None => {
                resolved_globals.all_resolved = false;

                self.errors.error_basic(name_span, format!("No Global of the name '{name}' was found. Did you forget to import it?"));

                ResolvedName{name_elem: None, linker : self.linker, errors: &self.errors, span: name_span}
            }
        }
    }
}

pub struct ResolvedName<'err_and_globals> {
    pub name_elem : Option<NameElem>,
    pub span : Span,
    pub errors : &'err_and_globals ErrorCollector,
    linker : *const Linker
}

impl<'err_and_globals> ResolvedName<'err_and_globals> {
    pub fn not_expected_global_error(self, expected : &str) {
        // SAFETY: The allocated linker objects aren't going to change. 
        let info = unsafe{&*self.linker}.get_linking_error_location(self.name_elem.unwrap());
        let infos = if let Some((file, definition_span)) = info.location {
            vec![error_info(definition_span, file, "Defined here")]
        } else {
            vec![]
        };
        let name = &info.full_name;
        let global_type = info.named_type;
        self.errors.error_with_info(self.span, format!("{name} is not a {expected}, it is a {global_type} instead!"), infos);
    }
    pub fn expect_constant(self) -> Option<ConstantUUID> {
        if let NameElem::Constant(id) = self.name_elem? {
            Some(id)
        } else {
            self.not_expected_global_error("Constant");
            None
        }
    }

    pub fn expect_type(self) -> Option<TypeUUID> {
        if let NameElem::Type(id) = self.name_elem? {
            Some(id)
        } else {
            self.not_expected_global_error("Type");
            None
        }
    }

    pub fn expect_module(self) -> Option<ModuleUUID> {
        if let NameElem::Module(id) = self.name_elem? {
            Some(id)
        } else {
            self.not_expected_global_error("Module");
            None
        }
    }
}



/// pub struct ModuleEditContext<'linker, 'err_and_globals> {
///     pub modules : InternalResolver<'linker, 'err_and_globals, ModuleUUIDMarker, Module>,
///     pub types : Resolver<'linker, 'err_and_globals, TypeUUIDMarker, NamedType>,
///     pub constants : Resolver<'linker, 'err_and_globals, ConstantUUIDMarker, NamedConstant>,
///     pub name_resolver : NameResolver<'linker, 'err_and_globals>,
///     pub errors : &'err_and_globals ErrorCollector
/// }
pub fn with_module_editing_context<F : for<'linker, 'errs> FnOnce(
    InternalResolver<'linker, 'errs, ModuleUUIDMarker, Module>,
    Resolver<'linker, 'errs, TypeUUIDMarker, NamedType>,
    Resolver<'linker, 'errs, ConstantUUIDMarker, NamedConstant>,
    NameResolver<'linker, 'errs>
)>(linker_ptr : *mut Linker, module_uuid : ModuleUUID, f : F) {
    let linker = unsafe{&mut *linker_ptr};
    let linker_modules_ptr : *const _ = &linker.modules;
    let md : &mut Module = &mut linker.modules[module_uuid];
    let file : &FileData = &linker.files[md.link_info.file];

    // Extract errors and resolved_globals for easier editing
    let errors_a = md.link_info.errors.take();
    let resolved_globals_a = RefCell::new(md.link_info.resolved_globals.take());

    let errors = &errors_a;
    let resolved_globals = &resolved_globals_a;

    // Use context
    f(
        InternalResolver{ working_on: md, arr: linker_modules_ptr, resolved_globals},
        Resolver{ arr: &linker.types, resolved_globals },
        Resolver{ arr: &linker.constants, resolved_globals },
        NameResolver{ file_text: &file.file_text, linker: linker_ptr, errors, resolved_globals }
    );

    // Store errors and resolved_globals back into module
    assert!(md.link_info.resolved_globals.is_untouched());
    assert!(md.link_info.errors.is_untouched());
    md.link_info.resolved_globals = resolved_globals_a.into_inner();
    md.link_info.errors = errors_a;
}
