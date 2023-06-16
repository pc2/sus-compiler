
use crate::tokenizer::TokenTypeIdx;

#[derive(Clone,Copy,Debug)]
pub struct Span(pub usize, pub usize);

impl From<usize> for Span {
    fn from(v : usize) -> Span {
        Span(v, v)
    }
}

#[derive(Debug)]
pub struct SignalDeclaration {
    pub span : Span,
    pub typ : SpanExpression,
    pub name_token : usize
}

pub type Bundle = Vec<SignalDeclaration>;

#[derive(Debug)]
pub struct Interface {
    pub span : Span,
    pub inputs : Bundle,
    pub outputs : Bundle
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

pub struct LocalVariableContext<'ast, 'prev> where 'prev : 'ast {
    pub locals : Vec<&'ast SignalDeclaration>,
    pub prev : Option<&'prev LocalVariableContext<'ast, 'prev>>
}

pub trait ASTWalker {
    fn walk_ast(ast : &ASTRoot) {

    }
}
