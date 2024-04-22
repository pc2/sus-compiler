use crate::linker::{ConstantUUIDMarker, ModuleUUIDMarker};

use super::*;


pub fn typecheck_all_modules(linker : &mut Linker) {
    let linker_modules : *const ArenaAllocator<Module, ModuleUUIDMarker> = &linker.modules;
    for (_id, module) in &mut linker.modules {
        println!("Typechecking {}", &module.link_info.name);
        let mut context = TypeCheckingContext{
            instructions : &mut module.flattened.instructions,
            errors : &module.flattened.errors,
            linker_modules,
            linker_types : &linker.types,
            linker_constants : &linker.constants
        };
        
        context.typecheck();
        context.generative_check();
        context.find_unused_variables(&module.flattened.interface_ports);
    }
}


struct TypeCheckingContext<'l, 'instr> {
    instructions : &'instr mut FlatAlloc<Instruction, FlatIDMarker>,
    errors : &'instr ErrorCollector,

    /// A constant ptr to all modules. Accessing it requires an unsafe dereference. 
    /// 
    /// The Typechecking code cannot modify any of the fields it reads from other modules. 
    /// 
    /// It can only modify the fields it sets itself. These fields are [WireInstance::typ] and [WireInstance::is_compiletime]
    linker_modules : *const ArenaAllocator<Module, ModuleUUIDMarker>,
    linker_types : &'l ArenaAllocator<NamedType, TypeUUIDMarker>,
    linker_constants : &'l ArenaAllocator<NamedConstant, ConstantUUIDMarker>
}

impl<'l, 'instr> TypeCheckingContext<'l, 'instr> {
    /*
        ==== Typechecking ====
    */
    fn typecheck_wire_is_of_type(&self, wire : &WireInstance, expected : &Type, context : &str) {
        typecheck(&wire.typ, wire.span, expected, context, self.linker_types, &self.errors);
    }

    fn typecheck_connection(&self, to : &ConnectionWrite, from : FlatID) {
        // Typecheck digging down into write side
        let conn_root = self.instructions[to.root].extract_wire_declaration();
        let mut write_to_type = Some(&conn_root.typ);
        for p in &to.path {
            match p {
                &ConnectionWritePathElement::ArrayIdx{idx, bracket_span} => {
                    let idx_wire = self.instructions[idx].extract_wire();
                    self.typecheck_wire_is_of_type(idx_wire, &INT_TYPE, "array index");
                    if let Some(wr) = write_to_type {
                        write_to_type = typecheck_is_array_indexer(wr, bracket_span.outer_span(), self.linker_types, &self.errors);
                    }
                }
            }
        }

        // Typecheck the value with target type
        let from_wire = self.instructions[from].extract_wire();
        if let Some(target_type) = write_to_type {
            self.typecheck_wire_is_of_type(from_wire, &target_type, "connection");
        }
    }

    fn typecheck(&mut self) {
        let look_at_queue : Vec<FlatID> = self.instructions.iter().map(|(id,_)| id).collect();

        for elem_id in look_at_queue {
            match &self.instructions[elem_id] {
                Instruction::SubModule(_) => {}
                Instruction::Declaration(decl) => {
                    if let Some(latency_spec) = decl.latency_specifier {
                        let latency_spec_wire = &self.instructions[latency_spec].extract_wire();
                        self.typecheck_wire_is_of_type(latency_spec_wire, &INT_TYPE, "latency specifier");
                    }

                    decl.typ.for_each_generative_input(&mut |param_id| {
                        self.typecheck_wire_is_of_type(self.instructions[param_id].extract_wire(), &INT_TYPE, "Array size");
                    });
                }
                Instruction::IfStatement(stm) => {
                    let wire = &self.instructions[stm.condition].extract_wire();
                    self.typecheck_wire_is_of_type(wire, &BOOL_TYPE, "if statement condition")
                }
                Instruction::ForStatement(stm) => {
                    let loop_var = &self.instructions[stm.loop_var_decl].extract_wire_declaration();
                    let start = &self.instructions[stm.start].extract_wire();
                    let end = &self.instructions[stm.end].extract_wire();
                    self.typecheck_wire_is_of_type(start, &loop_var.typ, "for loop");
                    self.typecheck_wire_is_of_type(end, &loop_var.typ, "for loop");
                }
                Instruction::Wire(w) => {
                    let result_typ = match &w.source {
                        &WireSource::WireRead(from_wire) => {
                            self.instructions[from_wire].extract_wire_declaration().typ.clone()
                        }
                        &WireSource::UnaryOp{op, right} => {
                            let right_wire = self.instructions[right].extract_wire();
                            typecheck_unary_operator(op, &right_wire.typ, right_wire.span, self.linker_types, &self.errors)
                        }
                        &WireSource::BinaryOp{op, left, right} => {
                            let left_wire = self.instructions[left].extract_wire();
                            let right_wire = self.instructions[right].extract_wire();
                            let ((input_left_type, input_right_type), output_type) = get_binary_operator_types(op);
                            self.typecheck_wire_is_of_type(left_wire, &input_left_type, &format!("{op} left"));
                            self.typecheck_wire_is_of_type(right_wire, &input_right_type, &format!("{op} right"));
                            output_type
                        }
                        &WireSource::ArrayAccess{arr, arr_idx, bracket_span:_} => {
                            let arr_wire = self.instructions[arr].extract_wire();
                            let arr_idx_wire = self.instructions[arr_idx].extract_wire();
                
                            self.typecheck_wire_is_of_type(arr_idx_wire, &INT_TYPE, "array index");
                            if let Some(typ) = typecheck_is_array_indexer(&arr_wire.typ, arr_wire.span, self.linker_types, &self.errors) {
                                typ.clone()
                            } else {
                                Type::Error
                            }
                        }
                        WireSource::Constant(value) => {
                            value.get_type_of_constant()
                        }
                        &WireSource::NamedConstant(id) => {
                            let NamedConstant::Builtin{name:_, typ, val:_} = &self.linker_constants[id];
                            typ.clone()
                        }
                    };
                    let Instruction::Wire(w) = &mut self.instructions[elem_id] else {unreachable!()};
                    w.typ = result_typ;
                }
                Instruction::Write(conn) => {
                    self.typecheck_connection(&conn.to, conn.from);
                }
            }
        }

        // Post type application. Flag any remaining Type::Unknown
        for (_id, inst) in self.instructions.iter() {
            inst.for_each_embedded_type(&mut |typ, span| {
                if typ.contains_error_or_unknown::<false, true>() {
                    self.errors.error_basic(span, format!("Unresolved Type: {}", typ.to_string(self.linker_types)))
                }
            });
        }
    }

    /*
        ==== Generative Code Checking ====
    */
    fn must_be_compiletime_with_info<CtxFunc : FnOnce() -> Vec<ErrorInfo>>(&self, wire : &WireInstance, context : &str, ctx_func : CtxFunc) {
        if !wire.is_compiletime {
            self.errors.error_with_info(wire.span, format!("{context} must be compile time"), ctx_func());
        }
    }
    fn must_be_compiletime(&self, wire : &WireInstance, context : &str) {
        self.must_be_compiletime_with_info(wire, context, || Vec::new());
    }

    fn generative_check(&mut self) {
        let mut runtime_if_stack : Vec<(FlatID, Span)> = Vec::new();

        let mut declaration_depths : FlatAlloc<Option<usize>, FlatIDMarker> = self.instructions.iter().map(|_| None).collect();

        for inst_id in self.instructions.id_range() {
            while let Some((end_id, span)) = runtime_if_stack.pop() {
                if end_id != inst_id {
                    runtime_if_stack.push((end_id, span));
                    break;
                }
            }
            match &self.instructions[inst_id] {
                Instruction::SubModule(_) => {}
                Instruction::Declaration(decl) => {
                    if decl.identifier_type.is_generative() {
                        assert!(declaration_depths[inst_id].is_none());
                        declaration_depths[inst_id] = Some(runtime_if_stack.len())
                    }

                    if let Some(latency_specifier) = decl.latency_specifier {
                        self.must_be_compiletime(self.instructions[latency_specifier].extract_wire(), "Latency specifier");
                    }

                    decl.typ.for_each_generative_input(&mut |param_id| {
                        self.must_be_compiletime(self.instructions[param_id].extract_wire(), "Array size");
                    });
                }
                Instruction::Wire(wire) => {
                    let mut is_generative = true;
                    if let WireSource::WireRead(from) = &wire.source {
                        let decl = self.instructions[*from].extract_wire_declaration();
                        if !decl.identifier_type.is_generative() {
                            is_generative = false;
                        }
                    } else {
                        wire.source.for_each_input_wire(&mut |source_id| {
                            let source_wire = self.instructions[source_id].extract_wire();
                            if !source_wire.is_compiletime {
                                is_generative = false;
                            }
                        });
                    }
                    let Instruction::Wire(wire) = &mut self.instructions[inst_id] else {unreachable!()};
                    wire.is_compiletime = is_generative;
                }
                Instruction::Write(conn) => {
                    let decl = self.instructions[conn.to.root].extract_wire_declaration();

                    if decl.read_only {
                        self.errors.error_with_info(conn.to.span, "Cannot Assign to Read-Only value", vec![decl.make_declared_here(self.errors.file)]);
                    }

                    let from_wire = self.instructions[conn.from].extract_wire();
                    match conn.to.write_modifiers {
                        WriteModifiers::Connection{num_regs : _, regs_span : _} => {
                            if decl.identifier_type.is_generative() {
                                // Check that whatever's written to this declaration is also generative
                                self.must_be_compiletime_with_info(from_wire, "Assignments to generative variables", || vec![decl.make_declared_here(self.errors.file)]);

                                // Check that this declaration isn't used in a non-compiletime if
                                let declared_at_depth = declaration_depths[conn.to.root].unwrap();
            
                                if runtime_if_stack.len() > declared_at_depth {
                                    let mut infos = Vec::new();
                                    infos.push(decl.make_declared_here(self.errors.file));
                                    for (_, if_cond_span) in &runtime_if_stack[declared_at_depth..] {
                                        infos.push(error_info(*if_cond_span, self.errors.file, "Runtime Condition here"));
                                    }
                                    self.errors.error_with_info(conn.to.span, "Cannot write to generative variables in runtime conditional block", infos);
                                }
                            }
                        }
                        WriteModifiers::Initial{initial_kw_span} => {
                            if decl.identifier_type != IdentifierType::State {
                                self.errors.error_with_info(initial_kw_span, "Initial values can only be given to state registers!", vec![decl.make_declared_here(self.errors.file)])
                            }
                            self.must_be_compiletime(from_wire, "initial value assignment")
                        }
                    }
                }
                Instruction::IfStatement(if_stmt) => {
                    let condition_wire = self.instructions[if_stmt.condition].extract_wire();
                    if !condition_wire.is_compiletime {
                        runtime_if_stack.push((if_stmt.else_end, condition_wire.span));
                    }
                }
                Instruction::ForStatement(_) => {}
            }
        }
    }

    /* 
        ==== Additional Warnings ====
    */
    fn find_unused_variables(&self, interface : &InterfacePorts<FlatID>) {
        // Setup Wire Fanouts List for faster processing
        let mut instance_fanins : FlatAlloc<Vec<FlatID>, FlatIDMarker> = self.instructions.iter().map(|_| Vec::new()).collect();

        for (inst_id, inst) in self.instructions.iter() {
            match inst {
                Instruction::Write(conn) => {
                    instance_fanins[conn.to.root].push(conn.from);
                }
                Instruction::SubModule(sm) => {
                    for w in sm.interface_ports.outputs() {
                        instance_fanins[*w].push(inst_id);
                    }
                    for port in sm.interface_ports.inputs() {
                        instance_fanins[inst_id].push(*port);
                    }
                }
                Instruction::Declaration(decl) => {
                    decl.typ.for_each_generative_input(&mut |id| instance_fanins[inst_id].push(id));
                }
                Instruction::Wire(wire) => {
                    wire.source.for_each_input_wire(&mut |id| instance_fanins[inst_id].push(id));
                }
                Instruction::IfStatement(stm) => {
                    for id in UUIDRange(stm.then_start, stm.else_end) {
                        if let Instruction::Write(conn) = &self.instructions[id] {
                            instance_fanins[conn.to.root].push(stm.condition);
                        }
                    }
                }
                Instruction::ForStatement(stm) => {
                    instance_fanins[stm.loop_var_decl].push(stm.start);
                    instance_fanins[stm.loop_var_decl].push(stm.end);
                }
            }
        }

        let mut is_instance_used_map : FlatAlloc<bool, FlatIDMarker> = self.instructions.iter().map(|_| false).collect();

        let mut wire_to_explore_queue : Vec<FlatID> = Vec::new();

        for port in interface.outputs() {
            is_instance_used_map[*port] = true;
            wire_to_explore_queue.push(*port);
        }

        while let Some(item) = wire_to_explore_queue.pop() {
            for from in &instance_fanins[item] {
                if !is_instance_used_map[*from] {
                    is_instance_used_map[*from] = true;
                    wire_to_explore_queue.push(*from);
                }
            }
        }

        // Now produce warnings from the unused list
        for (id, inst) in self.instructions.iter() {
            if !is_instance_used_map[id] {
                if let Instruction::Declaration(decl) = inst {
                    if decl.is_declared_in_this_module {
                        self.errors.warn_basic(decl.name_span, "Unused Variable: This variable does not affect the output ports of this module");
                    }
                }
            }
        }
    }
}
