use super::*;
use crate::{
    errors::{ErrorInfoObject, FileKnowingErrorInfoObject},
    flattening::{Declaration, GlobalReference, Interface, Port, SubModuleInstance},
    typing::abstract_type::{AbstractGlobalReference, AbstractInnerType, AbstractRankedType},
};

impl<'l> GlobalResolver<'l> {
    pub fn get_submodule(
        &'l self,
        sm_ref: &'l AbstractGlobalReference<ModuleUUID>,
    ) -> RemoteSubModule<'l> {
        RemoteSubModule {
            template_args: &sm_ref.template_arg_types,
            md: &self[sm_ref.id],
        }
    }
    pub fn get_declared_submodule(
        &'l self,
        submod_instr: &'l SubModuleInstance,
    ) -> RemoteSubModule<'l> {
        let AbstractInnerType::Interface(md_ref, _interface) = &submod_instr.typ.inner else {
            unreachable!("Must be an interface!")
        };
        RemoteSubModule {
            template_args: &md_ref.template_arg_types,
            md: &self[md_ref.id],
        }
    }
    pub fn get_global_constant(
        &'l self,
        cst: &'l GlobalReference<ConstantUUID>,
    ) -> RemoteGlobalConstant<'l> {
        RemoteGlobalConstant {
            cst: &self[cst.id],
            template_args: &cst.template_arg_types,
        }
    }
}

#[derive(Clone, Copy)]
pub struct RemoteGlobalConstant<'l> {
    pub cst: &'l NamedConstant,
    pub template_args: &'l TVec<AbstractRankedType>,
}
impl<'l> RemoteGlobalConstant<'l> {
    pub fn get_target_decl(&self) -> RemoteDeclaration<'l> {
        RemoteDeclaration::new(
            &self.cst.link_info,
            self.cst.output_decl,
            self.template_args,
        )
    }
}
#[derive(Clone, Copy)]
pub struct RemoteSubModule<'l> {
    pub md: &'l Module,
    pub template_args: &'l TVec<AbstractRankedType>,
}
impl<'l> RemoteSubModule<'l> {
    pub fn get_port(self, port_id: PortID) -> RemotePort<'l> {
        RemotePort {
            parent: self,
            port: &self.md.ports[port_id],
        }
    }
    pub fn get_interface_reference(self, interface_id: InterfaceID) -> RemoteInterface<'l> {
        let interface = &self.md.interfaces[interface_id];
        RemoteInterface {
            parent: self,
            interface,
        }
    }
}
#[derive(Clone, Copy)]
pub struct RemoteInterface<'l> {
    pub parent: RemoteSubModule<'l>,
    pub interface: &'l Interface,
}
impl<'l> RemoteInterface<'l> {
    pub fn get_port(self, port_id: PortID) -> RemotePort<'l> {
        self.parent.get_port(port_id)
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
pub struct RemotePort<'l> {
    pub parent: RemoteSubModule<'l>,
    pub port: &'l Port,
}
impl<'l> RemotePort<'l> {
    pub fn get_decl(&self) -> RemoteDeclaration<'l> {
        RemoteDeclaration::new(
            &self.parent.md.link_info,
            self.port.declaration_instruction,
            self.parent.template_args,
        )
    }
    pub fn make_info(&self) -> ErrorInfo {
        self.get_decl().make_info()
    }
}
impl FileKnowingErrorInfoObject for RemotePort<'_> {
    fn make_global_info(&self, _files: &ArenaAllocator<FileData, FileUUIDMarker>) -> ErrorInfo {
        self.make_info()
    }
}

#[derive(Clone, Copy)]
pub struct RemoteDeclaration<'l> {
    pub link_info: &'l LinkInfo,
    pub remote_decl: &'l Declaration,
    pub template_arguments: &'l TVec<AbstractRankedType>,
}
impl<'l> RemoteDeclaration<'l> {
    pub fn new(
        link_info: &'l LinkInfo,
        decl_id: FlatID,
        template_arguments: &'l TVec<AbstractRankedType>,
    ) -> Self {
        Self {
            link_info,
            remote_decl: link_info.instructions[decl_id].unwrap_declaration(),
            template_arguments,
        }
    }
    pub fn make_info(&self) -> ErrorInfo {
        self.remote_decl.make_info(self.link_info.file).unwrap()
    }
}
impl FileKnowingErrorInfoObject for RemoteDeclaration<'_> {
    fn make_global_info(&self, _files: &ArenaAllocator<FileData, FileUUIDMarker>) -> ErrorInfo {
        self.make_info()
    }
}

impl Linker {
    pub fn get_all_global_ids(&self) -> Vec<GlobalUUID> {
        /*self.files
        .iter()
        .map(|(_, f)| f.associated_values.iter().copied())
        .flatten()
        .collect()*/

        let m_iter = self.modules.iter().map(|(id, _)| id.into());
        let t_iter = self.types.iter().map(|(id, _)| id.into());
        let c_iter = self.constants.iter().map(|(id, _)| id.into());

        m_iter.chain(t_iter).chain(c_iter).collect()
    }
    /// Pass over a Global Object immutably, thereby giving access to the rest of the context
    pub fn immutable_pass<OT>(
        &mut self,
        pass_name: &'static str,
        obj_id: GlobalUUID,
        f: impl FnOnce(&LinkInfo, &ErrorCollector, &GlobalResolver<'_>) -> OT,
    ) -> OT {
        let working_on_mut = Self::get_link_info_mut(
            &mut self.modules,
            &mut self.types,
            &mut self.constants,
            obj_id,
        );
        let errors = working_on_mut.take_errors(&self.files);
        let globals = working_on_mut.resolved_globals.take();

        let link_info: &LinkInfo = self.get_link_info(obj_id);

        println!("{pass_name} {}", &link_info.name);
        let _panic_guard =
            SpanDebugger::new(pass_name, &link_info.name, &self.files[link_info.file]);

        let globals = GlobalResolver::new(self, globals);

        let result = f(link_info, &errors, &globals);

        let errors = errors.into_storage();
        let globals = globals.decommission();
        let working_on_mut = Self::get_link_info_mut(
            &mut self.modules,
            &mut self.types,
            &mut self.constants,
            obj_id,
        );
        working_on_mut.reabsorb_errors(errors);
        working_on_mut.reabsorb_globals(globals);

        result
    }
    pub fn mutable_pass(
        &mut self,
        obj_id: GlobalUUID,
        f: impl FnOnce(&mut LinkInfo, &ErrorCollector),
    ) {
        let working_on_mut = Self::get_link_info_mut(
            &mut self.modules,
            &mut self.types,
            &mut self.constants,
            obj_id,
        );
        let errors = working_on_mut.take_errors(&self.files);

        f(working_on_mut, &errors);

        working_on_mut.reabsorb_errors(errors.into_storage());
    }
}
