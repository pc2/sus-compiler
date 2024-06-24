#![doc = include_str!("../README.md")]

#![cfg_attr(not(all(feature = "lsp")), allow(dead_code, unused_assignments, unused_variables))]

mod util;
mod block_vector;
mod arena_alloc;

mod file_position;
mod parser;
mod errors;
mod value;
mod flattening;
mod instantiation;
mod debug;
mod config;

#[cfg(feature = "codegen")]
mod codegen;

mod codegen_fallback;

mod concrete_type;
mod abstract_type;

mod dev_aid;
mod linker;

mod compiler_top;

use std::io::Write;
use std::ops::Deref;
use std::error::Error;
use std::fs::File;
use config::{config, parse_args};
use flattening::Module;
use codegen_fallback::gen_verilog_code;
use dev_aid::ariadne_interface::*;

fn codegen_to_file(md : &Module) {
    let module_name = md.link_info.name.deref();
    let mut out_file = File::create(format!("verilog_output/{module_name}.v")).unwrap();
    md.instantiations.for_each_instance(|inst| {
        if inst.errors.did_error {
            println!("Instantiating error: {}<{:?}>", md.link_info.name, md.link_info.template_arguments);
            return; // Continue
        }

        println!("Instantiating success: {}<{:?}>", md.link_info.name, md.link_info.template_arguments);
    
        //println!("Generating Verilog for {module_name}:");
        // gen_ctx.to_circt();
        let code = gen_verilog_code(md, &inst, true);
    
        write!(out_file, "{}", code).unwrap();
    });
}

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let file_paths = parse_args();
    
    let config = config();

    if config.use_lsp {
        #[cfg(feature = "lsp")]
        return dev_aid::lsp::lsp_main();

        #[cfg(not(feature = "lsp"))]
        panic!("LSP not enabled!")
    }

    let (linker, mut paths_arena) = compile_all(file_paths);
    print_all_errors(&linker, &mut paths_arena);
    
    if config.codegen {
        for (_id, md) in &linker.modules {
            codegen_to_file(md);
        }
    }

    Ok(())
}
