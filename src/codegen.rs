use std::{ops::Deref, io};

use crate::{linker::Linker, flattening::{FlattenedModule, WireID}, ast::{Module, Span}, arena_alloc::ListAllocator};


use moore_circt::{hw, comb, mlir::{self, Owned, builder, OperationExt, SingleBlockOp}, mlir::{Context, OwnedContext, DialectHandle, Builder, Value, Type}};
use num::bigint::BigInt;


pub struct GenerationContext {
    global_ctx : Owned<Context>
}

impl GenerationContext {
    pub fn new() -> Self {
        let global_ctx = moore_circt::mlir::context::OwnedContext::new();
        global_ctx.load_dialect(hw::dialect());
        global_ctx.load_dialect(comb::dialect());
        Self{global_ctx}
    }

    pub fn to_circt(&self) {
        let ctx = *self.global_ctx.deref();
        //moore_circt::hw::
        let module = moore_circt::ModuleOp::new(ctx);

        let mod_ctx = module.context();

        let mut builder = Builder::new(mod_ctx);
        builder.set_insertion_point_to_start(module.block());

        //mlir_builder.set_loc(span_to_loc(mod_ctx, hir.span()));
        //mlir_builder.set_insertion_point_to_end(self.into_mlir.block());

        //builder.set_insertion_point_to_start(module.into());
        
        let int32 = mlir::ty::get_integer_type(ctx, 32);
        //entity_builder.add_input("a", int32);
        //entity_builder.add_input("b", int32);
        //entity_builder.add_output("o", int32);

        //let built_entity = entity_builder.build_entity(builder)

        let const_val = BigInt::from(5);
        let const_value = hw::ConstantOp::new(&mut builder, 5, &const_val).into();

        let add = comb::AddOp::new(&mut builder, const_value, const_value);
        
        module.print(io::stdout().lock(), true);
    }
}

/*
use calyx_ir::*;

pub fn to_calyx(md : &Module, flattened : &FlattenedModule, linker : &Linker) {
    let mut test_comp = calyx_ir::Component::new("name", vec![], true, true, None);
    let c = calyx_ir::Cell::new(name, prototype);
    test_comp.cells.add(calyx_ir::rrc(c));

    let ctx = calyx_ir::Context {
        components: todo!(),
        lib : calyx_ir::LibrarySignatures::default(),
        entrypoint: todo!(),
        bc: todo!(),
        extra_opts: todo!(),
        metadata: todo!(),
    };

    // Builds a component
    let builder = calyx_ir::Builder::new(&mut test_comp, &ctx.lib);

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
*/
