
use crate::tokenizer::{TokenTypeIdx, TokenExtraInfo};
use core::ops::Range;

// Token span. Indices are INCLUSIVE
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Span(pub usize, pub usize);
impl Span {
    pub fn as_range(&self) -> Range<usize> {
        self.0..self.1+1
    }
    pub fn len(&self) -> usize {
        self.1-self.0+1
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct FilePos {
    pub char_idx : usize,
    pub row : usize,
    pub col : usize
}

// Char span, for chars in file. start is INCLUSIVE, end is EXCLUSIVE. It's a bit weird to make the distinction, but it works out
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct CharSpan{
    pub file_pos : FilePos,
    pub length : usize
}



pub fn cvt_span_to_char_span(sp : Span, char_sp_buf : &[CharSpan]) -> CharSpan {
    let file_pos = char_sp_buf[if sp.0 < char_sp_buf.len() {sp.0} else {char_sp_buf.len()-1}].file_pos;
    let length = char_sp_buf[if sp.1 < char_sp_buf.len() {sp.1} else {char_sp_buf.len()-1}].end_pos() - file_pos.char_idx;

    CharSpan{file_pos, length}
}

impl CharSpan {
    pub fn as_range(&self) -> Range<usize> {
        self.file_pos.char_idx..self.file_pos.char_idx+self.length
    }
    pub fn end_pos(&self) -> usize {
        self.file_pos.char_idx + self.length
    }
}

impl From<Span> for Range<usize> {
    fn from(sp : Span) -> Self {
        sp.as_range()
    }
}
impl From<CharSpan> for Range<usize> {
    fn from(sp : CharSpan) -> Self {
        sp.as_range()
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum IdentifierType {
    Input,
    Output,
    Local,
    State
}

impl From<usize> for Span {
    fn from(v : usize) -> Span {
        Span(v, v)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IdentifierToken {
    pub position : usize,
    pub name_idx : TokenExtraInfo
}

#[derive(Debug)]
pub struct SignalDeclaration {
    pub span : Span,
    pub typ : SpanExpression,
    pub name : IdentifierToken,
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
    Named(TokenExtraInfo),
    Constant(TokenExtraInfo),
    BoolConstant(bool),
    UnaryOp(Box<(TokenTypeIdx, usize/*Operator token */, SpanExpression)>),
    BinOp(Box<(SpanExpression, TokenTypeIdx, usize/*Operator token */, SpanExpression)>),
    Array(Vec<SpanExpression>), // first[second, third, ...]
    FuncCall(Vec<SpanExpression>) // first(second, third, ...)
}

impl Expression {
    pub fn new_binop(left : SpanExpression, op : TokenTypeIdx, op_pos : usize/*Operator token */, right : SpanExpression) -> SpanExpression {
        let span = Span(left.1.0, right.1.1);
        (Expression::BinOp(Box::new((left, op, op_pos, right))), span)
    }
}
pub type SpanExpression = (Expression, Span);
pub type SpanStatement = (Statement, Span);

#[derive(Debug)]
pub enum Statement {
    Declare(SignalDeclaration), // type v;
    DeclareAssign(SignalDeclaration, SpanExpression), // type v = expr;
    Assign(SpanExpression, SpanExpression), // v = expr;
    Mention(SpanExpression),
    Block(Vec<SpanStatement>),
    PipelineStage(usize),
    TimelineStage(usize)
}

#[derive(Debug)]
pub struct Module {
    pub span : Span,
    pub name : IdentifierToken,
    pub interface : Interface,
    pub code : Vec<SpanStatement>
}

#[derive(Debug)]
pub struct ASTRoot {
    pub modules : Vec<Module>
}

pub struct VariableContext<'prev, 'ast> where 'prev : 'ast {
    locals : Vec<&'ast SignalDeclaration>,
    prev : Option<&'prev VariableContext<'prev, 'ast>>
}

impl<'prev, 'ast> VariableContext<'prev, 'ast> {
    pub fn get_declaration_for(&self, name : TokenExtraInfo) -> Option<&'ast SignalDeclaration> {
        for decl in &self.locals {
            if decl.name.name_idx == name {
                return Some(decl);
            }
        }
        self.prev?.get_declaration_for(name)
    }
    pub fn add_declaration(&mut self, new_local : &'ast SignalDeclaration) {
        self.locals.push(new_local);
    }
    pub fn new_initial() -> Self {
        Self{locals : Vec::new(), prev : None}
    }
    pub fn new_extend(prev : &'prev Self) -> Self {
        Self{locals : Vec::new(), prev : Some(prev)}
    }
}

pub fn for_each_identifier_in_expression<F>((expr, span) : &SpanExpression, func : &mut F) where F: FnMut(IdentifierToken) -> () {
    match expr {
        Expression::Named(id) => {
            assert!(span.0 == span.1);
            func(IdentifierToken{name_idx : *id, position : span.0})
        },
        Expression::BoolConstant(_v) => {},
        Expression::Constant(_v) => {},
        Expression::UnaryOp(b) => {
            let (_operator, _operator_pos, right) = &**b;
            for_each_identifier_in_expression(&right, func);
        }
        Expression::BinOp(b) => {
            let (left, _operator, _operator_pos, right) = &**b;
            for_each_identifier_in_expression(&left, func);
            for_each_identifier_in_expression(&right, func);
        },
        Expression::Array(args) | Expression::FuncCall(args) => {
            for arg in args {
                for_each_identifier_in_expression(arg, func);
            }
        }
    }
}

#[allow(unused_variables)]
pub trait ASTWalker {
    fn visit_module_name(&mut self, module_name : IdentifierToken) {}
    fn visit_declaration(&mut self, decl : &SignalDeclaration, context : &VariableContext) {}
    fn visit_expression(&mut self, expr : &SpanExpression, context : &VariableContext) {}
    fn visit_assignment(&mut self, to : &SpanExpression, expr : &SpanExpression, context : &VariableContext) {}
}

fn walk_ast_code_block<W : ASTWalker>(walker : &mut W, code_block : &[SpanStatement], outer_context : &VariableContext) {
    let mut local_context = VariableContext::new_extend(outer_context);
    for statement in code_block {
        match &statement.0 {
            Statement::Declare(decl) => {
                local_context.add_declaration(decl);
                walker.visit_declaration(&decl, &local_context);
            }
            Statement::DeclareAssign(decl, expr) => {
                local_context.add_declaration(decl);
                walker.visit_declaration(decl, &local_context);
                let tmp_local_expr = (Expression::Named(decl.name.name_idx), Span::from(decl.name.position));
                walker.visit_assignment(expr, &tmp_local_expr, &local_context);
                walker.visit_expression(expr, &local_context);
                walker.visit_expression(&tmp_local_expr, &local_context);
            }
            Statement::Assign(to, expr) => {
                walker.visit_expression(to, &local_context);
                walker.visit_expression(expr, &local_context);
                walker.visit_assignment(to, expr, &local_context);
            }
            Statement::Mention(expr) => {
                walker.visit_expression(expr, &local_context);
            }
            Statement::Block(code) => {
                walk_ast_code_block(walker, &code, &local_context);
            }
            Statement::PipelineStage(_pos) => {
                
            }
            Statement::TimelineStage(_pos) => {
                
            }
        }
    }
}

pub fn walk_ast<W : ASTWalker>(walker : &mut W, ast : &ASTRoot, global_context : &VariableContext) {
    for module in &ast.modules {
        walker.visit_module_name(module.name);
        let mut local_context = VariableContext::new_extend(global_context);
        for decl in &module.interface.inputs {
            walker.visit_declaration(decl, &local_context);
            local_context.add_declaration(decl);
        }
        for decl in &module.interface.outputs {
            walker.visit_declaration(decl, &local_context);
            local_context.add_declaration(decl);
        }

        walk_ast_code_block(walker, &module.code, &local_context);
    }
}



/*
General AST Code, not used, but may be useful to convert to

#[derive(Debug,Clone,Copy)]
pub enum ValueIdentifierType {
    Input,
    Output,
    Local,
    State
}

#[derive(Debug,Clone,Copy)]
pub enum TypeIdentifierType {
    Type,
    Module,
    Interface
}

#[derive(Debug,Clone,Copy)]
pub enum StatementType {
    Declare, // (Declaration)
    DeclareAssign, // (Declaration, 
    Assign,
    Mention,
    Block
}

#[derive(Debug,Clone,Copy)]
pub enum ExpressionType {
    Named, // No Contents
    BinOp(TokenTypeIdx), // (Expression, Expression)
    UniOp(TokenTypeIdx) // (Expression)
}

#[derive(Debug,Clone,Copy)]
pub enum NodeType {
    Error, // Parsing error type. Always return as much info as possible. Can contain anything
    Module, // (TypeIdentifier, ArgList (input), ArgList (output), Statement(Block))

    Statement(StatementType), // Enum Statement
    Expression(ExpressionType), // Enum Expression
    TypeExpr,

    ArgList, // Declaration[]
    Declaration, // (TypeExpr, Expression(Named))
}

#[derive(Debug)]
pub struct Node {
    pub typ : NodeType,
    pub subnodes : Box<[Node]>,
    pub token_span : Span
}

 */