#![doc = include_str!("../README.md")]

#![cfg_attr(not(all(feature = "lsp")), allow(dead_code, unused_assignments, unused_variables))]

mod util;
mod block_vector;
mod arena_alloc;
mod list_of_lists;

mod file_position;
mod parser;
mod errors;
mod value;
mod flattening;
mod instantiation;

#[cfg(feature = "codegen")]
mod codegen;

mod codegen_fallback;

mod typing;

mod dev_aid;
mod linker;

use std::io::Write;
use std::{env, ops::Deref};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use flattening::Module;
use codegen_fallback::gen_verilog_code;
use dev_aid::syntax_highlighting::*;
use linker::{Linker, ModuleUUID};

fn codegen_to_file(linker : &Linker, id : ModuleUUID, md : &Module) -> Option<()> {
    let Some(inst) = linker.instantiate(id) else {
        println!("Module {} instantiation encountered errors.", md.link_info.name);

        return None;
    };

    let module_name = md.link_info.name.deref();

    //println!("Generating Verilog for {module_name}:");
    // gen_ctx.to_circt();
    let code = gen_verilog_code(md, &inst, true);

    let mut out_file = File::create(format!("verilog_output/{module_name}.v")).unwrap();
    write!(out_file, "{}", code).unwrap();
    Some(())
}

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut args = env::args();

    let _executable_path = args.next();

    let mut file_paths : Vec<PathBuf> = Vec::new();
    let mut is_lsp = None;
    let mut lsp_port = 25000;
    let mut codegen = None;
    let mut codegen_all = false;
    let mut settings = SyntaxHighlightSettings{
        show_tokens : false
    };
    
    while let Some(arg) = args.next() {
        if arg.starts_with("-") {
            if let Some((name, value)) = arg.split_once("=") {
                match name {
                    "--socket" => {
                        lsp_port = u16::from_str_radix(value, 10).unwrap();
                    }
                    other => panic!("Unknown option {other}"),
                }
            } else {
                match arg.as_str() {
                    "--lsp" => {
                        is_lsp = Some(false);
                    }
                    "--lsp-debug" => {
                        is_lsp = Some(true);
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
                        panic!("Unknown option {other}");
                    }
                }
            }
        } else {
            file_paths.push(PathBuf::from(arg));
        }
    }
    
    if let Some(debug) = is_lsp {
        #[cfg(feature = "lsp")]
        return dev_aid::lsp::lsp_main(lsp_port, debug);

        #[cfg(not(feature = "lsp"))]
        panic!("LSP not enabled!")
    }
    if file_paths.len() == 0 {
        // Quick debugging
        file_paths.push(PathBuf::from("resetNormalizer.sus"));
        file_paths.push(PathBuf::from("multiply_add.sus"));
        file_paths.push(PathBuf::from("tinyTestFile.sus"));
        codegen_all = true;
        //codegen = Some("first_bit_idx_6".to_owned());
    }

    let (linker, mut paths_arena) = compile_all(file_paths);
    print_all_errors(&linker, &mut paths_arena);
    
    // #[cfg(feature = "codegen")]
    if let Some(module_name) = codegen {
        //let gen_ctx = codegen::GenerationContext::new();
        
        let Some(id) = linker.get_module_id(&module_name) else {
            panic!("Module {module_name} does not exist!");
        };

        let md = &linker.modules[id];
        
        codegen_to_file(&linker, id, md);
    }

    if codegen_all {
        for (id, md) in &linker.modules {
            codegen_to_file(&linker, id, md);
        }
    }

    Ok(())
}
