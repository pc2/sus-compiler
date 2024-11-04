use std::ffi::OsStr;
use std::path::PathBuf;
use std::str::FromStr;

use crate::config::EarlyExitUpTo;
use crate::linker::{get_builtin_constant, get_builtin_type, AFTER_INITIAL_PARSE_CP};
use crate::prelude::*;
use crate::value::{TypedValue, Value};

use tree_sitter::Parser;

use crate::{
    config::config, debug::SpanDebugger, errors::ErrorStore, file_position::FileText,
    linker::FileData,
};

use crate::flattening::{
    flatten_all_modules, gather_initial_file_data, perform_lints, typecheck_all_modules, Module
};

const STD_LIB_PATH: &str = env!("SUS_COMPILER_STD_LIB_PATH");

pub trait LinkerExtraFileInfoManager {
    /// This is there to give an acceptable identifier that can be printed
    fn convert_filename(&self, path : &PathBuf) -> String {
        path.to_string_lossy().into_owned()
    }
    fn on_file_added(&mut self, _file_id : FileUUID, _linker : &Linker) {}
    fn on_file_updated(&mut self, _file_id : FileUUID, _linker : &Linker) {}
    fn before_file_remove(&mut self, _file_id : FileUUID, _linker : &Linker) {}
}

impl LinkerExtraFileInfoManager for () {}

impl Linker {
    pub fn add_standard_library<ExtraInfoManager : LinkerExtraFileInfoManager>(&mut self, info_mngr : &mut ExtraInfoManager) {
        assert!(self.modules.is_empty());
        assert!(self.types.is_empty());
        assert!(self.constants.is_empty());
        println!("Standard Library Directory: {STD_LIB_PATH}");
        let stl_path = PathBuf::from_str(STD_LIB_PATH).expect("Standard library directory is not a valid path?");
        self.add_all_files_in_directory(&stl_path, info_mngr);

        assert_eq!(self.types[get_builtin_type("int")].link_info.name, "int");
        assert_eq!(self.types[get_builtin_type("bool")].link_info.name, "bool");

        assert_eq!(self.constants[get_builtin_constant("true")].link_info.name, "true");
        assert_eq!(self.constants[get_builtin_constant("false")].link_info.name, "false");
        self.constants[get_builtin_constant("true")].val = TypedValue::from_value(Value::Bool(true));
        self.constants[get_builtin_constant("false")].val = TypedValue::from_value(Value::Bool(false));
    }

    pub fn add_all_files_in_directory<ExtraInfoManager : LinkerExtraFileInfoManager>(&mut self, directory : &PathBuf, info_mngr : &mut ExtraInfoManager) {
        for file in std::fs::read_dir(directory).unwrap() {
            let file_path = file.unwrap().path().canonicalize().unwrap();
            if file_path.is_file() && file_path.extension() == Some(OsStr::new("sus")) {
                let file_text = std::fs::read_to_string(&file_path).unwrap();
                let file_identifier : String = info_mngr.convert_filename(&file_path);
                self.add_file(file_identifier, file_text, info_mngr);
            }
        }
    }

    pub fn add_file<ExtraInfoManager : LinkerExtraFileInfoManager>(&mut self, file_identifier: String, text: String, info_mngr : &mut ExtraInfoManager) -> FileUUID {
        // File doesn't yet exist
        assert!(!self.files.iter().any(|fd| fd.1.file_identifier == file_identifier));

        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_sus::language()).unwrap();
        let tree = parser.parse(&text, None).unwrap();
    
        let file_id = self.files.reserve();
        self.files.alloc_reservation(
            file_id,
            FileData {
                file_identifier,
                file_text: FileText::new(text),
                tree,
                associated_values: Vec::new(),
                parsing_errors: ErrorStore::new(),
            },
        );
    
        self.with_file_builder(file_id, |builder| {
            let mut span_debugger =
                SpanDebugger::new("gather_initial_file_data in add_file", builder.file_data);
            gather_initial_file_data(builder);
            span_debugger.defuse();
        });

        info_mngr.on_file_added(file_id, self);

        file_id
    }

    // When --feature lsp is not used, this gives a warning
    #[allow(dead_code)]
    pub fn add_or_update_file<ExtraInfoManager : LinkerExtraFileInfoManager>(&mut self, file_identifier: &str, text: String, info_mngr : &mut ExtraInfoManager) {
        if let Some(file_id) = self.find_file(file_identifier) {
            let file_data = self.remove_everything_in_file(file_id);
            
            let mut parser = Parser::new();
            parser.set_language(&tree_sitter_sus::language()).unwrap();
            let tree = parser.parse(&text, None).unwrap();
            
            file_data.parsing_errors = ErrorStore::new();
            file_data.file_text = FileText::new(text);
            file_data.tree = tree;
            
            self.with_file_builder(file_id, |builder| {
                let mut span_debugger =
                SpanDebugger::new("gather_initial_file_data in update_file", builder.file_data);
                gather_initial_file_data(builder);
                span_debugger.defuse();
            });

            info_mngr.on_file_updated(file_id, self);
        } else {
            self.add_file(file_identifier.to_owned(), text, info_mngr);
        }
    }

    pub fn find_file(&self, file_identifier: &str) -> Option<FileUUID> {
        self.files.find(|_id, f| f.file_identifier == file_identifier)
    }

    pub fn recompile_all(&mut self) {
        // First reset all modules back to post-gather_initial_file_data
        for (_, md) in &mut self.modules {
            let Module {
                link_info,
                instructions,
                instantiations,
                ..
            } = md;
            link_info.reset_to(AFTER_INITIAL_PARSE_CP);
            instructions.clear();
            instantiations.clear_instances()
        }
        for (_, typ) in &mut self.types {
            typ.link_info.reset_to(AFTER_INITIAL_PARSE_CP);
        }
        for (_, cst) in &mut self.constants {
            cst.link_info.reset_to(AFTER_INITIAL_PARSE_CP);
        }
        if config().early_exit == EarlyExitUpTo::Initialize {return}

        flatten_all_modules(self);
        config().for_each_debug_module(config().debug_print_module_contents, &self.modules, |md| {
            md.print_flattened_module(&self.files[md.link_info.file]);
        });
        if config().early_exit == EarlyExitUpTo::Flatten {return}

        typecheck_all_modules(self);

        config().for_each_debug_module(config().debug_print_module_contents, &self.modules, |md| {
            md.print_flattened_module(&self.files[md.link_info.file]);
        });
        if config().early_exit == EarlyExitUpTo::AbstractTypecheck {return}

        perform_lints(self);
        
        if config().early_exit == EarlyExitUpTo::Lint {return}

        // Make an initial instantiation of all modules
        // Won't be possible once we have template modules
        for (_id, md) in &self.modules {
            //md.print_flattened_module();
            // Already instantiate any modules without parameters
            // Currently this is all modules
            let span_debug_message = format!("instantiating {}", &md.link_info.name);
            let mut span_debugger = SpanDebugger::new(
                &span_debug_message,
                &self.files[md.link_info.file],
            );
            // Can immediately instantiate modules that have no template args
            if md.link_info.template_arguments.is_empty() {
                let _inst = md.instantiations.instantiate(md, self, FlatAlloc::new());
            }
            span_debugger.defuse();
        }
        if config().early_exit == EarlyExitUpTo::Instantiate {return}
    }
}
