#![doc = include_str!("../README.md")]

mod util;
mod arena_alloc;

mod file_position;
mod errors;
mod value;
mod typing;
mod to_string;
mod flattening;
mod instantiation;
mod debug;
mod config;

#[cfg(feature = "codegen")]
mod codegen;

mod codegen_fallback;

mod dev_aid;
mod linker;

mod compiler_top;

use std::rc::Rc;
use std::io::Write;
use std::ops::Deref;
use std::error::Error;
use std::fs::File;
use config::{config, parse_args};
use flattening::Module;
use codegen_fallback::gen_verilog_code;
use dev_aid::ariadne_interface::*;
use instantiation::InstantiatedModule;
use linker::Linker;

fn codegen_instance(inst: &InstantiatedModule, md: &Module, out_file: &mut File) {
    let inst_name = &inst.name;
    if inst.errors.did_error {
        println!("Instantiating error: {inst_name}");
        return; // Continue
    }
    println!("Instantiating success: {inst_name}");
    let code = gen_verilog_code(md, &inst, true);
    write!(out_file, "// {inst_name}\n{code}").unwrap();

    
    //println!("Generating Verilog for {module_name}:");
    // gen_ctx.to_circt();
}

fn codegen_to_file(md : &Module) {
    let module_name = md.link_info.name.deref();
    let mut out_file = File::create(format!("verilog_output/{module_name}.v")).unwrap();
    md.instantiations.for_each_instance(|_template_args, inst| {
        codegen_instance(inst.as_ref(), md, &mut out_file)
    });
}

fn codegen_with_dependencies(linker : &Linker, md : &Module, file_name : &str) {
    let mut out_file = File::create(format!("verilog_output/{file_name}.v")).unwrap();

    let mut top_level_instances : Vec<Rc<InstantiatedModule>> = Vec::new();
    md.instantiations.for_each_instance(|_template_args, inst| {
        top_level_instances.push(inst.clone());
    });
    let mut to_process_queue : Vec<(&InstantiatedModule, &Module)> = top_level_instances.iter().map(|v| (v.as_ref(), md)).collect();


    let mut cur_idx = 0;

    while cur_idx < to_process_queue.len() {
        let (cur_instance, cur_md) = to_process_queue[cur_idx];

        for (_, sub_mod) in &cur_instance.submodules {
            let new_inst = sub_mod.instance.as_ref().unwrap().as_ref();

            // Skip duplicates
            // Yeah yeah I know O(n²) but this list shouldn't grow too big. Fix if needed
            if to_process_queue.iter().any(|existing| std::ptr::eq(existing.0, new_inst)) {
                continue;
            }

            to_process_queue.push((new_inst, &linker.modules[sub_mod.module_uuid]));
        }

        codegen_instance(cur_instance, cur_md, &mut out_file);

        cur_idx += 1;
    }
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

    if let Some(md_name) = &config.codegen_module_and_dependencies_one_file {
        let md = linker.modules.iter().find(|(_, md)| &md.link_info.name == md_name).unwrap();

        codegen_with_dependencies(&linker, md.1, &format!("{md_name}_standalone"));
    }

    Ok(())
}
