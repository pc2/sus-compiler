mod util;
mod block_vector;
mod arena_alloc;
mod tokenizer;
mod parser;
mod errors;
mod ast;
mod flattening;
mod instantiation;

#[cfg(feature = "codegen")]
mod codegen;

mod typing;

mod dev_aid;
mod linker;

use std::env;
use std::error::Error;
use std::path::PathBuf;
use dev_aid::syntax_highlighting::*;
use linker::Named;


fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut args = env::args();

    let _executable_path = args.next();

    let mut file_paths : Vec<PathBuf> = Vec::new();
    let mut is_lsp = false;
    let mut codegen = None;
    let mut settings = SyntaxHighlightSettings{
        show_tokens : false
    };
    
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--lsp" => {
                is_lsp = true;
            }
            "--codegen" => {
                codegen = Some(args.next().expect("Expected a module name after --codegen"));
            }
            "--tokens" => {
                settings.show_tokens = true;
            }
            other => {
                file_paths.push(PathBuf::from(other));
            }
        }
    }
    
    #[cfg(feature = "lsp")]
    if is_lsp {
        return dev_aid::lsp::lsp_main(25000);
    }
    if file_paths.len() == 0 {
        // Quick debug file
        file_paths.push(PathBuf::from("resetNormalizer.sus"));
        //file_paths.push(PathBuf::from("multiply_add.sus"));
    }

    let (linker, paths_arena) = compile_all(file_paths);
    print_all_errors(&linker, &paths_arena);
    for (id, path) in &paths_arena {
        println!("\n\n[{}]: ", path.to_string_lossy());
        syntax_highlight_file(&linker, id, &settings);
    }

    #[cfg(feature = "codegen")]
    if let Some(module_name) = codegen {
        let gen_ctx = codegen::GenerationContext::new();

        let Some(named_obj) = linker.links.get_obj_by_name(&module_name) else {
            panic!("Module {module_name} does not exist!");
        };

        let Named::Module(md) = named_obj else {
            panic!("{module_name} is not a Module!");
        };

        

        //gen_ctx.to_circt();
    }

    Ok(())
}
