

use crate::{tokenizer::{TokenTypeIdx, get_token_type_name}, linker::{NamedUUID, FileUUID}, flattening::FlattenedModule, arena_alloc::{UUIDMarker, UUID, FlatAlloc}, instantiation::InstantiationList, value::Value};
use core::ops::Range;
use std::fmt::Display;

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
    Generative,
    Virtual // Generated at the interfaces of submodule instantiations
}

impl From<usize> for Span {
    fn from(v : usize) -> Span {
        Span(v, v)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LocalOrGlobal {
    Local(DeclID),
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
    pub name_token : usize,
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
pub struct CodeBlock {
    pub statements : Vec<SpanStatement>
}

#[derive(Debug)]
pub enum Statement {
    Declaration(DeclID),
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

    pub declarations : FlatAlloc<SignalDeclaration, DeclIDMarker>,
    pub ports : Box<[DeclID]>,
    pub outputs_start : usize,
    pub code : CodeBlock,

    pub flattened : FlattenedModule,

    pub instantiations : InstantiationList
}

impl Module {
    pub fn print_flattened_module(&self) {
        println!("Interface:");
        for (port_idx, port) in self.flattened.interface_ports.iter().enumerate() {
            let port_direction = if port_idx < self.flattened.outputs_start {"input"} else {"output"};
            let port_name = &self.declarations[self.ports[port_idx]].name;
            println!("    {port_direction} {port_name} -> {:?}", *port);
        }
        println!("Instantiations:");
        for (id, inst) in &self.flattened.instantiations {
            println!("    {:?}: {:?}", id, inst);
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub struct GlobalReference(pub Span, pub Option<NamedUUID>); // token index, and name span

#[derive(Debug)]
pub struct ASTRoot {
    pub modules : Vec<Module>
}
