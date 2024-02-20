

use crate::{errors::ErrorCollector, flattening::FlattenedModule, instantiation::InstantiationList, linker::FileUUID, tokenizer::{get_token_type_name, TokenTypeIdx, TokenizeResult}, value::Value};
use core::ops::Range;
use std::fmt::Display;


// Token span. Indices are INCLUSIVE
#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
pub struct Span(pub usize, pub usize);

impl Span {
    pub fn to_range<T : Clone>(&self, tokens : &[Range<T>]) -> Range<T> {
        let min = tokens[self.0].start.clone();
        let max = tokens[self.1].end.clone();
        min..max
    }
    pub fn new_overarching(left : Span, right : Span) -> Span {
        assert!(left.0 <= right.0);
        assert!(left.1 <= right.1);
        Span(left.0, right.1)
    }
    pub fn new_single_token(tok_idx : usize) -> Span {
        Span(tok_idx, tok_idx)
    }
    pub fn new_extend_to_include_token(left : Span, tok_idx : usize) -> Span {
        Span::new_overarching(left, Span::new_single_token(tok_idx))
    }
    pub fn dont_include_last_token(self) -> Span {
        self
    }
    pub fn only_last_token(self) -> Span {
        Span(self.1, self.1)
    }
    pub fn new_extend_before(tok_idx : usize, right : Span) -> Span {
        Span::new_overarching(Span::new_single_token(tok_idx), right)
    }
    pub fn new_across_tokens(start_tok : usize, end_tok : usize) -> Span {
        assert!(start_tok <= end_tok);
        Span(start_tok, end_tok)
    }
    pub fn whole_file_span(tokens : &TokenizeResult) -> Span {
        Span(0, tokens.token_types.len())
    }
    pub fn contains_token(&self, token_idx : usize) -> bool {
        token_idx >= self.0 && token_idx <= self.1
    }
    // Not really a useful quantity. Should only be used comparatively, find which is the nested-most span
    pub fn size(&self) -> usize {
        self.1 - self.0
    }
    #[track_caller]
    pub fn assert_is_single_token(&self) -> usize {
        assert!(self.1 == self.0, "Span is not singleton! {}..{}", self.0, self.1);
        self.0
    }
    pub fn is_single_token(&self) -> Option<usize> {
        if self.0 == self.1 {
            Some(self.0)
        } else {
            None
        }
    }
    pub fn difference_left(outer : Span, inner : Span) -> Span {
        assert!(outer.0 <= inner.0);
        assert!(outer.1 >= inner.1);

        Span(outer.0, inner.0 - 1) // temporary, because right now spans are still inclusive. 
        // Span(outer.0, inner.0)
    }
    pub fn difference_right(outer : Span, inner : Span) -> Span {
        assert!(outer.0 <= inner.0);
        assert!(outer.1 >= inner.1);

        Span(inner.1 + 1, outer.1) // temporary, because right now spans are still inclusive. 
        // Span(inner.1, outer.1)
    }
}

impl IntoIterator for Span {
    type Item = usize;

    type IntoIter = <std::ops::Range<usize> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Range{start : self.0, end : self.1 + 1}.into_iter()
    }
}

#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
pub struct BracketSpan(Span);

impl BracketSpan {
    pub fn from_outer(span : Span) -> Self {Self(span)}
    pub fn inner_span(&self) -> Span {
        Span(self.0.0 + 1, self.0.1 - 1)
    }
    pub fn outer_span(&self) -> Span {
        self.0
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
