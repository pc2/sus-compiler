
use num::bigint::BigUint;

use crate::{tokenizer::{TokenTypeIdx, get_token_type_name}, linker::{NamedUUID, FileUUID}, flattening::{FlattenedModule, WireIDMarker, WireID, FlattenedInterface}, arena_alloc::ListAllocator};
use core::ops::Range;
use std::fmt::Display;

// Token span. Indices are INCLUSIVE
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Span(pub usize, pub usize);

impl Span {
    pub fn to_range<T : Clone>(&self, tokens : &[Range<T>]) -> Range<T> {
        let min = tokens[self.0].start.clone();
        let max = tokens[self.1].end.clone();
        min..max
    }
}

impl IntoIterator for Span {
    type Item = usize;

    type IntoIter = <std::ops::Range<usize> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Range{start : self.0, end : self.1 + 1}.into_iter()
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
pub enum LocalOrGlobal {
    Local(WireID),
    Global(usize)
}


#[derive(Debug, Clone)]
pub enum TypeExpression {
    Named(usize), // position in referenced globals list
    Array(Box<(TypeExpression, SpanExpression)>)
}

pub type SpanTypeExpression = (TypeExpression, Span);

#[derive(Debug,Clone)]
pub struct SignalDeclaration {
    pub span : Span,
    pub typ : SpanTypeExpression,
    pub name : Box<str>, // File position
    pub identifier_type : IdentifierType
}

#[derive(Debug,Clone,Copy)]
pub struct Operator {
    pub op_typ : TokenTypeIdx
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(get_token_type_name(self.op_typ))
    }
}

#[derive(Debug,Clone)]
pub enum Value {
    Bool(bool),
    Integer(BigUint)
}

#[derive(Debug,Clone)]
pub enum Expression {
    Named(LocalOrGlobal),
    Constant(Value),
    UnaryOp(Box<(Operator, usize/*Operator token */, SpanExpression)>),
    BinOp(Box<(SpanExpression, Operator, usize/*Operator token */, SpanExpression)>),
    Array(Box<(SpanExpression, SpanExpression)>), // first[second]
    FuncCall(Vec<SpanExpression>) // first(second, third, ...)
}

impl Expression {
    pub fn new_binop(left : SpanExpression, op : Operator, op_pos : usize/*Operator token */, right : SpanExpression) -> SpanExpression {
        let span = Span(left.1.0, right.1.1);
        (Expression::BinOp(Box::new((left, op, op_pos, right))), span)
    }
}
pub type SpanExpression = (Expression, Span);
pub type SpanAssignableExpression = (AssignableExpression, Span);
pub type SpanStatement = (Statement, Span);

#[derive(Debug)]
pub enum AssignableExpression {
    Named{local_idx : WireID},
    ArrayIndex(Box<(SpanAssignableExpression, SpanExpression)>)
}

#[derive(Debug)]
pub struct AssignableExpressionWithModifiers {
    pub expr : SpanAssignableExpression,
    pub num_regs : u32
}

#[derive(Debug)]
pub struct CodeBlock {
    pub statements : Vec<SpanStatement>
}

#[derive(Debug)]
pub enum Statement {
    Declaration(WireID),
    Assign{to : Vec<AssignableExpressionWithModifiers>, eq_sign_position : Option<usize>, expr : SpanExpression}, // num_regs v = expr;
    If{condition : SpanExpression, then : CodeBlock, els : Option<CodeBlock>},
    Block(CodeBlock),
    TimelineStage(usize)
}

#[derive(Debug)]
pub struct LinkInfo {
    pub file : FileUUID,
    pub name : Box<str>,
    pub name_span : Span,
    pub span : Span,
    pub global_references : Vec<GlobalReference>,
    pub is_fully_linked : bool // Caches if self.global_references contains any INVALID references. 
}

#[derive(Debug)]
pub struct Module {
    pub link_info : LinkInfo,

    pub declarations : ListAllocator<SignalDeclaration, WireIDMarker>,
    pub code : CodeBlock,

    pub interface : FlattenedInterface,
    pub flattened : FlattenedModule
}

#[derive(Debug,Clone,Copy)]
pub struct GlobalReference(pub Span, pub NamedUUID); // token index, and name span

#[derive(Debug)]
pub struct ASTRoot {
    pub modules : Vec<Module>
}

pub trait IterIdentifiers {
    fn for_each_value<F>(&self, func : &mut F) where F : FnMut(LocalOrGlobal, usize) -> ();
}

impl IterIdentifiers for SpanExpression {
    fn for_each_value<F>(&self, func : &mut F) where F : FnMut(LocalOrGlobal, usize) -> () {
        let (expr, span) = self;
        match expr {
            Expression::Named(id) => {
                assert!(span.0 == span.1);
                func(*id, span.0);
            }
            Expression::Constant(_v) => {}
            Expression::UnaryOp(b) => {
                let (_operator, _operator_pos, right) = &**b;
                right.for_each_value(func);
            }
            Expression::BinOp(b) => {
                let (left, _operator, _operator_pos, right) = &**b;
                left.for_each_value(func);
                right.for_each_value(func);
            }
            Expression::FuncCall(args) => {
                for arg in args {
                    arg.for_each_value(func);
                }
            }
            Expression::Array(b) => {
                let (array, idx) = &**b;
                array.for_each_value(func);
                idx.for_each_value(func);
            }
        }
    }
}

impl IterIdentifiers for SpanAssignableExpression {
    fn for_each_value<F>(&self, func : &mut F) where F : FnMut(LocalOrGlobal, usize) -> () {
        let (expr, span) = self;
        match expr {
            AssignableExpression::Named{local_idx: id} => {
                assert!(span.0 == span.1);
                func(LocalOrGlobal::Local(*id), span.0);
            }
            AssignableExpression::ArrayIndex(b) => {
                let (array, idx) = &**b;
                array.for_each_value(func);
                idx.for_each_value(func);
            }
        }
    }
}

impl IterIdentifiers for CodeBlock {
    fn for_each_value<F>(&self, func : &mut F) where F : FnMut(LocalOrGlobal, usize) -> () {
        for (stmt, _span) in &self.statements {
            match stmt {
                Statement::Assign{to, eq_sign_position : _, expr} => {
                    for assign_to in to {
                        assign_to.expr.for_each_value(func);
                    }
                    expr.for_each_value(func);
                },
                Statement::Block(b) => {
                    b.for_each_value(func);
                },
                Statement::Declaration(_) => {}
                Statement::If { condition, then, els } => {
                    condition.for_each_value(func);
                    then.for_each_value(func);
                    if let Some(e) = &els {
                        e.for_each_value(func);
                    }
                }
                Statement::TimelineStage(_) => {}
            }
        }
    }
}

impl IterIdentifiers for Module {
    fn for_each_value<F>(&self, func : &mut F) where F : FnMut(LocalOrGlobal, usize) -> () {
        for (pos, decl) in &self.declarations {
            func(LocalOrGlobal::Local(pos), decl.span.1);
        }
        self.code.for_each_value(func);
    }
}
