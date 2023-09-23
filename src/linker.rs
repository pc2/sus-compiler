use std::{collections::{HashMap, HashSet}, ops::{IndexMut, Index}, path::PathBuf};

use crate::{ast::{Module, LinkInfo, GlobalReference, Span}, arena_alloc::{ArenaAllocator, UUID}, parser::{FullParseResult, TokenTreeNode}, tokenizer::Token, errors::{ErrorCollector, error_info}};

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

const BUILTIN_VALUES : [&'static str; 2] = [
    "true",
    "false"
];

pub trait Linkable {
    fn get_name<'a>(&self, linker : &'a Linker) -> &'a str;
    fn get_link_info(&self) -> Option<&LinkInfo>;
    fn get_link_info_mut(&mut self) -> Option<&mut LinkInfo>;
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
                let file = &linker.files[md.link_info.file];
                file.get_token_text(md.link_info.name_token)
            },
        }
    }
    fn get_link_info(&self) -> Option<&LinkInfo> {
        match self {
            NamedValue::Builtin(_) => None,
            NamedValue::Module(md) => {
                Some(&md.link_info)
            }
        }
    }
    fn get_link_info_mut(&mut self) -> Option<&mut LinkInfo> {
        match self {
            NamedValue::Builtin(_) => None,
            NamedValue::Module(md) => {
                Some(&mut md.link_info)
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
            Named::Value(v) => v.get_name(linker),
            Named::Type(t) => t.get_name(linker),
        }
    }
    fn get_link_info(&self) -> Option<&LinkInfo> {
        match self {
            Named::Value(v) => v.get_link_info(),
            Named::Type(t) => t.get_link_info()
        }
    }
    fn get_link_info_mut(&mut self) -> Option<&mut LinkInfo> {
        match self {
            Named::Value(v) => v.get_link_info_mut(),
            Named::Type(t) => t.get_link_info_mut()
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
            let module_name = &file_text[parse_result.tokens[md.link_info.name_token].get_range()];
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
        for (_file_uuid, file) in &self.files {
            for val_in_file in &file.associated_values {
                let link_info = self.links.globals[*val_in_file].get_link_info().unwrap();
                let vals_this_refers_to = self.links.resolve_dependencies(&file, &link_info.global_references);
                let link_info_mut = self.links.globals[*val_in_file].get_link_info_mut().unwrap();
                link_info_mut.resolved_globals = vals_this_refers_to;
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
            let info_0 = self.links.globals[colission.0].get_link_info().unwrap(); // Is always valid because colission.0 is 'the thing that conflicts with'
            let info_1_opt = self.links.globals[colission.1].get_link_info();

            let (info_a, info_b) = if info_0.file == file_uuid {
                if let Some(info_1) = info_1_opt {
                    (info_0, info_1)
                } else {
                    let this_object_name = file.get_token_text(info_0.name_token);
                    errors.error_basic(Span::from(info_0.name_token), format!("Cannot redeclare the builtin '{this_object_name}'"));
                    continue;
                }
            } else if let Some(info_1) = info_1_opt {
                if info_1.file == file_uuid {
                    (info_1, info_0)
                } else {
                    continue;
                }
            } else {
                continue;
            };
            let this_object_name = file.get_token_text(info_a.name_token);
            errors.error_with_info(Span::from(info_a.name_token), format!("Conflicting Declaration for the name '{this_object_name}'"), vec![
                error_info(Span::from(info_b.name_token), info_b.file, "Conflicting Declaration")
            ]);
        }
        
        // References not found
        for val_uuid in &self.files[file_uuid].associated_values {
            let object = &self.links.globals[*val_uuid];
            let object_link_info = object.get_link_info().unwrap(); // Always valid because it's part of file
            for (pos, ref_uuid) in object_link_info.resolved_globals.iter().enumerate() {
                if *ref_uuid == ValueUUID::INVALID {
                    let unresolved_reference = &object_link_info.global_references[pos];
                    let reference_span = Span(unresolved_reference[0], *unresolved_reference.last().unwrap());
                    let reference_text = file.get_span_text(reference_span);
                    errors.error_basic(reference_span, format!("No Value or Type of the name '{reference_text}' was found. Did you forget to import it?"));
                }
            }
        }
    }

    pub fn remove_files(&mut self, files : &[FileUUID]) {
        // For quick lookup if a reference disappears
        let mut back_reference_set = HashSet::new();

        // Remove the files and their referenced values
        for file in files {
            for v in self.files.free(*file).associated_values {
                back_reference_set.insert(v);
                self.links.globals.free(v);
            }
        }

        // Remove resolved globals
        for (_uuid, v) in &mut self.links.globals {
            if let Some(info) = v.get_link_info_mut() { // Builtins can't refer to other things
                for v in &mut info.resolved_globals {
                    if back_reference_set.contains(v) {
                        *v = ValueUUID::INVALID;
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
        }

        // Remove names from the global namespace
        self.links.global_namespace.retain(|_k, v| -> bool {
            if let Some(found_replacement) = conflict_replacements.get(v) {
                *v = *found_replacement;
                return true;
            }
            !back_reference_set.contains(v)
        });
    }

    /*pub fn relink(&mut self, file : FileUUID, file_text : String, ast : ASTRoot, mut errors : ErrorCollector) {
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
