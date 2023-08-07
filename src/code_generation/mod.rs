use crate::{ast::*, errors::{ParsingError, error_basic_str}};


pub struct GlobalContext {

}

type ToAssignable = usize;

#[derive(Debug)]
pub enum Operation {
    BinaryOp{out : ToAssignable, left : IdentifierIdx, op : Operator, right : IdentifierIdx},
    UnaryOp{out : ToAssignable, op : Operator, right : IdentifierIdx},
    Copy{out : ToAssignable, input : IdentifierIdx},
    Constant{out : ToAssignable, val : Value},
    FunctionCall{results : Vec<ToAssignable>, func_name : IdentifierIdx, args : Vec<IdentifierIdx>},
    ArrayAccess{result : ToAssignable, array : IdentifierIdx, args : Vec<IdentifierIdx>}
}

#[derive(Debug)]
pub struct LocalVar {
    span : Span,
    typ : Option<SpanExpression>,
    identifier_type : IdentifierType
}

#[derive(Debug)]
pub struct Flattened {
    operations : Vec<(Operation, usize)>,
    variables : Vec<LocalVar>
}

impl Flattened {
    /* returns the index of the output variable */
    fn new_local(&mut self, span : Span) -> usize {
        let new_tmp_id = self.variables.len();
        self.variables.push(LocalVar{span, typ : None, identifier_type : IdentifierType::Local});
        new_tmp_id
    }
    fn synthesize_expression(&mut self, (expr, span) : &SpanExpression) -> IdentifierIdx {
        match expr {
            Expression::Named(n) => {
               *n
            },
            Expression::BinOp(b) => {
                let (left, op, op_pos, right) = &**b;
                let left_id = self.synthesize_expression(left);
                let right_id = self.synthesize_expression(right);
                
                let new_idx = self.new_local(*span);

                self.operations.push((Operation::BinaryOp { out: new_idx, left: left_id, op: *op, right: right_id }, *op_pos));

                IdentifierIdx::new_local(new_idx)
            },
            Expression::UnaryOp(b) => {
                let (op, op_pos, right) = &**b;

                let right_id = self.synthesize_expression(right);
                let new_idx = self.new_local(*span);

                self.operations.push((Operation::UnaryOp { out: new_idx, op: *op, right: right_id }, *op_pos));

                IdentifierIdx::new_local(new_idx)
            },
            Expression::Constant(cst) => {
                let tmp_local = self.new_local(*span);
                self.operations.push((Operation::Constant { out: tmp_local, val: cst.clone() }, span.0));
                IdentifierIdx::new_local(tmp_local)
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
    fn synthesize_assign_to_expr(&mut self, (expr, span) : &SpanExpression, errors : &mut Vec<ParsingError<Span>>) -> Option<ToAssignable> {
        Some(match expr {
            Expression::Named(id) => {
                if let Some(local_id) = id.get_local() {
                    local_id
                } else {
                    errors.push(error_basic_str(*span, "Cannot assign to non-local variables!"));
                    return None
                }
            },
            Expression::Array(a) => {
                todo!();
            }
            other => {
                errors.push(error_basic_str(*span, "Cannot assign to this. Can only assign to variables, or array indices. (v = ..., or v[x][y]... = ..."));
                return None
            }
        })
    }
}

pub fn synthesize(module : &Module, errors : &mut Vec<ParsingError<Span>>) -> Flattened {
    let mut result = Flattened{variables : Vec::new(), operations : Vec::new()};

    for decl in &module.declarations {
        result.variables.push(LocalVar{span : decl.span, typ : Some(decl.typ.clone()), identifier_type : decl.identifier_type})
    }

    for (stmt, stmt_span) in &module.code {
        match stmt {
            Statement::Assign(to, value_expr, eq_sign_pos) => {
                if let Some(to_idx) = result.synthesize_assign_to_expr(to, errors) {
                    let value_idx = result.synthesize_expression(value_expr);
                    result.operations.push((Operation::Copy { out: to_idx, input: value_idx }, *eq_sign_pos))
                }
            },
            other => {
                todo!();
            }
        }
    }

    result
}
