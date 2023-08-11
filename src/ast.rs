
use num_bigint::BigUint;

use crate::{tokenizer::{TokenTypeIdx, TokenExtraInfo}, errors::{ParsingError, error_basic_str}};
use core::ops::Range;

use std::collections::HashMap;

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

const GLOBAL_IDENTIFIER_OFFSET : TokenExtraInfo = 1 << (TokenExtraInfo::BITS - 1);
#[derive(Debug, Clone, Copy)]
pub struct IdentifierIdx {
    name_idx : TokenExtraInfo
}

impl IdentifierIdx {
    pub fn new_local(local_idx : usize) -> IdentifierIdx {
        IdentifierIdx{name_idx : local_idx as TokenExtraInfo}
    }
    pub fn new_global(global_idx : TokenExtraInfo) -> IdentifierIdx {
        IdentifierIdx{name_idx : global_idx + GLOBAL_IDENTIFIER_OFFSET}
    }
    pub fn get_local(&self) -> Option<usize> {
        if self.name_idx < GLOBAL_IDENTIFIER_OFFSET {
            Some(self.name_idx as usize)
        } else {
            None
        }
    }
    pub fn get_global(&self) -> Option<TokenExtraInfo> {
        if self.name_idx >= GLOBAL_IDENTIFIER_OFFSET {
            Some((self.name_idx - GLOBAL_IDENTIFIER_OFFSET) as TokenExtraInfo)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IdentifierToken {
    pub position : usize,
    pub name_idx : TokenExtraInfo
}


#[derive(Debug, Clone)]
pub enum TypeExpression {
    Named(usize),
    Array(Box<(SpanTypeExpression, SpanExpression)>)
}

pub type SpanTypeExpression = (TypeExpression, Span);

#[derive(Debug,Clone)]
pub struct SignalDeclaration {
    pub span : Span,
    pub typ : SpanTypeExpression,
    pub name_idx : TokenExtraInfo,
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
    Named(IdentifierIdx),
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
pub type SpanStatement = (Statement, Span);

pub type SpanAssignableExpression = SpanExpression;

#[derive(Debug)]
pub enum Statement {
    Assign(Vec<SpanAssignableExpression>, SpanExpression), // v = expr;
    Block(Vec<SpanStatement>),
    TimelineStage(usize)
}

#[derive(Debug)]
pub struct Module {
    pub span : Span,
    pub name : IdentifierToken,
    pub declarations : Vec<SignalDeclaration>,
    pub code : Vec<SpanStatement>
}

#[derive(Debug)]
pub struct ASTRoot {
    pub modules : Vec<Module>
}

pub trait IterIdentifiers {
    fn for_each_value<F>(&self, func : &mut F) where F : FnMut(IdentifierIdx, usize) -> ();
}

impl IterIdentifiers for SpanExpression {
    fn for_each_value<F>(&self, func : &mut F) where F : FnMut(IdentifierIdx, usize) -> () {
        let (expr, span) = self;
        match expr {
            Expression::Named(id) => {
                assert!(span.0 == span.1);
                func(*id, span.0)
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

impl IterIdentifiers for SpanTypeExpression {
    fn for_each_value<F>(&self, func : &mut F) where F : FnMut(IdentifierIdx, usize) -> () {
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

pub fn for_each_expression_in_block<F>(block : &Vec<SpanStatement>, func : &mut F) where F: FnMut(&SpanExpression) {
    for (stmt, _span) in block {
        match stmt {
            Statement::Assign(to, v) => {
                for t in to {
                    func(t);
                }
                func(v);
            },
            Statement::Block(b) => {
                for_each_expression_in_block(b, func);
            },
            _other => {}
        }
    }
}

pub fn for_each_expression_in_module<F>(m : &Module, func : &mut F) where F : FnMut(&SpanExpression) {
    for (idx, d) in m.declarations.iter().enumerate() {
        /*if d.identifier_type != IdentifierType::Input && d.identifier_type != IdentifierType::Output {
            break;
        }*/ // Allow potential duplicates for locals
        let local_expr = (Expression::Named(IdentifierIdx::new_local(idx)), Span::from(d.span.1));
        func(&local_expr);
    }
    for_each_expression_in_block(&m.code, func);
}

pub struct GlobalContext {
    real_types : HashMap<usize, TypeExpression>,
    // aliases : todo!()
}

impl GlobalContext {
    pub fn parse_to_type((expr, span) : &SpanExpression, errors : &mut Vec<ParsingError<Span>>) -> Option<TypeExpression> {
        match expr {
            Expression::Named(idx) => {todo!();},
            Expression::Array(args) => {todo!();},
            other => {
                errors.push(error_basic_str(*span, "Unexpected part"));
                return None
            }
        }
    }
}

