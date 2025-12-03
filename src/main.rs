#![doc = include_str!("../README.md")]
#![allow(clippy::too_many_arguments)]

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
    crate::debug::setup_span_panic_handler();

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

    let mut linker = Linker::new();
    crate::debug::create_dump_on_panic(&mut linker, |linker| {
        let mut paths_arena = compile_all(linker, file_paths);
        print_all_errors(&*linker, &mut paths_arena.file_sources);

        crate::codegen::codegen(&*linker)
    })
}
