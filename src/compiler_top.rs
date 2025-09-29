use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::config::EarlyExitUpTo;
use crate::flattening::typecheck::{perform_lints, typecheck};
use crate::linker::GlobalObj;
use crate::linker::checkpoint::{
    AFTER_FLATTEN_CP, AFTER_INITIAL_PARSE_CP, AFTER_LINTS_CP, AFTER_TYPE_CHECK_CP,
};
use crate::prelude::*;
use crate::typing::concrete_type::ConcreteGlobalReference;

use sus_proc_macro::{get_builtin_const, get_builtin_type};
use tree_sitter::Parser;

use crate::{config::config, errors::ErrorStore, file_position::FileText, linker::FileData};

use crate::config::get_sus_home;
use crate::flattening::{flatten_all_globals, gather_initial_file_data};

pub fn get_std_dir() -> PathBuf {
    get_sus_home().join("std")
}
pub fn get_crash_dumps_dir() -> PathBuf {
    get_sus_home().join("crash_dumps")
}

/// Any extra operations that should happen when files are added or removed from the linker. Such as caching line offsets.
pub trait LinkerExtraFileInfoManager {
    /// This is there to give an acceptable identifier that can be printed
    fn convert_filename(&self, path: &Path) -> String;
    fn on_file_added(&mut self, _file_id: FileUUID, _linker: &Linker) {}
    fn on_file_updated(&mut self, _file_id: FileUUID, _linker: &Linker) {}
    fn before_file_remove(&mut self, _file_id: FileUUID, _linker: &Linker) {}
}

impl Linker {
    pub fn add_standard_library<ExtraInfoManager: LinkerExtraFileInfoManager>(
        &mut self,
        info_mngr: &mut ExtraInfoManager,
    ) {
        assert!(self.modules.is_empty());
        assert!(self.types.is_empty());
        assert!(self.constants.is_empty());
        let std_lib_path = get_std_dir();
        if !config().ci {
            info!(
                "Standard Library Directory: {}",
                std_lib_path.to_string_lossy()
            );
        }
        self.add_all_files_in_directory(&std_lib_path, info_mngr);
        for (_, f) in &mut self.files {
            f.is_std = true; // Mark standard library files
        }

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

    pub fn add_file<ExtraInfoManager: LinkerExtraFileInfoManager>(
        &mut self,
        file_path: &Path,
        info_mngr: &mut ExtraInfoManager,
    ) {
        let file_text = std::fs::read_to_string(file_path).unwrap();
        let file_identifier = info_mngr.convert_filename(file_path);
        self.add_file_text(file_identifier, file_text, info_mngr);
    }

    pub fn add_all_files_in_directory<ExtraInfoManager: LinkerExtraFileInfoManager>(
        &mut self,
        directory: &PathBuf,
        info_mngr: &mut ExtraInfoManager,
    ) {
        let dir_read = std::fs::read_dir(directory);
        let dir_read = match dir_read {
            Ok(d) => d,
            Err(_) => panic!("Can't read directory {}", directory.to_string_lossy()),
        };
        let mut files: Vec<_> = dir_read
            .map(|res| match res {
                Ok(path) => path.path(),
                Err(err) => panic!(
                    "No such file or directory {} in {}",
                    err,
                    directory.to_string_lossy()
                ),
            })
            .collect();
        files.sort();
        for file in files {
            let file_path = file.canonicalize().unwrap();
            if file_path.is_file() && file_path.extension() == Some(OsStr::new("sus")) {
                self.add_file(&file_path, info_mngr);
            }
        }
    }

    pub fn add_file_text<ExtraInfoManager: LinkerExtraFileInfoManager>(
        &mut self,
        file_identifier: String,
        text: String,
        info_mngr: &mut ExtraInfoManager,
    ) -> FileUUID {
        // File doesn't yet exist
        assert!(
            !self
                .files
                .iter()
                .any(|fd| fd.1.file_identifier == file_identifier)
        );

        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_sus::language()).unwrap();
        let tree = parser.parse(&text, None).unwrap();

        let file_id = self.files.alloc(FileData {
            file_identifier,
            file_text: FileText::new(text),
            tree,
            associated_values: Vec::new(),
            parsing_errors: ErrorStore::new(),
            is_std: false,
        });

        self.with_file_builder(file_id, |builder| {
            crate::debug::debug_context(
                "gather_initial_file_data in add_file",
                builder.file_data.file_identifier.clone(),
                builder.file_data,
                || gather_initial_file_data(builder),
            );
        });
        let assoc_vals = self.files[file_id].associated_values.clone();
        self.checkpoint(&assoc_vals, AFTER_INITIAL_PARSE_CP);

        info_mngr.on_file_added(file_id, self);

        file_id
    }

    // When --feature lsp is not used, this gives a warning
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
                crate::debug::debug_context(
                    "gather_initial_file_data in update_file",
                    builder.file_data.file_identifier.clone(),
                    builder.file_data,
                    || gather_initial_file_data(builder),
                );
            });
            let assoc_vals = self.files[file_id].associated_values.clone();
            self.checkpoint(&assoc_vals, AFTER_INITIAL_PARSE_CP);

            info_mngr.on_file_updated(file_id, self);
        } else {
            self.add_file_text(file_identifier.to_owned(), text, info_mngr);
        }
    }

    pub fn find_file(&self, file_identifier: &str) -> Option<FileUUID> {
        self.files
            .find(|_id, f| f.file_identifier == file_identifier)
    }

    pub fn recompile_all_report_panics(&mut self) {
        crate::debug::create_dump_on_panic(self, |slf| slf.recompile_all())
    }

    pub fn recompile_all(&mut self) {
        let config = config();

        self.instantiator.borrow_mut().clear_instances();

        let global_ids = self.get_all_global_ids();
        // First reset all modules back to post-gather_initial_file_data
        for id in &global_ids {
            let link_info = &mut self.globals[*id];

            link_info.reset_to(AFTER_INITIAL_PARSE_CP);
            link_info.instructions.clear();
        }
        if config.early_exit == EarlyExitUpTo::Initialize {
            return;
        }

        flatten_all_globals(self);

        self.checkpoint(&global_ids, AFTER_FLATTEN_CP);
        if config.early_exit == EarlyExitUpTo::Flatten {
            return;
        }

        for global_id in &global_ids {
            self.pass("Typechecking", *global_id, |pass, errors, files| {
                typecheck(pass, errors);

                if crate::debug::is_enabled("print-abstract") {
                    let (md, globals) = pass.get_with_context();
                    if let GlobalObj::Module(md) = md {
                        md.print_flattened_module(&files[md.link_info.file], globals.globals);
                    }
                }
            });
        }
        self.checkpoint(&global_ids, AFTER_TYPE_CHECK_CP);

        for (_, md) in &self.modules {
            md.assert_valid();
        }

        if config.early_exit == EarlyExitUpTo::AbstractTypecheck {
            return;
        }

        for global_id in &global_ids {
            self.pass("Lints", *global_id, |pass, errors, files| {
                perform_lints(pass, errors, files);
            });
        }
        self.checkpoint(&global_ids, AFTER_LINTS_CP);

        if config.early_exit == EarlyExitUpTo::Lint {
            return;
        }

        // Make an initial instantiation of all modules
        // Won't be possible once we have template modules
        for (id, md) in &self.modules {
            // Already instantiate any modules without parameters
            // Can immediately instantiate modules that have no template args
            if md.link_info.parameters.is_empty() {
                let _inst = self.instantiator.instantiate(
                    self,
                    ConcreteGlobalReference {
                        id,
                        template_args: FlatAlloc::new(),
                    },
                );
            }
        }
        if config.early_exit == EarlyExitUpTo::Instantiate {}
    }
}
