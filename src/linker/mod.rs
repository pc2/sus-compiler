use crate::{flattening::Instruction, prelude::*};

pub mod checkpoint;
mod resolver;
use arrayvec::ArrayVec;
pub use resolver::*;

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use tree_sitter::Tree;

use crate::{
    alloc::ArenaAllocator,
    file_position::FileText,
    flattening::Module,
    util::{const_str_position, const_str_position_in_tuples},
    value::{TypedValue, Value},
};

use crate::errors::{CompileError, ErrorInfo, ErrorLevel, ErrorStore};

use crate::flattening::{StructType, TypingAllocator};

use crate::typing::template::TemplateInputs;

use self::checkpoint::CheckPoint;

const BUILTIN_TYPES: [&'static str; 2] = ["bool", "int"];

const BUILTIN_CONSTANTS: [(&'static str, Value); 2] =
    [("true", Value::Bool(true)), ("false", Value::Bool(false))];

pub const fn get_builtin_type(name: &'static str) -> TypeUUID {
    if let Some(is_type) = const_str_position(name, &BUILTIN_TYPES) {
        TypeUUID::from_hidden_value(is_type)
    } else {
        unreachable!()
    }
}

pub const fn get_builtin_constant(name: &'static str) -> ConstantUUID {
    if let Some(is_constant) = const_str_position_in_tuples(name, &BUILTIN_CONSTANTS) {
        ConstantUUID::from_hidden_value(is_constant)
    } else {
        unreachable!()
    }
}

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
            result.push_str(&file_text[*s]);
            result.push('\n');
        }
        result
    }
}

#[derive(Debug)]
pub enum IsExtern {
    Normal,
    Extern,
    Builtin
}

pub const AFTER_INITIAL_PARSE_CP : usize = 0;
pub const AFTER_FLATTEN_CP : usize = 1;
pub const AFTER_TYPECHECK_CP : usize = 2;
pub const AFTER_LINTS_CP : usize = 3;


#[derive(Debug)]
pub struct LinkInfo {
    pub file: FileUUID,
    pub span: Span,
    pub name: String,
    pub name_span: Span,
    pub documentation: Documentation,
    pub errors: ErrorStore,
    pub resolved_globals: ResolvedGlobals,
    pub is_extern : IsExtern,

    /// Created in Stage 2: Flattening
    /// 
    /// Is only temporary. It's used during typechecking to allocate the type unification block
    pub type_variable_alloc: TypingAllocator,

    pub template_arguments: TemplateInputs,

    /// Created in Stage 2: Flattening. type data is filled out during Typechecking
    pub instructions: FlatAlloc<Instruction, FlatIDMarker>,

    /// Reset checkpoints. These are to reset errors and resolved_globals for incremental compilation. 
    /// 
    /// TODO the system is there, just need to actually do incremental compilation
    /// 
    /// Right now it already functions as a sanity check, to make sure no steps in building modules/types are skipped
    pub checkpoints : ArrayVec<CheckPoint, 4>
}

impl LinkInfo {
    pub fn get_full_name(&self) -> String {
        format!("::{}", self.name)
    }
    pub fn get_span_file(&self) -> SpanFile {
        (self.name_span, self.file)
    }
}

pub struct LinkingErrorLocation {
    pub named_type: &'static str,
    pub full_name: String,
    pub location: SpanFile,
}

#[derive(Debug)]
pub struct NamedConstant {
    pub link_info: LinkInfo,
    pub output_decl: FlatID,
    pub val: TypedValue
}

impl NamedConstant {
    pub fn get_value(&self) -> &TypedValue {
        &self.val
    }
}

pub struct FileData {
    pub file_identifier: String,
    pub file_text: FileText,
    pub parsing_errors: ErrorStore,
    /// In source file order
    pub associated_values: Vec<NameElem>,
    pub tree: tree_sitter::Tree,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NameElem {
    Module(ModuleUUID),
    Type(TypeUUID),
    Constant(ConstantUUID),
}

impl From<ModuleUUID> for NameElem {
    fn from(value: ModuleUUID) -> Self {
        NameElem::Module(value)
    }
}

impl From<TypeUUID> for NameElem {
    fn from(value: TypeUUID) -> Self {
        NameElem::Type(value)
    }
}

impl From<ConstantUUID> for NameElem {
    fn from(value: ConstantUUID) -> Self {
        NameElem::Constant(value)
    }
}

enum NamespaceElement {
    Global(NameElem),
    Colission(Box<[NameElem]>),
}

// Represents the fully linked set of all files. Incremental operations such as adding and removing files can be performed
pub struct Linker {
    pub types: ArenaAllocator<StructType, TypeUUIDMarker>,
    pub modules: ArenaAllocator<Module, ModuleUUIDMarker>,
    pub constants: ArenaAllocator<NamedConstant, ConstantUUIDMarker>,
    pub files: ArenaAllocator<FileData, FileUUIDMarker>,
    global_namespace: HashMap<String, NamespaceElement>,
}

impl Linker {
    pub fn new() -> Linker {
        Linker {
            types: ArenaAllocator::new(),
            modules: ArenaAllocator::new(),
            constants: ArenaAllocator::new(),
            files: ArenaAllocator::new(),
            global_namespace: HashMap::new(),
        }
    }

    pub fn get_link_info(&self, global: NameElem) -> &LinkInfo {
        match global {
            NameElem::Module(md_id) => &self.modules[md_id].link_info,
            NameElem::Type(typ_id) => &self.types[typ_id].link_info,
            NameElem::Constant(cst_id) => &self.constants[cst_id].link_info
        }
    }
    pub fn get_link_info_mut<'l>(
        modules: &'l mut ArenaAllocator<Module, ModuleUUIDMarker>,
        types: &'l mut ArenaAllocator<StructType, TypeUUIDMarker>,
        constants: &'l mut ArenaAllocator<NamedConstant, ConstantUUIDMarker>,
        global: NameElem
    ) -> &'l mut LinkInfo {
        match global {
            NameElem::Module(md_id) => &mut modules[md_id].link_info,
            NameElem::Type(typ_id) => &mut types[typ_id].link_info,
            NameElem::Constant(cst_id) => &mut constants[cst_id].link_info
        }
    }
    pub fn get_full_name(&self, global: NameElem) -> String {
        match global {
            NameElem::Module(id) => self.modules[id].link_info.get_full_name(),
            NameElem::Type(id) => self.types[id].link_info.get_full_name(),
            NameElem::Constant(id) => self.constants[id].link_info.get_full_name(),
        }
    }
    fn get_linking_error_location(&self, global: NameElem) -> LinkingErrorLocation {
        let named_type = match global {
            NameElem::Module(_) => "Module",
            NameElem::Type(_) => "Struct",
            NameElem::Constant(_) => "Constant"
        };
        let link_info = self.get_link_info(global);
        LinkingErrorLocation {
            named_type,
            full_name: link_info.get_full_name(),
            location: link_info.get_span_file(),
        }
    }
    fn for_all_duplicate_declaration_errors<F: FnMut(&CompileError)>(&self, file_uuid: FileUUID, f: &mut F) {
        // Conflicting Declarations
        for item in &self.global_namespace {
            let NamespaceElement::Colission(colission) = &item.1 else {
                continue;
            };
            let infos: Vec<&LinkInfo> =
                colission.iter().map(|id| self.get_link_info(*id)).collect();

            for (idx, info) in infos.iter().enumerate() {
                if info.file != file_uuid {continue}
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
                
                f(&CompileError {
                    position: info.name_span,
                    reason,
                    infos,
                    level: ErrorLevel::Error,
                });
            }
        }
    }

    fn for_all_errors_after_compile<F: FnMut(&CompileError)>(
        &self,
        file_uuid: FileUUID,
        func: &mut F,
    ) {
        for v in &self.files[file_uuid].associated_values {
            match v {
                NameElem::Module(md_id) => {
                    let md = &self.modules[*md_id];
                    for e in &md.link_info.errors {
                        func(e)
                    }
                    md.instantiations.for_each_error(func);
                }
                NameElem::Type(_) => {}
                NameElem::Constant(_) => {}
            }
        }
    }

    pub fn for_all_errors_in_file<F: FnMut(&CompileError)>(&self, file_uuid: FileUUID, mut f: F) {
        for err in &self.files[file_uuid].parsing_errors {
            f(err);
        }
        self.for_all_duplicate_declaration_errors(file_uuid, &mut f);
        self.for_all_errors_after_compile(file_uuid, &mut f);
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
                NameElem::Module(id) => {
                    self.modules.free(id);
                }
                NameElem::Type(id) => {
                    self.types.free(id);
                }
                NameElem::Constant(id) => {
                    self.constants.free(id);
                }
            }
        }

        // Remove from global namespace
        self.global_namespace.retain(|_, v| match v {
            NamespaceElement::Global(g) => !to_remove_set.contains(g),
            NamespaceElement::Colission(colission) => {
                let mut retain_vec =
                    std::mem::replace::<Box<[NameElem]>>(colission, Box::new([])).into_vec();
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

    pub fn with_file_builder<F: FnOnce(FileBuilder<'_>)>(&mut self, file_id: FileUUID, f: F) {
        let mut associated_values = Vec::new();
        let mut parsing_errors =
            std::mem::replace(&mut self.files[file_id].parsing_errors, ErrorStore::new());
        let file_data = &self.files[file_id];
        let other_parsing_errors = ErrorCollector::from_storage(parsing_errors.take(), file_id, &self.files);

        f(FileBuilder {
            file_id,
            tree: &file_data.tree,
            file_data: &file_data,
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

pub struct FileBuilder<'linker> {
    pub file_id: FileUUID,
    pub tree: &'linker Tree,
    pub file_data: &'linker FileData,
    pub files: &'linker ArenaAllocator<FileData, FileUUIDMarker>,
    pub other_parsing_errors: &'linker ErrorCollector<'linker>,
    associated_values: &'linker mut Vec<NameElem>,
    global_namespace: &'linker mut HashMap<String, NamespaceElement>,
    modules: &'linker mut ArenaAllocator<Module, ModuleUUIDMarker>,
    types: &'linker mut ArenaAllocator<StructType, TypeUUIDMarker>,
    constants: &'linker mut ArenaAllocator<NamedConstant, ConstantUUIDMarker>,
}

impl<'linker> FileBuilder<'linker> {
    fn add_name(&mut self, name: String, new_obj_id: NameElem) {
        match self.global_namespace.entry(name) {
            std::collections::hash_map::Entry::Occupied(mut occ) => {
                let new_val = match occ.get_mut() {
                    NamespaceElement::Global(g) => Box::new([*g, new_obj_id]),
                    NamespaceElement::Colission(coll) => {
                        let mut vec = std::mem::replace(coll, Box::new([])).into_vec();
                        vec.reserve(1); // Make sure to only allocate one extra element
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
        let new_module_uuid = NameElem::Module(self.modules.alloc(md));
        self.associated_values.push(new_module_uuid);
        self.add_name(module_name, new_module_uuid);
    }

    pub fn add_type(&mut self, typ: StructType) {
        let type_name = typ.link_info.name.clone();
        let new_type_uuid = NameElem::Type(self.types.alloc(typ));
        self.associated_values.push(new_type_uuid);
        self.add_name(type_name, new_type_uuid);
    }

    pub fn add_const(&mut self, cst: NamedConstant) {
        let const_name = cst.link_info.name.clone();
        let new_const_uuid = NameElem::Constant(self.constants.alloc(cst));
        self.associated_values.push(new_const_uuid);
        self.add_name(const_name, new_const_uuid);
    }
}
