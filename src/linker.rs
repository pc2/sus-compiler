use std::{collections::HashMap, ops::{IndexMut, Index}, path::PathBuf};

use crate::{ast::{Module, Location, Dependencies, GlobalReference, Span}, arena_alloc::{ArenaAllocator, UUID}, parser::{FullParseResult, TokenTreeNode}, tokenizer::Token, errors::{ErrorCollector, error_info}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NamedUUIDMarker;
pub type ValueUUID = UUID<NamedUUIDMarker>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileUUIDMarker;
pub type FileUUID = UUID<FileUUIDMarker>;

const BUILTIN_TYPES : [&'static str; 2] = [
    "bool",
    "int"
];

const BUILTIN_VALUES : [&'static str; 2] = [
    "true",
    "false"
];

pub trait Linkable {
    fn get_name<'a>(&self, linker : &'a Linker) -> &'a str;
    fn get_full_name(&self, linker : &Linker) -> String {
        let mut full_name = match self.get_file() {
            FileUUID::INVALID => "<builtin>".to_owned(),
            f => linker.files[f].file_path.to_string_lossy().into_owned()
        };
        full_name += "::";
        full_name += self.get_name(linker);
        full_name
    }
    fn get_location(&self) -> Option<&Location>;
    fn get_dependencies(&self) -> &Dependencies;
    fn get_dependencies_mut(&mut self) -> &mut Dependencies;
    fn get_file(&self) -> FileUUID {
        if let Some(loc) = self.get_location() {
            loc.file
        } else {
            FileUUID::INVALID
        }
    }
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
    fn get_name<'a>(&self, linker : &'a Linker) -> &'a str {
        match self {
            NamedValue::Builtin(name) => name,
            NamedValue::Module(md) => {
                let file = &linker.files[md.location.file];
                file.get_token_text(md.location.name_token)
            },
        }
    }
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
    fn get_name<'a>(&self, _linker : &'a Linker) -> &'a str {
        match self {
            NamedType::Builtin(name) => name,
        }
    }
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
    fn get_name<'a>(&self, linker : &'a Linker) -> &'a str {
        match self {
            Named::Value(v) => v.get_name(linker),
            Named::Type(t) => t.get_name(linker),
        }
    }
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
        for name in BUILTIN_VALUES {
            let id = globals.alloc(Named::Value(NamedValue::Builtin(name)));
            let already_exisits = global_namespace.insert(name.to_owned(), id);
            assert!(already_exisits.is_none());
        }

        Links{globals, name_colissions : Vec::new(), global_namespace}
    }

    fn resolve_dependencies(&self, file : &FileData, deps : &[GlobalReference]) -> Vec<ValueUUID> {
        deps.iter().map(|reference| {
            let reference_name_str = file.get_token_text(reference[0]);

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
            let module_name = &file_text[parse_result.tokens[md.location.name_token].get_range()];
            let new_module_uuid = self.links.globals.alloc(Named::Value(NamedValue::Module(md)));
            associated_values.push(new_module_uuid);
            match self.links.global_namespace.entry(module_name.to_owned()) {
                std::collections::hash_map::Entry::Occupied(occ) => {
                    self.links.name_colissions.push((new_module_uuid, *occ.get()));
                },
                std::collections::hash_map::Entry::Vacant(vac) => {
                    vac.insert(new_module_uuid);
                },
            }
        }
        self.files.alloc_reservation(file, FileData { file_text, file_path, tokens: parse_result.tokens, token_hierarchy: parse_result.token_hierarchy, parsing_errors, associated_values});
    }

    // This should be called once all modules have been added. Adds errors for globals it couldn't match
    pub fn link(mut self) -> Linker {
        for (_file_uuid, file) in &mut self.files {
            for idx in &file.associated_values {
                let deps = self.links.globals[*idx].get_dependencies();
                let vals_this_refers_to = self.links.resolve_dependencies(&file, &deps.global_references);
                let deps_mut = self.links.globals[*idx].get_dependencies_mut();
                deps_mut.resolved_globals = vals_this_refers_to;
            }
        }
        Linker{links: self.links, files : self.files}
    }
}

impl Linker {
    pub fn get_linking_errors(&self, file_uuid : FileUUID, errors : &mut ErrorCollector) {
        let file = &self.files[file_uuid];

        // Conflicting Declarations
        for colission in &self.links.name_colissions {
            let file_0 = self.links.globals[colission.0].get_file();
            let file_1 = self.links.globals[colission.1].get_file();
            
            let (main_object, other_object) = if file_0 == file_uuid {
                (colission.0, colission.1)
            } else if file_1 == file_uuid {
                (colission.1, colission.0)
            } else {
                continue;
            };
            let main_location = self.links.globals[main_object].get_location().unwrap(); // This is always valid, because we're getting the location of an object in the current file
            let this_object_name = file.get_token_text(main_location.name_token);
            if let Some(other_location) = self.links.globals[other_object].get_location() {
                errors.error_with_info(Span::from(main_location.name_token), format!("Conflicting Declaration for the name '{this_object_name}'"), vec![
                    error_info(Span::from(other_location.name_token), other_location.file, "Conflicting Declaration")
                ]);
            } else {
                errors.error_basic(Span::from(main_location.name_token), format!("Cannot redeclare the builtin '{this_object_name}'"));
            }
        }
        
        // References not found

        for val_uuid in &self.files[file_uuid].associated_values {
            let object = &self.links.globals[*val_uuid];
            let object_dependencies = object.get_dependencies();
            for (pos, ref_uuid) in object_dependencies.resolved_globals.iter().enumerate() {
                if *ref_uuid == ValueUUID::INVALID {
                    let unresolved_reference = &object_dependencies.global_references[pos];
                    let reference_span = Span(unresolved_reference[0], *unresolved_reference.last().unwrap());
                    let reference_text = file.get_span_text(reference_span);
                    errors.error_basic(reference_span, format!("No Value or Type of the name '{reference_text}' was found. Did you forget to import it?"));
                }
            }
        }
    }
    /*pub fn remove(&mut self, file : FileUUID) {

    }
    pub fn relink(&mut self, file : FileUUID, file_text : String, ast : ASTRoot, mut errors : ErrorCollector) {
        match self.files.entry(file_name) {
            Entry::Occupied(mut exists) => {
                let existing_entry = exists.get_mut();

                for ValueUUID(v) in &mut existing_entry.associated_values {

                }
            },
            Entry::Vacant(new_entry) => {

            },
        }
    }*/
}
