use std::{ops::{Deref, Range}, iter::zip};

use crate::{
    ast::{Span, Value, Module, Expression, SpanExpression, LocalOrGlobal, Operator, AssignableExpression, SpanAssignableExpression, Statement, CodeBlock, IdentifierType, GlobalReference, TypeExpression, DeclIDMarker, DeclID},
    linker::{Linker, Named, Linkable, get_builtin_uuid, FileUUID, NamedUUID},
    errors::{ErrorCollector, error_info}, arena_alloc::{ListAllocator, UUID, UUIDMarker, FlatAlloc}, tokenizer::kw, typing::{Type, typecheck_unary_operator, get_binary_operator_types, typecheck, typecheck_is_array_indexer}, block_vector::BlockVec
};

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub struct FlatIDMarker;
impl UUIDMarker for FlatIDMarker {const DISPLAY_NAME : &'static str = "obj_";}
pub type FlatID = UUID<FlatIDMarker>;

pub type SpanFlatID = (FlatID, Span);

pub type FieldID = usize;

#[derive(Debug)]
pub enum ConnectionWritePathElement {
    ArrayIdx(SpanFlatID),
    //StructField(FieldID)
}

// These are assignable connections
#[derive(Debug)]
pub struct ConnectionWrite {
    pub root : FlatID,
    pub path : Vec<ConnectionWritePathElement>,
    pub span : Span
}

impl ConnectionWrite {
    pub fn simple(root : FlatID, span : Span) -> ConnectionWrite {
        ConnectionWrite { root, path: Vec::new(), span }
    }
}

#[derive(Debug,Clone,Copy)]
pub struct InterfacePort {
    pub is_input : bool,
    pub id : FlatID
}

#[derive(Debug)]
pub enum WireSource {
    NamedWire{read_only : bool, identifier_type : IdentifierType, decl_id : Option<DeclID>},
    UnaryOp{op : Operator, right : SpanFlatID},
    BinaryOp{op : Operator, left : SpanFlatID, right : SpanFlatID},
    ArrayAccess{arr : SpanFlatID, arr_idx : SpanFlatID},
    Constant{value : Value},
}

#[derive(Debug)]
pub struct WireInstance {
    pub typ : Type,
    pub inst : WireSource
}

#[derive(Debug)]
pub enum Instantiation {
    SubModule{module_uuid : NamedUUID, name : Box<str>, typ_span : Span, interface_wires : Vec<InterfacePort>},
    Wire(WireInstance),
    Connection(Connection),
    Error
}

impl Instantiation {
    pub fn extract_wire(&self) -> &WireInstance {
        let Self::Wire(w) = self else {panic!("extract_wire on not a wire!")};
        w
    }
}

#[derive(Debug)]
pub struct Connection {
    pub num_regs : i64,
    pub from : SpanFlatID,
    pub to : ConnectionWrite,
    pub condition : FlatID
}

struct FlatteningContext<'l, 'm, 'fl> {
    decl_to_flat_map : FlatAlloc<FlatID, DeclIDMarker>,
    instantiations : &'fl ListAllocator<Instantiation, FlatIDMarker>,
    errors : &'fl ErrorCollector,

    linker : &'l Linker,
    module : &'m Module,
}

impl<'l, 'm, 'fl> FlatteningContext<'l, 'm, 'fl> {
    fn typecheck(&self, wire : SpanFlatID, expected : &Type, context : &str) -> Option<()> {
        let found = &self.instantiations[wire.0].extract_wire().typ;
        typecheck(found, wire.1, expected, context, self.linker, &self.errors)
    }
    pub fn map_to_type(&self, type_expr : &TypeExpression, global_references : &[GlobalReference]) -> Option<Type> {
        match type_expr {
            TypeExpression::Named(n) => Some(Type::Named(global_references[*n].1)),
            TypeExpression::Array(b) => {
                let (array_type_expr, array_size_expr) = b.deref();
                let array_element_type = self.map_to_type(array_type_expr, global_references)?;
                let array_size_wire = self.flatten_single_expr(array_size_expr, FlatID::INVALID)?;
                Some(Type::Array(Box::new((array_element_type, array_size_wire.0))))
            },
        }
    }
    // May also error, for example when array accesses happen on non-array types
    fn get_connectionwrite_type(&self, cw : &ConnectionWrite) -> Option<&Type> {
        let mut current_type = &self.instantiations[cw.root].extract_wire().typ;
        for p in &cw.path {
            match p {
                ConnectionWritePathElement::ArrayIdx(idx) => {
                    let index_was_int = self.typecheck(*idx, &Type::Named(get_builtin_uuid("int")), "array index");
                    current_type = typecheck_is_array_indexer(current_type, idx.1, self.linker, &self.errors)?;
                    index_was_int?;
                }
            }
        }
        Some(current_type)
    }
    fn create_connection(&self, connection : Connection) -> Option<()> {
        let expected_type = self.get_connectionwrite_type(&connection.to)?;

        self.typecheck(connection.from, &expected_type, "connection")?;

        self.instantiations.alloc(Instantiation::Connection(connection));

        Some(())
    }
    fn alloc_module_interface(&self, name : Box<str>, module : &Module, module_uuid : NamedUUID, typ_span : Span) -> Instantiation {
        let interface_wires = module.interface.interface_wires.iter().map(|port| {
            let identifier_type = if port.is_input {
                IdentifierType::Input
            } else {
                IdentifierType::Output
            };
            InterfacePort{is_input : port.is_input, id : self.instantiations.alloc(Instantiation::Wire(WireInstance{typ: port.typ.clone(), inst : WireSource::NamedWire { read_only : !port.is_input, identifier_type, decl_id : None }}))}
        }).collect();

        Instantiation::SubModule{name, module_uuid, typ_span, interface_wires}
    }
    fn desugar_func_call(&self, func_and_args : &[SpanExpression], closing_bracket_pos : usize, condition : FlatID) -> Option<(&Module, &[InterfacePort])> {
        let (name_expr, name_expr_span) = &func_and_args[0]; // Function name is always there
        let func_instantiation_id = match name_expr {
            Expression::Named(LocalOrGlobal::Local(l)) => {
                self.decl_to_flat_map[*l]
            }
            Expression::Named(LocalOrGlobal::Global(g)) => {
                let module_ref = self.module.link_info.global_references[*g];

                let dependency = self.linker.try_get_module(module_ref, &self.errors)?;
                let new_module_interface = self.alloc_module_interface(dependency.link_info.name.clone(), dependency, module_ref.1, *name_expr_span);
                self.instantiations.alloc(new_module_interface)
            }
            _other => {
                self.errors.error_basic(*name_expr_span, "Function call name cannot be an expression");
                return None;
            }
        };
        let func_instantiation = &self.instantiations[func_instantiation_id];
        let Instantiation::SubModule{module_uuid, name, typ_span, interface_wires} = func_instantiation else {unreachable!("It should be proven {func_instantiation:?} was a Module!");};
        let Named::Module(md) = &self.linker.links.globals[*module_uuid] else {unreachable!("UUID Should be a module!");};
        let (inputs, output_range) = md.interface.get_function_sugar_inputs_outputs();

        let mut args = &func_and_args[1..];

        let arg_count = args.len();
        let expected_arg_count = inputs.len();
        if arg_count != expected_arg_count {
            let module_info = vec![error_info(md.link_info.span, md.link_info.file, "Interface defined here")];
            if arg_count > expected_arg_count {
                // Too many args, complain about excess args at the end
                let excess_args_span = Span(args[expected_arg_count].1.0, closing_bracket_pos - 1);
                self.errors.error_with_info(excess_args_span, format!("Excess argument. Function takes {expected_arg_count} args, but {arg_count} were passed."), module_info);
                // Shorten args to still get proper type checking for smaller arg array
                args = &args[..expected_arg_count];
            } else {
                // Too few args, mention missing argument names
                self.errors.error_with_info(Span::from(closing_bracket_pos), format!("Too few arguments. Function takes {expected_arg_count} args, but {arg_count} were passed."), module_info);
            }
        }

        for (field, arg_expr) in zip(inputs, args) {
            if let Some(arg_read_side) = self.flatten_single_expr(arg_expr, condition) {
                if self.typecheck(arg_read_side, &md.interface.interface_wires[field].typ, "submodule output") == None {
                    continue;
                }
                let func_input_port = &interface_wires[field];
                self.create_connection(Connection { num_regs: 0, from: arg_read_side, to: ConnectionWrite::simple(func_input_port.id, *name_expr_span), condition });
            }
        }

        Some((md, &interface_wires[output_range]))
    }
    fn flatten_single_expr(&self, (expr, expr_span) : &SpanExpression, condition : FlatID) -> Option<SpanFlatID> {
        let single_connection_side = match expr {
            Expression::Named(LocalOrGlobal::Local(l)) => {
                assert!(self.decl_to_flat_map[*l] != UUID::INVALID);
                self.decl_to_flat_map[*l]
            }
            Expression::Named(LocalOrGlobal::Global(g)) => {
                let r = self.module.link_info.global_references[*g];
                let cst = self.linker.try_get_constant(r, &self.errors)?;
                self.instantiations.alloc(Instantiation::Wire(WireInstance{typ : cst.get_type(), inst : WireSource::Constant{value : cst}}))
            }
            Expression::Constant(cst) => {
                self.instantiations.alloc(Instantiation::Wire(WireInstance{typ : cst.get_type(), inst : WireSource::Constant{value : cst.clone()}}))
            }
            Expression::UnaryOp(op_box) => {
                let (op, _op_pos, operate_on) = op_box.deref();
                let right = self.flatten_single_expr(operate_on, condition)?;
                let found = &self.instantiations[right.0].extract_wire().typ;
                let output_type = typecheck_unary_operator(*op, found, right.1, self.linker, &self.errors);
                self.instantiations.alloc(Instantiation::Wire(WireInstance{typ : output_type, inst : WireSource::UnaryOp{op : *op, right}}))
            }
            Expression::BinOp(binop_box) => {
                let (left_expr, op, _op_pos, right_expr) = binop_box.deref();
                let left = self.flatten_single_expr(left_expr, condition)?;
                let right = self.flatten_single_expr(right_expr, condition)?;
                let ((input_left_type, input_right_type), output_type) = get_binary_operator_types(*op);
                self.typecheck(left, &input_left_type, &format!("{op} left"))?;
                self.typecheck(right, &input_right_type, &format!("{op} right"))?;
                self.instantiations.alloc(Instantiation::Wire(WireInstance{typ : output_type, inst : WireSource::BinaryOp{op : *op, left, right}}))
            }
            Expression::Array(arr_box) => {
                let (left, right, bracket_span) = arr_box.deref();
                let arr = self.flatten_single_expr(left, condition)?;
                let arr_idx = self.flatten_single_expr(right, condition)?;
                
                let index_was_int = self.typecheck(arr_idx, &Type::Named(get_builtin_uuid("int")), "array index");
                let array_type = &self.instantiations[arr.0].extract_wire().typ;
                let typ = typecheck_is_array_indexer(array_type, arr.1, self.linker, &self.errors)?.clone();
                index_was_int?; // Do both for better typechecking diagnostics
                self.instantiations.alloc(Instantiation::Wire(WireInstance{typ, inst : WireSource::ArrayAccess{arr, arr_idx}}))
            }
            Expression::FuncCall(func_and_args) => {
                let (md, outputs) = self.desugar_func_call(func_and_args, expr_span.1, condition)?;

                if outputs.len() != 1 {
                    let info = error_info(md.link_info.span, md.link_info.file, "Module Defined here");
                    self.errors.error_with_info(*expr_span, "A function called in this context may only return one result. Split this function call into a separate line instead.", vec![info]);
                    return None;
                }

                outputs[0].id
            }
        };
        Some((single_connection_side, *expr_span))
    }
    fn flatten_assignable_expr(&self, (expr, span) : &SpanAssignableExpression, condition : FlatID) -> Option<ConnectionWrite> {
        Some(match expr {
            AssignableExpression::Named{local_idx} => {
                let root = self.decl_to_flat_map[*local_idx];
                let WireSource::NamedWire { read_only, identifier_type : _, decl_id } = &self.instantiations[root].extract_wire().inst else {
                    unreachable!("Attempting to assign to a Instantiation::PlainWire")
                };
                if *read_only {
                    let decl_info = error_info(self.module.declarations[*local_idx].span, self.errors.file, "Declared here");
                    self.errors.error_with_info(*span, "Cannot Assign to Read-Only value", vec![decl_info]);
                    return None
                }
                ConnectionWrite{root, path : Vec::new(), span : *span}
            }
            AssignableExpression::ArrayIndex(arr_box) => {
                let (arr, idx, bracket_span) = arr_box.deref();
                let flattened_arr_expr_opt = self.flatten_assignable_expr(arr, condition);
                
                let idx_local = self.flatten_single_expr(idx, condition)?;

                let mut flattened_arr_expr = flattened_arr_expr_opt?; // only unpack the subexpr after flattening the idx, so we catch all errors

                flattened_arr_expr.path.push(ConnectionWritePathElement::ArrayIdx(idx_local));

                flattened_arr_expr
            }
        })
    }
    fn extend_condition(&self, condition : FlatID, additional_condition : SpanFlatID) -> FlatID {
        if condition == FlatID::INVALID {
            additional_condition.0
        } else {
            let bool_typ = Type::Named(get_builtin_uuid("bool"));
            assert!(self.instantiations[condition].extract_wire().typ == bool_typ);
            self.instantiations.alloc(Instantiation::Wire(WireInstance{typ : bool_typ, inst : WireSource::BinaryOp{op: Operator{op_typ : kw("&")}, left : (condition, additional_condition.1), right : additional_condition}}))
        }
    }
    fn flatten_code(&mut self, code : &CodeBlock, condition : FlatID) {
        for (stmt, stmt_span) in &code.statements {
            match stmt {
                Statement::Declaration(decl_id) => {
                    let decl = &self.module.declarations[*decl_id];

                    let Some(typ) = self.map_to_type(&decl.typ.0, &self.module.link_info.global_references) else {continue;};
                    let typ_span = decl.typ.1;

                    let decl_typ_root_reference = typ.get_root();
                    let inst = if decl_typ_root_reference == UUID::INVALID {
                        Instantiation::Error // Error's covered by linker
                    } else {
                        match &self.linker.links.globals[decl_typ_root_reference] {
                            Named::Constant(c) => {
                                self.errors.error_basic(typ_span, format!("This should be the type of a declaration, but it refers to the constant '{}'", c.get_full_name()));
                                Instantiation::Error
                            }
                            Named::Module(md) => {
                                if let Type::Named(name) = typ {
                                    self.alloc_module_interface(decl.name.clone(), md, name, typ_span)
                                } else {
                                    todo!("Implement submodule arrays");
                                    //Instantiation::Error
                                }
                            }
                            Named::Type(_) => {
                                assert!(decl.identifier_type != IdentifierType::Input);
                                assert!(decl.identifier_type != IdentifierType::Output);
                                Instantiation::Wire(WireInstance{typ, inst : WireSource::NamedWire{read_only : false, identifier_type : decl.identifier_type, decl_id : Some(*decl_id)}})
                            }
                        }
                    };

                    let wire_id = self.instantiations.alloc(inst);
                    self.decl_to_flat_map[*decl_id] = wire_id;
                }
                Statement::If{condition : condition_expr, then, els} => {
                    let Some(if_statement_condition) = self.flatten_single_expr(condition_expr, condition) else {continue;};
                    let bool_typ = Type::Named(get_builtin_uuid("bool"));
                    if self.typecheck(if_statement_condition, &bool_typ, "if statement condition") == None {continue;}
                    let then_condition = self.extend_condition(condition, if_statement_condition);
                    self.flatten_code(then, then_condition);
                    if let Some(e) = els {
                        let else_condition_bool = (self.instantiations.alloc(Instantiation::Wire(WireInstance{typ : bool_typ, inst : WireSource::UnaryOp{op : Operator{op_typ : kw("!")}, right : if_statement_condition}})), condition_expr.1);
                        let else_condition = self.extend_condition(condition, else_condition_bool);
                        self.flatten_code(e, else_condition);
                    }
                }
                Statement::Assign{to, expr : (Expression::FuncCall(func_and_args), func_span), eq_sign_position} => {
                    let Some((md, outputs)) = self.desugar_func_call(&func_and_args, func_span.1, condition) else {return;};

                    let func_name_span = func_and_args[0].1;
                    let num_func_outputs = outputs.len();
                    let num_targets = to.len();
                    if num_targets != num_func_outputs {
                        let info = vec![error_info(md.link_info.span, md.link_info.file, "Module Defined here")];
                        if num_targets > num_func_outputs {
                            let excess_results_span = Span(to[num_func_outputs].expr.1.0, to.last().unwrap().expr.1.1);
                            self.errors.error_with_info(excess_results_span, format!("Excess output targets. Function returns {num_func_outputs} results, but {num_targets} targets were given."), info);
                        } else {
                            let too_few_targets_pos = if let Some(eq) = eq_sign_position {Span::from(*eq)} else {func_name_span};
                            self.errors.error_with_info(too_few_targets_pos, format!("Too few output targets. Function returns {num_func_outputs} results, but {num_targets} targets were given."), info);
                        }
                    }

                    for (field, to_i) in zip(outputs, to) {
                        let Some(write_side) = self.flatten_assignable_expr(&to_i.expr, condition) else {return;};
                        self.create_connection(Connection{num_regs : to_i.num_regs, from: (field.id, func_name_span), to: write_side, condition});
                    }
                },
                Statement::Assign{to, expr : non_func_expr, eq_sign_position : _} => {
                    if to.len() == 1 {
                        let Some(read_side) = self.flatten_single_expr(non_func_expr, condition) else {return;};
                        let t = &to[0];
                        let Some(write_side) = self.flatten_assignable_expr(&t.expr, condition) else {return;};
                        self.create_connection(Connection{num_regs : t.num_regs, from: read_side, to: write_side, condition});
                    } else {
                        self.errors.error_basic(*stmt_span, format!("Non-function assignments must only output exactly 1 instead of {}", to.len()));
                    }
                },
                Statement::Block(inner_code) => {
                    self.flatten_code(inner_code, condition);
                },
                Statement::TimelineStage(_) => {/*TODO */}
            }
        }
    }
}

#[derive(Debug)]
pub struct FlattenedInterfacePort {
    pub wire_id : FlatID,
    pub is_input : bool,
    pub typ : Type,
    pub port_name : Box<str>,
    pub span : Span
}

#[derive(Debug, Default)]
pub struct FlattenedInterface {
    pub interface_wires : Vec<FlattenedInterfacePort>, // Indexed by FieldID
}

impl FlattenedInterface {
    pub fn new() -> Self {
        FlattenedInterface { interface_wires: Vec::new() }
    }
    pub fn get_function_sugar_inputs_outputs(&self) -> (Range<FieldID>, Range<FieldID>) {
        let mut last_output = self.interface_wires.len() - 1;
        
        while last_output > 0 {
            last_output -= 1;
            if self.interface_wires[last_output].is_input {
                last_output += 1;
                break;
            }
        }
        
        let mut last_input = last_output - 1;
        while last_input > 0 {
            last_input -= 1;
            if !self.interface_wires[last_input].is_input {
                last_input += 1;
                break;
            }
        }

        (last_input..last_output, last_output..self.interface_wires.len())
    }
}

#[derive(Debug)]
pub struct FlattenedModule {
    pub instantiations : ListAllocator<Instantiation, FlatIDMarker>,
    pub errors : ErrorCollector
}

impl FlattenedModule {
    pub fn empty(file : FileUUID) -> FlattenedModule {
        FlattenedModule {
            instantiations : ListAllocator::new(),
            errors : ErrorCollector::new(file)
        }
    }
    /* 
    Required to do this first, this only initializes the interfaces, so that proper flattening can typecheck
    Produces an initial FlattenedModule, in which the interface types have already been resolved. 
    Must be further processed by flatten, but this requires all modules to have been Initial Flattened for dependency resolution
    */
    pub fn initialize_interfaces(linker : &Linker, module : &Module) -> (FlattenedInterface, FlattenedModule, FlatAlloc<FlatID, DeclIDMarker>) {
        let mut interface = FlattenedInterface::new();
        
        let flat_mod = FlattenedModule {
            instantiations: ListAllocator::new(),
            errors: ErrorCollector::new(module.link_info.file)
        };

        let mut context = FlatteningContext{
            decl_to_flat_map: module.declarations.iter().map(|_| UUID::INVALID).collect(),
            instantiations: &flat_mod.instantiations,
            errors: &flat_mod.errors,
            linker,
            module,
        };

        for (decl_id, decl) in &module.declarations {
            let is_input = match decl.identifier_type {
                IdentifierType::Input => true,
                IdentifierType::Output => false,
                IdentifierType::Local | IdentifierType::State => continue
            };
            
            let (wire_id, typ) = if let Some(typ) = context.map_to_type(&decl.typ.0, &module.link_info.global_references) {
                let wire_id = context.instantiations.alloc(Instantiation::Wire(WireInstance{typ : typ.clone(), inst : WireSource::NamedWire{read_only: is_input, identifier_type : decl.identifier_type, decl_id : Some(decl_id)}}));
                (wire_id, typ)
            } else {
                (UUID::INVALID, Type::Named(UUID::INVALID))
            };
            interface.interface_wires.push(FlattenedInterfacePort { wire_id, is_input, typ, port_name: decl.name.clone(), span: decl.span });
            context.decl_to_flat_map[decl_id] = wire_id;
        }

        let decl_to_flat_map = context.decl_to_flat_map;
        (interface, flat_mod, decl_to_flat_map)
    }

    /*
    This method flattens all given code into a simple set of assignments, operators and submodules. 
    It already does basic type checking and assigns a type to every wire. 
    The Generating Structure of the code is not yet executed. 
    It is template-preserving
    */
    pub fn flatten(&self, module : &Module, linker : &Linker, decl_to_flat_map : FlatAlloc<FlatID, DeclIDMarker>) {
        let mut context = FlatteningContext {
            decl_to_flat_map : decl_to_flat_map,
            instantiations : &self.instantiations,
            errors : &self.errors,
            module,
            linker,
        };
        context.flatten_code(&module.code, FlatID::INVALID);
    }

    pub fn find_unused_variables(&self, md : &Module) {
        // Setup Wire Fanouts List for faster processing
        let mut connection_fanin : FlatAlloc<Vec<FlatID>, FlatIDMarker> = self.instantiations.iter().map(|_| Vec::new()).collect();

        for (_id, conn) in &self.instantiations {
            if let Instantiation::Connection(conn) = conn {
                connection_fanin[conn.to.root].push(conn.from.0);
                if conn.condition != UUID::INVALID {
                    connection_fanin[conn.to.root].push(conn.condition);
                }
            }
        }

        let mut is_instance_used_map : FlatAlloc<bool, FlatIDMarker> = self.instantiations.iter().map(|_| false).collect();

        let mut wire_to_explore_queue : Vec<FlatID> = Vec::new();

        for port in &md.interface.interface_wires {
            if !port.is_input {
                is_instance_used_map[port.wire_id] = true;
                wire_to_explore_queue.push(port.wire_id);
            }
        }

        println!("Pre Explore");
        println!("{:?}", connection_fanin);
        println!("{:?}", is_instance_used_map);
        println!("{:?}", wire_to_explore_queue);

        while let Some(item) = wire_to_explore_queue.pop() {
            let mut func = |from| {
                if !is_instance_used_map[from] {
                    is_instance_used_map[from] = true;
                    wire_to_explore_queue.push(from);
                }
            };
            match &self.instantiations[item] {
                Instantiation::Wire(wire) => {
                    match &wire.inst {
                        WireSource::NamedWire{read_only : _, identifier_type : _, decl_id : _} => {}
                        WireSource::UnaryOp{op : _, right} => {func(right.0);}
                        WireSource::BinaryOp{op : _, left, right} => {func(left.0); func(right.0);}
                        WireSource::ArrayAccess{arr, arr_idx} => {func(arr.0); func(arr_idx.0)}
                        WireSource::Constant{value : _} => {}
                    }
                }
                Instantiation::SubModule{module_uuid : _, name : _, typ_span : _, interface_wires} => {
                    for port in interface_wires {
                        if port.is_input {
                            func(port.id);
                        }
                    }
                }
                Instantiation::Connection(_) => {unreachable!()}
                Instantiation::Error => {}
            }
            for from in &connection_fanin[item] {
                if !is_instance_used_map[*from] {
                    is_instance_used_map[*from] = true;
                    wire_to_explore_queue.push(*from);
                }
            }
        }

        // Now produce warnings from the unused list
        for (id, inst) in &self.instantiations {
            if !is_instance_used_map[id] {
                if let Instantiation::Wire(WireInstance{typ : _, inst : WireSource::NamedWire { read_only : _, identifier_type : _, decl_id : Some(decl_id) }}) = inst {
                    self.errors.warn_basic(Span::from(md.declarations[*decl_id].name_token), "Unused Variable: This variable does not affect the output ports of this module");
                }
            }
        }
    }
}
