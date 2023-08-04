use crate::{ast::*, tokenizer::TokenExtraInfo};


pub struct GlobalContext {

}


#[derive(Debug)]
pub enum Operation {
    BinaryOp{out : usize, left : IdentifierIdx, op : Operator, right : IdentifierIdx},
    UnaryOp{out : usize, op : Operator, right : IdentifierIdx},
    Copy{out : usize, input : IdentifierIdx},
    Constant{out : usize, val : Value},
    FunctionCall(Box<(Vec<usize>, IdentifierIdx, Vec<IdentifierIdx>)>) // Function Name
}

#[derive(Debug)]
pub struct Flattened {
    operations : Vec<(Operation, usize)>,
    variables : Vec<(Span, Option<SignalDeclaration>)>,
    time_slices : Vec<usize> // indexes in self.operations. First slice starts at 0 .. time_slices[0], last slice is time_slices[time_slices.len()] .. 
}

impl Flattened {
    /* returns the index of the output variable */
    fn new_local(&mut self, span : Span) -> usize {
        let new_tmp_id = self.variables.len();
        self.variables.push((span, None));
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
            Expression::FuncCall(args) => {
                let mut arg_iter = args.iter();
                let func_name = arg_iter.next().unwrap();
                for arg in arg_iter {

                }
            }
        }
    }
}
