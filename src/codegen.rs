use crate::{linker::Linker, flattening::{FlattenedModule, WireID}, ast::{Module, Span}, arena_alloc::ListAllocator};


use calyx_ir::*;


pub fn to_calyx(md : &Module, flattened : &FlattenedModule, linker : &Linker) {
    calyx_ir::
    //calyx_ir::Builder::new(component, lib);
    calyx_ir::build_assignments!();

    let mut test_comp = calyx_ir::Component::new("name", vec![], true, true, None);
    test_comp.

    let builder = calyx_ir::Builder::new(component, lib)

    let ctx : calyx_ir::Context = calyx_ir::Builder;

    let backend = calyx_backend::VerilogBackend;

    let res : CalyxResult<()> = backend.emit(ctx: &ir::Context, file: &mut OutputFile)
}


pub fn gen_verilog_code(md : &Module, flattened : &FlattenedModule, linker : &Linker) {
    let mut cur_wire_id : usize = 0;
    let mut wire_names = flattened.wires.map(&mut |_id, _| {
        let name = format!("w_{cur_wire_id}");
        cur_wire_id += 1;
        name
    });

    let mut cur_inst_id : usize = 0;
    let mut instance_names = flattened.instantiations.map(&mut |_id, _| {
        let name = format!("inst_{cur_inst_id}");
        cur_inst_id += 1;
        name
    });

    let file = &linker.files[md.link_info.file];
    for (idx, v) in md.declarations.iter().enumerate() {
        let name = file.file_text[v.name.clone()].to_owned();
        match &flattened.local_map[idx].wire_or_instance {
            crate::flattening::WireOrInstantiation::Wire(w_idx) => {
                wire_names[*w_idx] = name;
            },
            crate::flattening::WireOrInstantiation::Instantiation(i_idx) => {
                instance_names[*i_idx] = name;
            },
            crate::flattening::WireOrInstantiation::Other(_) => {},
        }
    }

    println!("Module {} {{", &file.file_text[file.tokens[md.link_info.name_token].get_range()]);
    for (id, w) in &flattened.wires {
        println!("\twire {:?} {};", w.typ, &wire_names[id]);
    }
    println!();
    for (id, i) in &flattened.instantiations {
        println!("\tinstantiation {:?} {};", i, &instance_names[id]);
    }
    println!();
    for conn in &flattened.connections {
        let regs = "reg ".repeat(conn.num_regs as usize);


        if conn.condition != WireID::INVALID {
            
        }
        println!("\t·{regs}{:?} {};", i, &instance_names[id]);
    }
    println!("}}")
}

