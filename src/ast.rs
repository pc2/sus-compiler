

use crate::{errors::ErrorCollector, file_position::{BracketSpan, Span}, flattening::FlattenedModule, instantiation::InstantiationList, linker::FileUUID, tokenizer::{get_token_type_name, TokenTypeIdx}, value::Value};
use core::ops::Range;
use std::fmt::Display;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum IdentifierType {
    Input,
    Output,
    Local,
    State,
    Generative
}

impl IdentifierType {
    pub fn get_keyword(&self) -> &'static str {
        match self {
            IdentifierType::Input => "input",
            IdentifierType::Output => "output",
            IdentifierType::Local => "",
            IdentifierType::State => "state",
            IdentifierType::Generative => "gen",
        }
    }
    pub fn is_generative(&self) -> bool {
        *self == IdentifierType::Generative
    }
    pub fn is_port(&self) -> bool {
        *self == IdentifierType::Input || *self == IdentifierType::Output
    }
}

#[derive(Debug)]
pub enum TypeExpression {
    Named, // SpanTypeExpression Span gives name
    Array(Box<(SpanTypeExpression, SpanExpression)>)
}

pub type SpanTypeExpression = (TypeExpression, Span);

#[derive(Debug)]
pub struct SignalDeclaration {
    pub name_span : Span,
    pub typ : SpanTypeExpression,
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

#[derive(Debug)]
pub struct Identifier {
    pub span : Span
}

#[derive(Debug)]
pub enum Expression {
    Named(Identifier),
    Constant(Value),
    UnaryOp(Box<(Operator, usize/*Operator token */, SpanExpression)>),
    BinOp(Box<(SpanExpression, Operator, usize/*Operator token */, SpanExpression)>),
    Array(Box<(SpanExpression, SpanExpression, BracketSpan)>), // first[second]
    FuncCall(Vec<SpanExpression>) // first(second, third, ...)
}

impl Expression {
    pub fn new_binop(left : SpanExpression, op : Operator, op_pos : usize/*Operator token */, right : SpanExpression) -> SpanExpression {
        let span = Span::new_overarching(left.1, right.1);
        (Expression::BinOp(Box::new((left, op, op_pos, right))), span)
    }
}
pub type SpanExpression = (Expression, Span);
pub type SpanStatement = (Statement, Span);

#[derive(Debug)]
pub enum LeftExpression {
    Assignable(Expression),
    Declaration(SignalDeclaration)
}

#[derive(Debug)]
pub enum AssignableExpressionModifiers {
    LatencyAdding{num_regs : i64, regs_span : Span},
    Initial{initial_token : usize},
    NoModifiers
}

#[derive(Debug)]
pub struct AssignableExpressionWithModifiers {
    pub expr : LeftExpression,
    pub span : Span,
    pub modifiers : AssignableExpressionModifiers
}

#[derive(Debug)]
pub struct RangeExpression {
    pub from : SpanExpression,
    pub to : SpanExpression
}

#[derive(Debug)]
pub enum Statement {
    Assign{to : Vec<AssignableExpressionWithModifiers>, eq_sign_position : Option<usize>, expr : Option<SpanExpression>}, // num_regs v = expr;
    If{condition : SpanExpression, then : CodeBlock, els : Option<CodeBlock>},
    For{var : SignalDeclaration, range : RangeExpression, code : CodeBlock},
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


#[derive(Debug)]
pub struct ParsedInterface {
    pub ports : Vec<SignalDeclaration>,
    pub outputs_start : usize
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

    pub interface : ParsedInterface,
    pub code : CodeBlock,

    pub flattened : FlattenedModule,

    pub instantiations : InstantiationList
}

impl Module {
    pub fn print_flattened_module(&self) {
        println!("[[{}]]:", self.link_info.name);
        println!("Interface:");
        for (port, is_input) in self.flattened.interface_ports.iter() {
            let port_direction = if is_input {"input"} else {"output"};
            let port_name = &self.flattened.instructions[port].extract_wire_declaration().name;
            println!("    {port_direction} {port_name} -> {:?}", port);
        }
        println!("Instantiations:");
        for (id, inst) in &self.flattened.instructions {
            println!("    {:?}: {:?}", id, inst);
        }
    }
}

#[derive(Debug)]
pub struct ASTRoot {
    pub modules : Vec<Module>,
    pub errors : ErrorCollector
}
