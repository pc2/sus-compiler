
use num_bigint::BigUint;

use crate::{tokenizer::TokenTypeIdx, linker::{ValueUUID, FileUUID}};
use core::ops::Range;

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
    Local(usize),
    Global(usize)
}


#[derive(Debug, Clone)]
pub enum TypeExpression {
    Named(usize), // position in referenced globals list
    Array(Box<(SpanTypeExpression, SpanExpression)>)
}

pub type SpanTypeExpression = (TypeExpression, Span);

#[derive(Debug,Clone)]
pub struct SignalDeclaration {
    pub span : Span,
    pub typ : SpanTypeExpression,
    pub name : Range<usize>, // File position
    pub identifier_type : IdentifierType
}

#[derive(Debug,Clone,Copy)]
pub struct Operator {
    pub op_typ : TokenTypeIdx
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
    Named{local_idx : usize, num_regs : usize},
    ArrayIndex(Box<(SpanAssignableExpression, SpanExpression)>)
}

#[derive(Debug)]
pub enum Statement {
    Assign(Vec<SpanAssignableExpression>, SpanExpression), // v = expr;
    Block(Vec<SpanStatement>),
    TimelineStage(usize)
}

#[derive(Debug)]
pub struct LinkInfo {
    pub file : FileUUID,
    pub name_token : usize,
    pub span : Span,
    pub global_references : Vec<(GlobalReference, ValueUUID)>
}

#[derive(Debug)]
pub struct Module {
    pub link_info : LinkInfo,

    pub declarations : Vec<SignalDeclaration>,
    pub code : Vec<SpanStatement>
}

pub type GlobalReference = Vec<usize>; // token index, and name span

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
            AssignableExpression::Named{local_idx: id, num_regs : _} => {
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

impl IterIdentifiers for SpanTypeExpression {
    fn for_each_value<F>(&self, func : &mut F) where F : FnMut(LocalOrGlobal, usize) -> () {
        let (typ, _span) = self;
        match typ {
            TypeExpression::Named(_n) => {
                // is type
            }
            TypeExpression::Array(b) => {
                let (arr_typ, arr_size) = &**b;
                arr_typ.for_each_value(func);
                arr_size.for_each_value(func);
            }
        }
    }
}

pub fn for_each_assign_in_block<F>(block : &Vec<SpanStatement>, func : &mut F) where F: FnMut(&Vec<SpanAssignableExpression>, &SpanExpression) {
    for (stmt, _span) in block {
        match stmt {
            Statement::Assign(to, v) => {
                func(to, v);
            },
            Statement::Block(b) => {
                for_each_assign_in_block(b, func);
            },
            _other => {}
        }
    }
}

impl IterIdentifiers for Module {
    fn for_each_value<F>(&self, func : &mut F) where F : FnMut(LocalOrGlobal, usize) -> () {
        for (pos, decl) in self.declarations.iter().enumerate() {
            func(LocalOrGlobal::Local(pos), decl.span.1);
        }
        for_each_assign_in_block(&self.code, &mut |to, v| {
            for assign_to in to {
                assign_to.for_each_value(func);
            }
            v.for_each_value(func);
        });
    }
}





