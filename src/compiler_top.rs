use std::rc::Rc;

use tree_sitter::Parser;

use crate::{
    errors::ErrorCollector,
    file_position::FileText,
    flattening::{initialization::gather_initial_file_data, FlattenedModule},
    instantiation::InstantiatedModule,
    linker::{FileData, FileUUID, Linker, ModuleUUID}
};

pub fn add_file(text : String, linker : &mut Linker) -> FileUUID {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_sus::language()).unwrap();
    let tree = parser.parse(&text, None).unwrap();
    
    let file_id = linker.files.reserve();
    linker.files.alloc_reservation(file_id, FileData{
        parsing_errors : ErrorCollector::new(file_id, text.len()),
        file_text : FileText::new(text),
        tree,
        associated_values : Vec::new()
    });

    let mut builder = linker.get_file_builder(file_id);
    gather_initial_file_data(&mut builder);

    file_id
}

pub fn update_file(text : String, file_id : FileUUID, linker : &mut Linker) {
    let file_data = linker.remove_everything_in_file(file_id);

    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_sus::language()).unwrap();
    let tree = parser.parse(&text, None).unwrap();

    file_data.parsing_errors.reset(text.len());
    file_data.file_text = FileText::new(text);
    file_data.tree = tree;

    let mut builder = linker.get_file_builder(file_id);
    gather_initial_file_data(&mut builder);
}

pub fn recompile_all(linker : &mut Linker) {
    // Flatten all modules
    let id_vec : Vec<ModuleUUID> = linker.modules.iter().map(|(id, _)| id).collect();
    for id in id_vec {
        let md = &linker.modules[id];// Have to get them like this, so we don't have a mutable borrow on self.modules across the loop
        println!("Flattening {}", md.link_info.name);

        let flattened = FlattenedModule::flatten(&linker, md);
        println!("Typechecking {}", &md.link_info.name);

        let md = &mut linker.modules[id]; // Convert to mutable ptr
        md.flattened = flattened;
        md.instantiations.clear_instances();
    }

    // Can't merge these loops, because instantiation can only be done once all modules have been type checked
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

    md.instantiations.instantiate(&md.link_info.name, &md.flattened, linker)
}
