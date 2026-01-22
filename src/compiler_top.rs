use crate::prelude::*;

use std::cell::OnceCell;
use std::ffi::OsStr;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::config::EarlyExitUpTo;
use crate::linker::checkpoint::{
    AFTER_FLATTEN_CP, AFTER_INITIAL_PARSE_CP, AFTER_LINTS_CP, AFTER_TYPE_CHECK_CP,
};
use crate::linker::{GlobalObj, UniqueFileID};
use crate::typing::concrete_type::ConcreteGlobalReference;

use sus_proc_macro::{get_builtin_const, get_builtin_type};
use tree_sitter::Parser;

use crate::{config::config, errors::ErrorStore, file_position::FileText, linker::FileData};

use crate::flattening::{flatten_all_globals, gather_initial_file_data};

pub fn get_std_dir() -> PathBuf {
    config().sus_home.join("std")
}

impl Linker {
    pub fn add_standard_library(&mut self) {
        assert!(self.modules.is_empty());
        assert!(self.types.is_empty());
        assert!(self.constants.is_empty());
        let std_lib_path = get_std_dir();
        self.add_all_files_in_directory_recurse(&std_lib_path);
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
            self.types[get_builtin_type!("double")].link_info.name,
            "double"
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

    pub fn add_all_files_in_directory_recurse(&mut self, directory: &Path) {
        let dir_read = std::fs::read_dir(directory);
        let dir_read = match dir_read {
            Ok(d) => d,
            Err(_) => fatal_exit!("Can't read directory {}", directory.to_string_lossy()),
        };
        let mut files: Vec<_> = dir_read
            .map(|res| match res {
                Ok(path) => path.path(),
                Err(err) => fatal_exit!(
                    "No such file or directory {} in {}",
                    err,
                    directory.to_string_lossy()
                ),
            })
            .collect();
        files.sort();
        for file in files {
            let file_path = file.canonicalize().unwrap();
            if file_path.is_dir() {
                self.add_all_files_in_directory_recurse(&file_path);
            } else if file_path.is_file() && file_path.extension() == Some(OsStr::new("sus")) {
                let file_identifier = UniqueFileID::from_path(&file_path);
                self.add_or_update_file_from_disk(file_identifier);
            }
        }
    }

    pub fn add_file_or_directory(&mut self, path: &Path) {
        if path.is_dir() {
            self.add_all_files_in_directory_recurse(path);
        } else {
            if path.is_file() {
                let file_identifier = UniqueFileID::from_path(&path);
                self.add_or_update_file_from_disk(file_identifier);
            } else {
                fatal_exit!(
                    "{} is neither a file nor a directory!",
                    path.to_string_lossy()
                )
            }
        }
    }

    // When --feature lsp is not used, this gives a warning
    #[allow(dead_code)]
    pub fn add_or_update_file_text(
        &mut self,
        file_identifier: UniqueFileID,
        text: String,
    ) -> FileUUID {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_sus::language()).unwrap();
        let tree = parser.parse(&text, None).unwrap();
        let file_text = FileText::new(text);

        let file_id = if let Some(file_id) = self.find_file(&file_identifier) {
            let file_data = self.remove_everything_in_file(file_id);

            // overwrite file_identifier with updated file_identifier, such that renames are caught.
            file_data.file_identifier = file_identifier;
            file_data.parsing_errors = ErrorStore::new();
            file_data.file_text = file_text;
            file_data.tree = tree;

            file_id
        } else {
            self.files.alloc(FileData {
                file_identifier,
                file_text,
                tree,
                associated_values: Vec::new(),
                parsing_errors: ErrorStore::new(),
                is_std: false,
                ariadne_source: OnceCell::new(),
            })
        };

        self.with_file_builder(file_id, |builder| {
            crate::debug::debug_context(
                "gather_initial_file_data in update_file",
                builder.file_data.file_identifier.name.clone(),
                || gather_initial_file_data(builder),
            );
        });
        let assoc_vals = &self.files[file_id].associated_values;
        self.globals.checkpoint(assoc_vals, AFTER_INITIAL_PARSE_CP);

        file_id
    }

    // When --feature lsp is not used, this gives a warning
    #[allow(dead_code)]
    pub fn add_or_update_file_from_disk(&mut self, mut file_identifier: UniqueFileID) -> FileUUID {
        let file = file_identifier.inode.as_mut().unwrap().as_file_mut();
        let mut file_content = String::new();

        if let Err(reason) = file.read_to_string(&mut file_content) {
            let file_path_disp = &file_identifier.name;
            fatal_exit!(
                "Could not open file '{file_path_disp}' for syntax highlighting because {reason}"
            )
        };

        self.add_or_update_file_text(file_identifier, file_content)
    }

    pub fn find_file(&self, file_identifier: &UniqueFileID) -> Option<FileUUID> {
        self.files
            .find(|_id, f| &f.file_identifier == file_identifier)
    }

    pub fn recompile_all(&mut self) {
        let config = config();

        self.instantiator.clear_instances();

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

        self.globals.checkpoint(&global_ids, AFTER_FLATTEN_CP);
        if config.early_exit == EarlyExitUpTo::Flatten {
            return;
        }

        for global_id in &global_ids {
            self.pass("Typechecking", *global_id, |pass, errors, files| {
                crate::flattening::typecheck::typecheck(pass, errors);

                if crate::debug::is_enabled("print-abstract") {
                    let (md, globals) = pass.get_with_context();
                    if let GlobalObj::Module(md) = md {
                        md.print_flattened_module(files, globals.globals);
                    }
                }
            });
        }
        self.globals.checkpoint(&global_ids, AFTER_TYPE_CHECK_CP);

        for (_, md) in &self.modules {
            md.assert_valid();
        }

        if config.early_exit == EarlyExitUpTo::AbstractTypecheck {
            return;
        }

        for global_id in &global_ids {
            self.pass("Lints", *global_id, |pass, errors, files| {
                crate::flattening::typecheck::perform_lints(pass, errors, files);
            });
        }
        self.globals.checkpoint(&global_ids, AFTER_LINTS_CP);

        if config.early_exit == EarlyExitUpTo::Lint {
            return;
        }

        if config.top_modules.is_empty() {
            info!("Selecting all parameter-less modules as --top");

            // Make an initial instantiation of all modules
            // Won't be possible once we have template modules
            for (id, md) in &self.globals.modules {
                // Already instantiate any modules without parameters
                // Can immediately instantiate modules that have no template args
                if md.link_info.parameters.is_empty() {
                    let _inst = self.instantiator.instantiate(
                        &self.globals,
                        &self.files,
                        ConcreteGlobalReference {
                            id,
                            template_args: FlatAlloc::new(),
                        },
                    );
                }
            }
        } else {
            for top in &config.top_modules {
                match self.get_by_name(top) {
                    Ok(GlobalObj::Module(id)) => {
                        let md = &self.modules[id];
                        if md.link_info.parameters.is_empty() {
                            let _inst = self.instantiator.instantiate(
                                &self.globals,
                                &self.files,
                                ConcreteGlobalReference {
                                    id,
                                    template_args: FlatAlloc::new(),
                                },
                            );
                        } else {
                            let md_with_args = md.link_info.display_full_name_and_args(
                                &self.files[md.link_info.span.file].file_text,
                            );
                            fatal_exit!(
                                "Can't instantiate module {md_with_args} as top-level module, because it has parameters"
                            )
                        }
                    }
                    Ok(obj) => {
                        let kind = obj.get_kind_name();
                        fatal_exit!("{kind} {top} is not a module! It can't be a --top");
                    }
                    Err(e) => fatal_exit!("{}", e.get_main_message()),
                }
            }
        }
    }
}
