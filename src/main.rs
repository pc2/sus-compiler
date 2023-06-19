
mod tokenizer;
mod parser;
mod errors;
mod ast;

mod lsp;

use lsp::syntax_highlighting::*;

fn main() {
    let file_path = "multiply_add.sus";
    
    syntax_highlight_file(file_path);
}
