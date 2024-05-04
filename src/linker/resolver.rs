
//! This module provides a safe interface to edit both the current module, and access other modules in the linker. 

use std::ops::Deref;

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
    pub fn is_untouched(&self) -> bool {
        self.referenced_globals.is_empty() && self.all_resolved
    }
}


/// SAFETY: [Linker::modules] itself is never modified
pub struct UnsafeGlobalResolver {
    linker : *const Linker,
    file_text : *const FileText,
    pub errors : ErrorCollector,
    resolved_globals : RefCell<ResolvedGlobals>
}

impl UnsafeGlobalResolver {
    /// SAFETY: Files are never touched, and as long as this object is managed properly linker will also exist long enough. 
    pub fn resolve_global<'slf>(&'slf self, name_span : Span) -> ResolvedName<'slf> {
        let name = &unsafe{&*self.file_text}[name_span];
        let linker = unsafe{&*self.linker};

        let mut resolved_globals = self.resolved_globals.borrow_mut();
        match linker.global_namespace.get(name) {
            Some(NamespaceElement::Global(found)) => {
                resolved_globals.referenced_globals.push(*found);
                ResolvedName{name_elem: Some(*found), resolver: self, span: name_span}
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

                ResolvedName{name_elem: None, resolver: self, span: name_span}
            }
            None => {
                resolved_globals.all_resolved = false;

                self.errors.error_basic(name_span, format!("No Global of the name '{name}' was found. Did you forget to import it?"));

                ResolvedName{name_elem: None, resolver: self, span: name_span}
            }
        }
    }

    /// SAFETY: User must cast the pointer to a safe reference themselves. 
    /// 
    /// That means, if the user has a mutable module, it needs to return a tighter lifetime to prevent module editing. 
    /// 
    /// Otherwise this can be 'linker
    unsafe fn get_module_unsafe(&self, index: ModuleUUID) -> *const Module {
        &unsafe{&*self.linker}.modules[index]
    }
    /// SAFETY: User must cast the pointer to a safe reference themselves. 
    /// 
    /// That means, if the user has a mutable type, it needs to return a tighter lifetime to prevent type editing. 
    /// 
    /// Otherwise this can be 'linker
    unsafe fn get_type_unsafe(&self, index: TypeUUID) -> *const NamedType {
        &unsafe{&*self.linker}.types[index]
    }
    /// SAFETY: User must cast the pointer to a safe reference themselves. 
    /// 
    /// That means, if the user has a mutable constant, it needs to return a tighter lifetime to prevent constant editing. 
    /// 
    /// Otherwise this can be 'linker
    unsafe fn get_constant_unsafe(&self, index: ConstantUUID) -> *const NamedConstant {
        &unsafe{&*self.linker}.constants[index]
    }
}

pub struct ResolvedName<'l> {
    pub name_elem : Option<NameElem>,
    pub span : Span,
    pub resolver : &'l UnsafeGlobalResolver
}

impl<'l> ResolvedName<'l> {
    pub fn not_expected_global_error(self, expected : &str) {
        let info = unsafe{&*self.resolver.linker}.get_linking_error_location(self.name_elem.unwrap());
        let infos = if let Some((file, definition_span)) = info.location {
            vec![error_info(definition_span, file, "Defined here")]
        } else {
            vec![]
        };
        let name = &info.full_name;
        let global_type = info.named_type;
        self.resolver.errors.error_with_info(self.span, format!("{name} is not a {expected}, it is a {global_type} instead!"), infos);
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



pub struct ModuleEditContext<'md, 'linker> {
    /// The module we are currently editing
    pub md : &'md mut Module,

    pub file : &'linker FileData,

    resolver : UnsafeGlobalResolver
}

impl<'md, 'linker> ModuleEditContext<'md ,'linker> {
    /// See [ModuleEditContext::drop]
    pub fn new(linker_ptr : *const Linker, file : &'linker FileData, md : &'md mut Module) -> Self {
        Self {
            file,
            resolver : UnsafeGlobalResolver {
                linker : linker_ptr,
                file_text : &file.file_text,
                errors : md.link_info.errors.take(),
                resolved_globals : RefCell::new(ResolvedGlobals::empty())
            },
            md
        }
    }

    /// Returns the requested module. 
    /// 
    /// SAFETY
    /// 
    /// We cleverly restrict the lifetime to self's lifetime. 
    /// 
    /// This prohibits mutating [Self::md] while holding a reference to another module (or itself)
    pub fn get_module<'s>(&'s self, module_uuid : ModuleUUID) -> &'s Module {
        unsafe {
            &*self.resolver.get_module_unsafe(module_uuid)
        }
    }

    /// Returns a type from the requested id
    /// 
    /// This module can return a 'linker, since we're only mutating Modules, thus not preventing mutable access to [Self::md]
    #[allow(dead_code)]
    pub fn get_type(&self, type_uuid : TypeUUID) -> &'linker NamedType {
        unsafe {
            &*self.resolver.get_type_unsafe(type_uuid)
        }
    }

    /// Returns a type from the requested id
    /// 
    /// This module can return a 'linker, since we're only mutating Modules, thus not preventing mutable access to [Self::md]
    #[allow(dead_code)]
    pub fn get_constant(&self, constant_uuid : ConstantUUID) -> &'linker NamedConstant {
        unsafe {
            &*self.resolver.get_constant_unsafe(constant_uuid)
        }
    }
}

/// Don't actually need [::core::ops::DerefMut]
impl<'md, 'linker> Deref for ModuleEditContext<'md, 'linker> {
    type Target = UnsafeGlobalResolver;

    fn deref(&self) -> &Self::Target {
        &self.resolver
    }
}

impl<'md, 'linker> Drop for ModuleEditContext<'md, 'linker> {
    /// Places errors and resolved globals back in Module's LinkInfo
    /// 
    /// See [ModuleEditContext::new]
    fn drop(&mut self) {
        // Make sure nothing has been stored in these in the meantime
        assert!(self.md.link_info.resolved_globals.is_untouched());
        assert!(self.md.link_info.errors.is_untouched());
        self.md.link_info.resolved_globals = self.resolved_globals.replace(ResolvedGlobals::empty());
        self.md.link_info.errors = self.resolver.errors.take();
    }
}
