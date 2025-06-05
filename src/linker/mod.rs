use crate::{
    flattening::{Instruction, NamedConstant},
    instantiation::instantiation_cache::Instantiator,
    prelude::*,
    typing::{
        template::{GenerativeParameterKind, Parameter, TVec, TemplateKind, TypeParameterKind},
        type_inference::AbstractTypeSubstitutor,
    },
};

pub mod checkpoint;
mod resolver;
pub use resolver::*;

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use tree_sitter::Tree;

use crate::{alloc::ArenaAllocator, file_position::FileText, flattening::Module};

use crate::errors::{CompileError, ErrorInfo, ErrorLevel, ErrorStore};

use crate::flattening::StructType;

use self::checkpoint::CheckPoint;

/// Documentation can be attached to [Module], [StructType], [NamedConstant], [crate::flattening::Declaration]
#[derive(Debug, Clone)]
pub struct Documentation {
    pub gathered: Box<[Span]>,
}

impl Documentation {
    pub fn to_string(&self, file_text: &FileText) -> String {
        let mut total_length = self.gathered.len().saturating_sub(1);
        for s in self.gathered.iter() {
            total_length += s.size();
        }
        let mut result = String::with_capacity(total_length);
        for s in self.gathered.iter() {
            let text = &file_text[*s];

            if text.trim().is_empty() {
                // Force a newline by adding a blank comment (like Rust does it?)
                // need two spaces beforehand to force a newline in markdown, according to https://github.com/microsoft/vscode/issues/86291#issuecomment-561841915
                result.push_str("  \n");
            } else {
                result.push_str(text);
            }
        }
        result
    }
}

/// [Module], [StructType], or [NamedConstant] annotation that specifies exceptions to code generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsExtern {
    /// Code is generated for this through the regular channel (See [crate::codegen])
    ///
    /// ```sus
    /// module md {}
    /// ```
    Normal,
    /// Modules that are provided externally, and thus no code should be generated for these
    ///
    /// ```sus
    /// extern module md {}
    /// ```
    Extern,
    /// Builtins, like escape hatches for Latency Counting & domains
    ///
    /// ```sus
    /// __builtin__ module md {}
    /// ```
    Builtin,
}

pub const AFTER_INITIAL_PARSE_CP: usize = 0;
pub const AFTER_FLATTEN_CP: usize = 1;
pub const AFTER_TYPECHECK_CP: usize = 2;
pub const AFTER_DOMAINCHECK_CP: usize = 3;
pub const AFTER_LINTS_CP: usize = 4;

/// Represents any global. Stored in [Linker] and each is uniquely indexed by [GlobalUUID]
///
/// Base class for [Module], [StructType], [NamedConstant]
#[derive(Debug)]
pub struct LinkInfo {
    pub file: FileUUID,
    pub span: Span,
    pub name: String,
    pub name_span: Span,
    pub documentation: Documentation,
    pub errors: ErrorStore,
    pub resolved_globals: ResolvedGlobals,
    pub is_extern: IsExtern,

    /// Created in Stage 2: Flattening
    ///
    /// Removed in Stage 3: Typechecking
    ///
    /// Is only temporary. It's used during typechecking to allocate the type unification block
    pub type_variable_alloc: Option<Box<AbstractTypeSubstitutor>>,

    pub template_parameters: TVec<Parameter>,

    /// Created in Stage 2: Flattening. type data is filled out during Typechecking
    pub instructions: FlatAlloc<Instruction, FlatIDMarker>,

    /// Reset checkpoints. These are to reset errors and resolved_globals for incremental compilation.
    ///
    /// TODO the system is there, just need to actually do incremental compilation (#49)
    ///
    /// Right now it already functions as a sanity check, to make sure no steps in building modules/types are skipped
    pub checkpoints: Vec<CheckPoint>,
}

impl LinkInfo {
    pub fn get_full_name(&self) -> String {
        // Feelin iffy about namespaces, so just return self.name
        self.name.clone()
        // format!("::{}", self.name)
    }
    pub fn get_span_file(&self) -> SpanFile {
        (self.name_span, self.file)
    }
    pub fn get_full_name_and_template_args(&self, file_text: &FileText) -> String {
        let mut template_args: Vec<&str> = Vec::new();
        for (_id, t) in &self.template_parameters {
            match &t.kind {
                TemplateKind::Type(TypeParameterKind {}) => template_args.push(&t.name),
                TemplateKind::Value(GenerativeParameterKind {
                    decl_span,
                    declaration_instruction: _,
                }) => template_args.push(&file_text[*decl_span]),
            }
        }

        format!("{} #({})", self.get_full_name(), template_args.join(", "))
    }
    pub fn get_instruction_span(&self, instr_id: FlatID) -> Span {
        match &self.instructions[instr_id] {
            Instruction::SubModule(sm) => sm.module_ref.get_total_span(),
            Instruction::Declaration(decl) => decl.decl_span,
            Instruction::Expression(w) => w.span,
            Instruction::IfStatement(if_stmt) => self.get_instruction_span(if_stmt.condition),
            Instruction::ForStatement(for_stmt) => {
                self.get_instruction_span(for_stmt.loop_var_decl)
            }
        }
    }
}

/// Data associated with a file. Such as the text, the parse tree, and all [Module]s, [StructType]s, or [NamedConstant]s.
///
/// All FileDatas are stored in [Linker::files], and indexed by [FileUUID]
pub struct FileData {
    pub file_identifier: String,
    pub file_text: FileText,
    pub parsing_errors: ErrorStore,
    /// In source file order
    pub associated_values: Vec<GlobalUUID>,
    pub tree: tree_sitter::Tree,
}

/// Globally references any [Module], [StructType], or [NamedConstant] in [Linker]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobalUUID {
    Module(ModuleUUID),
    Type(TypeUUID),
    Constant(ConstantUUID),
}

impl GlobalUUID {
    #[track_caller]
    pub fn unwrap_module(&self) -> ModuleUUID {
        let GlobalUUID::Module(id) = self else {
            unreachable!("Not a ModuleUUID!")
        };
        *id
    }
    #[track_caller]
    pub fn unwrap_type(&self) -> TypeUUID {
        let GlobalUUID::Type(id) = self else {
            unreachable!("Not a TypeUUID!")
        };
        *id
    }
    #[track_caller]
    pub fn unwrap_const(&self) -> ConstantUUID {
        let GlobalUUID::Constant(id) = self else {
            unreachable!("Not a ConstantUUID!")
        };
        *id
    }
}

impl From<ModuleUUID> for GlobalUUID {
    fn from(value: ModuleUUID) -> Self {
        GlobalUUID::Module(value)
    }
}

impl From<TypeUUID> for GlobalUUID {
    fn from(value: TypeUUID) -> Self {
        GlobalUUID::Type(value)
    }
}

impl From<ConstantUUID> for GlobalUUID {
    fn from(value: ConstantUUID) -> Self {
        GlobalUUID::Constant(value)
    }
}

enum NamespaceElement {
    Global(GlobalUUID),
    Colission(Box<[GlobalUUID]>),
}

/// The global singleton object that collects all [Module]s, [StructType]s, and [NamedConstant]s that are in the current SUS codebase.
///
/// There should only be one [Linker] globally.
///
/// See [LinkInfo], this contains shared data between all global objects in the whole progam.
///
/// It also keeps track of the global namespace.
///
/// Incremental operations such as adding and removing files can be performed on this
pub struct Linker {
    pub types: ArenaAllocator<StructType, TypeUUIDMarker>,
    pub modules: ArenaAllocator<Module, ModuleUUIDMarker>,
    pub constants: ArenaAllocator<NamedConstant, ConstantUUIDMarker>,
    pub files: ArenaAllocator<FileData, FileUUIDMarker>,
    pub instantiator: Instantiator,
    global_namespace: HashMap<String, NamespaceElement>,
}

impl Default for Linker {
    fn default() -> Self {
        Self::new()
    }
}

impl Linker {
    pub fn new() -> Linker {
        Linker {
            types: ArenaAllocator::new(),
            modules: ArenaAllocator::new(),
            constants: ArenaAllocator::new(),
            files: ArenaAllocator::new(),
            instantiator: Instantiator::new(),
            global_namespace: HashMap::new(),
        }
    }

    pub fn get_link_info(&self, global: GlobalUUID) -> &LinkInfo {
        match global {
            GlobalUUID::Module(md_id) => &self.modules[md_id].link_info,
            GlobalUUID::Type(typ_id) => &self.types[typ_id].link_info,
            GlobalUUID::Constant(cst_id) => &self.constants[cst_id].link_info,
        }
    }
    pub fn get_link_info_mut<'l>(
        modules: &'l mut ArenaAllocator<Module, ModuleUUIDMarker>,
        types: &'l mut ArenaAllocator<StructType, TypeUUIDMarker>,
        constants: &'l mut ArenaAllocator<NamedConstant, ConstantUUIDMarker>,
        global: GlobalUUID,
    ) -> &'l mut LinkInfo {
        match global {
            GlobalUUID::Module(md_id) => &mut modules[md_id].link_info,
            GlobalUUID::Type(typ_id) => &mut types[typ_id].link_info,
            GlobalUUID::Constant(cst_id) => &mut constants[cst_id].link_info,
        }
    }
    fn iter_link_infos(&self) -> impl Iterator<Item = (GlobalUUID, &LinkInfo)> {
        let md_iter = self
            .modules
            .iter()
            .map(|(id, obj)| (id.into(), &obj.link_info));
        let typ_iter = self
            .types
            .iter()
            .map(|(id, obj)| (id.into(), &obj.link_info));
        let cst_iter = self
            .constants
            .iter()
            .map(|(id, obj)| (id.into(), &obj.link_info));

        md_iter.chain(typ_iter).chain(cst_iter)
    }

    fn collect_duplicate_declaration_errors(
        &self,
        all_errors: &mut ArenaAllocator<ErrorStore, FileUUIDMarker>,
    ) {
        // Conflicting Declarations
        for item in &self.global_namespace {
            let NamespaceElement::Colission(colission) = &item.1 else {
                continue;
            };
            let infos: Vec<&LinkInfo> =
                colission.iter().map(|id| self.get_link_info(*id)).collect();

            for (idx, info) in infos.iter().enumerate() {
                let mut conflict_infos = Vec::new();
                for (idx_2, conflicts_with) in infos.iter().enumerate() {
                    if idx_2 == idx {
                        continue;
                    }
                    conflict_infos.push(conflicts_with);
                }
                let this_object_name = &info.name;
                let infos = conflict_infos
                    .iter()
                    .map(|conf_info| ErrorInfo {
                        position: conf_info.name_span,
                        file: conf_info.file,
                        info: "Conflicts with".to_owned(),
                    })
                    .collect();

                let reason = format!("'{this_object_name}' conflicts with other declarations:");

                all_errors[info.file].push(CompileError {
                    position: info.name_span,
                    reason,
                    infos,
                    level: ErrorLevel::Error,
                });
            }
        }
    }

    fn collect_errors_after_compile(
        &self,
        all_errs: &mut ArenaAllocator<ErrorStore, FileUUIDMarker>,
    ) {
        for (_id, link_info) in self.iter_link_infos() {
            all_errs[link_info.file].append(&link_info.errors);
        }
        for (_id, inst) in self.instantiator.borrow().iter() {
            let file_id = self.modules[inst.global_ref.id].link_info.file;
            all_errs[file_id].append(&inst.errors);
        }
    }

    pub fn collect_all_errors(&self) -> ArenaAllocator<ErrorStore, FileUUIDMarker> {
        let mut result = self
            .files
            .map(|_id, file_data| file_data.parsing_errors.clone());

        self.collect_duplicate_declaration_errors(&mut result);
        self.collect_errors_after_compile(&mut result);

        for (_, errs) in &mut result {
            errs.sort();
        }

        result
    }

    pub fn remove_everything_in_file(&mut self, file_uuid: FileUUID) -> &mut FileData {
        // For quick lookup if a reference disappears
        let mut to_remove_set = HashSet::new();

        let file_data = &mut self.files[file_uuid];
        // Remove referenced data in file
        for v in file_data.associated_values.drain(..) {
            let was_new_item_in_set = to_remove_set.insert(v);
            assert!(was_new_item_in_set);
            match v {
                GlobalUUID::Module(id) => {
                    self.modules.free(id);
                }
                GlobalUUID::Type(id) => {
                    self.types.free(id);
                }
                GlobalUUID::Constant(id) => {
                    self.constants.free(id);
                }
            }
        }

        // Remove from global namespace
        self.global_namespace.retain(|_, v| match v {
            NamespaceElement::Global(g) => !to_remove_set.contains(g),
            NamespaceElement::Colission(colission) => {
                let mut retain_vec =
                    std::mem::replace::<Box<[GlobalUUID]>>(colission, Box::new([])).into_vec();
                retain_vec.retain(|g| !to_remove_set.contains(g));
                *colission = retain_vec.into_boxed_slice();
                colission.len() > 0
            }
        });

        file_data
    }

    #[allow(dead_code)]
    pub fn remove_file(&mut self, file_uuid: FileUUID) {
        self.remove_everything_in_file(file_uuid);
        self.files.free(file_uuid);
    }

    pub fn with_file_builder(&mut self, file_id: FileUUID, f: impl FnOnce(FileBuilder<'_>)) {
        let mut associated_values = Vec::new();
        let mut parsing_errors =
            std::mem::replace(&mut self.files[file_id].parsing_errors, ErrorStore::new());
        let file_data = &self.files[file_id];
        let other_parsing_errors =
            ErrorCollector::from_storage(parsing_errors.take(), file_id, &self.files);

        f(FileBuilder {
            file_id,
            tree: &file_data.tree,
            file_data,
            files: &self.files,
            other_parsing_errors: &other_parsing_errors,
            associated_values: &mut associated_values,
            global_namespace: &mut self.global_namespace,
            types: &mut self.types,
            modules: &mut self.modules,
            constants: &mut self.constants,
        });

        let parsing_errors = other_parsing_errors.into_storage();
        let file_data = &mut self.files[file_id];
        file_data.parsing_errors = parsing_errors;
        file_data.associated_values = associated_values;
    }
}

/// Temporary builder for [crate::flattening::initialization]
pub struct FileBuilder<'linker> {
    pub file_id: FileUUID,
    pub tree: &'linker Tree,
    pub file_data: &'linker FileData,
    pub files: &'linker ArenaAllocator<FileData, FileUUIDMarker>,
    pub other_parsing_errors: &'linker ErrorCollector<'linker>,
    associated_values: &'linker mut Vec<GlobalUUID>,
    global_namespace: &'linker mut HashMap<String, NamespaceElement>,
    modules: &'linker mut ArenaAllocator<Module, ModuleUUIDMarker>,
    types: &'linker mut ArenaAllocator<StructType, TypeUUIDMarker>,
    constants: &'linker mut ArenaAllocator<NamedConstant, ConstantUUIDMarker>,
}

impl FileBuilder<'_> {
    fn add_name(&mut self, name: String, new_obj_id: GlobalUUID) {
        match self.global_namespace.entry(name) {
            std::collections::hash_map::Entry::Occupied(mut occ) => {
                let new_val = match occ.get_mut() {
                    NamespaceElement::Global(g) => Box::new([*g, new_obj_id]),
                    NamespaceElement::Colission(coll) => {
                        let mut vec = std::mem::replace(coll, Box::new([])).into_vec();
                        vec.push(new_obj_id);
                        vec.into_boxed_slice()
                    }
                };
                occ.insert(NamespaceElement::Colission(new_val));
            }
            std::collections::hash_map::Entry::Vacant(vac) => {
                vac.insert(NamespaceElement::Global(new_obj_id));
            }
        }
    }

    pub fn add_module(&mut self, md: Module) {
        let module_name = md.link_info.name.clone();
        let new_module_uuid = GlobalUUID::Module(self.modules.alloc(md));
        self.associated_values.push(new_module_uuid);
        self.add_name(module_name, new_module_uuid);
    }

    pub fn add_type(&mut self, typ: StructType) {
        let type_name = typ.link_info.name.clone();
        let new_type_uuid = GlobalUUID::Type(self.types.alloc(typ));
        self.associated_values.push(new_type_uuid);
        self.add_name(type_name, new_type_uuid);
    }

    pub fn add_const(&mut self, cst: NamedConstant) {
        let const_name = cst.link_info.name.clone();
        let new_const_uuid = GlobalUUID::Constant(self.constants.alloc(cst));
        self.associated_values.push(new_const_uuid);
        self.add_name(const_name, new_const_uuid);
    }
}
