use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc};

use crate::errors::CompileError;
use crate::instantiation::perform_instantiation;
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
        object_id: ConcreteGlobalReference<ModuleUUID>,
    ) -> Option<Rc<InstantiatedModule>> {
        let cache_borrow = self.cache.borrow_mut();

        let instance = if let Some(found) = cache_borrow.cache.get(&object_id) {
            found.clone()
        } else {
            std::mem::drop(cache_borrow);

            let global_ref = Rc::new(object_id);

            let name = global_ref.display(linker).to_string();

            let md = &linker.modules[global_ref.id];
            let file = &linker.files[md.link_info.file];
            let result = crate::debug::debug_context("instantiating", name, file, || {
                perform_instantiation(linker, global_ref.clone())
            });

            let result_ref = Rc::new(result);
            let mut cache_borrow = self.cache.borrow_mut();
            assert!(cache_borrow
                .cache
                .insert(global_ref, result_ref.clone())
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
