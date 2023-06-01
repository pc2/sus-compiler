
use crate::{tokenizer::*, errors::ParsingError};

use std::iter::Peekable;
use core::slice::Iter;
use core::ops::Range;

// TokenRange, denotes the range of tokens that this object encompasses
type TR = Range<usize>;

#[derive(Debug)]
enum SignalType<'a> {
    Named(&'a str)
}

#[derive(Debug)]
struct SignalDeclaration<'a> {
    typ : SignalType<'a>,
    name : &'a str
}

type Bundle<'a> = Vec<SignalDeclaration<'a>>;

#[derive(Debug)]
struct Interface<'a> {
    inputs : Bundle<'a>,
    outputs : Bundle<'a>
}

struct InterspersedVec<T, I> {
    first : T,
    others : Vec<(I,T)>
}

enum Expression<'a> {
    Named(&'a str),
    Sum(Vec<(TokenTypeIdx, )>)
}

enum Statement<'a> {
    Declare(SignalDeclaration<'a>),
    Assign(&'a str, Expression<'a>)
}

type StatementBlock<'a> = Vec<Statement<'a>>;

#[derive(Debug)]
struct HardwareModule<'a> {
    name : &'a str,
    interface : Interface<'a>
}

#[derive(Debug)]
pub struct ASTRoot<'a> {
    modules : Vec<HardwareModule<'a>>
}

type TokenStream<'a> = Peekable<Iter<'a, Token<'a>>>;

fn eat<'a>(mut token_stream : &mut TokenStream<'a>, expected_token_type : TokenTypeIdx) -> Result<&'a str, ParsingError<'a>> {
    let found = token_stream.next().unwrap();
    if found.typ == expected_token_type {
        Ok(found.text)
    } else {
        Err(ParsingError::new_error_incorrect_token(expected_token_type, found, "while reading module context"))
    }
}

fn parse_signal_type<'a>(token_stream : &mut TokenStream<'a>) -> Result<SignalType<'a>, ParsingError<'a>> {
    let type_name = eat(token_stream, TOKEN_IDENTIFIER)?;
    Ok(SignalType::Named(type_name))
}

fn parse_signal_declaration<'a>(token_stream : &mut TokenStream<'a>) -> Result<SignalDeclaration<'a>, ParsingError<'a>> {
    let sig_type = parse_signal_type(token_stream)?;
    let sig_name = eat(token_stream, TOKEN_IDENTIFIER)?;
    Ok(SignalDeclaration{typ : sig_type, name : sig_name})
}

fn parse_bundle<'a>(token_stream : &mut TokenStream<'a>) -> Result<Bundle<'a>, ParsingError<'a>> {
    let mut result : Bundle<'a> = Vec::new();
    loop {
        result.push(parse_signal_declaration(token_stream)?);
        if token_stream.peek().unwrap().typ != kw(",") {
            return Ok(result);
        }
        token_stream.next();
    }
}

fn parse_interface<'a>(token_stream : &mut TokenStream<'a>) -> Result<Interface<'a>, ParsingError<'a>> {
    let inputs = if token_stream.peek().unwrap().typ != kw("->") {
        parse_bundle(token_stream)?
    } else {
        Vec::new()
    };

    let outputs = if token_stream.peek().unwrap().typ == kw("->") {
        token_stream.next();
        parse_bundle(token_stream)?
    } else {
        Vec::new()
    };

    Ok(Interface{inputs : inputs, outputs : outputs})
}

fn parse_module<'a>(token_stream : &mut TokenStream<'a>) -> Result<HardwareModule<'a>, ParsingError<'a>> {
    let module_name = eat(token_stream, TOKEN_IDENTIFIER)?;
    eat(token_stream, kw(":"))?;

    let module_interface = parse_interface(token_stream)?;

    // WIP TokenRange, and parsing the module implementation

    Ok(HardwareModule{name: module_name, interface : module_interface})
}

pub fn parse<'a>(tokens : &'a [Token<'a>]) -> Result<ASTRoot<'a>, ParsingError<'a>> {
    let mut token_iter : TokenStream<'a> = tokens.iter().peekable();

    let mut modules : Vec<HardwareModule> = Vec::new();
    loop {
        let tok = token_iter.next().unwrap();
        
        if tok.typ == TOKEN_END_OF_FILE {
            return Ok(ASTRoot{modules : modules});
        } else if tok.typ == kw("module") {
            modules.push(parse_module(&mut token_iter)?);
        }
    }
}
