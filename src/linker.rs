use std::{collections::{HashMap, HashSet}, rc::Rc, cell::RefCell};

use crate::{
    arena_alloc::{ArenaAllocator, UUIDMarker, UUID},
    errors::{error_info, ErrorCollector},
    file_position::{FileText, Span},
    flattening::{FlatID, FlattenedModule, Instruction, Module, WireInstance, WireSource},
    instantiation::{InstantiatedModule, InstantiationList},
    parser::{Cursor, Documentation, FullParseResult, SUS},
    typing::{Type, WrittenType},
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
    global_namespace : HashMap<String, NamespaceElement>,
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

    fn add_name(&mut self, name: String, new_obj_id: NameElem) {
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

    #[allow(dead_code)]
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
        
        {
            let mut walker = Cursor::new_at_root(&parse_result.tree, &parse_result.file_text);
            walker.list(SUS.source_file_kind, |cursor| {
                let (kind, span) = cursor.kind_span();
                assert!(kind == SUS.module_kind);
                let name_span = cursor.go_down_no_check(|cursor| {cursor.field_span(SUS.name_field, SUS.identifier_kind)});
                let md = Module{
                    link_info: LinkInfo {
                        documentation: cursor.extract_gathered_comments(),
                        file,
                        name: parse_result.file_text[name_span].to_owned(),
                        name_span,
                        span
                    },
                    flattened: FlattenedModule::empty(ErrorCollector::new(file, parse_result.file_text.len())),
                    instantiations: InstantiationList::new()
                };
                let module_name = md.link_info.name.clone();
                let new_module_uuid = NameElem::Module(self.modules.alloc(md));
                associated_values.push(new_module_uuid);
                self.add_name(module_name, new_module_uuid);
            });
        }
        
        self.files.alloc_reservation(file, FileData{
            file_text : parse_result.file_text,
            tree: parse_result.tree,
            parsing_errors : parse_result.errors,
            associated_values
        });
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

            let flattened = FlattenedModule::flatten(&self, md);
            println!("Typechecking {}", &md.link_info.name);

            let md = &mut self.modules[id]; // Convert to mutable ptr
            md.flattened = flattened;
            md.instantiations.clear_instances();
        }

        // Can't merge these loops, because instantiation can only be done once all modules have been type checked
        for (id, _md) in &self.modules {
            //md.print_flattened_module();
            // Already instantiate any modules without parameters
            // Currently this is all modules
            let _inst = self.instantiate(id);
        }
    }

    pub fn instantiate(&self, module_id : ModuleUUID) -> Option<Rc<InstantiatedModule>> {
        let md = &self.modules[module_id];
        println!("Instantiating {}", md.link_info.name);

        md.instantiations.instantiate(&md.link_info.name, &md.flattened, self)
    }

    pub fn get_info_about_source_location<'linker>(&'linker self, position : usize, file : FileUUID) -> Option<(LocationInfo<'linker>, Span)> {
        let mut location_builder = LocationInfoBuilder::new(position);
        
        for global in &self.files[file].associated_values {
            match *global {
                NameElem::Module(md_id) => {
                    let md = &self.modules[md_id];
                    if md.link_info.span.contains_pos(position) {
                        location_builder.update(md.link_info.name_span, LocationInfo::Global(NameElem::Module(md_id)));
                        for (id, inst) in &md.flattened.instructions {
                            match inst {
                                Instruction::SubModule(sm) => {
                                    location_builder.update(sm.module_name_span, LocationInfo::Global(NameElem::Module(sm.module_uuid)));
                                }
                                Instruction::Declaration(decl) => {
                                    match decl.typ_expr.get_deepest_selected(position) {
                                        Some(WrittenType::Named(span, name_id)) => {
                                            location_builder.update(*span, LocationInfo::Global(NameElem::Type(*name_id)));
                                        }
                                        Some(typ) => {
                                            location_builder.update(typ.get_span(), LocationInfo::Type(typ));
                                        }
                                        None => {}
                                    }
                                    if decl.declaration_itself_is_not_written_to && decl.name_span.contains_pos(position) {
                                        location_builder.update(decl.name_span, LocationInfo::WireRef(md, id));
                                    }
                                }
                                Instruction::Wire(wire) => {
                                    let loc_info = if let WireSource::WireRead(decl_id) = &wire.source {
                                        LocationInfo::WireRef(md, *decl_id)
                                    } else {
                                        LocationInfo::Temporary(md, id, wire)
                                    };
                                    location_builder.update(wire.span, loc_info);
                                }
                                Instruction::Write(write) => {
                                    location_builder.update(write.to.root_span, LocationInfo::WireRef(md, write.to.root));
                                }
                                Instruction::IfStatement(_) | Instruction::ForStatement(_) => {}
                            };
                        }
                    }
                }
                NameElem::Type(_) => {
                    todo!()
                }
                NameElem::Constant(_) => {
                    todo!()
                }
            }
        }
        if let Some(instr) = location_builder.best_instruction {
            Some((instr, location_builder.best_span))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum LocationInfo<'linker> {
    WireRef(&'linker Module, FlatID),
    Temporary(&'linker Module, FlatID, &'linker WireInstance),
    Type(&'linker WrittenType),
    Global(NameElem)
}

struct LocationInfoBuilder<'linker> {
    best_instruction : Option<LocationInfo<'linker>>,
    best_span : Span,
    position : usize
}

impl<'linker> LocationInfoBuilder<'linker> {
    fn new(token_idx : usize) -> Self {
        Self{
            best_instruction : None,
            best_span : Span::MAX_POSSIBLE_SPAN,
            position: token_idx
        }
    }
    fn update(&mut self, span : Span, info : LocationInfo<'linker>) {
        if span.contains_pos(self.position) && span.size() <= self.best_span.size() {
            //assert!(span.size() < self.best_span.size());
            // May not be the case. Do prioritize later ones, as they tend to be nested
            self.best_span = span;
            self.best_instruction = Some(info);
        }
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

