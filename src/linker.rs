use std::{collections::{HashMap, HashSet}, ops::{IndexMut, Index}};

use crate::{ast::{Module, LinkInfo, Span, Value, GlobalReference}, arena_alloc::{ArenaAllocator, UUID}, parser::{FullParseResult, TokenTreeNode}, tokenizer::Token, errors::{ErrorCollector, error_info}, flattening::flatten};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NamedUUIDMarker;
pub type ValueUUID = UUID<NamedUUIDMarker>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileUUIDMarker;
pub type FileUUID = UUID<FileUUIDMarker>;

const BUILTIN_TYPES : [&'static str; 2] = [
    "bool",
    "int"
];

const BUILTIN_CONSTANTS : [(&'static str, Value); 2] = [
    ("true", Value::Bool(true)),
    ("false", Value::Bool(false))
];

pub struct LinkingErrorLocation<'a> {
    pub named_type : &'static str,
    pub name : &'a str,
    pub location : Option<(FileUUID, usize)>
}

pub trait Linkable {
    fn get_name<'a>(&self, linker : &'a Linker) -> &'a str;
    fn get_full_name(&self, linker : &Linker) -> String {
        format!("::{}", self.get_name(linker))
    }
    fn get_linking_error_location<'a>(&self, linker : &'a Linker) -> LinkingErrorLocation<'a>;
    fn get_link_info(&self) -> Option<&LinkInfo>;
    fn get_link_info_mut(&mut self) -> Option<&mut LinkInfo>;
}

#[derive(Debug)]
pub enum NamedConstant {
    Builtin(&'static str, Value)
}

#[derive(Debug)]
pub enum NamedType {
    Builtin(&'static str)
}

#[derive(Debug)]
pub enum Named {
    Constant(NamedConstant),
    Module(Module),
    Type(NamedType)
}

impl Linkable for NamedConstant {
    fn get_name<'a>(&self, _linker : &'a Linker) -> &'a str {
        match self {
            NamedConstant::Builtin(name, _) => name
        }
    }
    fn get_linking_error_location<'a>(&self, _linker : &'a Linker) -> LinkingErrorLocation<'a> {
        match self {
            NamedConstant::Builtin(name, _) => LinkingErrorLocation { named_type: "Builtin Constant", name, location: None }
        }
    }
    fn get_link_info(&self) -> Option<&LinkInfo> {
        match self {
            NamedConstant::Builtin(_, _) => None
        }
    }
    fn get_link_info_mut(&mut self) -> Option<&mut LinkInfo> {
        match self {
            NamedConstant::Builtin(_, _) => None
        }
    }
}

impl Linkable for NamedType {
    fn get_name<'a>(&self, _linker : &'a Linker) -> &'a str {
        match self {
            NamedType::Builtin(name) => name,
        }
    }
    fn get_linking_error_location<'a>(&self, _linker : &'a Linker) -> LinkingErrorLocation<'a> {
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

impl Linkable for Named {
    fn get_name<'a>(&self, linker : &'a Linker) -> &'a str {
        match self {
            Named::Constant(v) => v.get_name(linker),
            Named::Type(t) => t.get_name(linker),
            Named::Module(md) => {
                let file = &linker.files[md.link_info.file];
                file.get_token_text(md.link_info.name_token)
            },
        }
    }
    fn get_linking_error_location<'a>(&self, linker : &'a Linker) -> LinkingErrorLocation<'a> {
        match self {
            Named::Constant(v) => v.get_linking_error_location(linker),
            Named::Type(t) => t.get_linking_error_location(linker),
            Named::Module(md) => {
                let file = &linker.files[md.link_info.file];
                LinkingErrorLocation { named_type: "Module", name : file.get_token_text(md.link_info.name_token), location: Some((md.link_info.file, md.link_info.name_token)) }
            }
        }
    }
    fn get_link_info(&self) -> Option<&LinkInfo> {
        match self {
            Named::Constant(v) => v.get_link_info(),
            Named::Type(t) => t.get_link_info(),
            Named::Module(md) => {
                Some(&md.link_info)
            }
        }
    }
    fn get_link_info_mut(&mut self) -> Option<&mut LinkInfo> {
        match self {
            Named::Constant(v) => v.get_link_info_mut(),
            Named::Type(t) => t.get_link_info_mut(),
            Named::Module(md) => {
                Some(&mut md.link_info)
            }
        }
    }
}

pub struct FileData {
    pub file_text : String,
    pub tokens : Vec<Token>,
    pub token_hierarchy : Vec<TokenTreeNode>,
    pub parsing_errors : ErrorCollector,
    pub associated_values : Vec<ValueUUID>
}

impl FileData {
    fn get_token_text(&self, token_idx : usize) -> &str {
        &self.file_text[self.tokens[token_idx].get_range()]
    }
    fn get_span_text(&self, token_span : Span) -> &str {
        &self.file_text[self.tokens[token_span.0].get_range().start .. self.tokens[token_span.1].get_range().end]
    }
}

// All modules in the workspace
pub struct Links {
    pub globals : ArenaAllocator<Named, NamedUUIDMarker>,
    global_namespace : HashMap<String, ValueUUID>,
    name_colissions : Vec<(ValueUUID, ValueUUID)>
}

// Represents the fully linked set of all files. Incremental operations such as adding and removing files can be performed
pub struct Linker {
    pub links : Links,
    pub files : ArenaAllocator<FileData, FileUUIDMarker>
}

impl Links {
    pub fn new() -> Links {
        // Add builtins
        let mut globals = ArenaAllocator::new();
        let mut global_namespace = HashMap::new();
        
        for name in BUILTIN_TYPES {
            let id = globals.alloc(Named::Type(NamedType::Builtin(name)));
            let already_exisits = global_namespace.insert(name.to_owned(), id);
            assert!(already_exisits.is_none());
        }
        for (name, val) in BUILTIN_CONSTANTS {
            let id = globals.alloc(Named::Constant(NamedConstant::Builtin(name, val)));
            let already_exisits = global_namespace.insert(name.to_owned(), id);
            assert!(already_exisits.is_none());
        }

        Links{globals, name_colissions : Vec::new(), global_namespace}
    }

    fn resolve_dependencies(namespace : &HashMap<String, ValueUUID>, file : &FileData, link_info : &mut LinkInfo) {
        let mut all_resolved = true;
        for GlobalReference(reference_span, uuid) in &mut link_info.global_references {
            if *uuid == ValueUUID::INVALID {
                let reference_name_str = file.get_span_text(*reference_span);

                *uuid = if let Some(found) = namespace.get(reference_name_str) {
                    *found
                } else {
                    all_resolved = false;
                    UUID::INVALID
                }
            }
        }
        link_info.is_fully_linked = all_resolved;
    }

    fn add_name(&mut self, module_name: &str, new_module_uuid: UUID<NamedUUIDMarker>) {
        match self.global_namespace.entry(module_name.to_owned()) {
            std::collections::hash_map::Entry::Occupied(occ) => {
                self.name_colissions.push((new_module_uuid, *occ.get()));
            },
            std::collections::hash_map::Entry::Vacant(vac) => {
                vac.insert(new_module_uuid);
            },
        }
    }
}

impl Index<ValueUUID> for Links {
    type Output = Named;

    fn index(&self, index: ValueUUID) -> &Self::Output {
        &self.globals[index]
    }
}
impl IndexMut<ValueUUID> for Links {
    fn index_mut(&mut self, index: ValueUUID) -> &mut Self::Output {
        &mut self.globals[index]
    }
}

// This is a class that efficiently collects all files when initially starting, and links them together once all are present. 
// Converts to a proper Linker using self.link()
pub struct PreLinker {
    links : Links,
    files : ArenaAllocator<FileData, FileUUIDMarker>
}

impl PreLinker {
    pub fn new() -> PreLinker {
        PreLinker { links: Links::new(), files: ArenaAllocator::new() }
    }
    pub fn reserve_file(&mut self) -> FileUUID {
        self.files.reserve()
    }
    pub fn add_reserved_file(&mut self, file : FileUUID, file_text : String, parse_result : FullParseResult, parsing_errors : ErrorCollector) {
        let mut associated_values = Vec::new();
        for md in parse_result.ast.modules {
            let module_name = &file_text[parse_result.tokens[md.link_info.name_token].get_range()];
            let new_module_uuid = self.links.globals.alloc(Named::Module(md));
            associated_values.push(new_module_uuid);
            self.links.add_name(module_name, new_module_uuid);
        }
        self.files.alloc_reservation(file, FileData { file_text, tokens: parse_result.tokens, token_hierarchy: parse_result.token_hierarchy, parsing_errors, associated_values});
    }

    // This should be called once all modules have been added. Adds errors for globals it couldn't match
    pub fn link(mut self) -> Linker {
        for (_file_uuid, file) in &self.files {
            for val_in_file in &file.associated_values {
                let link_info = self.links.globals[*val_in_file].get_link_info_mut().unwrap();
                Links::resolve_dependencies(&self.links.global_namespace, &file, link_info);
            }
        }
        Linker{links: self.links, files : self.files}
    }
}

fn add_error(file: &FileData, info_a: &LinkInfo, info_b: &LinkInfo, errors: &mut ErrorCollector) {
    let this_object_name = file.get_token_text(info_a.name_token);
    errors.error_with_info(Span::from(info_a.name_token), format!("Conflicting Declaration for the name '{this_object_name}'"), vec![
        error_info(Span::from(info_b.name_token), info_b.file, "Conflicting Declaration")
    ]);
}

impl Linker {
    pub fn get_linking_errors(&self, file_uuid : FileUUID, errors : &mut ErrorCollector) {
        let file = &self.files[file_uuid];

        // Conflicting Declarations
        for colission in &self.links.name_colissions {
            let info_0 = self.links.globals[colission.0].get_link_info().unwrap(); // Is always valid because colission.0 is 'the thing that conflicts with'
            let info_1_opt = self.links.globals[colission.1].get_link_info();

            if info_0.file == file_uuid {
                if let Some(info_1) = info_1_opt {
                    add_error(file, info_0, info_1, errors);
                    if info_1.file == file_uuid {
                        add_error(file, info_1, info_0, errors);
                    }
                } else {
                    let this_object_name = file.get_token_text(info_0.name_token);
                    errors.error_basic(Span::from(info_0.name_token), format!("Cannot redeclare the builtin '{this_object_name}'"));
                }
            } else if let Some(info_1) = info_1_opt {
                if info_1.file == file_uuid {
                    add_error(file, info_1, info_0, errors);
                }
            }
        }
        
        // References not found
        for val_uuid in &self.files[file_uuid].associated_values {
            let object = &self.links.globals[*val_uuid];
            let object_link_info = object.get_link_info().unwrap(); // Always valid because it's part of file
            if object_link_info.is_fully_linked {
                continue; // Early exit because we know this object contains no linking errors
            }
            for GlobalReference(reference_span, ref_uuid) in &object_link_info.global_references {
                if *ref_uuid == ValueUUID::INVALID {
                    let reference_text = file.get_span_text(*reference_span);
                    errors.error_basic(*reference_span, format!("No Value or Type of the name '{reference_text}' was found. Did you forget to import it?"));
                }
            }
        }
    }

    pub fn remove_file_datas(&mut self, files : &[FileUUID]) {
        // For quick lookup if a reference disappears
        let mut back_reference_set = HashSet::new();

        // Remove the files and their referenced values
        for file in files {
            for v in &self.files[*file].associated_values {
                back_reference_set.insert(v);
                self.links.globals.free(*v);
            }
        }

        // Remove resolved globals
        for (_uuid, v) in &mut self.links.globals {
            if let Some(info) = v.get_link_info_mut() { // Builtins can't refer to other things
                for GlobalReference(_name, v) in &mut info.global_references {
                    if back_reference_set.contains(v) {
                        *v = ValueUUID::INVALID;
                        info.is_fully_linked = false;
                    }
                }
            }
        }

        // Remove possible conflicts
        let mut conflict_replacements = HashMap::new();
        let nc = &mut self.links.name_colissions;
        let mut i = 0;
        while i < nc.len() {
            let (c_0, c_1) = nc[i];
            if back_reference_set.contains(&c_1) {
                let last = *nc.last().unwrap();
                nc[i] = last;
                nc.pop();
            } else {
                // does not contain c_1, but does contain c_0. Have to recreate conflicts containing c_0 to instead refer to c_1
                let last = *nc.last().unwrap();
                nc[i] = last;
                nc.pop();
                conflict_replacements.insert(c_0, c_1);
            }
            i += 1;
        }
        if !conflict_replacements.is_empty() {
            for conflict in nc {
                if let Some(replacement) = conflict_replacements.get(&conflict.0) {
                    conflict.0 = *replacement;
                }
            }

            // Remove names from the global namespace
            self.links.global_namespace.retain(|_k, v| -> bool {
                !back_reference_set.contains(v)
            });
        } else {
            // Remove names from the global namespace, also have to rename renamed things
            self.links.global_namespace.retain(|_k, v| -> bool {
                if let Some(found_replacement) = conflict_replacements.get(v) {
                    *v = *found_replacement;
                    return true;
                }
                !back_reference_set.contains(v)
            });
        }
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
    
    pub fn add_reserved_file(&mut self, file : FileUUID, file_text : String, parse_result : FullParseResult, parsing_errors : ErrorCollector) {
        let mut associated_values = Vec::new();
        for md in parse_result.ast.modules {
            let module_name = &file_text[parse_result.tokens[md.link_info.name_token].get_range()];
            let new_module_uuid = self.links.globals.alloc(Named::Module(md));
            associated_values.push(new_module_uuid);
            self.links.add_name(module_name, new_module_uuid);
        }
        self.files.alloc_reservation(file, FileData { file_text, tokens: parse_result.tokens, token_hierarchy: parse_result.token_hierarchy, parsing_errors, associated_values});

        for (_uuid, val_in_file) in &mut self.links.globals {
            if let Some(link_info) = val_in_file.get_link_info_mut() {
                if link_info.is_fully_linked {
                    continue; // Early continue, because we know this object is already fully linked
                }
                Links::resolve_dependencies(&self.links.global_namespace, &self.files[link_info.file], link_info);
            }
        }
    }

    pub fn relink(&mut self, file : FileUUID, file_text : String, parse_result : FullParseResult, parsing_errors : ErrorCollector) {
        self.remove_file_datas(&[file]);
        self.files.revert_to_reservation(file);
        self.add_reserved_file(file, file_text, parse_result, parsing_errors);
    }

    pub fn get_constant(&self, GlobalReference(identifier_span, uuid) : GlobalReference, errors : &mut ErrorCollector) -> Option<Value> {
        match &self.links.globals[uuid] {
            Named::Constant(NamedConstant::Builtin(_name, v)) => {
                Some(v.clone())
            },
            other => {
                let info = other.get_linking_error_location(self);
                let infos = if let Some((file, position)) = info.location {
                    vec![error_info(Span::from(position), file, "Defined here")]
                } else {
                    vec![]
                };
                let name = info.name;
                let ident_type = info.named_type;
                errors.error_with_info(identifier_span, format!("{ident_type} {name} is not a Constant!"), infos);
                None
            }
        }
    }

    pub fn get_module(&self, GlobalReference(identifier_span, uuid) : GlobalReference, errors : &mut ErrorCollector) -> Option<&Module> {
        match &self.links.globals[uuid] {
            Named::Module(md) => {
                Some(md)
            },
            other => {
                let info = other.get_linking_error_location(self);
                let infos = if let Some((file, position)) = info.location {
                    vec![error_info(Span::from(position), file, "Defined here")]
                } else {
                    vec![]
                };
                let name = info.name;
                let ident_type = info.named_type;
                errors.error_with_info(identifier_span, format!("{ident_type} {name} is not a Module!"), infos);
                None
            }
        }
    }

    pub fn flatten_all_modules_in_file(&self, file : FileUUID, errors : &mut ErrorCollector) {
        for md_uuid in &self.files[file].associated_values {
            let named = &self.links.globals[*md_uuid];
            if let Named::Module(md) = named {
                if !md.link_info.is_fully_linked {
                    continue;
                }
                let flt = flatten(md, &self, errors);
                println!("{:?}", flt);
            }
        }
    }
}
