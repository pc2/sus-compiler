mod shared;
pub mod system_verilog;

use crate::codegen::system_verilog::gen_verilog_code;
use crate::prelude::*;

use crate::to_string::join_shorten_filename;
use crate::{InstantiatedModule, Linker};

use crate::config::{TargetLanguage, VERSION_INFO, config};

use std::collections::HashSet;
use std::path::Path;
use std::process::ExitCode;
use std::{fs::File, io::Write};

fn make_output_file(path: &Path) -> File {
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            fatal_exit!(
                "Could not create the output file {}: {e}",
                path.to_string_lossy()
            );
        }
    };

    let generation_time = chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false);
    if let Err(e) = write!(
        file,
        "// THIS IS A GENERATED FILE (Generated at {generation_time})\n// This file was generated with SUS Compiler {VERSION_INFO}\n"
    ) {
        fatal_exit!("Error while writing to {}: {e}", path.to_string_lossy());
    }

    file
}

/// Performs a topological sort of the module hierarchy. When finished stack contains the partial order of dependencies, with leaf submodules at the front, and the top level modules at the end
fn order_dependencies<'inst>(
    seen: &mut HashSet<*const InstantiatedModule>,
    stack: &mut Vec<&'inst InstantiatedModule>,
    md: &'inst InstantiatedModule,
) {
    assert!(!md.errors.did_error);
    if !seen.insert(md) {
        return; // already saw this module
    }

    for (_, sm) in &md.submodules {
        let sm_md = sm.instance.get().unwrap(); // No errors should have occured for the module
        order_dependencies(seen, stack, sm_md);
    }

    stack.push(md);
}

pub fn codegen(linker: &Linker) -> ExitCode {
    let config = config();
    if config.codegen_file.is_none() && config.codegen_separate_folder.is_none() {
        return ExitCode::SUCCESS; // early exit, to save work
    }
    assert_eq!(config.target_language, TargetLanguage::SystemVerilog);
    let instantiatior = linker.instantiator.borrow();

    let mut all_instances = HashSet::new();
    let mut dependency_stack = Vec::new();
    let mut any_error = false;
    if config.top_modules.is_empty() {
        for (id, _) in &linker.modules {
            for (_, md) in instantiatior.iter_for_module(id) {
                if !md.errors.did_error {
                    order_dependencies(&mut all_instances, &mut dependency_stack, md);
                } else {
                    any_error = true;
                    error!("Cannot codegen {} due to errors!", md.name);
                }
            }
        }
    } else {
        for top in &config.top_modules {
            let md_id = linker.get_by_name(top).unwrap().unwrap_module();
            for (_, md) in instantiatior.iter_for_module(md_id) {
                if !md.errors.did_error {
                    order_dependencies(&mut all_instances, &mut dependency_stack, md);
                } else {
                    any_error = true;
                    error!("Cannot codegen {} due to errors!", md.name);
                }
            }
        }
    }
    if let Some(path) = &config.codegen_file {
        let mut out_file = make_output_file(path);
        for md in dependency_stack.iter().rev() {
            let code = gen_verilog_code(md, linker);
            if let Err(e) = out_file.write(code.as_bytes()) {
                fatal_exit!("Error while writing to {}: {e}", path.to_string_lossy());
            }
        }
    }
    if let Some(output_folder) = &config.codegen_separate_folder {
        if let Err(e) = std::fs::create_dir_all(output_folder) {
            fatal_exit!(
                "Could not create the output directory {}: {e}",
                output_folder.to_string_lossy()
            );
        }
        for (id, md) in &linker.modules {
            let filename = join_shorten_filename(&md.link_info.name, ".sv");
            let path = output_folder.join(filename);
            let mut out_file = make_output_file(&path);
            for (_global_ref, inst) in instantiatior.iter_for_module(id) {
                let code = gen_verilog_code(inst, linker);
                if let Err(e) = write!(out_file, "{code}") {
                    fatal_exit!("Error while writing to {}: {e}", path.to_string_lossy());
                }
            }
        }
    }
    if any_error {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
