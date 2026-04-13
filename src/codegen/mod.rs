mod patches;
mod sv_utils;
mod system_verilog;

use log::logger;

use crate::codegen::system_verilog::generate_systemverilog;
use crate::codegen::system_verilog::generate_testbench_stub;
use crate::prelude::*;

use crate::{InstantiatedModule, Linker};

use crate::config::{TargetLanguage, VERSION_INFO, config};

use std::collections::HashSet;
use std::io::stdout;
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

    if !config().ci {
        let gen_time = chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false);
        if let Err(e) = write!(
            file,
            "// THIS IS A GENERATED FILE (Generated at {gen_time})\n// This file was generated with SUS Compiler {VERSION_INFO}\n"
        ) {
            fatal_exit!("Error while writing to {}: {e}", path.to_string_lossy());
        }
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

    let mut all_instances = HashSet::new();
    let mut dependency_stack = Vec::new();
    let mut any_error = false;
    for top in &linker.instantiator.tops {
        let inst = linker.instantiator.get(top);
        if !inst.errors.did_error {
            order_dependencies(&mut all_instances, &mut dependency_stack, inst);
        } else {
            any_error = true;
            error!("Cannot codegen {} due to errors!", inst.name);
        }
    }
    if let Some(path) = &config.codegen_file {
        let mut out_file = make_output_file(path);

        if !config.ci {
            info!(
                "Codegen to {}",
                path.canonicalize().unwrap().to_string_lossy()
            );
        }

        for md in dependency_stack.iter().rev() {
            let code = generate_systemverilog(md, linker);
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
        if !config.ci {
            info!(
                "Codegen to directory {}",
                output_folder.canonicalize().unwrap().to_string_lossy()
            );
        }

        for (id, md) in &linker.modules {
            let filename = sanitize_filename(&md.link_info.name, ".sv");
            let path = output_folder.join(filename);
            let mut out_file = make_output_file(&path);
            for (_global_ref, inst) in linker.instantiator.iter_for_module(id) {
                let code = generate_systemverilog(inst, linker);
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

pub fn maybe_gen_tb(linker: &Linker) {
    if !config().gen_tb {
        return;
    }
    info!("===== Printing Testbench Stubs to STDOUT =====");
    // Flush since we want to print to STDOUT so the testbench stubs can be redirected to a file
    // don't want the streams to get de-synced with STDERR
    logger().flush();
    for top in &linker.instantiator.tops {
        let inst = linker.instantiator.get(top);
        if inst.errors.did_error {
            error!("Cannot give testbench for {} due to errors!", inst.name);
            continue;
        }
        let testbench_stub = generate_testbench_stub(inst, linker);
        // Explicitly print to STDOUT.
        println!("{testbench_stub}");
    }
    // And flush again, such that we're in sync again for the STDERR print afterwards
    let _ = stdout().flush();
    info!("===== Done Printing Testbench Stubs to STDOUT =====");
}

// Limit total folder/file name byte size to 255, which is the maximum on just about every platform
const MAX_FILENAME_LEN: usize = 255;

#[cfg(target_os = "linux")]
const INVALID_CHARS: &[char] = &['/'];
#[cfg(not(target_os = "linux"))]
const INVALID_CHARS: &[char] = &['\\', '/', ':', '*', '?', '"', '<', '>', '|']; // Mostly for windows. 

/// Shorten the total string (name + postfix) such that `format!("{name}{postfix}").len() <= MAX_FILENAME_LEN`
pub fn sanitize_filename(name: &str, postfix: &str) -> String {
    let max_len = MAX_FILENAME_LEN - postfix.len();
    if name.len() <= max_len {
        format!("{name}{postfix}")
    } else {
        let mut shortened = String::with_capacity(name.len() + postfix.len() + 1); // One for CString \0
        for c in name.chars() {
            if shortened.len() + c.len_utf8() >= max_len {
                break;
            }
            let new_c = if INVALID_CHARS.contains(&c) { ' ' } else { c };
            shortened.push(new_c);
        }
        let result = format!("{shortened}{postfix}");
        warn!(
            "Filename {name}{postfix} was shortened to {result} to avoid too long filenames on some platforms"
        );
        result
    }
}
