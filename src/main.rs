mod util;
mod block_vector;
mod arena_alloc;
mod tokenizer;
mod parser;
mod errors;
mod ast;
mod value;
mod flattening;
mod instantiation;

#[cfg(feature = "codegen")]
mod codegen;

mod codegen_fallback;

mod typing;

mod dev_aid;
mod linker;

use std::{env, ops::Deref};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use ast::Module;
use codegen_fallback::gen_verilog_code;
use dev_aid::syntax_highlighting::*;
use linker::{Named, Linker, NamedUUID};

fn codegen_to_file(linker : &Linker, id : NamedUUID, md : &Module) -> Option<()> {
    let inst = linker.instantiate(id)?;

    let module_name = md.link_info.name.deref();

    if inst.errors.did_error.get() {
        println!("There were instantiation errors in {module_name}");
        return None;
    }
    //println!("Generating Verilog for {module_name}:");
    // gen_ctx.to_circt();
    let code = gen_verilog_code(md, &inst);

    let mut out_file = File::create(format!("verilog_output/{module_name}.v")).unwrap();
    write!(out_file, "{}", code).unwrap();
    Some(())
}

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut args = env::args();

    let _executable_path = args.next();

    let mut file_paths : Vec<PathBuf> = Vec::new();
    let mut is_lsp = false;
    let mut codegen = None;
    let mut codegen_all = false;
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
            "--codegen-all" => {
                codegen_all = true;
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
        // Quick debugging
        file_paths.push(PathBuf::from("resetNormalizer.sus"));
        file_paths.push(PathBuf::from("multiply_add.sus"));
        codegen_all = true;
        //codegen = Some("first_bit_idx_6".to_owned());
    }

    let (linker, paths_arena) = compile_all(file_paths);
    print_all_errors(&linker, &paths_arena);
    for (id, path) in &paths_arena {
        println!("\n\n[{}]: ", path.to_string_lossy());
        syntax_highlight_file(&linker, id, &settings);
    }

    // #[cfg(feature = "codegen")]
    if let Some(module_name) = codegen {
        //let gen_ctx = codegen::GenerationContext::new();
        
        let Some(id) = linker.get_obj_id(&module_name) else {
            panic!("Module {module_name} does not exist!");
        };

        let Named::Module(md) = &linker.globals[id] else {
            panic!("{module_name} is not a Module!");
        };
        
        codegen_to_file(&linker, id, md);
    }

    if codegen_all {
        for (id, obj) in &linker.globals {
            if let Named::Module(md) = obj {
                codegen_to_file(&linker, id, md);
            }
        }
    }

    Ok(())
}
