use std::ops::{Deref, Range};

use crate::{
    ast::{Span, Value, Module, SpanTypeExpression, Expression, SpanStatement, SpanExpression, LocalOrGlobal, Operator, AssignableExpression, SpanAssignableExpression},
    linker::{ValueUUID, Linker, NamedValue, Named},
    errors::{ErrorCollector, error_info}
};



struct LocalVariable {
    pub location : Span,
    pub typ : Option<SpanTypeExpression>
}

// These are assignable connections
enum ConnectionRead {
    Local(usize),
    ArrayIdx{arr_local : usize, idx_local : usize},
    //StructField{struct_local : usize, field : usize},
    FuncOutput{instantiation_idx : usize, field : usize},
    Constant(Value)
}
enum ConnectionWrite {
    Local(usize),
    ArrayIdx(Box<(ConnectionWrite, usize)>),
    //StructField(Box<(ConnectionWrite, usize)>),
    FuncInput{instantiation_idx : usize, field : usize}
}

type SpanConnectionRead = (ConnectionRead, Span);

enum Instantiation {
    Named(ValueUUID),
    UnaryOp(Operator),
    BinaryOp(Operator)
}

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

struct FlatteningContext<'l, 'm, 'e> {
    locals : Vec<LocalVariable>,
    instantiations : Vec<Instantiation>,
    connections : Vec<Connection>,
    linker : &'l Linker,
    module : &'m Module,
    errors : &'e mut ErrorCollector
}

impl<'l, 'm, 'e> FlatteningContext<'l, 'm, 'e> {
    fn convert_to_local(&mut self, (read_connection, location) : SpanConnectionRead) -> usize {
        if let ConnectionRead::Local(l) = read_connection {
            l
        } else {
            let new_local_idx = self.locals.len();
            self.locals.push(LocalVariable { location, typ: None });
            self.connections.push(Connection::new(ConnectionWrite::Local(new_local_idx), (read_connection, location)));
            new_local_idx
        }
    }
    fn desugar_func_call(&mut self, func_and_args : &[SpanExpression], closing_bracket_pos : usize) -> Option<(&Module, usize, Range<usize>)> {
        let (name_expr, name_expr_span) = &func_and_args[0]; // Function name is always there
        let func_instantiation = match name_expr {
            Expression::Named(LocalOrGlobal::Local(l)) => {
                todo!(); // TODO explicit interface instantiation
            }
            Expression::Named(LocalOrGlobal::Global(g)) => {
                let instantiation_idx = self.instantiations.len();
                let module_uuid = self.module.link_info.global_references[*g];
                self.instantiations.push(Instantiation::Named(module_uuid.1));
                instantiation_idx
            }
            _other => {
                self.errors.error_basic(*name_expr_span, "Function call cannot be an expression");
                return None;
            }
        };
        let Instantiation::Named(module_uuid) = self.instantiations[func_instantiation] else {panic!("Instantiation is not named!");};
        let Named::Value(NamedValue::Module(md)) = &self.linker.links.globals[module_uuid] else {panic!("UUID Is not module!");};
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
    fn flatten_single_expr(&mut self, (expr, expr_span) : &SpanExpression) -> Option<SpanConnectionRead> {
        let single_connection_side = match expr {
            Expression::Named(LocalOrGlobal::Local(l)) => {
                ConnectionRead::Local(*l)
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
                let new_instantiation_idx = self.instantiations.len();
                self.instantiations.push(Instantiation::UnaryOp(*op));
                let write = ConnectionWrite::FuncInput{instantiation_idx : new_instantiation_idx, field : 0};
                self.connections.push(Connection::new(write, flat_operate_on));
                ConnectionRead::FuncOutput{instantiation_idx : new_instantiation_idx, field : 1}
            }
            Expression::BinOp(binop_box) => {
                let (left, op, _op_pos, right) = binop_box.deref();
                let flat_left = self.flatten_single_expr(left)?;
                let flat_right = self.flatten_single_expr(right)?;
                let new_instantiation_idx = self.instantiations.len();
                self.instantiations.push(Instantiation::BinaryOp(*op));
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
                Some(ConnectionWrite::Local(*local_idx))
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
    fn flatten_code(&mut self, code : &Vec<SpanStatement>) {
        for (stmt, _stmt_span) in code {
            match stmt {
                crate::ast::Statement::Assign{to, expr : (Expression::FuncCall(func_and_args), func_span), eq_sign_position} => {
                    let Some((md, instantiation_idx, output_range)) = self.desugar_func_call(&func_and_args, func_span.1) else {return;};

                    let func_name_span = func_and_args[0].1;
                    let num_func_outputs = output_range.len();
                    let num_targets = to.len();
                    let assign_list = if num_targets != num_func_outputs {
                        let info = vec![error_info(md.link_info.span, md.link_info.file, "Module Defined here")];
                        if num_targets > num_func_outputs {
                            let excess_results_span = Span(to[num_func_outputs].expr.1.0, to.last().unwrap().expr.1.1);
                            self.errors.error_with_info(excess_results_span, format!("Excess output targets. Function returns {num_func_outputs} results, but {num_targets} targets were given."), info);
                            to
                        } else {
                            let too_few_targets_pos = if let Some(eq) = eq_sign_position {Span::from(*eq)} else {func_name_span};
                            self.errors.error_with_info(too_few_targets_pos, format!("Too few output targets. Function returns {num_func_outputs} results, but {num_targets} targets were given."), info);
                            &to[..num_targets]
                        }
                    } else {
                        to
                    };

                    for (i, to_i) in assign_list.iter().enumerate() {
                        let Some(write_side) = self.flatten_assignable_expr(&to_i.expr) else {return;};
                        let field = output_range.start + i;
                        let read_side = ConnectionRead::FuncOutput{instantiation_idx, field};
                        self.connections.push(Connection{num_regs : to_i.num_regs, from: (read_side, func_name_span), to: write_side});
                    }
                },
                crate::ast::Statement::Assign{to, expr : non_func_expr, eq_sign_position : _} => {
                    assert!(to.len() == 1);
                    let Some(read_side) = self.flatten_single_expr(non_func_expr) else {return;};
                    let t = &to[0];
                    let Some(write_side) = self.flatten_assignable_expr(&t.expr) else {return;};
                    self.connections.push(Connection{num_regs : t.num_regs, from: read_side, to: write_side});
                },
                crate::ast::Statement::Block(inner_code) => {
                    self.flatten_code(inner_code);
                },
                crate::ast::Statement::TimelineStage(_) => {/*TODO */}
            }
        }
    }
}

pub fn flatten(module : &Module, linker : &Linker, errors : &mut ErrorCollector) -> FlattenedModule {
    let locals = module.declarations.iter().map(|decl| {
        LocalVariable{location : decl.span, typ : Some(decl.typ.clone())}
    }).collect();
    
    let mut result = FlatteningContext{
        locals,
        instantiations : Vec::new(),
        connections : Vec::new(),
        module,
        linker,
        errors
    };

    result.flatten_code(&module.code);

    FlattenedModule { locals: result.locals, instantiations: result.instantiations, connections: result.connections }
}

pub struct FlattenedModule {
    locals : Vec<LocalVariable>,
    instantiations : Vec<Instantiation>,
    connections : Vec<Connection>
}
