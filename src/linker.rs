use std::{collections::{HashMap, HashSet}, cell::RefCell};

use tree_sitter::Tree;

use crate::{
    arena_alloc::{ArenaAllocator, UUIDMarker, UUID},
    errors::{error_info, ErrorCollector},
    file_position::{FileText, Span},
    flattening::Module,
    parser::Documentation,
    typing::Type,
    util::{const_str_position, const_str_position_in_tuples},
    value::Value
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleUUIDMarker;
impl UUIDMarker for ModuleUUIDMarker {const DISPLAY_NAME : &'static str = "module_";}
pub type ModuleUUID = UUID<ModuleUUIDMarker>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeUUIDMarker;
impl UUIDMarker for TypeUUIDMarker {const DISPLAY_NAME : &'static str = "type_";}
pub type TypeUUID = UUID<TypeUUIDMarker>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConstantUUIDMarker;
impl UUIDMarker for ConstantUUIDMarker {const DISPLAY_NAME : &'static str = "constant_";}
pub type ConstantUUID = UUID<ConstantUUIDMarker>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileUUIDMarker;
impl UUIDMarker for FileUUIDMarker {const DISPLAY_NAME : &'static str = "file_";}
pub type FileUUID = UUID<FileUUIDMarker>;

const BUILTIN_TYPES : [&'static str; 2] = [
    "bool",
    "int"
];

const BUILTIN_CONSTANTS : [(&'static str, Value); 2] = [
    ("true", Value::Bool(true)),
    ("false", Value::Bool(false))
];

// Goes together with Links::new
pub const fn get_builtin_type(name : &'static str) -> TypeUUID {
    if let Some(is_type) = const_str_position(name, &BUILTIN_TYPES) {
        TypeUUID::from_hidden_value(is_type)
    } else {
        unreachable!()
    }
}

#[allow(dead_code)]
pub const fn get_builtin_constant(name : &'static str) -> ConstantUUID {
    if let Some(is_constant) = const_str_position_in_tuples(name, &BUILTIN_CONSTANTS) {
        ConstantUUID::from_hidden_value(is_constant)
    } else {
        unreachable!()
    }
}

#[derive(Debug)]
pub struct LinkInfo {
    pub file : FileUUID,
    pub name : String,
    pub name_span : Span,
    pub span : Span,
    pub documentation : Documentation
}

impl LinkInfo {
    pub fn get_full_name(&self) -> String {
        format!("::{}", self.name)
    }
}

pub struct LinkingErrorLocation {
    pub named_type : &'static str,
    pub full_name : String,
    pub location : Option<(FileUUID, Span)>
}

pub trait Linkable {
    fn get_name(&self) -> &str;
    fn get_full_name(&self) -> String {
        format!("::{}", self.get_name())
    }
    fn get_linking_error_location(&self) -> LinkingErrorLocation;
    fn get_link_info(&self) -> Option<&LinkInfo>;
    fn get_link_info_mut(&mut self) -> Option<&mut LinkInfo>;
}

#[derive(Debug)]
pub enum NamedConstant {
    Builtin{name : &'static str, typ : Type, val : Value}
}

#[derive(Debug)]
pub enum NamedType {
    Builtin(&'static str)
}

impl Linkable for NamedConstant {
    fn get_name(&self) -> &'static str {
        match self {
            NamedConstant::Builtin{name, typ:_, val:_} => name
        }
    }
    fn get_linking_error_location(&self) -> LinkingErrorLocation {
        LinkingErrorLocation { named_type: "Builtin Constant", full_name : self.get_full_name(), location: None }
    }
    fn get_link_info(&self) -> Option<&LinkInfo> {
        match self {
            NamedConstant::Builtin{name:_, typ:_, val:_} => None
        }
    }
    fn get_link_info_mut(&mut self) -> Option<&mut LinkInfo> {
        match self {
            NamedConstant::Builtin{name:_, typ:_, val:_} => None
        }
    }
}

impl Linkable for NamedType {
    fn get_name(&self) -> &'static str {
        match self {
            NamedType::Builtin(name) => name,
        }
    }
    fn get_linking_error_location(&self) -> LinkingErrorLocation {
        LinkingErrorLocation { named_type: "Builtin Type", full_name : self.get_full_name(), location: None }
    }
    fn get_link_info(&self) -> Option<&LinkInfo> {
        match self {
            NamedType::Builtin(_) => None,
        }
    }
    fn get_link_info_mut(&mut self) -> Option<&mut LinkInfo> {
        match self {
            NamedType::Builtin(_) => None,
        }
    }
}

pub struct FileData {
    pub file_text : FileText,
    pub parsing_errors : ErrorCollector,
    pub associated_values : Vec<NameElem>,
    pub tree : tree_sitter::Tree
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum NameElem {
    Module(ModuleUUID),
    Type(TypeUUID),
    Constant(ConstantUUID)
}

enum NamespaceElement {
    Global(NameElem),
    Colission(Box<[NameElem]>)
}

// Represents the fully linked set of all files. Incremental operations such as adding and removing files can be performed
pub struct Linker {
    pub types : ArenaAllocator<NamedType, TypeUUIDMarker>,
    pub modules : ArenaAllocator<Module, ModuleUUIDMarker>,
    pub constants : ArenaAllocator<NamedConstant, ConstantUUIDMarker>,
    pub files : ArenaAllocator<FileData, FileUUIDMarker>,
    global_namespace : HashMap<String, NamespaceElement>
}

impl Linker {
    pub fn new() -> Linker {
        let mut result = Linker{
            types : ArenaAllocator::new(),
            modules : ArenaAllocator::new(),
            constants : ArenaAllocator::new(),
            files : ArenaAllocator::new(),
            global_namespace : HashMap::new()
        };

        fn add_known_unique_name(result : &mut Linker, name : String, new_obj_id : NameElem) {
            let already_exisits = result.global_namespace.insert(name.into(), NamespaceElement::Global(new_obj_id));
            assert!(already_exisits.is_none());
        }
        
        // Add builtins
        for name in BUILTIN_TYPES {
            let id = result.types.alloc(NamedType::Builtin(name));
            add_known_unique_name(&mut result, name.into(), NameElem::Type(id));
        }
        for (name, val) in BUILTIN_CONSTANTS {
            let id = result.constants.alloc(NamedConstant::Builtin{name, typ : val.get_type_of_constant(), val});
            add_known_unique_name(&mut result, name.into(), NameElem::Constant(id));
        }

        result
    }

    pub fn get_module_id(&self, name : &str) -> Option<ModuleUUID> {
        let NamespaceElement::Global(NameElem::Module(id)) = self.global_namespace.get(name)? else {return None};
        Some(*id)
    }
    #[allow(dead_code)]
    pub fn get_type_id(&self, name : &str) -> Option<TypeUUID> {
        let NamespaceElement::Global(NameElem::Type(id)) = self.global_namespace.get(name)? else {return None};
        Some(*id)
    }
    #[allow(dead_code)]
    pub fn get_constant_id(&self, name : &str) -> Option<ConstantUUID> {
        let NamespaceElement::Global(NameElem::Constant(id)) = self.global_namespace.get(name)? else {return None};
        Some(*id)
    }

    pub fn get_link_info(&self, global : NameElem) -> Option<&LinkInfo> {
        match global {
            NameElem::Module(md_id) => Some(&self.modules[md_id].link_info),
            NameElem::Type(_) => {
                None // Can't define types yet
            }
            NameElem::Constant(_) => {
                None // Can't define constants yet
            }
        }
    }
    pub fn get_full_name(&self, global : NameElem) -> String {
        match global {
            NameElem::Module(id) => self.modules[id].link_info.get_full_name(),
            NameElem::Type(id) => self.types[id].get_full_name(),
            NameElem::Constant(id) => self.constants[id].get_full_name(),
        }
    }
    fn get_linking_error_location(&self, global : NameElem) -> LinkingErrorLocation {
        match global {
            NameElem::Module(id) => {
                let md = &self.modules[id];
                LinkingErrorLocation{named_type: "Module", full_name : md.link_info.get_full_name(), location: Some((md.link_info.file, md.link_info.name_span))}
            }
            NameElem::Type(id) => self.types[id].get_linking_error_location(),
            NameElem::Constant(id) => self.constants[id].get_linking_error_location(),
        }
    }
    fn get_duplicate_declaration_errors(&self, file_uuid : FileUUID, errors : &ErrorCollector) {
        // Conflicting Declarations
        for item in &self.global_namespace {
            let NamespaceElement::Colission(colission) = &item.1 else {continue};
            let infos : Vec<Option<&LinkInfo>> = colission.iter().map(|id| self.get_link_info(*id)).collect();

            for (idx, info) in infos.iter().enumerate() {
                let Some(info) = info else {continue}; // Is not a builtin
                if info.file != file_uuid {continue} // Not for this file
                let mut conflict_infos = Vec::new();
                let mut builtin_conflict = false;
                for (idx_2, conflicts_with) in infos.iter().enumerate() {
                    if idx_2 == idx {continue}
                    if let Some(conflicts_with) = conflicts_with {
                        conflict_infos.push(conflicts_with);
                    } else {
                        assert!(!builtin_conflict);
                        builtin_conflict = true;
                    }
                }
                let this_object_name = &info.name;
                let infos = conflict_infos.iter().map(|conf_info| error_info(conf_info.name_span, conf_info.file, "Conflicts with".to_owned())).collect();
                let reason = if builtin_conflict {
                    format!("Cannot redeclare the builtin '{this_object_name}'")
                } else {
                    format!("'{this_object_name}' conflicts with other declarations:")
                };
                errors.error_with_info(info.name_span, reason, infos);
            }
        }
    }

    fn get_flattening_errors(&self, file_uuid : FileUUID, errors : &ErrorCollector) {
        for v in &self.files[file_uuid].associated_values {
            match v {
                NameElem::Module(md_id) => {
                    let md = &self.modules[*md_id];
                    errors.ingest(&md.parsing_errors);
                    errors.ingest(&md.flattened.errors);
                    md.instantiations.collect_errors(errors);
                }
                NameElem::Type(_) => {}
                NameElem::Constant(_) => {}
            }
        }
    }

    pub fn get_all_errors_in_file(&self, file_uuid : FileUUID) -> ErrorCollector {
        let errors = self.files[file_uuid].parsing_errors.clone();
        self.get_duplicate_declaration_errors(file_uuid, &errors);
        self.get_flattening_errors(file_uuid, &errors);
        errors
    }

    pub fn remove_everything_in_file(&mut self, file_uuid : FileUUID) -> &mut FileData {
        // For quick lookup if a reference disappears
        let mut to_remove_set = HashSet::new();

        let file_data = &mut self.files[file_uuid];
        // Remove referenced data in file
        for v in file_data.associated_values.drain(..) {
            let was_new_item_in_set = to_remove_set.insert(v);
            assert!(was_new_item_in_set);
            match v {
                NameElem::Module(id) => {self.modules.free(id);}
                NameElem::Type(id) => {self.types.free(id);}
                NameElem::Constant(id) => {self.constants.free(id);}
            }
        }

        // Remove from global namespace
        self.global_namespace.retain(|_, v|  {
            match v {
                NamespaceElement::Global(g) => {
                    !to_remove_set.contains(g)
                }
                NamespaceElement::Colission(colission) => {
                    let mut retain_vec = std::mem::replace::<Box<[NameElem]>>(colission, Box::new([])).into_vec();
                    retain_vec.retain(|g| !to_remove_set.contains(g));
                    *colission = retain_vec.into_boxed_slice();
                    colission.len() > 0
                }
            }
        });

        file_data
    }

    #[allow(dead_code)]
    pub fn remove_file(&mut self, file_uuid : FileUUID) {
        self.remove_everything_in_file(file_uuid);
        self.files.free(file_uuid);
    }

    pub fn get_file_builder(&mut self, file_id : FileUUID) -> FileBuilder<'_> {
        let file_data = &mut self.files[file_id];
        FileBuilder{
            file_id,
            tree: &file_data.tree,
            file_text: &file_data.file_text,
            other_parsing_errors : &file_data.parsing_errors,
            associated_values: &mut file_data.associated_values,
            global_namespace: &mut self.global_namespace,
            types: &mut self.types,
            modules: &mut self.modules,
            constants: &mut self.constants
        }
    }
}



pub struct FileBuilder<'linker> {
    pub file_id : FileUUID,
    pub tree : &'linker Tree,
    pub file_text : &'linker FileText, 
    pub other_parsing_errors : &'linker ErrorCollector,
    associated_values : &'linker mut Vec<NameElem>,
    global_namespace : &'linker mut HashMap<String, NamespaceElement>,
    #[allow(dead_code)]
    types : &'linker mut ArenaAllocator<NamedType, TypeUUIDMarker>,
    modules : &'linker mut ArenaAllocator<Module, ModuleUUIDMarker>,
    #[allow(dead_code)]
    constants : &'linker mut ArenaAllocator<NamedConstant, ConstantUUIDMarker>
}

impl<'linker> FileBuilder<'linker> {
    fn add_name(&mut self, name : String, new_obj_id : NameElem) {
        match self.global_namespace.entry(name) {
            std::collections::hash_map::Entry::Occupied(mut occ) => {
                let new_val = match occ.get_mut() {
                    NamespaceElement::Global(g) => {
                        Box::new([*g, new_obj_id])
                    }
                    NamespaceElement::Colission(coll) => {
                        let mut vec = std::mem::replace(coll, Box::new([])).into_vec();
                        vec.reserve(1); // Make sure to only allocate one extra element
                        vec.push(new_obj_id);
                        vec.into_boxed_slice()
                    }
                };
                occ.insert(NamespaceElement::Colission(new_val));
            },
            std::collections::hash_map::Entry::Vacant(vac) => {
                vac.insert(NamespaceElement::Global(new_obj_id));
            },
        }
    }

    pub fn add_module(&mut self, md : Module) {
        let module_name = md.link_info.name.clone();
        let new_module_uuid = NameElem::Module(self.modules.alloc(md));
        self.associated_values.push(new_module_uuid);
        self.add_name(module_name, new_module_uuid);
    }
}

#[derive(Debug)]
pub struct ResolvedGlobals {
    referenced_globals : Vec<NameElem>,
    all_resolved : bool
}

impl ResolvedGlobals {
    pub fn new() -> ResolvedGlobals {
        ResolvedGlobals{referenced_globals : Vec::new(), all_resolved : true}
    }
}

pub struct GlobalResolver<'linker> {
    linker : &'linker Linker,
    pub file : &'linker FileData,
    resolved_globals : RefCell<Option<ResolvedGlobals>>
}

impl<'linker> GlobalResolver<'linker> {
    pub fn new(linker : &'linker Linker, file_id : FileUUID) -> GlobalResolver<'linker> {
        GlobalResolver{
            linker,
            file : &linker.files[file_id],
            resolved_globals : RefCell::new(Some(ResolvedGlobals::new()))
        }
    }

    pub fn extract_resolved_globals(&self) -> ResolvedGlobals {
        let sub_resolved = self.resolved_globals.replace(None);
        sub_resolved.unwrap()
    }

    pub fn new_sublinker(&self, file_id : FileUUID) -> GlobalResolver<'linker> {
        let this_resolved = self.extract_resolved_globals();
        GlobalResolver{
            linker : self.linker,
            file : &self.linker.files[file_id],
            resolved_globals : RefCell::new(Some(this_resolved))
        }
    }

    pub fn reabsorb_sublinker(&self, sub : Self) {
        let sub_resolved = sub.extract_resolved_globals();
        let old_should_be_none = self.resolved_globals.replace(Some(sub_resolved));
        assert!(old_should_be_none.is_none());
    }

    pub fn resolve_global<'error_collector>(&self, name_span : Span, errors : &'error_collector ErrorCollector) -> ResolvedNameElem<'linker, 'error_collector> {
        let name = &self.file.file_text[name_span];

        let mut resolved_globals_borrow = self.resolved_globals.borrow_mut();
        let resolved_globals = resolved_globals_borrow.as_mut().unwrap();
        match self.linker.global_namespace.get(name) {
            Some(NamespaceElement::Global(found)) => {
                resolved_globals.referenced_globals.push(*found);
                ResolvedNameElem{name_elem: Some(*found), linker: self.linker, span: name_span, errors}
            }
            Some(NamespaceElement::Colission(coll)) => {
                resolved_globals.all_resolved = false;

                let decl_infos = coll.iter().map(|collider_global| {
                    let err_loc = self.linker.get_linking_error_location(*collider_global);
                    if let Some((file, span)) = err_loc.location {
                        error_info(span, file, format!("{} {} declared here", err_loc.named_type, err_loc.full_name))
                    } else {
                        // Kinda hacky, point the 'builtin' back to the declaration location because builtins don't have a location
                        error_info(name_span, errors.file, format!("{} {}", err_loc.named_type, err_loc.full_name))
                    }
                }).collect();

                errors.error_with_info(name_span, format!("There were colliding imports for the name '{name}'. Pick one and import it by name."), decl_infos);

                ResolvedNameElem{name_elem: None, linker: self.linker, span: name_span, errors}
            }
            None => {
                resolved_globals.all_resolved = false;

                errors.error_basic(name_span, format!("No Global of the name '{name}' was found. Did you forget to import it?"));

                ResolvedNameElem{name_elem: None, linker: self.linker, span: name_span, errors}
            }
        }
    }

    pub fn get_module(&self, index: ModuleUUID) -> &'linker Module {
        &self.linker.modules[index]
    }
    pub fn get_constant(&self, index: ConstantUUID) -> &'linker NamedConstant {
        &self.linker.constants[index]
    }
    #[allow(dead_code)]
    pub fn get_type(&self, index: TypeUUID) -> &'linker NamedType {
        &self.linker.types[index]
    }
}

impl<'linker> Drop for GlobalResolver<'linker> {
    fn drop(&mut self) {
        // resolved_globals must have been consumed
        assert!(self.resolved_globals.get_mut().is_none());
    }
}


pub struct ResolvedNameElem<'l, 'e> {
    pub name_elem : Option<NameElem>,
    pub span : Span,
    linker : &'l Linker,
    pub errors : &'e ErrorCollector
}

impl<'l, 'e> ResolvedNameElem<'l, 'e> {
    pub fn not_expected_global_error(self, expected : &str) {
        let info = self.linker.get_linking_error_location(self.name_elem.unwrap());
        let infos = if let Some((file, definition_span)) = info.location {
            vec![error_info(definition_span, file, "Defined here")]
        } else {
            vec![]
        };
        let name = &info.full_name;
        let global_type = info.named_type;
        self.errors.error_with_info(self.span, format!("{name} is not a {expected}, it is a {global_type} instead!"), infos);
    }
    pub fn expect_constant(self) -> Option<ConstantUUID> {
        if let NameElem::Constant(id) = self.name_elem? {
            Some(id)
        } else {
            self.not_expected_global_error("Constant");
            None
        }
    }

    pub fn expect_type(self) -> Option<TypeUUID> {
        if let NameElem::Type(id) = self.name_elem? {
            Some(id)
        } else {
            self.not_expected_global_error("Type");
            None
        }
    }

    pub fn expect_module(self) -> Option<ModuleUUID> {
        if let NameElem::Module(id) = self.name_elem? {
            Some(id)
        } else {
            self.not_expected_global_error("Module");
            None
        }
    }
}

