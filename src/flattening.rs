use std::ops::{Deref, Range};

use crate::{
    ast::{Span, Value, Module, Expression, SpanExpression, LocalOrGlobal, Operator, AssignableExpression, SpanAssignableExpression, Statement, CodeBlock, AssignableExpressionWithModifiers, TypeExpression},
    linker::{ValueUUID, Linker, Named, Linkable},
    errors::{ErrorCollector, error_info}, arena_alloc::{ListAllocator, UUID}
};

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
struct WireIDMarker;
type WireID = UUID<WireIDMarker>;

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
struct InstantiationIDMarker;
type InstantiationID = UUID<InstantiationIDMarker>;

#[derive(Debug)]
enum WireOrInstantiation {
    Wire(WireID),
    Instantiation(InstantiationID),
    Other(ValueUUID)
}

#[derive(Debug)]
struct LocalVariable {
    pub location : Span,
    pub wire_or_instance : WireOrInstantiation
}

// These are assignable connections
#[derive(Debug)]
enum ConnectionRead {
    Local(WireID),
    ArrayIdx{arr_local : WireID, idx_local : WireID},
    //StructField{struct_local : usize, field : usize},
    FuncOutput{instantiation_idx : InstantiationID, field : usize},
    Constant(Value)
}
#[derive(Debug)]
enum ConnectionWrite {
    Local(WireID),
    ArrayIdx(Box<(ConnectionWrite, WireID)>),
    //StructField(Box<(ConnectionWrite, usize)>),
    FuncInput{instantiation_idx : InstantiationID, field : usize}
}

type SpanConnectionRead = (ConnectionRead, Span);

#[derive(Debug)]
enum Instantiation {
    Named(ValueUUID),
    UnaryOp(Operator),
    BinaryOp(Operator)
}

#[derive(Debug)]
struct Connection {
    num_regs : u32,
    from : SpanConnectionRead,
    to : ConnectionWrite
}
impl Connection {
    fn new(to : ConnectionWrite, from : SpanConnectionRead) -> Connection {
        Connection{num_regs : 0, from, to}
    }
}

#[derive(Debug)]
struct Wire {
    typ : Option<TypeExpression>
}

struct FlatteningContext<'l, 'm, 'e> {
    named_locals_to_object_map : Vec<LocalVariable>,
    wires : ListAllocator<Wire, WireIDMarker>,
    instantiations : ListAllocator<Instantiation, InstantiationIDMarker>,
    connections : Vec<Connection>,

    linker : &'l Linker,
    module : &'m Module,
    errors : &'e mut ErrorCollector
}

impl<'l, 'm, 'e> FlatteningContext<'l, 'm, 'e> {
    fn new(module : &'m Module, linker : &'l Linker, errors : &'e mut ErrorCollector) -> Self {
        let mut wires = ListAllocator::new();
        let mut instantiations = ListAllocator::new();
        
        let named_locals_to_object_map = module.declarations.iter().map(|decl| {
            let decl_typ_root_reference = module.link_info.global_references[decl.typ.0.get_root()];
            let wire_or_instance = match &linker.links.globals[decl_typ_root_reference.1] {
                Named::Constant(c) => {
                    errors.error_basic(decl_typ_root_reference.0, format!("This should be the type of a declaration, but it refers to the constant '{}'", c.get_full_name(linker)));
                    WireOrInstantiation::Other(decl_typ_root_reference.1)
                }
                Named::Module(_) => {
                    match decl.typ.0 {
                        TypeExpression::Named(name_ref_idx) => {
                            let name_ref = module.link_info.global_references[name_ref_idx].1;
                            WireOrInstantiation::Instantiation(instantiations.alloc(Instantiation::Named(name_ref)))
                        }
                        TypeExpression::Array(_) => todo!(),
                    }
                }
                Named::Type(_) => {
                    WireOrInstantiation::Wire(wires.alloc(Wire{typ : Some(decl.typ.0.clone())}))
                }
            };
            LocalVariable{location : decl.span, wire_or_instance}
        }).collect();
        
        Self {
            named_locals_to_object_map,
            wires,
            instantiations,
            connections : Vec::new(),
            module,
            linker,
            errors
        }
    }
    fn convert_to_local(&mut self, (read_connection, location) : SpanConnectionRead) -> WireID {
        if let ConnectionRead::Local(l) = read_connection {
            l
        } else {
            let new_local_idx = self.wires.alloc(Wire{typ : None});
            self.connections.push(Connection::new(ConnectionWrite::Local(new_local_idx), (read_connection, location)));
            new_local_idx
        }
    }
    fn desugar_func_call(&mut self, func_and_args : &[SpanExpression], closing_bracket_pos : usize) -> Option<(&Module, InstantiationID, Range<usize>)> {
        let (name_expr, name_expr_span) = &func_and_args[0]; // Function name is always there
        let func_instantiation = match name_expr {
            Expression::Named(LocalOrGlobal::Local(l)) => {
                todo!(); // TODO explicit interface instantiation
            }
            Expression::Named(LocalOrGlobal::Global(g)) => {
                let module_uuid = self.module.link_info.global_references[*g];
                self.instantiations.alloc(Instantiation::Named(module_uuid.1))
            }
            _other => {
                self.errors.error_basic(*name_expr_span, "Function call cannot be an expression");
                return None;
            }
        };
        let Instantiation::Named(module_uuid) = self.instantiations[func_instantiation] else {panic!("Instantiation is not named!");};
        let Named::Module(md) = &self.linker.links.globals[module_uuid] else {panic!("UUID Is not module!");};
        let (input_range, output_range) = md.get_function_sugar_inputs_outputs();

        let mut args = &func_and_args[1..];

        let arg_count = args.len();
        let expected_arg_count = input_range.len();
        if arg_count != expected_arg_count {
            let module_info = vec![error_info(Span::from(md.link_info.name_token), md.link_info.file, "Interface defined here")];
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

        for (i, a) in args.iter().enumerate() {
            let func_input_field = input_range.start + i;
            if let Some(arg_read_side) = self.flatten_single_expr(a) {
                self.connections.push(Connection::new(ConnectionWrite::FuncInput{instantiation_idx: func_instantiation, field : func_input_field}, arg_read_side));
            }
        }

        Some((md, func_instantiation, output_range))
    }
    fn cast_to_wire(&mut self, local_idx : usize) -> Option<WireID> {
        match &self.named_locals_to_object_map[local_idx].wire_or_instance {
            WireOrInstantiation::Wire(w) => Some(*w),
            WireOrInstantiation::Instantiation(inst) => {
                todo!();
            }
            WireOrInstantiation::Other(value_uuid) => {
                todo!();
            }
        }
    }
    fn flatten_single_expr(&mut self, (expr, expr_span) : &SpanExpression) -> Option<SpanConnectionRead> {
        let single_connection_side = match expr {
            Expression::Named(LocalOrGlobal::Local(l)) => {
                ConnectionRead::Local(self.cast_to_wire(*l)?)
            }
            Expression::Named(LocalOrGlobal::Global(g)) => {
                let r = self.module.link_info.global_references[*g];
                let cst = self.linker.get_constant(r, self.errors)?;
                ConnectionRead::Constant(cst)
            }
            Expression::Constant(cst) => {
                ConnectionRead::Constant(cst.clone())
            }
            Expression::UnaryOp(op_box) => {
                let (op, _op_pos, operate_on) = op_box.deref();
                let flat_operate_on = self.flatten_single_expr(operate_on)?;
                let new_instantiation_idx = self.instantiations.alloc(Instantiation::UnaryOp(*op));
                let write = ConnectionWrite::FuncInput{instantiation_idx : new_instantiation_idx, field : 0};
                self.connections.push(Connection::new(write, flat_operate_on));
                ConnectionRead::FuncOutput{instantiation_idx : new_instantiation_idx, field : 1}
            }
            Expression::BinOp(binop_box) => {
                let (left, op, _op_pos, right) = binop_box.deref();
                let flat_left = self.flatten_single_expr(left)?;
                let flat_right = self.flatten_single_expr(right)?;
                let new_instantiation_idx = self.instantiations.alloc(Instantiation::BinaryOp(*op));
                let write_left = ConnectionWrite::FuncInput{instantiation_idx : new_instantiation_idx, field : 0};
                let write_right = ConnectionWrite::FuncInput{instantiation_idx : new_instantiation_idx, field : 1};
                self.connections.push(Connection::new(write_left, flat_left));
                self.connections.push(Connection::new(write_right, flat_right));
                ConnectionRead::FuncOutput{instantiation_idx : new_instantiation_idx, field : 2}
            }
            Expression::Array(arr_box) => {
                let (left, right) = arr_box.deref();
                let flat_arr = self.flatten_single_expr(left)?;
                let flat_arr_idx = self.flatten_single_expr(right)?;
                let arr_local = self.convert_to_local(flat_arr);
                let idx_local = self.convert_to_local(flat_arr_idx);
                ConnectionRead::ArrayIdx{arr_local, idx_local}
            }
            Expression::FuncCall(func_and_args) => {
                let (md, func_instance, output_range) = self.desugar_func_call(func_and_args, expr_span.1)?;

                if output_range.len() != 1 {
                    let info = error_info(md.link_info.span, md.link_info.file, "Module Defined here");
                    self.errors.error_with_info(*expr_span, "A function called in this context may only return one result. Split this function call into a separate line instead.", vec![info]);
                    return None;
                }

                ConnectionRead::FuncOutput{instantiation_idx: func_instance, field: output_range.start}
            }
        };
        Some((single_connection_side, *expr_span))
    }
    fn flatten_assignable_expr(&mut self, (expr, _span) : &SpanAssignableExpression) -> Option<ConnectionWrite> {
        match expr {
            AssignableExpression::Named{local_idx} => {
                Some(ConnectionWrite::Local(self.cast_to_wire(*local_idx)?))
            }
            AssignableExpression::ArrayIndex(arr_box) => {
                let (arr, idx) = arr_box.deref();
                let flattened_expr = self.flatten_single_expr(idx)?;
                let idx_local = self.convert_to_local(flattened_expr);

                let flattened_arr_expr = self.flatten_assignable_expr(arr)?;

                Some(ConnectionWrite::ArrayIdx(Box::new((flattened_arr_expr, idx_local))))
            }
        }
    }
    fn flatten_code(&mut self, code : &CodeBlock) {
        for (stmt, _stmt_span) in &code.statements {
            match stmt {
                Statement::Declaration{local_id} => {
                    // TODO
                }
                Statement::If { condition, then, els } => {
                    //todo!()
                }
                Statement::Assign{to, expr : (Expression::FuncCall(func_and_args), func_span), eq_sign_position} => {
                    let Some((md, instantiation_idx, output_range)) = self.desugar_func_call(&func_and_args, func_span.1) else {return;};

                    let func_name_span = func_and_args[0].1;
                    let num_func_outputs = output_range.len();
                    let num_targets = to.len();
                    let assign_list: &[AssignableExpressionWithModifiers] = if num_targets != num_func_outputs {
                        let info = vec![error_info(md.link_info.span, md.link_info.file, "Module Defined here")];
                        if num_targets > num_func_outputs {
                            let excess_results_span = Span(to[num_func_outputs].expr.1.0, to.last().unwrap().expr.1.1);
                            self.errors.error_with_info(excess_results_span, format!("Excess output targets. Function returns {num_func_outputs} results, but {num_targets} targets were given."), info);
                            &to
                        } else {
                            let too_few_targets_pos = if let Some(eq) = eq_sign_position {Span::from(*eq)} else {func_name_span};
                            self.errors.error_with_info(too_few_targets_pos, format!("Too few output targets. Function returns {num_func_outputs} results, but {num_targets} targets were given."), info);
                            &to[..num_targets]
                        }
                    } else {
                        &to
                    };

                    for (i, to_i) in assign_list.iter().enumerate() {
                        let Some(write_side) = self.flatten_assignable_expr(&to_i.expr) else {return;};
                        let field = output_range.start + i;
                        let read_side = ConnectionRead::FuncOutput{instantiation_idx, field};
                        self.connections.push(Connection{num_regs : to_i.num_regs, from: (read_side, func_name_span), to: write_side});
                    }
                },
                Statement::Assign{to, expr : non_func_expr, eq_sign_position : _} => {
                    assert!(to.len() == 1);
                    let Some(read_side) = self.flatten_single_expr(non_func_expr) else {return;};
                    let t = &to[0];
                    let Some(write_side) = self.flatten_assignable_expr(&t.expr) else {return;};
                    self.connections.push(Connection{num_regs : t.num_regs, from: read_side, to: write_side});
                },
                Statement::Block(inner_code) => {
                    self.flatten_code(inner_code);
                },
                Statement::TimelineStage(_) => {/*TODO */}
            }
        }
    }
}

pub fn flatten(module : &Module, linker : &Linker, errors : &mut ErrorCollector) -> FlattenedModule {
    let mut result = FlatteningContext::new(module, linker, errors);
    result.flatten_code(&module.code);

    FlattenedModule { wires : result.wires, instantiations: result.instantiations, connections: result.connections }
}

#[derive(Debug)]
pub struct FlattenedModule {
    wires : ListAllocator<Wire, WireIDMarker>,
    instantiations : ListAllocator<Instantiation, InstantiationIDMarker>,
    connections : Vec<Connection>
}



#[derive(Debug)]
struct InstantiatedWire {
    typ : TypeExpression,
    latency : i64
}

#[derive(Debug)]
pub struct InstantiatedModule {
    wires : ListAllocator<InstantiatedWire, WireIDMarker>,
    instantiations : ListAllocator<Instantiation, InstantiationIDMarker>,
    connections : Vec<Connection>
}

