use std::collections::{HashMap, hash_map::Entry};

use crate::{ast::{Module, ASTRoot, Span, Location, Dependencies, FileName}, errors::{ErrorCollector, error_info}};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueUUID(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeUUID(usize);

const INVALID_UUID : usize = usize::MAX;

const BUILTIN_TYPES : [&'static str; 2] = [
    "bool",
    "int"
];

const VALUE_NAMES : &'static str = "Module Or Constant";
const TYPE_NAMES : &'static str = "Type";

pub trait Linkable {
    fn get_full_name(&self) -> &str;
    fn get_location(&self) -> Option<&Location>;
    fn get_dependencies(&self) -> &Dependencies;
    fn get_dependencies_mut(&mut self) -> &mut Dependencies;
}

pub enum NamedValue {
    Builtin(&'static str),
    Module(Module)
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
    fn get_full_name(&self) -> &str {
        match self {
            NamedValue::Builtin(name) => *name,
            NamedValue::Module(md) => {
                &md.full_name
            },
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

pub enum NamedType {
    Builtin(&'static str)
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
    fn get_full_name(&self) -> &str {
        match self {
            NamedType::Builtin(name) => *name,
        }
    }
    fn get_location(&self) -> Option<&Location> {
        match self {
            NamedType::Builtin(_name) => None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GlobalNamespaceNode {
    Value(ValueUUID),
    Type(TypeUUID),
}

pub struct FileData<T> {
    pub file_text : String,
    pub extra_data : T,
    pub errors : ErrorCollector,
    associated_values : Vec<ValueUUID>,
    associated_types : Vec<TypeUUID>
}

// All modules in the workspace
pub struct Links {
    values : Vec<NamedValue>,
    types : Vec<NamedType>,
    global_namespace : HashMap<String, GlobalNamespaceNode>
}

// All modules in the workspace
pub struct FileLinker<T> {
    links : Links,
    pub files : HashMap<FileName, FileData<T>>
}

impl Links {
    // Returns None for builtins
    fn get_location(&self, node : GlobalNamespaceNode) -> Option<&Location> {
        match node {
            GlobalNamespaceNode::Value(ValueUUID(pos)) => {
                self.values[pos].get_location()
            },
            GlobalNamespaceNode::Type(TypeUUID(pos)) => {
                self.types[pos].get_location()
            },
        }
    }

    pub fn new() -> Links {
        // Add builtins
        let named_values = Vec::new();
        let mut named_types = Vec::new();
        let mut global_namespace = HashMap::new();
        
        for name in BUILTIN_TYPES {
            let success = global_namespace.insert(name.to_owned(), GlobalNamespaceNode::Type(TypeUUID(named_types.len()))).is_none();
            assert!(success);
            named_types.push(NamedType::Builtin(name));
        } 

        Links{values: named_values, types: named_types, global_namespace}
    }
    pub fn add_file<T>(&mut self, file_text : String, ast : ASTRoot, mut errors : ErrorCollector, extra_data : T) -> FileData<T> {
        let mut associated_values = Vec::new();
        for md in ast.modules {
            let module_name = &file_text[md.name.text.clone()];
            match self.global_namespace.entry(module_name.to_owned()) {
                std::collections::hash_map::Entry::Occupied(occ) => {
                    let node = *occ.get();
                    if let Some(location) = self.get_location(node) {
                        errors.error_with_info(Span::from(md.name.position), format!("Conflicting Module Declaration for the name '{module_name}'"), vec![
                            error_info(location.span, location.file_name.clone(), "Conflicting Declaration")
                        ]);
                    } else {
                        errors.error_basic(Span::from(md.name.position), format!("Cannot redeclare the builtin '{module_name}'"));
                    }
                }
                std::collections::hash_map::Entry::Vacant(vac) => {
                    vac.insert(GlobalNamespaceNode::Value(ValueUUID(self.values.len())));
                    associated_values.push(ValueUUID(self.values.len()));
                    self.values.push(NamedValue::Module(md));
                }
            }
        }
        FileData{file_text, errors, associated_values, associated_types: Vec::new(), extra_data}
    }

    fn link_dependencies(&self, file_text : &str, deps : &Dependencies, errors : &mut ErrorCollector) -> (Vec<ValueUUID>, Vec<TypeUUID>) {
        let value_references : Vec<ValueUUID> = deps.global_references.iter().map(|reference| {
            let reference_span = Span(reference.last().unwrap().position, reference[0].position);
            let reference_name_str = &file_text[reference[0].text.clone()];
            match self.global_namespace.get(reference_name_str) {
                Some(GlobalNamespaceNode::Value(v)) => {
                    *v
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
        }).collect();

        let type_references : Vec<TypeUUID> = deps.type_references.iter().map(|reference| {
            let reference_span = Span(reference.last().unwrap().position, reference[0].position);
            let reference_name_str = &file_text[reference[0].text.clone()];
            match self.global_namespace.get(reference_name_str) {
                Some(GlobalNamespaceNode::Type(v)) => {
                    *v
                }
                Some(GlobalNamespaceNode::Value(ValueUUID(idx))) => {
                    let found_instead = &self.values[*idx];
                    let found_full_name = found_instead.get_full_name();
                    let infos = if let Some(loc) = found_instead.get_location() {
                        vec![error_info(loc.span, loc.file_name.clone(), "Defined here")]
                    } else {
                        vec![]
                    };
                    errors.error_with_info(reference_span, format!("No {TYPE_NAMES} of the name '{reference_name_str}' was found. Found Type '{found_full_name}'"), infos);
                    TypeUUID(INVALID_UUID)
                }
                None => {
                    errors.error_basic(reference_span, format!("No {TYPE_NAMES} of the name '{reference_name_str}' was found. Did you forget to import it?"));
                    TypeUUID(INVALID_UUID)
                }
            }
        }).collect();

        (value_references, type_references)
    }

    // This should be called once all modules have been added. Adds errors for globals it couldn't match
    pub fn link_all<T>(mut self, mut files : HashMap<FileName, FileData<T>>) -> FileLinker<T> {
        for (_file_name, file) in &mut files {
            for ValueUUID(idx) in &file.associated_values {
                let deps = self.values[*idx].get_dependencies();
                let (vals_this_refers_to, types_this_refers_to) = self.link_dependencies(&file.file_text, deps, &mut file.errors);
                let deps_mut = self.values[*idx].get_dependencies_mut();
                deps_mut.resolved_globals = vals_this_refers_to;
                deps_mut.resolved_types = types_this_refers_to;
            }
            for TypeUUID(idx) in &file.associated_types {
                let deps = self.types[*idx].get_dependencies();
                let (vals_this_refers_to, types_this_refers_to) = self.link_dependencies(&file.file_text, deps, &mut file.errors);
                let deps_mut = self.types[*idx].get_dependencies_mut();
                deps_mut.resolved_globals = vals_this_refers_to;
                deps_mut.resolved_types = types_this_refers_to;
            }
        }
        FileLinker{links: self, files}
    }
}

impl<ExtraData> FileLinker<ExtraData> {
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
}
