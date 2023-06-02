
use crate::{tokenizer::*, errors::*};

use std::iter::{Peekable, Enumerate};
use core::slice::Iter;

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
/*
fn eat<'a>(mut token_stream : &mut TokenStream<'a>, expected_token_type : TokenTypeIdx) -> Result<&'a str, ParsingError<'a>> {
    let found = token_stream.next().unwrap();
    if found.typ == expected_token_type {
        Ok(found.text)
    } else {
        Err(error_incorrect_token(expected_token_type, found, "while reading module context"))
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
}*/

pub enum TokenTreeNode {
    PlainToken(TokenTypeIdx, usize), // Has the index of the given token to the global Token array
    // Code between '{' and '}', '(' and ')', or '[' and ']' exclusive. Contains sublist of tokens, index of open, index of close bracket
    Block(TokenTypeIdx, Vec<Self>, usize, usize), 
}

struct TokenHierarchyStackElem {
    open_bracket : TokenTypeIdx, 
    open_bracket_pos : usize,
    parent : Vec<TokenTreeNode>
}

pub fn to_token_hierarchy<'a>(tokens : &[Token<'a>]) -> (Vec<TokenTreeNode>, Vec<ParsingError<'a>>) {
    let mut cur_token_slab : Vec<TokenTreeNode> = Vec::new();
    let mut stack : Vec<TokenHierarchyStackElem> = Vec::new(); // Type of opening bracket, token position, Token Subtree
    let mut errors : Vec<ParsingError<'a>> = Vec::new();

    for (idx, tok) in tokens.iter().enumerate() {
        match is_bracket(tok.typ) {
            IsBracket::Open => {
                stack.push(TokenHierarchyStackElem{open_bracket : tok.typ, open_bracket_pos : idx, parent : cur_token_slab});
                cur_token_slab = Vec::new();
            },
            IsBracket::Close => {
                if let Some(cur_block) = stack.pop() {
                    if closes(cur_block.open_bracket, tok.typ) { // All is well. This bracket was closed properly. Happy path!
                        let mut parent_cur_token_slab = cur_block.parent;
                        parent_cur_token_slab.push(TokenTreeNode::Block(cur_block.open_bracket, cur_token_slab, cur_block.open_bracket_pos, idx));
                        cur_token_slab = parent_cur_token_slab;
                    } else {
                        errors.push(error_unclosed_bracket(&tokens[idx], tok));
                        // Is this an incorrect starting bracket, or ending bracket?
                        // TODO add better error recovery
                    }
                } else {
                    // Too many close brackets
                    errors.push(error_basic_str(tok.text, "A close bracket had no corresponding opening bracket."));
                    continue;
                }
            },
            IsBracket::NotABracket => {
                cur_token_slab.push(TokenTreeNode::PlainToken(tok.typ, idx));
            }
        }
    }

    while let Some(unclosed) = stack.pop() {
        errors.push(error_basic_str(tokens[unclosed.open_bracket_pos].text, "Bracket was not closed before EOF!"))
    }

    (cur_token_slab, errors)
}

pub fn parse<'a>(tokens : &[Token<'a>]) -> Result<ASTRoot<'a>, ParsingError<'a>> {
    let (token_hierarchy, errors) = to_token_hierarchy(tokens);


    //let mut token_iter : TokenStream<'a> = tokens.iter().peekable();

    let mut modules : Vec<HardwareModule> = Vec::new();



    /*while let Some(tok) = token_iter.next() {
        if tok.typ == kw("module") {
            modules.push(parse_module(&mut token_iter)?);
        }
    }*/
    Ok(ASTRoot{modules : modules})
}
