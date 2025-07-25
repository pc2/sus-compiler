#![doc = include_str!("../README.md")]

mod alloc;
mod append_only_vec;

mod config;
mod debug;
mod errors;
mod file_position;
mod flattening;
mod instantiation;
mod latency;
mod prelude;
mod to_string;
mod typing;
mod util;
mod value;

mod codegen;

mod dev_aid;
mod linker;

mod compiler_top;

use std::error::Error;
use std::io::Write;

use prelude::*;

use codegen::{CodeGenBackend, VHDLCodegenBackend, VerilogCodegenBackend};
use config::{config, initialize_config_from_cli_args, EarlyExitUpTo};
use dev_aid::ariadne_interface::*;
use flattening::Module;
use instantiation::InstantiatedModule;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    initialize_config_from_cli_args();

    let config = config();

    let file_paths = config.files.clone();

    let codegen_backend = match config.target_language {
        config::TargetLanguage::SystemVerilog => {
            Box::new(VerilogCodegenBackend) as Box<dyn CodeGenBackend>
        }
        config::TargetLanguage::Vhdl => Box::new(VHDLCodegenBackend) as Box<dyn CodeGenBackend>,
    };

    if config.use_lsp {
        return dev_aid::lsp::lsp_main();
    }

    debug::setup_panic_handler();

    let (linker, mut paths_arena) = compile_all(file_paths);
    print_all_errors(&linker, &mut paths_arena.file_sources);

    if config.early_exit != EarlyExitUpTo::CodeGen {
        return Ok(());
    }

    if config.codegen {
        if let Some(standalone) = &config.standalone {
            let top_md_name = &standalone.top_module;
            let Some(md) = linker
                .modules
                .iter()
                .find(|(_, md)| &md.link_info.name == top_md_name)
            else {
                let mut err_lock = std::io::stderr().lock();
                writeln!(err_lock, "Unknown module {top_md_name}").unwrap();
                std::process::exit(1);
            };

            codegen_backend.codegen_with_dependencies(&linker, md.0, &standalone.file_path);
        } else {
            for (id, md) in &linker.modules {
                codegen_backend.codegen_to_file(id, md, &linker);
            }
        }
    }
    Ok(())
}
