
use crate::{ast::*, errors::ErrorCollector};


#[derive(Debug)]
pub enum Assignable {
    Named{local_idx : usize, num_regs : usize},
    Array{to : Box<Assignable>, value : LocalOrGlobal}
}

#[derive(Debug)]
pub enum Operation {
    BinaryOp{out : Assignable, left : LocalOrGlobal, op : Operator, right : LocalOrGlobal},
    UnaryOp{out : Assignable, op : Operator, right : LocalOrGlobal},
    Copy{out : Assignable, input : LocalOrGlobal},
    Constant{out : Assignable, val : Value},
    FunctionCall{results : Vec<Assignable>, func_name : LocalOrGlobal, args : Vec<LocalOrGlobal>},
    ArrayAccess{result : Assignable, array : LocalOrGlobal, args : Vec<LocalOrGlobal>}
}

#[derive(Debug)]
pub struct LocalVar {
    span : Span,
    typ : Option<SpanTypeExpression>,
    identifier_type : IdentifierType
}

#[derive(Debug)]
pub struct Flattened {
    operations : Vec<(Operation, Span)>,
    variables : Vec<LocalVar>
}

impl Flattened {
    /* returns the index of the output variable */
    fn new_local(&mut self, span : Span) -> usize {
        let new_tmp_id = self.variables.len();
        self.variables.push(LocalVar{span, typ : None, identifier_type : IdentifierType::Local});
        new_tmp_id
    }
    fn flatten_expression(&mut self, (expr, span) : &SpanExpression) -> LocalOrGlobal {
        match expr {
            Expression::Named(n) => {
               *n
            },
            Expression::BinOp(b) => {
                let (left, op, op_pos, right) = &**b;
                let left_id = self.flatten_expression(left);
                let right_id = self.flatten_expression(right);
                
                let new_idx = self.new_local(*span);

                self.operations.push((Operation::BinaryOp { out: Assignable::Named{local_idx : new_idx, num_regs : 0}, left: left_id, op: *op, right: right_id }, Span::from(*op_pos)));

                LocalOrGlobal::Local(new_idx)
            },
            Expression::UnaryOp(b) => {
                let (op, op_pos, right) = &**b;

                let right_id = self.flatten_expression(right);
                let new_idx = self.new_local(*span);

                self.operations.push((Operation::UnaryOp { out: Assignable::Named{local_idx : new_idx, num_regs : 0}, op: *op, right: right_id }, Span::from(*op_pos)));

                LocalOrGlobal::Local(new_idx)
            },
            Expression::Constant(cst) => {
                let tmp_local = self.new_local(*span);
                self.operations.push((Operation::Constant { out: Assignable::Named{local_idx : tmp_local, num_regs : 0}, val: cst.clone() }, *span));
                LocalOrGlobal::Local(tmp_local)
            },
            Expression::FuncCall(args) => {
                /*let mut arg_iter = args.iter();
                let func_name = arg_iter.next().unwrap();
                let args = arg_iter.collect();
*/
                todo!();
            },
            Expression::Array(args) => {
                todo!();
            }
        }
    }
    fn flatten_assign_to_expr(&mut self, (assign_to, _span) : &SpanAssignableExpression) -> Assignable {
        match assign_to {
            AssignableExpression::Named { local_idx, num_regs } => {
                Assignable::Named{local_idx : *local_idx, num_regs : *num_regs}
            }
            AssignableExpression::ArrayIndex(b) => {
                let (sub_assign, index_expr) = &**b;

                let sub_assignable = self.flatten_assign_to_expr(sub_assign);
                let idx_expr_result_local = self.flatten_expression(index_expr);

                Assignable::Array { to: Box::new(sub_assignable), value: idx_expr_result_local }
            }
        }
    }

    pub fn flatten(module : &Module) -> Flattened {
        let mut result = Flattened{variables : Vec::new(), operations : Vec::new()};
    
        for decl in &module.declarations {
            result.variables.push(LocalVar{span : decl.span, typ : Some(decl.typ.clone()), identifier_type : decl.identifier_type})
        }
    
        for (stmt, _stmt_span) in &module.code {
            match stmt {
                Statement::Assign(to_list, value_expr) => {
                    if let Expression::FuncCall(name_and_args) = &value_expr.0 {
                        let outputs = to_list.iter().map(
                            |t| result.flatten_assign_to_expr(t)
                        ).collect();
                        
                        let func_call_pos = name_and_args[0].1;
                        let mut args_iter = name_and_args.iter().map(
                            |a| result.flatten_expression(a)
                        );
                        let name = args_iter.next().unwrap();
                        let args = args_iter.collect();

                        result.operations.push((Operation::FunctionCall{results: outputs, func_name: name, args}, func_call_pos));
                    } else {
                        let value_span = value_expr.1;
                        let value_result = result.flatten_expression(value_expr);
                        assert!(to_list.len() == 1);
                        let output = result.flatten_assign_to_expr(&to_list[0]);

                        result.operations.push((Operation::Copy{out: output, input: value_result}, value_span));
                    };
                },
                other => {
                    todo!();
                }
            }
        }
    
        result
    }

    pub fn typecheck(&mut self, errors : &mut ErrorCollector) {

    }
}






