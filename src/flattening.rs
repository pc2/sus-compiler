use std::ops::Deref;

use crate::{
    ast::{Span, Value, Module, Expression, SpanExpression, LocalOrGlobal, Operator, AssignableExpression, SpanAssignableExpression, Statement, CodeBlock, AssignableExpressionWithModifiers, TypeExpression, Type, SpanType},
    linker::{NamedUUID, Linker, Named, Linkable},
    errors::{ErrorCollector, error_info}, arena_alloc::{ListAllocator, UUID}, tokenizer::kw
};

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub struct WireIDMarker;
pub type WireID = UUID<WireIDMarker>;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub struct OutsideWireID(pub WireID);

pub type SpanWireID = (WireID, Span);

// These are assignable connections
#[derive(Debug)]
pub enum ConnectionWrite {
    Local(WireID),
    ArrayIdx(Box<(ConnectionWrite, SpanWireID)>),
    StructField(Box<(ConnectionWrite, OutsideWireID)>)
}

#[derive(Debug)]
pub enum Instantiation {
    PlainWire(Option<SpanType>),
    ExtractWire{typ : Option<SpanType>, extract_from : WireID, field : OutsideWireID},
    Named(NamedUUID),
    UnaryOp(Operator, SpanWireID),
    BinaryOp(Operator, SpanWireID, SpanWireID),
    Constant(Value),
    ArrayAccess(SpanWireID, SpanWireID)
}

#[derive(Debug)]
pub struct Connection {
    pub num_regs : u32,
    pub from : SpanWireID,
    pub to : ConnectionWrite,
    pub condition : WireID
}
impl Connection {
    fn new(to : ConnectionWrite, from : SpanWireID, condition : WireID) -> Connection {
        Connection{num_regs : 0, from, to, condition}
    }
}

struct FlatteningContext<'l, 'm, 'e> {
    instantiations : ListAllocator<Instantiation, WireIDMarker>,
    connections : Vec<Connection>,

    linker : &'l Linker,
    module : &'m Module,
    errors : &'e mut ErrorCollector
}

impl<'l, 'm, 'e> FlatteningContext<'l, 'm, 'e> {
    fn new(module : &'m Module, linker : &'l Linker, errors : &'e mut ErrorCollector) -> Self {
        let instantiations: ListAllocator<Instantiation, WireIDMarker> = module.declarations.map(&mut |id, decl| {
            let decl_typ_root_reference = module.link_info.global_references[decl.typ.0.get_root()];
            match &linker.links.globals[decl_typ_root_reference.1] {
                Named::Constant(c) => {
                    errors.error_basic(decl_typ_root_reference.0, format!("This should be the type of a declaration, but it refers to the constant '{}'", c.get_full_name()));
                    panic!()
                }
                Named::Module(_) => {
                    match decl.typ.0 {
                        TypeExpression::Named(name_ref_idx) => {
                            let name_ref = module.link_info.global_references[name_ref_idx].1;
                            Instantiation::Named(name_ref)
                        }
                        TypeExpression::Array(_) => todo!(),
                    }
                }
                Named::Type(_) => {
                    Instantiation::PlainWire(Some((decl.typ.0.map_to_type(|n| module.link_info.global_references[n].1), decl.typ.1)))
                }
            }
        });
        
        Self {
            instantiations,
            connections : Vec::new(),
            module,
            linker,
            errors
        }
    }
    fn desugar_func_call(&mut self, func_and_args : &[SpanExpression], closing_bracket_pos : usize, condition : WireID) -> Option<(&Module, WireID, Vec<OutsideWireID>)> {
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
        let (inputs, output_range) = md.get_function_sugar_inputs_outputs();

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

        for (i, arg_expr) in args.iter().enumerate() {
            let func_input_field = inputs[i];
            if let Some(arg_read_side) = self.flatten_single_expr(arg_expr, condition) {
                self.connections.push(Connection::new(ConnectionWrite::StructField(Box::new((ConnectionWrite::Local(func_instantiation), func_input_field))), arg_read_side, condition));
            }
        }

        Some((md, func_instantiation, output_range))
    }
    fn flatten_single_expr(&mut self, (expr, expr_span) : &SpanExpression, condition : WireID) -> Option<SpanWireID> {
        let single_connection_side = match expr {
            Expression::Named(LocalOrGlobal::Local(l)) => {
                *l
            }
            Expression::Named(LocalOrGlobal::Global(g)) => {
                let r = self.module.link_info.global_references[*g];
                let cst = self.linker.get_constant(r, self.errors)?;
                self.instantiations.alloc(Instantiation::Constant(cst))
            }
            Expression::Constant(cst) => {
                self.instantiations.alloc(Instantiation::Constant(cst.clone()))
            }
            Expression::UnaryOp(op_box) => {
                let (op, _op_pos, operate_on) = op_box.deref();
                let flat_op_on = self.flatten_single_expr(operate_on, condition)?;
                self.instantiations.alloc(Instantiation::UnaryOp(*op, flat_op_on))
            }
            Expression::BinOp(binop_box) => {
                let (left, op, _op_pos, right) = binop_box.deref();
                let flat_left = self.flatten_single_expr(left, condition)?;
                let flat_right = self.flatten_single_expr(right, condition)?;
                self.instantiations.alloc(Instantiation::BinaryOp(*op, flat_left, flat_right))
            }
            Expression::Array(arr_box) => {
                let (left, right) = arr_box.deref();
                let flat_arr = self.flatten_single_expr(left, condition)?;
                let flat_arr_idx = self.flatten_single_expr(right, condition)?;
                self.instantiations.alloc(Instantiation::ArrayAccess(flat_arr, flat_arr_idx))
            }
            Expression::FuncCall(func_and_args) => {
                let (md, func_instance, output_range) = self.desugar_func_call(func_and_args, expr_span.1, condition)?;

                if output_range.len() != 1 {
                    let info = error_info(md.link_info.span, md.link_info.file, "Module Defined here");
                    self.errors.error_with_info(*expr_span, "A function called in this context may only return one result. Split this function call into a separate line instead.", vec![info]);
                    return None;
                }

                self.instantiations.alloc(Instantiation::ExtractWire{typ: None, extract_from: func_instance, field: output_range[0]})
            }
        };
        Some((single_connection_side, *expr_span))
    }
    fn flatten_assignable_expr(&mut self, (expr, _span) : &SpanAssignableExpression, condition : WireID) -> Option<ConnectionWrite> {
        match expr {
            AssignableExpression::Named{local_idx} => {
                Some(ConnectionWrite::Local(*local_idx))
            }
            AssignableExpression::ArrayIndex(arr_box) => {
                let (arr, idx) = arr_box.deref();
                let idx_local = self.flatten_single_expr(idx, condition)?;

                let flattened_arr_expr = self.flatten_assignable_expr(arr, condition)?;

                Some(ConnectionWrite::ArrayIdx(Box::new((flattened_arr_expr, idx_local))))
            }
        }
    }
    fn extend_condition(&mut self, condition : WireID, additional_condition : SpanWireID) -> WireID {
        if condition == WireID::INVALID {
            additional_condition.0
        } else {
            self.instantiations.alloc(Instantiation::BinaryOp(Operator{op_typ : kw("&")}, (condition, additional_condition.1), additional_condition))
        }
    }
    fn flatten_code(&mut self, code : &CodeBlock, condition : WireID) {
        for (stmt, stmt_span) in &code.statements {
            match stmt {
                Statement::Declaration(local_id) => {
                    // TODO
                }
                Statement::AssumeBound{to, bound} => {
                    // TODO
                }
                Statement::If{condition : condition_expr, then, els} => {
                    let Some(then_condition_bool) = self.flatten_single_expr(condition_expr, condition) else {continue;};
                    let then_condition = self.extend_condition(condition, then_condition_bool);
                    self.flatten_code(then, then_condition);
                    if let Some(e) = els {
                        let else_condition_bool = (self.instantiations.alloc(Instantiation::UnaryOp(Operator{op_typ : kw("!")}, then_condition_bool)), condition_expr.1);
                        let else_condition = self.extend_condition(condition, else_condition_bool);
                        self.flatten_code(e, else_condition);
                    }
                }
                Statement::Assign{to, expr : (Expression::FuncCall(func_and_args), func_span), eq_sign_position} => {
                    let Some((md, instantiation_idx, outputs)) = self.desugar_func_call(&func_and_args, func_span.1, condition) else {return;};

                    let func_name_span = func_and_args[0].1;
                    let num_func_outputs = outputs.len();
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
                        let Some(write_side) = self.flatten_assignable_expr(&to_i.expr, condition) else {return;};
                        let field = outputs[i];
                        let w = self.instantiations.alloc(Instantiation::ExtractWire{typ: None, extract_from: instantiation_idx, field});
                        self.connections.push(Connection{num_regs : to_i.num_regs, from: (w, func_name_span), to: write_side, condition});
                    }
                },
                Statement::Assign{to, expr : non_func_expr, eq_sign_position : _} => {
                    if to.len() == 1 {
                        let Some(read_side) = self.flatten_single_expr(non_func_expr, condition) else {return;};
                        let t = &to[0];
                        let Some(write_side) = self.flatten_assignable_expr(&t.expr, condition) else {return;};
                        self.connections.push(Connection{num_regs : t.num_regs, from: read_side, to: write_side, condition});
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

    pub fn get_type(&self, ) {
        
    }
    pub fn type_check(&mut self) {
        for c in &self.connections {
            
        }
    }
}

pub fn flatten(module : &Module, linker : &Linker, errors : &mut ErrorCollector) -> FlattenedModule {
    let mut result = FlatteningContext::new(module, linker, errors);
    result.flatten_code(&module.code, WireID::INVALID);

    FlattenedModule{instantiations: result.instantiations, connections: result.connections}
}

#[derive(Debug)]
pub struct FlattenedModule {
    pub instantiations : ListAllocator<Instantiation, WireIDMarker>,
    pub connections : Vec<Connection>
}

