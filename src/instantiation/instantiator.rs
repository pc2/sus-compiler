use super::*;

use std::collections::BTreeMap;
use std::fmt::Display;
use std::rc::Rc;

use crate::config::config;
use crate::linker::{LinkerFiles, LinkerGlobals};
use crate::to_string::FmtWrapper;
use crate::typing::concrete_type::ConcreteGlobalReference;

use super::InstantiatedModule;

/// Stored per module [crate::flattening::Module].
/// With this you can instantiate a module for different sets of template arguments.
/// It caches the instantiations that have been made, such that they need not be repeated.
///
/// Also, with incremental builds (#49) this will be a prime area for investigation
#[derive(Debug, Default)]
pub struct Instantiator {
    cache: BTreeMap<Rc<ConcreteGlobalReference<ModuleUUID>>, InstantiatorCacheElem>,
    stack: Vec<Rc<ConcreteGlobalReference<ModuleUUID>>>,
}

#[derive(Debug)]
enum InstantiatorCacheElem {
    InProgress,
    Done(Rc<InstantiatedModule>),
}
impl InstantiatorCacheElem {
    pub fn unwrap(&self) -> &Rc<InstantiatedModule> {
        let_unwrap!(Self::Done(v), self);
        v
    }
}

pub enum InstantiateError {
    ErrorInModule,
    RecursionLimitExceeded { message: String },
}

impl Instantiator {
    pub fn new() -> Self {
        Self {
            cache: BTreeMap::new(),
            stack: Vec::new(),
        }
    }

    pub fn clear_instances(&mut self) {
        self.cache.clear()
    }

    pub fn display_cur_stack(&self, globals: &LinkerGlobals) -> impl Display {
        FmtWrapper(|f| {
            write!(f, "Current Instantiation Stack:")?;
            for e in &self.stack {
                write!(f, "\n- {}", e.display(globals))?;
            }
            Ok(())
        })
    }

    pub fn instantiate(
        &mut self,
        globals: &LinkerGlobals,
        linker_files: &LinkerFiles,
        object_id: ConcreteGlobalReference<ModuleUUID>,
    ) -> Result<Rc<InstantiatedModule>, InstantiateError> {
        let instance = if let Some(found) = self.cache.get(&object_id) {
            match found {
                InstantiatorCacheElem::Done(instantiated_module) => instantiated_module.clone(),
                InstantiatorCacheElem::InProgress => {
                    let obj_name = object_id.display(globals);
                    return Err(InstantiateError::RecursionLimitExceeded {
                        message: format!(
                            "{obj_name} depends on itself! Infinite Submodule Recursion is not allowed.\n{}\n- {obj_name}",
                            self.display_cur_stack(globals)
                        ),
                    });
                }
            }
        } else {
            let recursion_limit = config().recursion_limit;
            if self.stack.len() > recursion_limit {
                // Make sure we trigger on an actually recursive call, and not on the submodules of some innocent non-recursive submodule used within a deep recursion.
                // "10" occurences of this module name in the recursion stack seems good enough to call it "part of the bad recursion"
                if self.stack.iter().filter(|e| e.id == object_id.id).count() > 10 {
                    return Err(InstantiateError::RecursionLimitExceeded {
                        message: format!(
                            "Recursion limit ({recursion_limit}) reached! If a deeply nested recursion is intended, pass a higher value for `--recursion-limit`.\n{}",
                            self.display_cur_stack(globals)
                        ),
                    });
                }
            }
            let global_ref = Rc::new(object_id);
            self.stack.push(global_ref.clone());
            assert!(
                self.cache
                    .insert(global_ref.clone(), InstantiatorCacheElem::InProgress)
                    .is_none()
            );

            let name = global_ref.display(globals).to_string();

            let result = crate::debug::debug_context("instantiating", name, || {
                match start_instantiation(globals, linker_files, global_ref.clone()) {
                    Ok(mut context) => {
                        loop {
                            let submodules_to_instantiate = context.typecheck_step();
                            if submodules_to_instantiate.is_empty() {
                                break;
                            }
                            for (sm_id, sm_ref) in submodules_to_instantiate {
                                let instance = self.instantiate(globals, linker_files, sm_ref);
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

            self.stack.pop(); // should pop the global_ref we pushed to earlier
            let result = Rc::new(result);
            let_unwrap!(
                Some(InstantiatorCacheElem::InProgress),
                self.cache
                    .insert(global_ref, InstantiatorCacheElem::Done(result.clone()))
            );
            result
        };

        if !instance.errors.did_error {
            Ok(instance)
        } else {
            Err(InstantiateError::ErrorInModule)
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
        self.cache.iter().map(|v| (v.0, v.1.unwrap()))
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
        self.cache
            .iter()
            .filter(move |kv| kv.0.id == md_id)
            .map(|v| (v.0, v.1.unwrap()))
    }
}

impl Executed {
    pub fn into_module_typing_context<'l>(
        self,
        globals: &'l LinkerGlobals,
        linker_files: &'l LinkerFiles,
        md: &'l Module,
        global_ref: Rc<ConcreteGlobalReference<ModuleUUID>>,
        name: String,
    ) -> ModuleTypingContext<'l> {
        let errors = ErrorCollector::new_empty(md.link_info.span, linker_files);
        if let Err((position, reason)) = self.execution_status {
            errors.error(position, reason);
        }
        ModuleTypingContext {
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
        }
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
    linker_files: &'l LinkerFiles,
    global_ref: Rc<ConcreteGlobalReference<ModuleUUID>>,
) -> Result<ModuleTypingSuperContext<'l>, InstantiatedModule> {
    let md = &linker_globals.modules[global_ref.id];

    let name = global_ref.display(linker_globals).to_string();

    // Don't instantiate modules that already errored. Otherwise instantiator may crash
    if md.link_info.errors.did_error {
        let errors = ErrorCollector::new_empty(md.link_info.span, linker_files);
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
        let errors = ErrorCollector::new_empty(md.link_info.span, linker_files);
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

    let typed = exec.into_module_typing_context(linker_globals, linker_files, md, global_ref, name);
    let name = &typed.name;

    if typed.errors.did_error() {
        return Err(typed.into_instantiated_module());
    }

    if crate::debug::is_enabled("print-concrete-pre-typecheck") {
        eprintln!("[[Executed {name}]]");
        typed.print_instantiated_module();
    }

    debug!("Concrete Typechecking {name}");
    Ok(ModuleTypingSuperContext::start_typechecking(typed))
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
