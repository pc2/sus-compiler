use crate::prelude::*;

pub mod checkpoint;
mod resolver;
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

use crate::typing::{
    abstract_type::{DomainType, FullType},
    concrete_type::ConcreteType,
    template::TemplateInputs,
};

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

#[allow(dead_code)]
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

    /// Reset checkpoints. These are to reset errors and resolved_globals
    pub after_initial_parse_cp: CheckPoint,
    pub after_flatten_cp: Option<CheckPoint>,
}

impl LinkInfo {
    pub fn get_full_name(&self) -> String {
        format!("::{}", self.name)
    }
    pub fn get_span_file(&self) -> SpanFile {
        (self.span, self.file)
    }
}

pub struct LinkingErrorLocation {
    pub named_type: &'static str,
    pub full_name: String,
    pub location: Option<SpanFile>,
}

pub trait Linkable {
    fn get_name(&self) -> &str;
    fn get_full_name(&self) -> String {
        format!("::{}", self.get_name())
    }
    fn get_linking_error_location(&self) -> LinkingErrorLocation;
    fn get_link_info(&self) -> Option<&LinkInfo>;
    fn get_span_file(&self) -> Option<SpanFile> {
        self.get_link_info().map(|l| (l.span, l.file))
    }
}

#[derive(Debug)]
pub enum NamedConstant {
    Builtin { name: &'static str, val: TypedValue },
}

impl NamedConstant {
    pub fn get_concrete_type(&self) -> &ConcreteType {
        match self {
            NamedConstant::Builtin { name: _, val } => &val.typ,
        }
    }
    pub fn get_full_type(&self) -> FullType {
        FullType {
            typ: self.get_concrete_type().into(),
            domain: DomainType::Generative,
        }
    }
    pub fn get_value(&self) -> &TypedValue {
        match self {
            NamedConstant::Builtin { name: _, val } => &val,
        }
    }
}

impl Linkable for NamedConstant {
    fn get_name(&self) -> &'static str {
        match self {
            NamedConstant::Builtin { name, val: _ } => name,
        }
    }
    fn get_linking_error_location(&self) -> LinkingErrorLocation {
        LinkingErrorLocation {
            named_type: "Builtin Constant",
            full_name: self.get_full_name(),
            location: None,
        }
    }
    fn get_link_info(&self) -> Option<&LinkInfo> {
        match self {
            NamedConstant::Builtin { name: _, val: _ } => None,
        }
    }
}

impl Linkable for StructType {
    fn get_name(&self) -> &str {
        &self.link_info.name
    }
    fn get_linking_error_location(&self) -> LinkingErrorLocation {
        LinkingErrorLocation {
            named_type: "Struct",
            full_name: self.link_info.get_full_name(),
            location: Some((self.link_info.name_span, self.link_info.file)),
        }
    }
    fn get_link_info(&self) -> Option<&LinkInfo> {
        Some(&self.link_info)
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
        let mut result = Linker {
            types: ArenaAllocator::new(),
            modules: ArenaAllocator::new(),
            constants: ArenaAllocator::new(),
            files: ArenaAllocator::new(),
            global_namespace: HashMap::new(),
        };

        fn add_known_unique_name(result: &mut Linker, name: String, new_obj_id: NameElem) {
            let already_exisits = result
                .global_namespace
                .insert(name.into(), NamespaceElement::Global(new_obj_id));
            assert!(already_exisits.is_none());
        }

        for (name, val) in BUILTIN_CONSTANTS {
            let id = result.constants.alloc(NamedConstant::Builtin {
                name,
                val: TypedValue::from_value(val),
            });
            add_known_unique_name(&mut result, name.into(), NameElem::Constant(id));
        }

        result
    }

    pub fn get_link_info(&self, global: NameElem) -> Option<&LinkInfo> {
        match global {
            NameElem::Module(md_id) => Some(&self.modules[md_id].link_info),
            NameElem::Type(typ_id) => Some(&self.types[typ_id].link_info),
            NameElem::Constant(_) => {
                None // Can't define constants yet
            }
        }
    }
    pub fn get_link_info_mut<'l>(
        modules: &'l mut ArenaAllocator<Module, ModuleUUIDMarker>,
        types: &'l mut ArenaAllocator<StructType, TypeUUIDMarker>,
        global: NameElem
    ) -> Option<&'l mut LinkInfo> {
        match global {
            NameElem::Module(md_id) => Some(&mut modules[md_id].link_info),
            NameElem::Type(typ_id) => Some(&mut types[typ_id].link_info),
            NameElem::Constant(_) => {
                None // Can't define constants yet
            }
        }
    }
    pub fn get_full_name(&self, global: NameElem) -> String {
        match global {
            NameElem::Module(id) => self.modules[id].link_info.get_full_name(),
            NameElem::Type(id) => self.types[id].get_full_name(),
            NameElem::Constant(id) => self.constants[id].get_full_name(),
        }
    }
    fn get_linking_error_location(&self, global: NameElem) -> LinkingErrorLocation {
        match global {
            NameElem::Module(id) => {
                let md = &self.modules[id];
                LinkingErrorLocation {
                    named_type: "Module",
                    full_name: md.link_info.get_full_name(),
                    location: Some((md.link_info.name_span, md.link_info.file)),
                }
            }
            NameElem::Type(id) => self.types[id].get_linking_error_location(),
            NameElem::Constant(id) => self.constants[id].get_linking_error_location(),
        }
    }
    fn for_all_duplicate_declaration_errors<F: FnMut(&CompileError)>(
        &self,
        file_uuid: FileUUID,
        f: &mut F,
    ) {
        // Conflicting Declarations
        for item in &self.global_namespace {
            let NamespaceElement::Colission(colission) = &item.1 else {
                continue;
            };
            let infos: Vec<Option<&LinkInfo>> =
                colission.iter().map(|id| self.get_link_info(*id)).collect();

            for (idx, info) in infos.iter().enumerate() {
                let Some(info) = info else { continue }; // Is not a builtin
                if info.file != file_uuid {
                    continue;
                } // Not for this file
                let mut conflict_infos = Vec::new();
                let mut builtin_conflict = false;
                for (idx_2, conflicts_with) in infos.iter().enumerate() {
                    if idx_2 == idx {
                        continue;
                    }
                    if let Some(conflicts_with) = conflicts_with {
                        conflict_infos.push(conflicts_with);
                    } else {
                        assert!(!builtin_conflict);
                        builtin_conflict = true;
                    }
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
                let reason = if builtin_conflict {
                    format!("Cannot redeclare the builtin '{this_object_name}'")
                } else {
                    format!("'{this_object_name}' conflicts with other declarations:")
                };
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
        let other_parsing_errors = parsing_errors.take_for_editing(file_id, &self.files);

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
    #[allow(dead_code)]
    types: &'linker mut ArenaAllocator<StructType, TypeUUIDMarker>,
    #[allow(dead_code)]
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
}
