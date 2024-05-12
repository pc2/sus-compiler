use std::rc::Rc;

use tree_sitter::Parser;

use crate::{
    debug::SpanDebugger, errors::ErrorStore, file_position::FileText, flattening::{flatten_all_modules, initialization::gather_initial_file_data, typechecking::typecheck_all_modules, Module}, instantiation::InstantiatedModule, linker::{FileData, FileUUID, Linker, ModuleUUID}
};

pub fn add_file(text : String, linker : &mut Linker) -> FileUUID {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_sus::language()).unwrap();
    let tree = parser.parse(&text, None).unwrap();
    
    let file_id = linker.files.reserve();
    linker.files.alloc_reservation(file_id, FileData{
        parsing_errors : ErrorStore::new(),
        file_text : FileText::new(text),
        tree,
        associated_values : Vec::new()
    });

    linker.with_file_builder(file_id, |builder| {
        let mut span_debugger = SpanDebugger::new("gather_initial_file_data in add_file", builder.file_text);
        gather_initial_file_data(builder);
        span_debugger.defuse();
    });

    file_id
}

pub fn update_file(text : String, file_id : FileUUID, linker : &mut Linker) {
    let file_data = linker.remove_everything_in_file(file_id);

    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_sus::language()).unwrap();
    let tree = parser.parse(&text, None).unwrap();

    file_data.parsing_errors = ErrorStore::new();
    file_data.file_text = FileText::new(text);
    file_data.tree = tree;

    linker.with_file_builder(file_id, |builder| {
        let mut span_debugger = SpanDebugger::new("gather_initial_file_data in update_file", builder.file_text);
        gather_initial_file_data(builder);
        span_debugger.defuse();
    });
}

pub fn recompile_all(linker : &mut Linker) {
    // First reset all modules back to post-gather_initial_file_data
    for (_, md) in &mut linker.modules {
        let Module { link_info, module_ports:_, instructions, instantiations } = md;
        link_info.reset_to(link_info.after_initial_parse_cp);
        link_info.after_flatten_cp = None;
        instructions.clear();
        instantiations.clear_instances()
    }

    flatten_all_modules(linker);
    typecheck_all_modules(linker);

    // Make an initial instantiation of all modules
    // Won't be possible once we have template modules
    for (id, _md) in &linker.modules {
        //md.print_flattened_module();
        // Already instantiate any modules without parameters
        // Currently this is all modules
        let _inst = instantiate(&linker, id);
    }
}

pub fn instantiate(linker : &Linker, module_id : ModuleUUID) -> Option<Rc<InstantiatedModule>> {
    let md = &linker.modules[module_id];
    println!("Instantiating {}", md.link_info.name);

    md.instantiations.instantiate(&md.link_info.name, md, linker)
}
