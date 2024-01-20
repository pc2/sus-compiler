

use crate::{tokenizer::{TokenTypeIdx, get_token_type_name}, linker::FileUUID, flattening::FlattenedModule, arena_alloc::{UUIDMarker, UUID, FlatAlloc}, instantiation::InstantiationList, value::Value, errors::ErrorCollector};
use core::ops::Range;
use std::{fmt::Display, iter::zip};

// Token span. Indices are INCLUSIVE
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Span(pub usize, pub usize);


#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub struct DeclIDMarker;
impl UUIDMarker for DeclIDMarker {const DISPLAY_NAME : &'static str = "decl_";}
pub type DeclID = UUID<DeclIDMarker>;


impl Span {
    pub fn to_range<T : Clone>(&self, tokens : &[Range<T>]) -> Range<T> {
        let min = tokens[self.0].start.clone();
        let max = tokens[self.1].end.clone();
        min..max
    }
    #[track_caller]
    pub fn assert_is_single_token(&self) -> usize {
        assert!(self.1 == self.0, "Span is not singleton! {}..{}", self.0, self.1);
        self.0
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
    State,
    Generative
}

impl From<usize> for Span {
    fn from(v : usize) -> Span {
        Span(v, v)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LocalOrGlobal {
    Local(DeclID),
    Global(Span)
}


#[derive(Debug, Clone)]
pub enum TypeExpression {
    Named, // SpanTypeExpression Span gives name
    Array(Box<(SpanTypeExpression, SpanExpression)>)
}

pub type SpanTypeExpression = (TypeExpression, Span);

#[derive(Debug,Clone)]
pub struct SignalDeclaration {
    pub span : Span,
    pub name_token : usize,
    pub typ : SpanTypeExpression,
    pub name : Box<str>,
    pub identifier_type : IdentifierType,
    pub latency_expr : Option<SpanExpression>
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
pub enum Expression {
    Named(LocalOrGlobal),
    Constant(Value),
    UnaryOp(Box<(Operator, usize/*Operator token */, SpanExpression)>),
    BinOp(Box<(SpanExpression, Operator, usize/*Operator token */, SpanExpression)>),
    Array(Box<(SpanExpression, SpanExpression, Span/*Brackets */)>), // first[second]
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
    Named{local_idx : DeclID},
    ArrayIndex(Box<(SpanAssignableExpression, SpanExpression, Span/* Brackets */)>)
}

#[derive(Debug)]
pub struct AssignableExpressionWithModifiers {
    pub expr : SpanAssignableExpression,
    pub num_regs : i64
}

#[derive(Debug)]
pub struct RangeExpression {
    pub from : SpanExpression,
    pub to : SpanExpression
}

#[derive(Debug)]
pub enum Statement {
    Declaration(DeclID),
    Assign{to : Vec<AssignableExpressionWithModifiers>, eq_sign_position : Option<usize>, expr : SpanExpression}, // num_regs v = expr;
    If{condition : SpanExpression, then : CodeBlock, els : Option<CodeBlock>},
    For{var : DeclID, range : RangeExpression, code : CodeBlock},
    Block(CodeBlock)
}

#[derive(Debug)]
pub struct CodeBlock {
    pub statements : Vec<SpanStatement>
}

#[derive(Debug)]
pub struct LinkInfo {
    pub file : FileUUID,
    pub name : Box<str>,
    pub name_span : Span,
    pub span : Span
}

impl LinkInfo {
    pub fn get_full_name(&self) -> String {
        format!("::{}", self.name)
    }
}


#[derive(Debug, Clone)]
pub struct InterfacePorts<ID : Clone + Copy> {
    pub outputs_start : usize,
    pub ports : Box<[ID]>
}

impl<ID : Clone + Copy> InterfacePorts<ID> {
    pub fn empty() -> Self {
        InterfacePorts{outputs_start : 0, ports : Box::new([])}
    }

    // Todo, just treat all inputs and outputs as function call interface
    pub fn func_call_syntax_inputs(&self) -> Range<usize> {
        0..self.outputs_start
    }
    pub fn func_call_syntax_outputs(&self) -> Range<usize> {
        self.outputs_start..self.ports.len()
    }
    pub fn inputs(&self) -> &[ID] {
        &self.ports[..self.outputs_start]
    }
    pub fn outputs(&self) -> &[ID] {
        &self.ports[self.outputs_start..]
    }

    pub fn map<OtherID : Clone + Copy, MapFn : FnMut(ID, /*is_input : */bool) -> OtherID>(&self, f : &mut MapFn) -> InterfacePorts<OtherID> {
        InterfacePorts{
            ports : self.ports.iter().enumerate().map(|(idx, v)| f(*v, idx < self.outputs_start)).collect(),
            outputs_start : self.outputs_start
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (ID, /*is_input : */bool)> + '_ {
        self.ports.iter().enumerate().map(|(idx, v)| (*v, idx < self.outputs_start))
    }
}

#[derive(Debug)]
pub struct Module {
    pub link_info : LinkInfo,

    pub declarations : FlatAlloc<SignalDeclaration, DeclIDMarker>,
    pub ports : InterfacePorts<DeclID>,
    pub code : CodeBlock,

    pub flattened : FlattenedModule,

    pub instantiations : InstantiationList
}

impl Module {
    pub fn print_flattened_module(&self) {
        println!("Interface:");
        for ((port, is_input), port_decl) in zip(self.flattened.interface_ports.iter(), self.ports.ports.iter()) {
            let port_direction = if is_input {"input"} else {"output"};
            let port_name = &self.declarations[*port_decl].name;
            println!("    {port_direction} {port_name} -> {:?}", port);
        }
        println!("Instantiations:");
        for (id, inst) in &self.flattened.instantiations {
            println!("    {:?}: {:?}", id, inst);
        }
    }
}

#[derive(Debug)]
pub struct ASTRoot {
    pub modules : Vec<Module>,
    pub errors : ErrorCollector
}
