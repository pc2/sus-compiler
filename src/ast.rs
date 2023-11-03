
use num::bigint::BigUint;

use crate::{tokenizer::TokenTypeIdx, linker::{ValueUUID, FileUUID}, flattening::FlattenedModule};
use core::ops::Range;
use std::ops::Deref;

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
    Local(usize),
    Global(usize)
}


#[derive(Debug, Clone)]
pub enum TypeExpression {
    Named(usize), // position in referenced globals list
    Array(Box<(SpanTypeExpression, SpanExpression)>)
}

impl TypeExpression {
    pub fn get_root(&self) -> usize {
        match self {
            Self::Named(s) => *s,
            Self::Array(b) => {
                b.deref().0.0.get_root()
            }
        }
    }
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
    Named{local_idx : usize},
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
    Declaration{local_id : usize},
    Assign{to : Vec<AssignableExpressionWithModifiers>, eq_sign_position : Option<usize>, expr : SpanExpression}, // num_regs v = expr;
    If{condition : SpanExpression, then : CodeBlock, els : Option<CodeBlock>},
    Block(CodeBlock),
    TimelineStage(usize)
}

#[derive(Debug)]
pub struct LinkInfo {
    pub file : FileUUID,
    pub name_token : usize,
    pub span : Span,
    pub global_references : Vec<GlobalReference>,
    pub is_fully_linked : bool // Caches if self.global_references contains any INVALID references. 
}

#[derive(Debug)]
pub struct Module {
    pub link_info : LinkInfo,

    pub declarations : Vec<SignalDeclaration>,
    pub code : CodeBlock,

    pub flattened : Option<FlattenedModule>
}

impl Module {
    pub fn get_function_sugar_inputs_outputs(&self) -> (Range<usize>, Range<usize>) {
        let mut decl_iter = self.declarations.iter().enumerate();
        let mut input_range : Range<usize> = 0..0;
        let mut output_range : Range<usize> = 0..0;
        let mut last = if let Some((_pos, decl)) = decl_iter.next() {
            match decl.identifier_type {
                IdentifierType::Input => IdentifierType::Input,
                IdentifierType::Output => IdentifierType::Output,
                IdentifierType::Local | IdentifierType::State => {return (0..0, 0..0)}
            }
        } else {
            return (0..0, 0..0);
        };
        let mut last_valid_pos = 0;
        for (pos, decl) in decl_iter {
            if decl.identifier_type != last {
                match decl.identifier_type {
                    IdentifierType::Input => {
                        input_range.start = pos;
                        output_range.end = pos;
                    }
                    IdentifierType::Output => {
                        output_range.start = pos;
                        input_range.end = pos;
                    }
                    IdentifierType::Local | IdentifierType::State => {
                        break;
                    }
                }
                last = decl.identifier_type;
                last_valid_pos = pos;
            }
        }
        match last {
            IdentifierType::Input => {
                input_range.end = last_valid_pos + 1;
            }
            IdentifierType::Output => {
                output_range.end = last_valid_pos + 1;
            }
            _other => unreachable!()
        }
        (input_range, output_range)
    }
}

#[derive(Debug,Clone,Copy)]
pub struct GlobalReference(pub Span, pub ValueUUID); // token index, and name span

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
                Statement::Declaration { local_id : _ } => {}
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
        for (pos, decl) in self.declarations.iter().enumerate() {
            func(LocalOrGlobal::Local(pos), decl.span.1);
        }
        self.code.for_each_value(func);
    }
}
