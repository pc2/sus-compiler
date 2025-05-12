use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc};

use crate::debug::SpanDebugger;
use crate::errors::CompileError;
use crate::instantiation::unique_names::UniqueNames;
use crate::instantiation::{GenerationState, InstantiationContext, SubModuleOrWire};
use crate::typing::concrete_type::ConcreteGlobalReference;

use crate::prelude::*;

use super::InstantiatedModule;

/// Stored per module [Module].
/// With this you can instantiate a module for different sets of template arguments.
/// It caches the instantiations that have been made, such that they need not be repeated.
///
/// Also, with incremental builds (#49) this will be a prime area for investigation
#[derive(Debug)]
pub struct InstantiationCache {
    cache: HashMap<Rc<ConcreteGlobalReference<ModuleUUID>>, Rc<InstantiatedModule>>,
}

impl InstantiationCache {
    pub fn for_each_error(&self, func: &mut impl FnMut(&CompileError)) {
        for inst in self.cache.values() {
            for err in &inst.errors {
                func(err)
            }
        }
    }

    pub fn clear_instances(&mut self) {
        self.cache.clear()
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

pub struct Instantiator {
    /// TODO: Replace with Mutex & make multithreaded
    cache: RefCell<InstantiationCache>,
}

impl Default for Instantiator {
    fn default() -> Self {
        Self::new()
    }
}

impl Instantiator {
    pub fn new() -> Self {
        let cache = InstantiationCache {
            cache: HashMap::new(),
        };

        Self {
            cache: RefCell::new(cache),
        }
    }
    pub fn instantiate(
        &self,
        linker: &Linker,
        object_id: Rc<ConcreteGlobalReference<ModuleUUID>>,
    ) -> Option<Rc<InstantiatedModule>> {
        let cache_borrow = self.cache.borrow_mut();

        let instance = if let Some(found) = cache_borrow.cache.get(&object_id) {
            found.clone()
        } else {
            std::mem::drop(cache_borrow);

            let result = perform_instantiation(linker, object_id.clone());

            if crate::debug::is_enabled("dot-concrete-module") {
                crate::dev_aid::dot_graphs::display_generated_hardware_structure(&result, linker);
            }

            let result_ref = Rc::new(result);
            let mut cache_borrow = self.cache.borrow_mut();
            assert!(cache_borrow
                .cache
                .insert(object_id, result_ref.clone())
                .is_none());
            result_ref
        };

        if !instance.errors.did_error {
            Some(instance.clone())
        } else {
            None
        }
    }
    pub fn borrow(&self) -> std::cell::Ref<'_, InstantiationCache> {
        self.cache.borrow()
    }
    pub fn borrow_mut(&self) -> std::cell::RefMut<'_, InstantiationCache> {
        self.cache.borrow_mut()
    }
}

fn perform_instantiation(
    linker: &Linker,
    working_on_global_ref: Rc<ConcreteGlobalReference<ModuleUUID>>,
) -> InstantiatedModule {
    let md = &linker.modules[working_on_global_ref.id];

    let _panic_guard = SpanDebugger::new(
        "instantiating",
        &md.link_info.name,
        &linker.files[md.link_info.file],
    );

    let mut context = InstantiationContext {
        name: working_on_global_ref.pretty_print_concrete_instance(linker),
        generation_state: GenerationState {
            md,
            generation_state: md
                .link_info
                .instructions
                .map(|(_, _)| SubModuleOrWire::Unnasigned),
        },
        type_substitutor: Default::default(),
        //type_value_substitutor: Default::default(),
        condition_stack: Vec::new(),
        wires: FlatAlloc::new(),
        submodules: FlatAlloc::new(),
        interface_ports: md.ports.map(|_| None),
        errors: ErrorCollector::new_empty(md.link_info.file, &linker.files),
        unique_name_producer: UniqueNames::new(),
        working_on_global_ref,
        md,
        linker,
    };

    // Don't instantiate modules that already errored. Otherwise instantiator may crash
    if md.link_info.errors.did_error {
        println!(
            "Not Instantiating {} due to flattening errors",
            md.link_info.name
        );
        context.errors.set_did_error();
        return context.extract();
    }

    println!("Instantiating {}", md.link_info.name);

    if let Err(e) = context.execute_module() {
        context.errors.error(e.0, e.1);

        return context.extract();
    }

    if crate::debug::is_enabled("print-instantiated-modules-pre-concrete-typecheck") {
        println!("[[Executed {}]]", &context.name);
        for (id, w) in &context.wires {
            println!("{id:?} -> {w:?}");
        }
        for (id, sm) in &context.submodules {
            println!("SubModule {id:?}: {sm:?}");
        }
    }

    println!("Concrete Typechecking {}", md.link_info.name);
    context.typecheck();

    println!("Latency Counting {}", md.link_info.name);
    context.compute_latencies();

    println!("Checking array accesses {}", md.link_info.name);
    context.check_array_accesses();

    if crate::debug::is_enabled("print-instantiated-modules") {
        println!("[[Instantiated {}]]", context.name);
        for (id, w) in &context.wires {
            println!("{id:?} -> {w:?}");
        }
        for (id, sm) in &context.submodules {
            println!("SubModule {id:?}: {sm:?}");
        }
    }

    context.extract()
}
