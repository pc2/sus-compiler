use std::{ops::Deref, io};

use crate::{linker::Linker, flattening::{FlattenedModule, FlatID}, ast::{Module, Span}, arena_alloc::ListAllocator, instantiation::InstantiatedModule};


/*use moore_circt::{hw, comb, mlir::{self, Owned, builder, OperationExt, SingleBlockOp}, mlir::{Context, OwnedContext, DialectHandle, Builder, Value, Type}};
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
*/
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

*/
