mod domain_check;
mod lints;
mod type_check;

use crate::{
    alloc::ArenaAllocator,
    errors::{ErrorInfo, ErrorInfoObject, FileKnowingErrorInfoObject},
    flattening::typecheck::{domain_check::domain_check_all, type_check::typecheck_all_modules},
    linker::{FileData, GlobalResolver},
    typing::type_inference::{AbstractTypeSubstitutor, TypeSubstitutor, TypeUnifier},
};

use super::*;

use std::{
    cell::OnceCell,
    ops::{Deref, Index},
};

pub use lints::perform_lints;
pub fn typecheck_all(linker: &mut Linker) {
    typecheck_all_modules(linker);
    domain_check_all(linker);
}

struct DomainCheckingContext<'l> {
    globals: &'l GlobalResolver<'l>,
    errors: &'l ErrorCollector<'l>,
    instructions: &'l FlatAlloc<Instruction, FlatIDMarker>,
    domain_checker: TypeUnifier<TypeSubstitutor<DomainType>>,
}

struct TypeCheckingContext<'l> {
    globals: &'l GlobalResolver<'l>,
    errors: &'l ErrorCollector<'l>,
    instructions: &'l FlatAlloc<Instruction, FlatIDMarker>,
    type_checker: TypeUnifier<AbstractTypeSubstitutor>,
}

#[derive(Clone, Copy)]
struct RemoteSubModule<'l> {
    submodule: &'l SubModuleInstance,
    md: &'l Module,
}
impl<'l> RemoteSubModule<'l> {
    fn make(
        submodule_instr: FlatID,
        instructions: &'l impl Index<FlatID, Output = Instruction>,
        globals: &'l impl Index<ModuleUUID, Output = Module>,
    ) -> Self {
        let submodule = instructions[submodule_instr].unwrap_submodule();
        Self {
            submodule,
            md: &globals[submodule.module_ref.id],
        }
    }

    fn get_port(self, port_id: PortID) -> RemotePort<'l> {
        RemotePort {
            parent: self,
            port: &self.md.ports[port_id],
            remote_decl: self.md.get_port_decl(port_id),
            file: self.md.link_info.file,
        }
    }
    fn get_interface_reference(self, interface_id: InterfaceID) -> RemoteInterface<'l> {
        let interface = &self.md.interfaces[interface_id];
        RemoteInterface {
            parent: self,
            interface,
        }
    }
}
#[derive(Clone, Copy)]
struct RemoteInterface<'l> {
    parent: RemoteSubModule<'l>,
    interface: &'l Interface,
}
impl<'l> RemoteInterface<'l> {
    fn get_port(self, port_id: PortID) -> RemotePort<'l> {
        self.parent.get_port(port_id)
    }
    fn get_local_domain(self) -> DomainType {
        self.parent.submodule.local_interface_domains[self.interface.domain]
    }
}
/// For interfaces of this module
impl FileKnowingErrorInfoObject for RemoteInterface<'_> {
    fn make_global_info(&self, _files: &ArenaAllocator<FileData, FileUUIDMarker>) -> ErrorInfo {
        ErrorInfo {
            position: self.interface.name_span,
            file: self.parent.md.link_info.file,
            info: format!("Interface '{}' defined here", &self.interface.name),
        }
    }
}

#[derive(Clone, Copy)]
struct RemotePort<'l> {
    parent: RemoteSubModule<'l>,
    port: &'l Port,
    remote_decl: &'l Declaration,
    file: FileUUID,
}
impl<'l> RemotePort<'l> {
    fn get_local_domain(&self) -> DomainType {
        self.parent.submodule.local_interface_domains[self.port.domain]
    }
    fn make_info(&self) -> ErrorInfo {
        self.remote_decl.make_info(self.file).unwrap()
    }
    fn is_input(&self) -> bool {
        self.remote_decl.decl_kind.is_io_port().unwrap()
    }
}
impl FileKnowingErrorInfoObject for RemotePort<'_> {
    fn make_global_info(&self, _files: &ArenaAllocator<FileData, FileUUIDMarker>) -> ErrorInfo {
        self.make_info()
    }
}

/// Basically equivalent to [std::cell::OnceCell], but implements [std::ops::Deref] and automatically unwraps
/// This file defines a OnceCell variant for use with typechecking
///
/// Because in typechecking, we will always set it to uninitialized in Flatten, set it to an initial value (&self) in typechecking, and then finalize the type in (&mut self)
#[derive(Debug)]
pub struct TyCell<T>(OnceCell<T>);

impl<T: std::fmt::Debug> TyCell<T> {
    pub fn new() -> Self {
        Self::default()
    }
    #[track_caller]
    fn get_mut(&mut self) -> &mut T {
        self.0.get_mut().unwrap()
    }
    /// Private because only typechecking should be allowed to set TyCells
    #[track_caller]
    fn set(&self, v: T) {
        self.0.set(v).unwrap();
    }
}

impl<T> Default for TyCell<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Deref for TyCell<T> {
    type Target = T;

    #[track_caller]
    fn deref(&self) -> &Self::Target {
        self.0.get().expect("Deref on an unfinished TyCell!")
    }
}

/*
// This delegated IntoIterator impl causes infinite recursion due to a bug in rustc. https://github.com/rust-lang/rust/issues/106512
// Right now, just defer to .iter()
impl<'a, T> IntoIterator for &'a TyCell<T>
where
    &'a T: IntoIterator,
{
    type Item = <&'a T as IntoIterator>::Item; // NOTE diff
    type IntoIter = <&'a T as IntoIterator>::IntoIter; // NOTE diff
    fn into_iter(self) -> Self::IntoIter {
        self.0.get().unwrap().into_iter()
    }
}
*/
