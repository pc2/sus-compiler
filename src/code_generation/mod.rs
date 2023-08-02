use crate::ast::*;

#[derive(Debug,Clone,Copy)]
pub struct VariableRef {
    idx : usize
}

#[derive(Debug)]
pub enum Operation {
    BinaryOp{out : VariableRef, left : VariableRef, op : Operator, right : VariableRef},
    UnaryOp{out : VariableRef, op : Operator, right : VariableRef},
    //FunctionCall{TokenExtraInfo, } // Function Name
}

#[derive(Debug)]
pub struct Flattened<'ast> {
    operations : Vec<Operation>,
    variable_types : Vec<Option<&'ast SignalDeclaration>>
}

impl<'ast> Flattened<'ast> {
    /* returns the index of the output variable */
    /*fn synthesize_expression(&mut self, expr : &Expression, context : &VariableContext) -> usize {
        
    }*/
}
