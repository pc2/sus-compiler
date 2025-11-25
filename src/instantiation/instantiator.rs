use super::*;

use std::collections::BTreeMap;
use std::rc::Rc;

use crate::alloc::ArenaAllocator;
use crate::errors::CompileError;
use crate::linker::{FileData, LinkerGlobals};
use crate::typing::concrete_type::ConcreteGlobalReference;

use super::InstantiatedModule;

/// Stored per module [crate::flattening::Module].
/// With this you can instantiate a module for different sets of template arguments.
/// It caches the instantiations that have been made, such that they need not be repeated.
///
/// Also, with incremental builds (#49) this will be a prime area for investigation
#[derive(Debug, Default)]
pub struct Instantiator {
    cache: BTreeMap<Rc<ConcreteGlobalReference<ModuleUUID>>, Rc<InstantiatedModule>>,
}

impl Instantiator {
    pub fn new() -> Self {
        Self {
            cache: BTreeMap::new(),
        }
    }

    pub fn clear_instances(&mut self) {
        self.cache.clear()
    }

    pub fn instantiate(
        &mut self,
        linker_globals: &LinkerGlobals,
        linker_files: &ArenaAllocator<FileData, FileUUIDMarker>,
        object_id: ConcreteGlobalReference<ModuleUUID>,
    ) -> Option<Rc<InstantiatedModule>> {
        let instance = if let Some(found) = self.cache.get(&object_id) {
            found.clone()
        } else {
            let global_ref = Rc::new(object_id);

            let name = global_ref.display(linker_globals).to_string();

            let md = &linker_globals.modules[global_ref.id];
            let file = &linker_files[md.link_info.file];
            let result = crate::debug::debug_context("instantiating", name, file, || {
                match start_instantiation(linker_globals, linker_files, global_ref.clone()) {
                    Ok(mut context) => {
                        loop {
                            let submodules_to_instantiate = context.typecheck_step();
                            if submodules_to_instantiate.is_empty() {
                                break;
                            }
                            for (sm_id, sm_ref) in submodules_to_instantiate {
                                let instance =
                                    self.instantiate(linker_globals, linker_files, sm_ref);
                                context.apply_instantiated_submodule(sm_id, instance);
                            }
                        }
                        finish_instantiation(context)
                    }
                    Err(early_exit) => early_exit,
                }
            });

            if result.errors.did_error {
                error!("Failed to instantiate {}", result.name);
            } else {
                info!("Instantiated {}", result.name);
            }

            let result_ref = Rc::new(result);
            assert!(self.cache.insert(global_ref, result_ref.clone()).is_none());
            result_ref
        };

        if !instance.errors.did_error {
            Some(instance.clone())
        } else {
            None
        }
    }

    pub fn for_each_error(&self, func: &mut impl FnMut(&CompileError)) {
        for inst in self.cache.values() {
            for err in &inst.errors {
                func(err)
            }
        }
    }

    // Also passes over invalid instances. Instance validity should not be assumed!
    // Only used for things like syntax highlighting
    pub fn iter(
        &self,
    ) -> impl Iterator<
        Item = (
            &Rc<ConcreteGlobalReference<ModuleUUID>>,
            &Rc<InstantiatedModule>,
        ),
    > {
        self.cache.iter()
    }

    // Also passes over invalid instances. Instance validity should not be assumed!
    // Only used for things like syntax highlighting
    pub fn iter_for_module(
        &self,
        md_id: ModuleUUID,
    ) -> impl Iterator<
        Item = (
            &Rc<ConcreteGlobalReference<ModuleUUID>>,
            &Rc<InstantiatedModule>,
        ),
    > {
        self.cache.iter().filter(move |kv| kv.0.id == md_id)
    }
}

impl Executed {
    pub fn into_module_typing_context<'l>(
        self,
        globals: &'l LinkerGlobals,
        linker_files: &'l ArenaAllocator<FileData, FileUUIDMarker>,
        md: &'l Module,
        global_ref: Rc<ConcreteGlobalReference<ModuleUUID>>,
        name: String,
    ) -> (ModuleTypingContext<'l>, ValueUnifierAlloc) {
        let errors = ErrorCollector::new_empty(md.link_info.file, linker_files);
        if let Err((position, reason)) = self.execution_status {
            errors.error(position, reason);
        }
        let ctx = ModuleTypingContext {
            mangled_name: mangle_name(&name),
            name,
            global_ref,
            wires: self.wires,
            submodules: self.submodules,
            generation_state: self.generation_state,
            md,
            link_info: &md.link_info,
            globals,
            errors,
        };
        (ctx, self.type_var_alloc)
    }
}

/// Mangle the module name for use in code generation
fn mangle_name(str: &str) -> String {
    let mut result = String::with_capacity(str.len());

    let mut last_was_underscore = false;
    for c in str.chars() {
        if c.is_alphanumeric() {
            result.push(c);
            last_was_underscore = false;
        } else {
            // Max 1 underscore at a time, as some tools don't like it (#128)
            if !last_was_underscore {
                result.push('_');
            }
            last_was_underscore = true;
        }
    }
    result.trim_matches('_').to_owned()
}

#[allow(clippy::result_large_err)]
fn start_instantiation<'l>(
    linker_globals: &'l LinkerGlobals,
    linker_files: &'l ArenaAllocator<FileData, FileUUIDMarker>,
    global_ref: Rc<ConcreteGlobalReference<ModuleUUID>>,
) -> Result<ModuleTypingSuperContext<'l>, InstantiatedModule> {
    let md = &linker_globals.modules[global_ref.id];

    let name = global_ref.display(linker_globals).to_string();

    // Don't instantiate modules that already errored. Otherwise instantiator may crash
    if md.link_info.errors.did_error {
        let mut errors = ErrorCollector::new_empty(md.link_info.file, linker_files);
        errors.set_did_error();
        let msg = format!("Not Instantiating {name} due to abstract typing errors");
        errors.warn(md.link_info.name_span, msg);
        return Err(InstantiatedModule {
            global_ref,
            mangled_name: mangle_name(&name),
            name,
            errors: errors.into_storage(),
            interface_ports: Default::default(),
            wires: Default::default(),
            submodules: Default::default(),
            generation_state: md
                .link_info
                .instructions
                .map(|_| SubModuleOrWire::Unassigned),
        });
    }
    let submodules_with_abs_type_errors: HashSet<_> = md
        .link_info
        .resolved_globals
        .referenced_globals
        .iter()
        .filter_map(|global| {
            let found_link_info: &LinkInfo = &linker_globals[*global];

            found_link_info
                .errors
                .did_error
                .then(|| found_link_info.display_full_name().to_string())
        })
        .collect();

    if !submodules_with_abs_type_errors.is_empty() {
        let mut errors = ErrorCollector::new_empty(md.link_info.file, linker_files);
        errors.set_did_error();
        let mut msg =
            format!("Not Instantiating {name} due to abstract typing errors of submodules:\n");
        for s in submodules_with_abs_type_errors {
            writeln!(msg, "- {s}").unwrap();
        }
        errors.warn(md.link_info.name_span, msg);

        return Err(InstantiatedModule {
            global_ref,
            mangled_name: mangle_name(&name),
            name,
            errors: errors.into_storage(),
            interface_ports: Default::default(),
            wires: Default::default(),
            submodules: Default::default(),
            generation_state: md
                .link_info
                .instructions
                .map(|_| SubModuleOrWire::Unassigned),
        });
    }

    debug!("Executing {name}");
    let exec = execute::execute(&md.link_info, linker_globals, &global_ref.template_args);

    let (typed, type_var_alloc) =
        exec.into_module_typing_context(linker_globals, linker_files, md, global_ref, name);
    let name = &typed.name;

    if typed.errors.did_error() {
        return Err(typed.into_instantiated_module());
    }

    if crate::debug::is_enabled("print-concrete-pre-typecheck") {
        eprintln!("[[Executed {name}]]");
        typed.print_instantiated_module();
    }

    debug!("Concrete Typechecking {name}");
    Ok(ModuleTypingSuperContext::start_typechecking(
        typed,
        type_var_alloc,
    ))
}
fn finish_instantiation(context: ModuleTypingSuperContext) -> InstantiatedModule {
    let typed = context.finish();
    let name = &typed.name;

    if crate::debug::is_enabled("print-concrete") {
        eprintln!("[[Instantiated {name}]]");
        typed.print_instantiated_module();
    }

    if typed.errors.did_error() {
        return typed.into_instantiated_module();
    }

    debug!("Checking array accesses {name}");
    typed.check_subtypes();

    typed.into_instantiated_module()
}
