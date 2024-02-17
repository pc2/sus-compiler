//#![allow(dead_code)]

mod util;
mod block_vector;
mod arena_alloc;
mod list_of_lists;

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

use std::process::Stdio;
use std::{env, ops::Deref};
use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use ast::Module;
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
    let code = gen_verilog_code(md, &inst);

    let mut out_file = File::create(format!("verilog_output/{module_name}.v")).unwrap();
    write!(out_file, "{}", code).unwrap();
    Some(())
}

fn test_tree_sitter(path : &Path) {
    let code = read_to_string(path).unwrap();
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_sus::language()).expect("Error loading sus grammar");
    let tree = parser.parse(code, None).unwrap();

    let mut dot_cmd = std::process::Command::new("dot");
    dot_cmd.arg("-Tsvg");
    dot_cmd.arg("-Gcharset=latin1");
    dot_cmd.stdin(Stdio::piped());
    dot_cmd.stdout(Stdio::piped());
    let dot_proc = dot_cmd.spawn().unwrap();
    tree.print_dot_graph(dot_proc.stdin.as_ref().unwrap());
    let out = dot_proc.wait_with_output().unwrap();
    let mut out_file = File::create(format!("{}.svg", path.file_stem().unwrap().to_str().unwrap())).unwrap();
    out_file.write(&out.stdout).unwrap();
    
    /*
    let root = tree.root_node();
    let mut cursor = root.walk();
    for c in root.children(&mut cursor) {
        println!("{c:?}");
    }
    println!("{root:?}");*/
}


fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut args = env::args();

    let _executable_path = args.next();

    let mut file_paths : Vec<PathBuf> = Vec::new();
    let mut is_lsp = None;
    let mut codegen = None;
    let mut codegen_all = false;
    let mut test_sus_sitter = false;
    let mut settings = SyntaxHighlightSettings{
        show_tokens : false
    };
    
    while let Some(arg) = args.next() {
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
            "--tree" => {
                test_sus_sitter = true;
            }
            other => {
                file_paths.push(PathBuf::from(other));
            }
        }
    }
    
    #[cfg(feature = "lsp")]
    if let Some(debug) = is_lsp {
        return dev_aid::lsp::lsp_main(25000, debug);
    }
    if file_paths.len() == 0 {
        // Quick debugging
        file_paths.push(PathBuf::from("resetNormalizer.sus"));
        file_paths.push(PathBuf::from("multiply_add.sus"));
        codegen_all = true;
        //codegen = Some("first_bit_idx_6".to_owned());
    }

    if test_sus_sitter {
        for path in &file_paths {
            test_tree_sitter(&path);
        }
        return Ok(())
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
