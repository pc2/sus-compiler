
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
    pub name_idx : TokenExtraInfo,
    pub identifier_type : IdentifierType
}

#[derive(Debug,Clone,Copy)]
pub struct Operator {
    pub op_typ : TokenTypeIdx
}

#[derive(Debug)]
pub enum Expression {
    Named(IdentifierIdx),
    Constant(TokenExtraInfo),
    BoolConstant(bool),
    UnaryOp(Box<(Operator, usize/*Operator token */, SpanExpression)>),
    BinOp(Box<(SpanExpression, Operator, usize/*Operator token */, SpanExpression)>),
    Array(Vec<SpanExpression>), // first[second, third, ...]
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

#[derive(Debug)]
pub enum Statement {
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
    pub declarations : Vec<SignalDeclaration>,
    pub code : Vec<SpanStatement>
}

#[derive(Debug)]
pub struct ASTRoot {
    pub modules : Vec<Module>
}

pub fn for_each_identifier_in_expression<F>((expr, span) : &SpanExpression, func : &mut F) where F: FnMut(IdentifierIdx, usize) -> () {
    match expr {
        Expression::Named(id) => {
            assert!(span.0 == span.1);
            func(*id, span.0)
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

pub fn for_each_expression_in_block<F>(block : &Vec<SpanStatement>, func : &mut F) where F: FnMut(&SpanExpression) {
    for (stmt, _span) in block {
        match stmt {
            Statement::Assign(to, v) => {
                func(to);
                func(v);
            },
            Statement::Mention(m) => {
                func(m);
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
