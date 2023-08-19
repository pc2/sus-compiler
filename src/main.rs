
mod tokenizer;
mod parser;
mod errors;
mod ast;
mod code_generation;
mod global_context;

mod dev_aid;

use std::env;
use std::error::Error;
use dev_aid::syntax_highlighting::*;



fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut args = env::args();

    let _executable_path = args.next();

    match args.next() {
        None => {
            // Quick debug path
            let file_path = "multiply_add.sus";
            syntax_highlight_file(file_path);
        },
        #[cfg(feature = "lsp")]
        Some(cmd) if cmd == "--lsp" => {
            return dev_aid::lsp::lsp_main();
        },
        Some(file_path) => {
            syntax_highlight_file(&file_path);
        }
    }

    Ok(())
}

