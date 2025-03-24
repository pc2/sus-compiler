use std::ffi::OsStr;
use std::path::PathBuf;
use std::str::FromStr;

use crate::config::EarlyExitUpTo;
use crate::linker::AFTER_INITIAL_PARSE_CP;
use crate::prelude::*;

use sus_proc_macro::{get_builtin_const, get_builtin_type};
use tree_sitter::Parser;

use crate::{
    config::config, debug::SpanDebugger, errors::ErrorStore, file_position::FileText,
    linker::FileData,
};

use crate::flattening::{
    flatten_all_globals, gather_initial_file_data, perform_lints, typecheck_all_modules, Module,
};

const STD_LIB_PATH: &str = env!("SUS_COMPILER_STD_LIB_PATH");

/// Any extra operations that should happen when files are added or removed from the linker. Such as caching line offsets.
pub trait LinkerExtraFileInfoManager {
    /// This is there to give an acceptable identifier that can be printed
    // TODO Namespaces: Check if this is still needed
    /*    fn convert_filename(&self, path: &Path) -> String {
        path.to_string_lossy().into_owned()
    } */
    fn convert_filename(&self, path: &PathBuf) {}
    fn on_file_added(&mut self, _file_id: FileUUID, _linker: &Linker) {}
    fn on_file_updated(&mut self, _file_id: FileUUID, _linker: &Linker) {}
    fn before_file_remove(&mut self, _file_id: FileUUID, _linker: &Linker) {}
}

impl LinkerExtraFileInfoManager for () {}

impl Linker {
    pub fn add_standard_library<ExtraInfoManager: LinkerExtraFileInfoManager>(
        &mut self,
        info_mngr: &mut ExtraInfoManager,
    ) {
        assert!(self.modules.is_empty());
        assert!(self.types.is_empty());
        assert!(self.constants.is_empty());
        if !config().ci {
            println!("Standard Library Directory: {STD_LIB_PATH}");
        }
        let std_path = PathBuf::from_str(STD_LIB_PATH)
            .expect("Standard library directory is not a valid path?");
        // TODO: Only add files from standard directory if they are required by user file
        // (e.g. always add Core and only add util when imported)
        self.add_all_files_in_directory(&std_path, &std_path, info_mngr);

        // Sanity check for the names the compiler knows internally.
        // They are defined in std/core.sus
        // Critically, std/core.sus MUST be the first file to be loaded into the linker. Otherwise the IDs don't point to the correct objects
        assert_eq!(self.types[get_builtin_type!("int")].link_info.name, "int");
        assert_eq!(self.types[get_builtin_type!("bool")].link_info.name, "bool");
        assert_eq!(
            self.types[get_builtin_type!("float")].link_info.name,
            "float"
        );

        assert_eq!(
            self.constants[get_builtin_const!("true")].link_info.name,
            "true"
        );
        assert_eq!(
            self.constants[get_builtin_const!("false")].link_info.name,
            "false"
        );
        assert_eq!(
            self.constants[get_builtin_const!("__crash_compiler")]
                .link_info
                .name,
            "__crash_compiler"
        );
        assert_eq!(
            self.constants[get_builtin_const!("assert")].link_info.name,
            "assert"
        );
        assert_eq!(
            self.constants[get_builtin_const!("sizeof")].link_info.name,
            "sizeof"
        );
        assert_eq!(
            self.constants[get_builtin_const!("clog2")].link_info.name,
            "clog2"
        );
    }

    //pub fn add_all_files_in_directory<ExtraInfoManager : LinkerExtraFileInfoManager>(&mut self, base : &PathBuf, subdir_path : Vec<OsStr>, info_mngr : &mut ExtraInfoManager) {
    // Adds all files of directory to new subnamespace below the global name space
    pub fn add_all_files_in_directory<ExtraInfoManager: LinkerExtraFileInfoManager>(
        &mut self,
        base: &PathBuf,
        subdir: &PathBuf,
        info_mngr: &mut ExtraInfoManager,
    ) {
        let mut files: Vec<PathBuf> = std::fs::read_dir(subdir)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap();
        files.sort();
        for file in files {
            let file_path = file.canonicalize().unwrap();
            if file_path.is_file() && file_path.extension() == Some(OsStr::new("sus")) {
                // tokenize path into sub paths that will become namespaces
                let relative_path = file_path.strip_prefix(base);
                let mut relative_path = relative_path.unwrap().to_path_buf();
                relative_path.set_extension("");
                // stack that will hold all elements of the full namespace path
                // top of stack is the base dir
                let mut path_stack = self.make_path_vec(base, &file_path);
                path_stack.reverse(); // TODO: Make this happen globally once
                let file_text = std::fs::read_to_string(&file_path).unwrap();
                // TODO Namespaces: check if this is still needed
                //let file_identifier: String = info_mngr.convert_filename(&file_path);
                self.add_file(path_stack, file_text, info_mngr);
            }
            if file_path.is_dir() {
                self.add_all_files_in_directory(&base, &file_path, info_mngr); // recuse through all subdirs
            }
        }
    }

    // creates path vector for file_identifier and path in global namespace
    pub fn make_path_vec(&mut self, base: &PathBuf, file: &PathBuf) -> Vec<String> {
        // tokenize path into sub paths that will become namespaces
        let canon_file = file
            .canonicalize()
            .expect("No canon name found for '{:?}', are you sure that the file exists?");
        let relative_path = canon_file.strip_prefix(base);
        let mut relative_path = relative_path.unwrap().to_path_buf();
        relative_path.set_extension("");
        // stack that will hold all elements of the full namespace path
        // top of stack is the base dir
        let mut path_stack: Vec<String> = Vec::new();
        for subdir in relative_path.iter().rev() {
            path_stack.push(subdir.to_string_lossy().into_owned());
        }
        let mut base_string = base.to_string_lossy().into_owned();
        if let Some(temp) = self.root_spaces.get(&base_string) {
            base_string = temp.to_owned();
        }
        path_stack.push(base_string);
        return path_stack;
    }

    // prepare vec with imported namespaces to save as file context later
    fn gen_namespace_context<ExtraInfoManager: LinkerExtraFileInfoManager>(
        &mut self,
        file_identifier: &Vec<String>,
        _info_mngr: &mut ExtraInfoManager,
    ) -> Vec<Vec<String>> {
        let mut associated_namespaces = Vec::new();
        let core = vec!["std".to_owned(), "core".to_owned()];
        let util = vec!["std".to_owned(), "util".to_owned()];
        associated_namespaces.push(core);
        associated_namespaces.push(util);
        if !associated_namespaces.contains(file_identifier) {
            associated_namespaces.push(file_identifier.clone());
        };
        associated_namespaces
    }

    pub fn add_file<ExtraInfoManager: LinkerExtraFileInfoManager>(
        &mut self,
        file_identifier: Vec<String>,
        text: String,
        info_mngr: &mut ExtraInfoManager,
    ) -> FileUUID {
        // File doesn't yet exist
        assert!(!self
            .files
            .iter()
            .any(|fd| fd.1.file_identifier == file_identifier));

        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_sus::language()).unwrap();
        let tree = parser.parse(&text, None).unwrap();

        let associated_namespaces = self.gen_namespace_context(&file_identifier, info_mngr);

        let file_id = self.files.reserve();
        self.files.alloc_reservation(
            file_id,
            FileData {
                file_identifier,
                file_text: FileText::new(text),
                tree,
                associated_values: Vec::new(),
                associated_namespaces,
                parsing_errors: ErrorStore::new(),
            },
        );

        self.with_file_builder(file_id, |builder| {
            let _panic_guard =
                SpanDebugger::new("gather_initial_file_data in add_file", builder.file_data);
            gather_initial_file_data(builder);
        });

        info_mngr.on_file_added(file_id, self);

        file_id
    }

    // When --feature lsp is not used, this gives a warning
    // TODO, fix for namespaces, file search may be broken
    #[allow(dead_code)]
    pub fn add_or_update_file<ExtraInfoManager: LinkerExtraFileInfoManager>(
        &mut self,
        file_identifier: &str,
        text: String,
        info_mngr: &mut ExtraInfoManager,
    ) {
        if let Some(file_id) = self.find_file(file_identifier) {
            let file_data = self.remove_everything_in_file(file_id);

            let mut parser = Parser::new();
            parser.set_language(&tree_sitter_sus::language()).unwrap();
            let tree = parser.parse(&text, None).unwrap();

            file_data.parsing_errors = ErrorStore::new();
            file_data.file_text = FileText::new(text);
            file_data.tree = tree;

            self.with_file_builder(file_id, |builder| {
                let _panic_guard =
                    SpanDebugger::new("gather_initial_file_data in update_file", builder.file_data);
                gather_initial_file_data(builder);
            });

            info_mngr.on_file_updated(file_id, self);
        } else {
            //self.add_file(file_identifier.to_owned(), text, info_mngr);
        }
    }

    // TODO Namespaces: Fix for namespaces
    pub fn find_file(&self, file_identifier: &str) -> Option<FileUUID> {
        self.files
            .find(|_id, f| f.file_identifier.join("/") == file_identifier)
    }

    pub fn recompile_all(&mut self) {
        // First reset all modules back to post-gather_initial_file_data
        for (_, md) in &mut self.modules {
            let Module {
                link_info,
                instantiations,
                ..
            } = md;
            link_info.reset_to(AFTER_INITIAL_PARSE_CP);
            link_info.instructions.clear();
            instantiations.clear_instances()
        }
        for (_, typ) in &mut self.types {
            typ.link_info.reset_to(AFTER_INITIAL_PARSE_CP);
        }
        for (_, cst) in &mut self.constants {
            cst.link_info.reset_to(AFTER_INITIAL_PARSE_CP);
        }
        if config().early_exit == EarlyExitUpTo::Initialize {
            return;
        }

        flatten_all_globals(self);
        config().for_each_debug_module(config().debug_print_module_contents, &self.modules, |md| {
            md.print_flattened_module(&self.files[md.link_info.file]);
        });
        if config().early_exit == EarlyExitUpTo::Flatten {
            return;
        }

        typecheck_all_modules(self);

        config().for_each_debug_module(config().debug_print_module_contents, &self.modules, |md| {
            md.print_flattened_module(&self.files[md.link_info.file]);
        });
        if config().early_exit == EarlyExitUpTo::AbstractTypecheck {
            return;
        }

        perform_lints(self);

        if config().early_exit == EarlyExitUpTo::Lint {
            return;
        }

        // Make an initial instantiation of all modules
        // Won't be possible once we have template modules
        for (_id, md) in &self.modules {
            //md.print_flattened_module();
            // Already instantiate any modules without parameters
            // Currently this is all modules
            let span_debug_message = format!("instantiating {}", &md.link_info.name);
            let _panic_guard =
                SpanDebugger::new(&span_debug_message, &self.files[md.link_info.file]);
            // Can immediately instantiate modules that have no template args
            if md.link_info.template_parameters.is_empty() {
                let _inst = md.instantiations.instantiate(md, self, FlatAlloc::new());
            }
        }
        if config().early_exit == EarlyExitUpTo::Instantiate {}
    }
}
