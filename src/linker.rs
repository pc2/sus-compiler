use std::{collections::{HashMap, HashSet}, rc::Rc, cell::RefCell};

use crate::{ast::{Module, LinkInfo, Span}, arena_alloc::{ArenaAllocator, UUID, UUIDMarker}, parser::{FullParseResult, TokenTreeNode}, tokenizer::Token, errors::{ErrorCollector, error_info}, flattening::FlattenedModule, util::{const_str_position, const_str_position_in_tuples}, instantiation::InstantiatedModule, value::Value, typing::Type};

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

pub const fn get_builtin_constant(name : &'static str) -> ConstantUUID {
    if let Some(is_constant) = const_str_position_in_tuples(name, &BUILTIN_CONSTANTS) {
        ConstantUUID::from_hidden_value(is_constant)
    } else {
        unreachable!()
    }
}

pub struct LinkingErrorLocation<'a> {
    pub named_type : &'static str,
    pub name : &'a str,
    pub location : Option<(FileUUID, Span)>
}

pub trait Linkable {
    fn get_name(&self) -> &str;
    fn get_full_name(&self) -> String {
        format!("::{}", self.get_name())
    }
    fn get_linking_error_location<'a>(&'a self) -> LinkingErrorLocation<'a>;
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
    fn get_linking_error_location<'a>(&'a self) -> LinkingErrorLocation<'a> {
        match self {
            NamedConstant::Builtin{name, typ:_, val:_} => LinkingErrorLocation { named_type: "Builtin Constant", name, location: None }
        }
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
    fn get_linking_error_location<'a>(&'a self) -> LinkingErrorLocation<'a> {
        match self {
            NamedType::Builtin(name) => LinkingErrorLocation { named_type: "Builtin Type", name, location: None },
        }
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
    pub file_text : String,
    pub tokens : Vec<Token>,
    pub token_hierarchy : Vec<TokenTreeNode>,
    pub parsing_errors : ErrorCollector,
    pub associated_values : Vec<NameElem>
}

impl FileData {
    fn get_token_text(&self, token_idx : usize) -> &str {
        &self.file_text[self.tokens[token_idx].get_range()]
    }
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
    global_namespace : HashMap<Box<str>, NamespaceElement>,
    pub files : ArenaAllocator<FileData, FileUUIDMarker>
}

impl Linker {
    pub fn new() -> Linker {
        // Add builtins
        let mut types = ArenaAllocator::new();
        let modules = ArenaAllocator::new();
        let mut constants = ArenaAllocator::new();
        let files = ArenaAllocator::new();
        let mut global_namespace = HashMap::new();
        
        for name in BUILTIN_TYPES {
            let id = types.alloc(NamedType::Builtin(name));
            let already_exisits = global_namespace.insert(name.into(), NamespaceElement::Global(NameElem::Type(id)));
            assert!(already_exisits.is_none());
        }
        for (name, val) in BUILTIN_CONSTANTS {
            let id = constants.alloc(NamedConstant::Builtin{name, typ : val.get_type_of_constant(), val});
            let already_exisits = global_namespace.insert(name.into(), NamespaceElement::Global(NameElem::Constant(id)));
            assert!(already_exisits.is_none());
        }

        Linker{types, modules, constants, files, global_namespace}
    }

    pub fn get_module_id(&self, name : &str) -> Option<ModuleUUID> {
        let NamespaceElement::Global(NameElem::Module(id)) = self.global_namespace.get(name)? else {return None};
        Some(*id)
    }
    pub fn get_type_id(&self, name : &str) -> Option<TypeUUID> {
        let NamespaceElement::Global(NameElem::Type(id)) = self.global_namespace.get(name)? else {return None};
        Some(*id)
    }
    pub fn get_constant_id(&self, name : &str) -> Option<ConstantUUID> {
        let NamespaceElement::Global(NameElem::Constant(id)) = self.global_namespace.get(name)? else {return None};
        Some(*id)
    }

    fn add_name(&mut self, name: Box<str>, new_obj_id: NameElem) {
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
    fn get_link_info(&self, name_elem : NameElem) -> Option<&LinkInfo> {
        match name_elem {
            NameElem::Module(md_id) => Some(&self.modules[md_id].link_info),
            NameElem::Type(_) => {
                None // Can't define types yet
            }
            NameElem::Constant(_) => {
                None // Can't define constants yet
            }
        }
    }
    fn get_linking_error_location<'a>(&'a self, global : NameElem) -> LinkingErrorLocation<'a> {
        match global {
            NameElem::Module(id) => {
                let md = &self.modules[id];
                LinkingErrorLocation{named_type: "Module", name : &md.link_info.name, location: Some((md.link_info.file, md.link_info.name_span))}
            }
            NameElem::Type(id) => self.types[id].get_linking_error_location(),
            NameElem::Constant(id) => self.constants[id].get_linking_error_location(),
        }
    }
    fn get_duplicate_declaration_errors(&self, file_uuid : FileUUID, errors : &ErrorCollector) {
        // Conflicting Declarations
        for item in &self.global_namespace {
            let NamespaceElement::Colission(colission) = &item.1 else {continue};
            let infos : Box<[Option<&LinkInfo>]> = colission.iter().map(|id| self.get_link_info(*id)).collect();

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
                    errors.ingest(&md.flattened.errors);
                    md.instantiations.collect_errors(errors);
                }
                NameElem::Type(_) => {}
                NameElem::Constant(_) => {}
            }
        }
    }

    pub fn get_all_errors_in_file(&self, file_uuid : FileUUID, errors : &ErrorCollector) {
        self.get_duplicate_declaration_errors(file_uuid, errors);
        self.get_flattening_errors(file_uuid, errors);
    }

    pub fn remove_file_datas(&mut self, files : &[FileUUID]) {
        // For quick lookup if a reference disappears
        let mut to_remove_set = HashSet::new();

        // Remove the files and their referenced values
        for file in files {
            for v in &self.files[*file].associated_values {
                let was_new_item_in_set = to_remove_set.insert(v);
                assert!(was_new_item_in_set);
                match *v {
                    NameElem::Module(id) => {self.modules.free(id);}
                    NameElem::Type(id) => {self.types.free(id);}
                    NameElem::Constant(id) => {self.constants.free(id);}
                }
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
    }

    pub fn remove_files(&mut self, files : &[FileUUID]) {
        self.remove_file_datas(files);
        for uuid in files {
            self.files.free(*uuid);
        }
    }

    pub fn reserve_file(&mut self) -> FileUUID {
        self.files.reserve()
    }
    
    pub fn add_reserved_file(&mut self, file : FileUUID, parse_result : FullParseResult) {
        let mut associated_values = Vec::new();
        for md in parse_result.ast.modules {
            let module_name = md.link_info.name.clone();
            let new_module_uuid = NameElem::Module(self.modules.alloc(md));
            associated_values.push(new_module_uuid);
            self.add_name(module_name, new_module_uuid);
        }
        self.files.alloc_reservation(file, FileData { file_text : parse_result.file_text, tokens: parse_result.tokens, token_hierarchy: parse_result.token_hierarchy, parsing_errors : parse_result.ast.errors, associated_values});
    }

    pub fn relink(&mut self, file : FileUUID, parse_result : FullParseResult) {
        self.remove_file_datas(&[file]);
        self.files.revert_to_reservation(file);
        self.add_reserved_file(file, parse_result);
    }

    pub fn recompile_all(&mut self) {
        // Flatten all modules
        let id_vec : Vec<ModuleUUID> = self.modules.iter().map(|(id, _)| id).collect();
        for id in id_vec {
            let md = &self.modules[id];// Have to get them like this, so we don't have a mutable borrow on self.modules across the loop
            println!("Flattening {}", md.link_info.name);

            let mut flattened = FlattenedModule::initialize(&self, md);
            println!("Typechecking {}", &md.link_info.name);
            flattened.typecheck(self);
            flattened.find_unused_variables();

            let md = &mut self.modules[id]; // Convert to mutable ptr
            md.flattened = flattened;
            md.instantiations.clear_instances();
        }

        // Can't merge these loops, because instantiation can only be done once all modules have been type checked
        for (id, md) in &self.modules {
            println!("[[{}]]:", md.link_info.name);
            md.print_flattened_module();
            let inst = self.instantiate(id);
        }
    }

    pub fn instantiate(&self, module_id : ModuleUUID) -> Option<Rc<InstantiatedModule>> {
        let md = &self.modules[module_id];
        println!("Instantiating {}", md.link_info.name);

        md.instantiations.instantiate(&md.link_info.name, &md.flattened, self)
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

pub struct GlobalResolver<'linker, 'resolved_list> {
    linker : &'linker Linker,
    file : &'linker FileData,

    resolved_globals : &'resolved_list RefCell<ResolvedGlobals>
}

impl<'linker, 'resolved_list> GlobalResolver<'linker, 'resolved_list> {
    pub fn new(linker : &'linker Linker, file_id : FileUUID, resolved_globals : &'resolved_list RefCell<ResolvedGlobals>) -> GlobalResolver<'linker, 'resolved_list> {
        GlobalResolver{linker, file : &linker.files[file_id], resolved_globals}
    }

    pub fn new_sublinker(&self, file_id : FileUUID) -> GlobalResolver<'linker, 'resolved_list> {
        GlobalResolver{linker : self.linker, file : &self.linker.files[file_id], resolved_globals : self.resolved_globals}
    }

    pub fn resolve_global(&self, name_span : Span, errors : &ErrorCollector) -> Option<NameElem> {
        let name = self.file.get_token_text(name_span.assert_is_single_token());

        let mut resolved_globals = self.resolved_globals.borrow_mut();
        if let Some(NamespaceElement::Global(found)) = self.linker.global_namespace.get(name) {
            resolved_globals.referenced_globals.push(*found);
            Some(*found)
        } else {
            resolved_globals.all_resolved = false;

            errors.error_basic(name_span, format!("No Value or Type of the name '{name}' was found. Did you forget to import it?"));

            None
        }
    }

    pub fn try_resolve_global(&self, name_span : Span) -> Option<NameElem> {
        let name = self.file.get_token_text(name_span.assert_is_single_token());

        let mut resolved_globals = self.resolved_globals.borrow_mut();
        if let Some(NamespaceElement::Global(found)) = self.linker.global_namespace.get(name) {
            resolved_globals.referenced_globals.push(*found);
            Some(*found)
        } else {
            resolved_globals.all_resolved = false;

            None
        }
    }
    
    pub fn get_module(&self, uuid : ModuleUUID) -> &'linker Module {
        &self.linker.modules[uuid]
    }

    pub fn make_bad_error_location_error(&self, elem : NameElem, expected : &str, identifier_span : Span, errors : &ErrorCollector) {
        let info = self.linker.get_linking_error_location(elem);
        let infos = if let Some((file, span)) = info.location {
            vec![error_info(span, file, "Defined here")]
        } else {
            vec![]
        };
        let name = info.name;
        let ident_type = info.named_type;
        errors.error_with_info(identifier_span, format!("{ident_type} {name} is not a {expected}!"), infos);
    }
    pub fn resolve_constant(&self, identifier_span : Span, errors : &ErrorCollector) -> Option<ConstantUUID> {
        match self.resolve_global(identifier_span, errors)? {
            NameElem::Constant(id) => {
                Some(id)
            },
            other => {
                self.make_bad_error_location_error(other, "Constant", identifier_span, errors);
                None
            }
        }
    }

    pub fn resolve_type(&self, identifier_span : Span, errors : &ErrorCollector) -> Option<TypeUUID> {
        match self.resolve_global(identifier_span, errors)? {
            NameElem::Type(id) => {
                Some(id)
            },
            other => {
                self.make_bad_error_location_error(other, "Type", identifier_span, errors);
                None
            }
        }
    }

    pub fn resolve_module(&self, identifier_span : Span, errors : &ErrorCollector) -> Option<ModuleUUID> {
        match self.resolve_global(identifier_span, errors)? {
            NameElem::Module(id) => {
                Some(id)
            },
            other => {
                self.make_bad_error_location_error(other, "Module", identifier_span, errors);
                None
            }
        }
    }
}
