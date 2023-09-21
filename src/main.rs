
mod tokenizer;
mod parser;
mod errors;
mod ast;
mod code_generation;
mod arena_alloc;

mod dev_aid;
mod linker;

use std::env;
use std::error::Error;
use std::path::PathBuf;
use dev_aid::syntax_highlighting::*;


fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut args = env::args();

    let _executable_path = args.next();

    let mut file_paths : Vec<PathBuf> = Vec::new();
    let mut is_lsp : bool = false;
    for arg in args {
        if arg == "--lsp" {
            is_lsp = true;
        } else {
            file_paths.push(PathBuf::from(arg));
        }
    }
    #[cfg(feature = "lsp")]
    if is_lsp {
        return dev_aid::lsp::lsp_main();
    }
    if file_paths.len() == 0 {
        // Quick debug file
        file_paths.push(PathBuf::from("multiply_add.sus"));
    }
    syntax_highlight_file(file_paths);

    Ok(())
}

