#![doc = include_str!("../README.md")]

use prelude::*;

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

use std::process::ExitCode;

use config::config;
use dev_aid::ariadne_interface::*;
use instantiation::InstantiatedModule;

fn main() -> ExitCode {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    crate::config::parse_args();

    let config = config();

    let file_paths = config.files.clone();

    if config.lsp_settings.is_some() {
        return match dev_aid::lsp::lsp_main() {
            Ok(_) => ExitCode::SUCCESS,
            Err(err) => {
                fatal_exit!("LSP exited due to {err}");
            }
        };
    }

    debug::setup_panic_handler();

    let (linker, mut paths_arena) = compile_all(file_paths);
    print_all_errors(&linker, &mut paths_arena.file_sources);

    crate::codegen::codegen(&linker)
}
