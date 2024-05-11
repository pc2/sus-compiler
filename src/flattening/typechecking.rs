
use crate::{
    file_position::SpanFile,
    linker::{ConstantUUIDMarker, Linkable, ModuleUUIDMarker},
    typing::{get_binary_operator_types, typecheck, typecheck_is_array_indexer, typecheck_unary_operator, BOOL_TYPE, INT_TYPE}
};

use super::*;


pub fn typecheck_all_modules(linker : &mut Linker) {
    let linker_ptr : *mut Linker = linker;
    for (module_uuid, module) in &mut linker.modules {
        let ctx_info_string = format!("Typechecking {}", &module.link_info.name);
        println!("{ctx_info_string}");
        let mut span_debugger = SpanDebugger::new(&ctx_info_string, &linker.files[module.link_info.file].file_text);
        
        with_module_editing_context(linker_ptr, module_uuid, |modules, types, constants, name_resolver| {
            let mut context = TypeCheckingContext{
                errors : name_resolver.errors,
                modules,
                types,
                constants
            };
            
            context.typecheck();
            context.generative_check();
            context.find_unused_variables(&module.module_ports);    
        });

        span_debugger.defuse();
    }
}


struct TypeCheckingContext<'l, 'errs> {
    modules : InternalResolver<'l, 'errs, ModuleUUIDMarker, Module>,
    types : Resolver<'l, 'errs, TypeUUIDMarker, NamedType>,
    constants : Resolver<'l, 'errs, ConstantUUIDMarker, NamedConstant>,
    errors : &'errs ErrorCollector,
}

impl<'l, 'errs> Deref for TypeCheckingContext<'l, 'errs> {
    type Target = InternalResolver<'l, 'errs, ModuleUUIDMarker, Module>;

    fn deref(&self) -> &Self::Target {
        &self.modules
    }
}
impl<'l, 'errs> DerefMut for TypeCheckingContext<'l, 'errs> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.modules
    }
}

impl<'l, 'errs> TypeCheckingContext<'l, 'errs> {
    fn get_decl_of_module_port<'s>(&'s self, port : PortInfo) -> (&'s Declaration, FileUUID) {
        let submodule_id = self.working_on.instructions[port.submodule_flat].unwrap_submodule().module_uuid;
        let module = &self.modules[submodule_id];
        let decl = module.get_port_decl(port.port);
        (decl, module.link_info.file)
    }
    

    /*
        ==== Typechecking ====
    */
    fn typecheck_wire_is_of_type(&self, wire : &WireInstance, expected : &AbstractType, typ_decl_location : Option<SpanFile>, context : &str) {
        typecheck(&wire.typ, wire.span, expected, context, &self.types, typ_decl_location, &self.errors);
    }

    fn get_wire_ref_declaration_point(&self, wire_ref_root : &WireReferenceRoot) -> Option<SpanFile> {
        match wire_ref_root {
            WireReferenceRoot::LocalDecl(id) => {
                let decl_root = self.working_on.instructions[*id].unwrap_wire_declaration();
                Some((decl_root.get_span(), self.errors.file))
            },
            WireReferenceRoot::NamedConstant(cst) => {
                let linker_cst = &self.constants[*cst];
                linker_cst.get_span_file()
            }
            WireReferenceRoot::SubModulePort(port) => {
                let (decl, file) = self.get_decl_of_module_port(*port);
                Some((decl.typ_expr.get_span(), file))
            }
        }
    }

    fn get_type_of_wire_reference(&self, wire_ref : &WireReference) -> AbstractType {
        let mut write_to_type = match &wire_ref.root {
            WireReferenceRoot::LocalDecl(id) => {
                let decl_root = self.working_on.instructions[*id].unwrap_wire_declaration();
                decl_root.typ.clone()
            },
            WireReferenceRoot::NamedConstant(cst) => {
                let linker_cst = &self.constants[*cst];
                linker_cst.get_abstract_type()
            }
            WireReferenceRoot::SubModulePort(port) => {
                let (decl, _file) = self.get_decl_of_module_port(*port);
                decl.typ_expr.to_type()
            }
        };

        for p in &wire_ref.path {
            match p {
                &WireReferencePathElement::ArrayIdx{idx, bracket_span} => {
                    let idx_wire = self.working_on.instructions[idx].unwrap_wire();
                    self.typecheck_wire_is_of_type(idx_wire, &INT_TYPE, None, "array index");
                    write_to_type = typecheck_is_array_indexer(&write_to_type, bracket_span.outer_span(), &self.types, &self.errors).clone();
                }
            }
        }

        write_to_type
    }

    fn typecheck(&mut self) {
        let look_at_queue : Vec<FlatID> = self.working_on.instructions.iter().map(|(id,_)| id).collect();

        for elem_id in look_at_queue {
            match &self.working_on.instructions[elem_id] {
                Instruction::SubModule(_) => {}
                Instruction::Declaration(decl) => {
                    if let Some(latency_spec) = decl.latency_specifier {
                        let latency_spec_wire = &self.working_on.instructions[latency_spec].unwrap_wire();
                        self.typecheck_wire_is_of_type(latency_spec_wire, &INT_TYPE, None, "latency specifier");
                    }

                    decl.typ_expr.for_each_generative_input(&mut |param_id| {
                        self.typecheck_wire_is_of_type(self.working_on.instructions[param_id].unwrap_wire(), &INT_TYPE, None, "Array size");
                    });
                }
                Instruction::IfStatement(stm) => {
                    let wire = &self.working_on.instructions[stm.condition].unwrap_wire();
                    self.typecheck_wire_is_of_type(wire, &BOOL_TYPE, None, "if statement condition")
                }
                Instruction::ForStatement(stm) => {
                    let loop_var = &self.working_on.instructions[stm.loop_var_decl].unwrap_wire_declaration();
                    let start = &self.working_on.instructions[stm.start].unwrap_wire();
                    let end = &self.working_on.instructions[stm.end].unwrap_wire();
                    let loop_var_decl_span = Some((loop_var.get_span(), self.errors.file));
                    self.typecheck_wire_is_of_type(start, &loop_var.typ, loop_var_decl_span, "for loop");
                    self.typecheck_wire_is_of_type(end, &loop_var.typ, loop_var_decl_span, "for loop");
                }
                Instruction::Wire(w) => {
                    let result_typ = match &w.source {
                        WireSource::WireRead(from_wire) => {
                            self.get_type_of_wire_reference(from_wire)
                        }
                        &WireSource::UnaryOp{op, right} => {
                            let right_wire = self.working_on.instructions[right].unwrap_wire();
                            typecheck_unary_operator(op, &right_wire.typ, right_wire.span, &self.types, &self.errors)
                        }
                        &WireSource::BinaryOp{op, left, right} => {
                            let left_wire = self.working_on.instructions[left].unwrap_wire();
                            let right_wire = self.working_on.instructions[right].unwrap_wire();
                            let ((input_left_type, input_right_type), output_type) = get_binary_operator_types(op);
                            self.typecheck_wire_is_of_type(left_wire, &input_left_type, None, &format!("{op} left"));
                            self.typecheck_wire_is_of_type(right_wire, &input_right_type, None, &format!("{op} right"));
                            output_type
                        }
                        WireSource::Constant(value) => {
                            value.get_type_of_constant()
                        }
                    };
                    let Instruction::Wire(w) = &mut self.working_on.instructions[elem_id] else {unreachable!()};
                    w.typ = result_typ;
                }
                Instruction::Write(conn) => {
                    // Typecheck digging down into write side
                    let write_to_type = self.get_type_of_wire_reference(&conn.to);
                    let declared_here = self.get_wire_ref_declaration_point(&conn.to.root);

                    // Typecheck the value with target type
                    let from_wire = self.working_on.instructions[conn.from].unwrap_wire();

                    self.typecheck_wire_is_of_type(from_wire, &write_to_type, declared_here, "connection");
                }
            }
        }

        // Post type application. Flag any remaining Type::Unknown
        for (_id, inst) in self.working_on.instructions.iter() {
            inst.for_each_embedded_type(&mut |typ, span| {
                if typ.contains_error_or_unknown::<false, true>() {
                    self.errors.error_basic(span, format!("Unresolved Type: {}", typ.to_string(&self.types)))
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

    fn get_root_identifier_type(&self, wire_ref_root : &WireReferenceRoot) -> IdentifierType {
        match wire_ref_root {
            WireReferenceRoot::LocalDecl(decl_id) => {
                let decl = self.working_on.instructions[*decl_id].unwrap_wire_declaration();
                decl.identifier_type
            }
            WireReferenceRoot::NamedConstant(_) => {
                IdentifierType::Generative
            }
            WireReferenceRoot::SubModulePort(port) => {
                let (decl, _file) = self.get_decl_of_module_port(*port);
                decl.identifier_type
            }
        }
    }

    fn get_root_identifier_read_only(&self, wire_ref_root : &WireReferenceRoot) -> bool {
        match wire_ref_root {
            WireReferenceRoot::LocalDecl(decl_id) => {
                let decl = self.working_on.instructions[*decl_id].unwrap_wire_declaration();
                decl.read_only
            }
            WireReferenceRoot::NamedConstant(_) => {
                true
            }
            WireReferenceRoot::SubModulePort(port) => {
                let (decl, _file) = self.get_decl_of_module_port(*port);
                decl.identifier_type == IdentifierType::Output
            }
        }
    }

    fn generative_check(&mut self) {
        let mut runtime_if_stack : Vec<(FlatID, Span)> = Vec::new();

        let mut declaration_depths : FlatAlloc<Option<usize>, FlatIDMarker> = self.working_on.instructions.iter().map(|_| None).collect();

        for inst_id in self.working_on.instructions.id_range() {
            while let Some((end_id, span)) = runtime_if_stack.pop() {
                if end_id != inst_id {
                    runtime_if_stack.push((end_id, span));
                    break;
                }
            }
            match &self.working_on.instructions[inst_id] {
                Instruction::SubModule(_) => {}
                Instruction::Declaration(decl) => {
                    if decl.identifier_type.is_generative() {
                        assert!(declaration_depths[inst_id].is_none());
                        declaration_depths[inst_id] = Some(runtime_if_stack.len())
                    }

                    if let Some(latency_specifier) = decl.latency_specifier {
                        self.must_be_compiletime(self.working_on.instructions[latency_specifier].unwrap_wire(), "Latency specifier");
                    }

                    decl.typ_expr.for_each_generative_input(&mut |param_id| {
                        self.must_be_compiletime(self.working_on.instructions[param_id].unwrap_wire(), "Array size");
                    });
                }
                Instruction::Wire(wire) => {
                    let mut is_generative = true;
                    if let WireSource::WireRead(from) = &wire.source {
                        is_generative = self.get_root_identifier_type(&from.root) == IdentifierType::Generative;
                    } else {
                        wire.source.for_each_dependency(&mut |source_id| {
                            match &self.working_on.instructions[source_id] {
                                Instruction::SubModule(_sm) => {
                                    is_generative = false; // TODO generative submodules
                                }
                                Instruction::Wire(source_wire) => {
                                    if !source_wire.is_compiletime {
                                        is_generative = false;
                                    }
                                }
                                _other => unreachable!()
                            }
                        });
                    }
                    let Instruction::Wire(wire) = &mut self.working_on.instructions[inst_id] else {unreachable!()};
                    wire.is_compiletime = is_generative;
                }
                Instruction::Write(conn) => self.generative_check_write(conn, &mut declaration_depths, &mut runtime_if_stack),
                Instruction::IfStatement(if_stmt) => {
                    let condition_wire = self.working_on.instructions[if_stmt.condition].unwrap_wire();
                    if !condition_wire.is_compiletime {
                        runtime_if_stack.push((if_stmt.else_end, condition_wire.span));
                    }
                }
                Instruction::ForStatement(_) => {}
            }
        }
    }

    fn generative_check_write(&self, conn: &Write, declaration_depths: &mut FlatAlloc<Option<usize>, FlatIDMarker>, runtime_if_stack: &mut Vec<(UUID<FlatIDMarker>, Span)>) {
        let (decl, file) = match conn.to.root {
            WireReferenceRoot::LocalDecl(decl_id) => {
                let decl = self.working_on.instructions[decl_id].unwrap_wire_declaration();
                (decl, self.errors.file)
            }
            WireReferenceRoot::NamedConstant(_) => {
                self.errors.error_with_info(conn.to.root_span, "Cannot assign to a global", vec![]);
                return;
            }
            WireReferenceRoot::SubModulePort(port) => {
                self.get_decl_of_module_port(port)
            }
        };
    
        if self.get_root_identifier_read_only(&conn.to.root) {
            self.errors.error_with_info(conn.to.span, "Cannot Assign to Read-Only value", vec![decl.make_declared_here(file)]);
        }
    
        let from_wire = self.working_on.instructions[conn.from].unwrap_wire();
        match conn.write_modifiers {
            WriteModifiers::Connection{num_regs : _, regs_span : _} => {
                if decl.identifier_type.is_generative() {
                    // Check that whatever's written to this declaration is also generative
                    self.must_be_compiletime_with_info(from_wire, "Assignments to generative variables", || vec![decl.make_declared_here(file)]);
    
                    // Check that this declaration isn't used in a non-compiletime if
                    if let Some(root_flat) = conn.to.root.get_root_flat() {
                        let declared_at_depth = declaration_depths[root_flat].unwrap();
                
                        if runtime_if_stack.len() > declared_at_depth {
                            let mut infos = Vec::new();
                            infos.push(decl.make_declared_here(file));
                            for (_, if_cond_span) in &runtime_if_stack[declared_at_depth..] {
                                infos.push(error_info(*if_cond_span, file, "Runtime Condition here"));
                            }
                            self.errors.error_with_info(conn.to.span, "Cannot write to generative variables in runtime conditional block", infos);
                        }
                    }
                }
            }
            WriteModifiers::Initial{initial_kw_span} => {
                if decl.identifier_type != IdentifierType::State {
                    self.errors.error_with_info(initial_kw_span, "Initial values can only be given to state registers!", vec![decl.make_declared_here(file)])
                }
                self.must_be_compiletime(from_wire, "initial value assignment")
            }
        }
    }
    
    /* 
        ==== Additional Warnings ====
    */
    fn find_unused_variables(&self, ports : &ModulePorts) {
        // Setup Wire Fanouts List for faster processing
        let mut instance_fanins : FlatAlloc<Vec<FlatID>, FlatIDMarker> = self.working_on.instructions.iter().map(|_| Vec::new()).collect();

        for (inst_id, inst) in self.working_on.instructions.iter() {
            match inst {
                Instruction::Write(conn) => {
                    if let Some(flat_root) = conn.to.root.get_root_flat() {
                        instance_fanins[flat_root].push(conn.from);
                    }
                }
                Instruction::SubModule(_) => {} // TODO Dependencies should be added here if for example generative templates get added
                Instruction::Declaration(decl) => {
                    decl.typ_expr.for_each_generative_input(&mut |id| instance_fanins[inst_id].push(id));
                }
                Instruction::Wire(wire) => {
                    wire.source.for_each_dependency(&mut |id| instance_fanins[inst_id].push(id));
                }
                Instruction::IfStatement(stm) => {
                    for id in UUIDRange(stm.then_start, stm.else_end) {
                        if let Instruction::Write(conn) = &self.working_on.instructions[id] {
                            if let Some(flat_root) = conn.to.root.get_root_flat() {
                                instance_fanins[flat_root].push(stm.condition);
                            }
                        }
                    }
                }
                Instruction::ForStatement(stm) => {
                    instance_fanins[stm.loop_var_decl].push(stm.start);
                    instance_fanins[stm.loop_var_decl].push(stm.end);
                }
            }
        }

        let mut is_instance_used_map : FlatAlloc<bool, FlatIDMarker> = self.working_on.instructions.iter().map(|_| false).collect();

        let mut wire_to_explore_queue : Vec<FlatID> = Vec::new();

        for (_id, port) in &ports.ports {
            if port.id_typ == IdentifierType::Output {
                is_instance_used_map[port.declaration_instruction] = true;
                wire_to_explore_queue.push(port.declaration_instruction);

            }
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
        for (id, inst) in self.working_on.instructions.iter() {
            if !is_instance_used_map[id] {
                if let Instruction::Declaration(decl) = inst {
                    self.errors.warn_basic(decl.name_span, "Unused Variable: This variable does not affect the output ports of this module");
                }
            }
        }
    }
}
