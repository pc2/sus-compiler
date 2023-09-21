use std::{collections::HashMap, ops::{IndexMut, Index}, path::PathBuf};

use crate::{ast::{Module, Location, Dependencies, GlobalReference}, arena_alloc::{ArenaAllocator, UUID}, parser::{FullParseResult, TokenTreeNode}, tokenizer::Token, errors::ErrorCollector};

#[derive(Debug, Clone, Copy)]
pub struct NamedUUIDMarker;
pub type ValueUUID = UUID<NamedUUIDMarker>;

#[derive(Debug, Clone, Copy)]
pub struct FileUUIDMarker;
pub type FileUUID = UUID<FileUUIDMarker>;

const BUILTIN_TYPES : [&'static str; 2] = [
    "bool",
    "int"
];

pub trait Linkable {
    fn get_location(&self) -> Option<&Location>;
    fn get_dependencies(&self) -> &Dependencies;
    fn get_dependencies_mut(&mut self) -> &mut Dependencies;
}

#[derive(Debug)]
pub enum NamedValue {
    Builtin(&'static str),
    Module(Module)
}

#[derive(Debug)]
pub enum NamedType {
    Builtin(&'static str)
}

#[derive(Debug)]
pub enum Named {
    Value(NamedValue),
    Type(NamedType)
}

impl Linkable for NamedValue {
    fn get_dependencies(&self) -> &Dependencies {
        match self {
            NamedValue::Builtin(_) => unreachable!(),
            NamedValue::Module(md) => {
                &md.dependencies
            }
        }
    }
    fn get_dependencies_mut(&mut self) -> &mut Dependencies {
        match self {
            NamedValue::Builtin(_) => unreachable!(),
            NamedValue::Module(md) => {
                &mut md.dependencies
            }
        }
    }
    fn get_location(&self) -> Option<&Location> {
        match self {
            NamedValue::Builtin(_) => None,
            NamedValue::Module(md) => {
                Some(&md.location)
            }
        }
    }
}

impl Linkable for NamedType {
    fn get_dependencies(&self) -> &Dependencies {
        match self {
            NamedType::Builtin(_) => unreachable!(),
        }
    }
    fn get_dependencies_mut(&mut self) -> &mut Dependencies {
        match self {
            NamedType::Builtin(_) => unreachable!(),
        }
    }
    fn get_location(&self) -> Option<&Location> {
        match self {
            NamedType::Builtin(_name) => None
        }
    }
}

impl Linkable for Named {
    fn get_dependencies(&self) -> &Dependencies {
        match self {
            Named::Value(v) => v.get_dependencies(),
            Named::Type(t) => t.get_dependencies()
        }
    }
    fn get_dependencies_mut(&mut self) -> &mut Dependencies {
        match self {
            Named::Value(v) => v.get_dependencies_mut(),
            Named::Type(t) => t.get_dependencies_mut()
        }
    }
    fn get_location(&self) -> Option<&Location> {
        match self {
            Named::Value(v) => v.get_location(),
            Named::Type(t) => t.get_location()
        }
    }
}

pub struct FileData {
    pub file_text : String,
    pub file_path : PathBuf,
    pub tokens : Vec<Token>,
    pub token_hierarchy : Vec<TokenTreeNode>,
    pub parsing_errors : ErrorCollector,
    pub associated_values : Vec<ValueUUID>
}

// All modules in the workspace
pub struct Links {
    globals : ArenaAllocator<Named, NamedUUIDMarker>,
    global_namespace : HashMap<String, ValueUUID>,
    name_colissions : Vec<(ValueUUID, ValueUUID)>
}

// Represents the fully linked set of all files. Incremental operations such as adding and removing files can be performed
pub struct Linker {
    links : Links,
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

        Links{globals, name_colissions : Vec::new(), global_namespace}
    }

    fn resolve_dependencies(&self, file_text : &str, deps : &[GlobalReference]) -> Vec<ValueUUID> {
        deps.iter().map(|reference| {
            let reference_name_str = &file_text[reference[0].text.clone()];

            if let Some(found) = self.global_namespace.get(reference_name_str) {
                *found
            } else {
                UUID::INVALID
            }
        }).collect()
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
    pub fn add_reserved_file(&mut self, file : FileUUID, file_path : PathBuf, file_text : String, parse_result : FullParseResult, parsing_errors : ErrorCollector) {
        let mut associated_values = Vec::new();
        for md in parse_result.ast.modules {
            let module_name = &file_text[md.name.text.clone()];
            let new_module_uuid = self.links.globals.alloc(Named::Value(NamedValue::Module(md)));
            associated_values.push(new_module_uuid);
            self.links.global_namespace.insert(module_name.to_owned(), new_module_uuid);
        }
        self.files.alloc_reservation(file, FileData { file_text, file_path, tokens: parse_result.tokens, token_hierarchy: parse_result.token_hierarchy, parsing_errors, associated_values});
    }

    // This should be called once all modules have been added. Adds errors for globals it couldn't match
    pub fn link(mut self) -> Linker {
        for (_file_uuid, file) in &mut self.files {
            for idx in &file.associated_values {
                let deps = self.links.globals[*idx].get_dependencies();
                let vals_this_refers_to = self.links.resolve_dependencies(&file.file_text, &deps.global_references);
                let deps_mut = self.links.globals[*idx].get_dependencies_mut();
                deps_mut.resolved_globals = vals_this_refers_to;
            }
        }
        Linker{links: self.links, files : self.files}
    }
}
/*
impl Linker {
    pub fn get_linking_errors(&self, file : FileUUID) -> ErrorCollector {
        for colission in &self.links.name_colissions {
            if let Some(location) = self.links.globals[*uuid1].get_location() {
                errors.error_with_info(Span::from(md.name.position), format!("Conflicting Module Declaration for the name '{module_name}'"), vec![
                    error_info(location.span, location.file_name.clone(), "Conflicting Declaration")
                ]);
            } else {
                errors.error_basic(Span::from(md.name.position), format!("Cannot redeclare the builtin '{module_name}'"));
            }
        }
   
        Some(GlobalNamespaceNode::Type(TypeUUID(t))) => {
            let found_instead = &self.types[*t];
            let found_full_name = found_instead.get_full_name();
            let infos = if let Some(loc) = found_instead.get_location() {
                vec![error_info(loc.span, loc.file_name.clone(), "Defined here")]
            } else {
                vec![]
            };
            errors.error_with_info(reference_span, format!("No {VALUE_NAMES} of the name '{reference_name_str}' was found. Found Type '{found_full_name}'"), infos);
            ValueUUID(INVALID_UUID)
        }
        None => {
            errors.error_basic(reference_span, format!("No {VALUE_NAMES} of the name '{reference_name_str}' was found. Did you forget to import it?"));
            ValueUUID(INVALID_UUID)
        }
    }
    pub fn remove(&mut self, file_name : FileName) {

    }
    pub fn relink(&mut self, file_name : FileName, file_text : String, ast : ASTRoot, mut errors : ErrorCollector) {
        match self.files.entry(file_name) {
            Entry::Occupied(mut exists) => {
                let existing_entry = exists.get_mut();

                for ValueUUID(v) in &mut existing_entry.associated_values {

                }
            },
            Entry::Vacant(new_entry) => {

            },
        }
    }
}*/
