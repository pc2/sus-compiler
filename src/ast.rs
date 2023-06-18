
use crate::tokenizer::{Token,TokenTypeIdx};

#[derive(Clone,Copy,Debug)]
pub struct Span(pub usize, pub usize);

impl From<usize> for Span {
    fn from(v : usize) -> Span {
        Span(v, v)
    }
}

#[derive(Debug,Clone,Copy)]
pub enum IdentifierType {
    Input,
    Output,
    Local,
    State
}

#[derive(Debug)]
pub struct SignalDeclaration {
    pub span : Span,
    pub typ : SpanExpression,
    pub name_token : usize,
    pub identifier_type : IdentifierType
}

#[derive(Debug)]
pub struct Interface {
    pub span : Span,
    pub inputs : Vec<SignalDeclaration>,
    pub outputs : Vec<SignalDeclaration>
}

#[derive(Debug)]
pub enum Expression {
    Named(usize),
    Constant(usize),
    BinOp(Box<(SpanExpression, TokenTypeIdx, usize/*Operator token */, SpanExpression)>)
}
pub type SpanExpression = (Expression, Span);
pub type SpanStatement = (Statement, Span);

#[derive(Debug)]
pub enum Statement {
    Declare(SignalDeclaration), // type v;
    DeclareAssign(SignalDeclaration, SpanExpression), // type v = expr;
    Assign(SpanExpression, SpanExpression), // v = expr;
    Mention(SpanExpression),
    Block(Vec<SpanStatement>)
}

#[derive(Debug)]
pub struct Module {
    pub span : Span,
    pub name : usize,
    pub interface : Interface,
    pub code : Vec<SpanStatement>
}

#[derive(Debug)]
pub struct ASTRoot {
    pub modules : Vec<Module>
}

pub struct VariableContext<'prev, 'ast> where 'prev : 'ast {
    pub locals : Vec<(&'ast str, &'ast SignalDeclaration)>,
    pub prev : Option<&'prev VariableContext<'prev, 'ast>>
}

impl<'prev, 'ast> VariableContext<'prev, 'ast> {
    pub fn get_declaration_for_str(&self, name : &str) -> Option<&'ast SignalDeclaration> {
        for (n, decl) in &self.locals {
            if **n == *name {
                return Some(decl);
            }
        }
        self.prev?.get_declaration_for_str(name)
    }
    pub fn get_declaration_for(&self, tok_idx : usize, tokens : &[Token]) -> Option<&'ast SignalDeclaration> {
        self.get_declaration_for_str(tokens[tok_idx].text)
    }
    pub fn add_declaration(&mut self, new_local : &'ast SignalDeclaration, tokens : &[Token<'ast>]) {
        let name = tokens[new_local.name_token].text;
        self.locals.push((name, new_local));
    }
    pub fn new_initial() -> Self {
        Self{locals : Vec::new(), prev : None}
    }
    pub fn new_extend(prev : &'prev Self) -> Self {
        Self{locals : Vec::new(), prev : Some(prev)}
    }
}

pub fn for_each_identifier_in_expression<F>(expr : &Expression, func : &mut F) where F: FnMut(usize) -> () {
    match expr {
        Expression::Named(token) => func(*token),
        Expression::Constant(_) => {},
        Expression::BinOp(b) => {
            let (left, _operator, _operator_pos, right) = &**b;
            for_each_identifier_in_expression(&left.0, func);
            for_each_identifier_in_expression(&right.0, func);
        },
    }
}

pub trait ASTWalker {
    fn visit_module_name(&mut self, module_name : usize) {}
    fn visit_declaration(&mut self, decl : &SignalDeclaration, context : &VariableContext) {}
    fn visit_expression(&mut self, expr : &SpanExpression, context : &VariableContext) {}
    fn visit_assignment(&mut self, to : &SpanExpression, expr : &SpanExpression, context : &VariableContext) {}
}

fn walk_ast_code_block<W : ASTWalker>(walker : &mut W, code_block : &[SpanStatement], tokens : &[Token], outer_context : &VariableContext) {
    let mut local_context = VariableContext::new_extend(outer_context);
    for statement in code_block {
        match &statement.0 {
            Statement::Declare(decl) => {
                local_context.add_declaration(decl, tokens);
                walker.visit_declaration(&decl, &local_context);
            },
            Statement::DeclareAssign(decl, expr) => {
                local_context.add_declaration(decl, tokens);
                walker.visit_declaration(decl, &local_context);
                let tok = decl.name_token;
                let tmp_local_expr = (Expression::Named(tok), Span::from(tok));
                walker.visit_assignment(expr, &tmp_local_expr, &local_context);
                walker.visit_expression(expr, &local_context);
                walker.visit_expression(&tmp_local_expr, &local_context);
            },
            Statement::Assign(to, expr) => {
                walker.visit_expression(to, &local_context);
                walker.visit_expression(expr, &local_context);
                walker.visit_assignment(to, expr, &local_context);
            },
            Statement::Mention(expr) => {
                walker.visit_expression(expr, &local_context);
            }
            Statement::Block(code) => {
                walk_ast_code_block(walker, &code, tokens, &local_context);
            },
        }
    }
}

pub fn walk_ast<W : ASTWalker>(walker : &mut W, ast : &ASTRoot, tokens : &[Token], global_context : &VariableContext) {
    for module in &ast.modules {
        walker.visit_module_name(module.name);
        let mut local_context = VariableContext::new_extend(global_context);
        for decl in &module.interface.inputs {
            walker.visit_declaration(decl, &local_context);
            local_context.add_declaration(decl, tokens);
        }
        for decl in &module.interface.outputs {
            walker.visit_declaration(decl, &local_context);
            local_context.add_declaration(decl, tokens);
        }

        walk_ast_code_block(walker, &module.code, tokens, &local_context);
    }
}
