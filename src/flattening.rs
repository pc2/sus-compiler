use std::{ops::Deref, iter::zip};

use crate::{
    ast::{Span, Value, Module, Expression, SpanExpression, LocalOrGlobal, Operator, AssignableExpression, SpanAssignableExpression, Statement, CodeBlock, AssignableExpressionWithModifiers, IdentifierType},
    linker::{Linker, Named, Linkable, get_builtin_uuid},
    errors::{ErrorCollector, error_info}, arena_alloc::{ListAllocator, UUID}, tokenizer::kw, typing::{Type, get_unary_operator_types, get_binary_operator_types}
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
    ArrayIdx(Box<(SpanConnectionWrite, SpanWireID)>),
    StructField(Box<(SpanConnectionWrite, OutsideWireID)>)
}

pub type SpanConnectionWrite = (ConnectionWrite, Span);

#[derive(Debug)]
pub enum Instantiation {
    Instantiation{typ : Type, typ_span : Span},
    PlainWire{typ : Type, typ_span : Span},
    ExtractWire{typ : Type, extract_from : WireID, field : OutsideWireID},
    UnaryOp{typ : Type, op : Operator, right : SpanWireID},
    BinaryOp{typ : Type, op : Operator, left : SpanWireID, right : SpanWireID},
    ArrayAccess{typ : Type, arr : SpanWireID, arr_idx : SpanWireID},
    Constant{typ : Type, value : Value},
    Error
}

impl Instantiation {
    pub fn get_type(&self) -> &Type {
        match self {
            Instantiation::Instantiation{ typ, typ_span : _} => typ,
            Instantiation::PlainWire { typ, typ_span : _} => typ,
            Instantiation::ExtractWire { typ, extract_from : _, field : _} => typ,
            Instantiation::UnaryOp { typ, op : _, right : _} => typ,
            Instantiation::BinaryOp { typ, op : _, left : _, right : _} => typ,
            Instantiation::ArrayAccess { typ, arr : _, arr_idx : _} => typ,
            Instantiation::Constant { typ, value : _} => typ,
            Instantiation::Error => panic!("This was not properly resolved!")
        }
    }
}

#[derive(Debug)]
pub struct Connection {
    pub num_regs : u32,
    pub from : SpanWireID,
    pub to : SpanConnectionWrite,
    pub condition : WireID
}

struct FlatteningContext<'l, 'm, 'e> {
    instantiations : ListAllocator<Instantiation, WireIDMarker>,
    connections : Vec<Connection>,

    linker : &'l Linker,
    module : &'m Module,
    errors : &'e ErrorCollector
}

impl<'l, 'm, 'e> FlatteningContext<'l, 'm, 'e> {
    fn typecheck(&self, wire : SpanWireID, expected : &Type, context : &str) -> Option<()> {
        let found = self.instantiations[wire.0].get_type();
        if expected != found {
            let expected_name = expected.to_string(self.linker);
            let found_name = found.to_string(self.linker);
            self.errors.error_basic(wire.1, format!("Typing Error: {context} expects a {expected_name} but was given a {found_name}"));
            None
        } else {
            Some(())
        }
    }
    fn typecheck_is_array_indexer<'a>(&self, arr_type : &'a Type, span : Span) -> Option<&'a Type> {
        let Type::Array(arr_element_type) = arr_type else {
            let arr_type_name = arr_type.to_string(self.linker);
            self.errors.error_basic(span, format!("Typing Error: Attempting to index into this, but it is not of array type, instead found a {arr_type_name}"));
            return None;
        };
        Some(arr_element_type.deref())
    }
    // May also error, for example when array accesses happen on non-array types
    fn get_connectionwrite_type(&self, cw : &ConnectionWrite) -> Option<Type> {
        match cw {
            ConnectionWrite::Local(id) => Some(self.instantiations[*id].get_type().clone()),
            ConnectionWrite::ArrayIdx(arr_box) => {
                let (arr, arr_idx) = arr_box.deref();

                let index_was_int = self.typecheck(*arr_idx, &Type::Named(get_builtin_uuid("int")), "array index");

                let arr_type = self.get_connectionwrite_type(&arr.0)?;
                let arr_content_type = self.typecheck_is_array_indexer(&arr_type, arr.1)?;

                index_was_int?; // Do both for better typechecking diagnostics
                Some(arr_content_type.clone())
            },
            ConnectionWrite::StructField(struct_field_box) => {
                let ((struct_or_instance, struct_or_instance_span), OutsideWireID(outside_field)) = struct_field_box.deref();

                let ConnectionWrite::Local(id) = struct_or_instance else {todo!()};

                let Instantiation::Instantiation{typ : Type::Named(instantiation), typ_span} = &self.instantiations[*id] else {todo!()};

                let Named::Module(found) = &self.linker.links[*instantiation] else {panic!("Instantiation must be module!")};

                let field_decl = &found.declarations[*outside_field];

                if field_decl.identifier_type != IdentifierType::Input {
                    assert!(field_decl.identifier_type == IdentifierType::Output);
                    let field_decl_info = error_info(field_decl.span, found.link_info.file, "Output Defined Here");
                    self.errors.error_with_info(*struct_or_instance_span, "Cannot write to output of submodule!", vec![field_decl_info]);
                    return None;
                }

                let found_type = field_decl.typ.0.map_to_type(&found.link_info.global_references);

                Some(found_type)
            },
        }
    }
    fn create_connection(&mut self, connection : Connection) -> Option<()> {
        let expected_type = self.get_connectionwrite_type(&connection.to.0)?;

        self.typecheck(connection.from, &expected_type, "connection")?;

        self.connections.push(connection);

        Some(())
    }
    fn new(module : &'m Module, linker : &'l Linker, errors : &'e ErrorCollector) -> Self {
        let instantiations: ListAllocator<Instantiation, WireIDMarker> = module.declarations.map(&mut |_id, decl| {
            let typ = decl.typ.0.map_to_type(&module.link_info.global_references);
            let typ_span = decl.typ.1;

            let decl_typ_root_reference = typ.get_root();
            match &linker.links.globals[decl_typ_root_reference] {
                Named::Constant(c) => {
                    errors.error_basic(typ_span, format!("This should be the type of a declaration, but it refers to the constant '{}'", c.get_full_name()));
                    Instantiation::Error
                }
                Named::Module(_) => {
                    Instantiation::Instantiation{typ, typ_span}
                }
                Named::Type(_) => {
                    Instantiation::PlainWire{typ, typ_span}
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
    fn desugar_func_call(&mut self, func_and_args : &[SpanExpression], closing_bracket_pos : usize, condition : WireID) -> Option<(&Module, WireID, Vec<(OutsideWireID, Type)>)> {
        let (name_expr, name_expr_span) = &func_and_args[0]; // Function name is always there
        let func_instantiation = match name_expr {
            Expression::Named(LocalOrGlobal::Local(l)) => {
                *l
            }
            Expression::Named(LocalOrGlobal::Global(g)) => {
                let module_uuid = self.module.link_info.global_references[*g];
                self.instantiations.alloc(Instantiation::Instantiation{typ : Type::Named(module_uuid.1), typ_span : *name_expr_span})
            }
            _other => {
                self.errors.error_basic(*name_expr_span, "Function call cannot be an expression");
                return None;
            }
        };
        let Instantiation::Instantiation{typ : module_type, typ_span} = &self.instantiations[func_instantiation] else {panic!("Instantiation is not named!");};
        let Type::Named(module_id) = module_type else {todo!();};
        let Named::Module(md) = &self.linker.links.globals[*module_id] else {panic!("UUID Is not module!");};
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
            let func_input_field = &inputs[i];
            if let Some(arg_read_side) = self.flatten_single_expr(arg_expr, condition) {
                if self.typecheck(arg_read_side, &inputs[i].1, "submodule output") == None {
                    continue;
                }
                let func_instance_connectionwrite = (ConnectionWrite::Local(func_instantiation), *name_expr_span);
                let to = (ConnectionWrite::StructField(Box::new((func_instance_connectionwrite, func_input_field.0))), *name_expr_span);
                self.create_connection(Connection{num_regs: 0, to, from : arg_read_side, condition});
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
                self.instantiations.alloc(Instantiation::Constant{typ : cst.get_type(), value : cst})
            }
            Expression::Constant(cst) => {
                self.instantiations.alloc(Instantiation::Constant{typ : cst.get_type(), value : cst.clone()})
            }
            Expression::UnaryOp(op_box) => {
                let (op, _op_pos, operate_on) = op_box.deref();
                let right = self.flatten_single_expr(operate_on, condition)?;
                let (input_type, output_type) = get_unary_operator_types(*op);
                self.typecheck(right, &input_type, &format!("{op} input"))?;
                self.instantiations.alloc(Instantiation::UnaryOp{typ : output_type, op : *op, right})
            }
            Expression::BinOp(binop_box) => {
                let (left_expr, op, _op_pos, right_expr) = binop_box.deref();
                let left = self.flatten_single_expr(left_expr, condition)?;
                let right = self.flatten_single_expr(right_expr, condition)?;
                let ((input_left_type, input_right_type), output_type) = get_binary_operator_types(*op);
                self.typecheck(left, &input_left_type, &format!("{op} left"))?;
                self.typecheck(right, &input_right_type, &format!("{op} right"))?;
                self.instantiations.alloc(Instantiation::BinaryOp{typ : output_type, op : *op, left, right})
            }
            Expression::Array(arr_box) => {
                let (left, right) = arr_box.deref();
                let arr = self.flatten_single_expr(left, condition)?;
                let arr_idx = self.flatten_single_expr(right, condition)?;
                
                let index_was_int = self.typecheck(arr_idx, &Type::Named(get_builtin_uuid("int")), "array index");
                let array_type = self.instantiations[arr.0].get_type();
                let typ = self.typecheck_is_array_indexer(array_type, arr.1)?.clone();
                index_was_int?; // Do both for better typechecking diagnostics
                self.instantiations.alloc(Instantiation::ArrayAccess{typ, arr, arr_idx})
            }
            Expression::FuncCall(func_and_args) => {
                let (md, func_instance, outputs) = self.desugar_func_call(func_and_args, expr_span.1, condition)?;

                if outputs.len() != 1 {
                    let info = error_info(md.link_info.span, md.link_info.file, "Module Defined here");
                    self.errors.error_with_info(*expr_span, "A function called in this context may only return one result. Split this function call into a separate line instead.", vec![info]);
                    return None;
                }

                let only_output = outputs.into_iter().next().unwrap();

                self.instantiations.alloc(Instantiation::ExtractWire{typ: only_output.1, extract_from: func_instance, field: only_output.0})
            }
        };
        Some((single_connection_side, *expr_span))
    }
    fn flatten_assignable_expr(&mut self, (expr, span) : &SpanAssignableExpression, condition : WireID) -> Option<SpanConnectionWrite> {
        Some((match expr {
            AssignableExpression::Named{local_idx} => {
                ConnectionWrite::Local(*local_idx)
            }
            AssignableExpression::ArrayIndex(arr_box) => {
                let (arr, idx) = arr_box.deref();
                let idx_local = self.flatten_single_expr(idx, condition)?;

                let flattened_arr_expr = self.flatten_assignable_expr(arr, condition)?;

                ConnectionWrite::ArrayIdx(Box::new((flattened_arr_expr, idx_local)))
            }
        }, *span))
    }
    fn extend_condition(&mut self, condition : WireID, additional_condition : SpanWireID) -> WireID {
        if condition == WireID::INVALID {
            additional_condition.0
        } else {
            let bool_typ = Type::Named(get_builtin_uuid("bool"));
            assert!(*self.instantiations[condition].get_type() == bool_typ);
            self.instantiations.alloc(Instantiation::BinaryOp{typ : bool_typ, op: Operator{op_typ : kw("&")}, left : (condition, additional_condition.1), right : additional_condition})
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
                    let Some(if_statement_condition) = self.flatten_single_expr(condition_expr, condition) else {continue;};
                    let bool_typ = Type::Named(get_builtin_uuid("bool"));
                    if self.typecheck(if_statement_condition, &bool_typ, "if statement condition") == None {continue;}
                    let then_condition = self.extend_condition(condition, if_statement_condition);
                    self.flatten_code(then, then_condition);
                    if let Some(e) = els {
                        let else_condition_bool = (self.instantiations.alloc(Instantiation::UnaryOp{typ : bool_typ, op : Operator{op_typ : kw("!")}, right : if_statement_condition}), condition_expr.1);
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

                    for (field, to_i) in zip(outputs, assign_list) {
                        let Some(write_side) = self.flatten_assignable_expr(&to_i.expr, condition) else {return;};
                        let w = self.instantiations.alloc(Instantiation::ExtractWire{typ: field.1, extract_from: instantiation_idx, field : field.0});
                        self.create_connection(Connection{num_regs : to_i.num_regs, from: (w, func_name_span), to: write_side, condition});
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

pub fn flatten(module : &Module, linker : &Linker, errors : &ErrorCollector) -> FlattenedModule {
    let mut result = FlatteningContext::new(module, linker, errors);
    result.flatten_code(&module.code, WireID::INVALID);

    FlattenedModule{instantiations: result.instantiations, connections: result.connections}
}

#[derive(Debug)]
pub struct FlattenedModule {
    pub instantiations : ListAllocator<Instantiation, WireIDMarker>,
    pub connections : Vec<Connection>
}

