use super::*;
use crate::{
    errors::{ErrorInfoObject, FileKnowingErrorInfoObject},
    flattening::{Declaration, GlobalReference, InterfaceDeclaration, Port, SubModuleInstance},
    linker::checkpoint::ResolvedGlobalsCheckpoint,
    typing::abstract_type::{AbstractGlobalReference, AbstractInnerType, AbstractRankedType},
};

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

    pub fn pass(
        &mut self,
        pass_name: &'static str,
        global_id: GlobalUUID,
        f: impl FnOnce(&mut LinkerPass, &ErrorCollector, &ArenaAllocator<FileData, FileUUIDMarker>),
    ) {
        let working_on_mut = &mut self.globals[global_id];
        let error_store = std::mem::take(&mut working_on_mut.errors);
        let errors = ErrorCollector::from_storage(error_store, working_on_mut.file, &self.files);
        let resolved_globals = std::mem::take(&mut working_on_mut.resolved_globals);

        println!("{pass_name} {}", &working_on_mut.name);
        let _panic_guard = SpanDebugger::new(
            pass_name,
            working_on_mut.name.clone(),
            &self.files[working_on_mut.file],
        );

        let mut linker_pass = LinkerPass {
            resolved_globals,
            globals: &mut self.globals,
            global_namespace: &self.global_namespace,
            cur_global: global_id,
        };
        f(&mut linker_pass, &errors, &self.files);

        let resolved_globals = linker_pass.resolved_globals;
        let working_on_mut = &mut self.globals[global_id];
        assert!(working_on_mut.errors.is_untouched());
        working_on_mut.errors = errors.into_storage();
        assert!(working_on_mut.resolved_globals.is_untouched());
        working_on_mut.resolved_globals = resolved_globals;
    }
}

pub struct LinkerPass<'l> {
    resolved_globals: ResolvedGlobals,
    globals: &'l mut LinkerGlobals,
    global_namespace: &'l HashMap<String, NamespaceElement>,
    cur_global: GlobalUUID,
}

impl<'l> LinkerPass<'l> {
    pub fn get_with_context(&mut self) -> (GlobalRef<'_>, GlobalResolver<'_, '_>) {
        let obj = self.globals.get(self.cur_global);
        let global_resolver = GlobalResolver {
            globals: self.globals,
            global_namespace: self.global_namespace,
            resolved_globals: RefCell::new(&mut self.resolved_globals),
        };
        (obj, global_resolver)
    }
    pub fn get_mut(&mut self) -> GlobalRefMut<'_> {
        self.globals.get_mut(self.cur_global)
    }
}

/// See [GlobalResolver]
#[derive(Debug)]
pub struct ResolvedGlobals {
    pub referenced_globals: Vec<GlobalUUID>,
    all_resolved: bool,
}

impl Default for ResolvedGlobals {
    fn default() -> Self {
        Self {
            referenced_globals: Vec::new(),
            all_resolved: true,
        }
    }
}

impl ResolvedGlobals {
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
pub struct GlobalResolver<'linker, 'from> {
    pub globals: &'linker LinkerGlobals,
    global_namespace: &'linker HashMap<String, NamespaceElement>,
    resolved_globals: RefCell<&'from mut ResolvedGlobals>,
}

impl<'linker, 'from> GlobalResolver<'linker, 'from> {
    fn get_linking_error_location(&self, global: GlobalUUID) -> LinkingErrorLocation {
        let named_type = match global {
            GlobalUUID::Module(_) => "Module",
            GlobalUUID::Type(_) => "Struct",
            GlobalUUID::Constant(_) => "Constant",
        };
        let link_info = self.get(global).get_link_info();
        LinkingErrorLocation {
            named_type,
            full_name: link_info.get_full_name(),
            location: link_info.get_span_file(),
        }
    }

    pub fn resolve_global(
        &self,
        name_span: Span,
        name: &str,
        errors: &ErrorCollector,
    ) -> Option<GlobalUUID> {
        match self.global_namespace.get(name) {
            Some(NamespaceElement::Global(found)) => {
                self.resolved_globals
                    .borrow_mut()
                    .referenced_globals
                    .push(*found);
                Some(*found)
            }
            Some(NamespaceElement::Colission(coll)) => {
                self.resolved_globals.borrow_mut().all_resolved = false;

                let mut err_ref = errors.error(name_span, format!("There were colliding imports for the name '{name}'. Pick one and import it by name."));

                for collider_global in coll.iter() {
                    let err_loc = self.get_linking_error_location(*collider_global);
                    err_ref = err_ref.info(
                        err_loc.location,
                        format!("{} {} declared here", err_loc.named_type, err_loc.full_name),
                    );
                }

                None
            }
            None => {
                self.resolved_globals.borrow_mut().all_resolved = false;

                errors.error(
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
        errors: &ErrorCollector,
    ) where
        GlobalUUID: From<ID>,
    {
        // SAFETY: The allocated linker objects aren't going to change.
        let info = self.get_linking_error_location(GlobalUUID::from(global_ref.id));
        let name = &info.full_name;
        let global_type = info.named_type;
        let err_ref = errors.error(
            global_ref.name_span,
            format!("{name} is not a {expected}, it is a {global_type} instead!"),
        );
        err_ref.info(info.location, "Defined here");
    }

    pub fn get(&self, id: GlobalUUID) -> GlobalRef<'linker> {
        self.resolved_globals
            .borrow_mut()
            .referenced_globals
            .push(id);
        self.globals.get(id)
    }

    pub fn get_module(&self, index: ModuleUUID) -> &'linker Module {
        self.resolved_globals
            .borrow_mut()
            .referenced_globals
            .push(GlobalUUID::Module(index));
        &self.globals[index]
    }

    pub fn get_type(&self, index: TypeUUID) -> &'linker StructType {
        self.resolved_globals
            .borrow_mut()
            .referenced_globals
            .push(GlobalUUID::Type(index));
        &self.globals[index]
    }

    pub fn get_constant(&self, index: ConstantUUID) -> &'linker NamedConstant {
        self.resolved_globals
            .borrow_mut()
            .referenced_globals
            .push(GlobalUUID::Constant(index));
        &self.globals[index]
    }

    pub fn get_submodule(
        &self,
        sm_ref: &'linker AbstractGlobalReference<ModuleUUID>,
    ) -> RemoteSubModule<'linker, &'linker TVec<TemplateKind<AbstractRankedType, ()>>> {
        RemoteSubModule {
            template_args: &sm_ref.template_arg_types,
            md: self.get_module(sm_ref.id),
        }
    }
    pub fn get_declared_submodule(
        &self,
        submod_instr: &'linker SubModuleInstance,
    ) -> RemoteSubModule<'linker, &'linker TVec<TemplateKind<AbstractRankedType, ()>>> {
        let AbstractInnerType::Interface(md_ref, _) = &submod_instr.typ.inner else {
            unreachable!("Must be an interface!")
        };
        RemoteSubModule {
            template_args: &md_ref.template_arg_types,
            md: self.get_module(md_ref.id),
        }
    }
    pub fn get_global_constant(
        &self,
        cst: &'linker GlobalReference<ConstantUUID>,
    ) -> RemoteGlobalConstant<'linker, &'linker TVec<TemplateKind<AbstractRankedType, ()>>> {
        RemoteGlobalConstant {
            cst: self.get_constant(cst.id),
            template_args: &cst.template_arg_types,
        }
    }
}

#[derive(Clone, Copy)]
pub struct RemoteGlobalConstant<'l, TemplateT> {
    pub cst: &'l NamedConstant,
    pub template_args: TemplateT,
}
impl<'l, TemplateT: Copy> RemoteGlobalConstant<'l, TemplateT> {
    pub fn get_target_decl(&self) -> RemoteDeclaration<'l, TemplateT> {
        RemoteDeclaration::new(
            &self.cst.link_info,
            self.cst.output_decl,
            Some(self.template_args),
        )
    }
}
#[derive(Clone, Copy)]
pub struct RemoteSubModule<'l, TemplateT> {
    pub md: &'l Module,
    /// None if this is the module itself
    pub template_args: TemplateT,
}
impl<'l, TemplateT: Copy> RemoteSubModule<'l, TemplateT> {
    pub fn get_decl(self, decl_id: FlatID) -> RemoteDeclaration<'l, TemplateT> {
        RemoteDeclaration {
            link_info: &self.md.link_info,
            remote_decl: self.md.link_info.instructions[decl_id].unwrap_declaration(),
            template_args: Some(self.template_args),
        }
    }
    pub fn get_fn(self, fn_decl_id: FlatID) -> RemoteFn<'l, TemplateT> {
        RemoteFn {
            parent: LocalOrRemoteParentModule::Remote(self),
            fn_decl: self.md.link_info.instructions[fn_decl_id].unwrap_interface(),
        }
    }

    pub fn get_port(self, port_id: PortID) -> RemotePort<'l, TemplateT> {
        RemotePort {
            parent: self,
            port: &self.md.ports[port_id],
        }
    }
}

#[derive(Clone, Copy)]
pub enum LocalOrRemoteParentModule<'l, TemplateT> {
    Remote(RemoteSubModule<'l, TemplateT>),
    Local(&'l LinkInfo),
}
impl<'l, TemplateT: Copy> LocalOrRemoteParentModule<'l, TemplateT> {
    pub fn get_decl(self, decl_id: FlatID) -> RemoteDeclaration<'l, TemplateT> {
        match self {
            LocalOrRemoteParentModule::Remote(remote_sub_module) => {
                remote_sub_module.get_decl(decl_id)
            }
            LocalOrRemoteParentModule::Local(link_info) => RemoteDeclaration {
                link_info,
                remote_decl: link_info.instructions[decl_id].unwrap_declaration(),
                template_args: None,
            },
        }
    }
}

#[derive(Clone, Copy)]
pub struct RemoteFn<'l, TemplateT> {
    pub parent: LocalOrRemoteParentModule<'l, TemplateT>,
    pub fn_decl: &'l InterfaceDeclaration,
}
/// For interfaces of this module
impl<TemplateT> FileKnowingErrorInfoObject for RemoteFn<'_, TemplateT> {
    fn make_global_info(&self, _files: &ArenaAllocator<FileData, FileUUIDMarker>) -> ErrorInfo {
        let link_info = match &self.parent {
            LocalOrRemoteParentModule::Remote(remote_sub_module) => &remote_sub_module.md.link_info,
            LocalOrRemoteParentModule::Local(link_info) => link_info,
        };
        ErrorInfo {
            position: self.fn_decl.name_span,
            file: link_info.file,
            info: format!("Interface '{}' defined here", &self.fn_decl.name),
        }
    }
}

#[derive(Clone, Copy)]
pub struct RemotePort<'l, TemplateT> {
    pub parent: RemoteSubModule<'l, TemplateT>,
    pub port: &'l Port,
}
impl<'l, TemplateT: Copy> RemotePort<'l, TemplateT> {
    pub fn get_decl(&self) -> RemoteDeclaration<'l, TemplateT> {
        RemoteDeclaration::new(
            &self.parent.md.link_info,
            self.port.declaration_instruction,
            Some(self.parent.template_args),
        )
    }
    pub fn make_info(&self) -> ErrorInfo {
        self.get_decl().make_info()
    }
}
impl<TemplateT: Copy> FileKnowingErrorInfoObject for RemotePort<'_, TemplateT> {
    fn make_global_info(&self, _files: &ArenaAllocator<FileData, FileUUIDMarker>) -> ErrorInfo {
        self.make_info()
    }
}

#[derive(Clone, Copy)]
pub struct RemoteDeclaration<'l, TemplateT> {
    pub link_info: &'l LinkInfo,
    pub remote_decl: &'l Declaration,
    /// None if this is a local declaration
    pub template_args: Option<TemplateT>,
}
impl<'l, TemplateT> RemoteDeclaration<'l, TemplateT> {
    pub fn new(link_info: &'l LinkInfo, decl_id: FlatID, template_args: Option<TemplateT>) -> Self {
        Self {
            link_info,
            remote_decl: link_info.instructions[decl_id].unwrap_declaration(),
            template_args,
        }
    }
    pub fn make_info(&self) -> ErrorInfo {
        self.remote_decl.make_info(self.link_info.file).unwrap()
    }
}
impl<TemplateT> FileKnowingErrorInfoObject for RemoteDeclaration<'_, TemplateT> {
    fn make_global_info(&self, _files: &ArenaAllocator<FileData, FileUUIDMarker>) -> ErrorInfo {
        self.make_info()
    }
}
