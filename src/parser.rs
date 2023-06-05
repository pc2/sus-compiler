
use crate::{tokenizer::*, errors::*};

use std::iter::{Peekable, Enumerate};
use core::slice::Iter;

type TokenPos = usize;

#[derive(Clone,Copy,Debug)]
pub struct Span(TokenPos,TokenPos);

#[derive(Debug)]
enum SignalType {
    Named(TokenPos)
}
type SpanSignalType = (SignalType, Span);

#[derive(Debug)]
struct SignalDeclaration {
    span : Span,
    typ : SpanSignalType,
    name_token : TokenPos
}

type Bundle = Vec<SignalDeclaration>;

#[derive(Debug)]
struct Interface {
    span : Span,
    inputs : Bundle,
    outputs : Bundle
}

#[derive(Debug)]
enum AddSubtract {
    Add,
    Subtract
}
#[derive(Debug)]
enum MulDiv {
    Multiply,
    Divide,
    Modulo
}
#[derive(Debug)]
enum Expression {
    Named(TokenPos),
    Sum(Vec<(AddSubtract, SpanExpression)>),
    Mul(Vec<(MulDiv, SpanExpression)>)
}
type SpanExpression = (Expression, Span);

#[derive(Debug)]
enum Statement {
    Declare(SignalDeclaration),
    Assign(TokenPos, SpanExpression)
}


#[derive(Debug)]
enum CodeBlock {

}

#[derive(Debug)]
struct Module {
    span : Span,
    name : TokenPos,
    interface : Interface
}

#[derive(Debug)]
pub struct ASTRoot {
    modules : Vec<Module>
}

pub enum TokenTreeNode {
    PlainToken(TokenTypeIdx, usize), // Has the index of the given token to the global Token array
    // Code between '{' and '}', '(' and ')', or '[' and ']' exclusive. Contains sublist of tokens, index of open, index of close bracket
    Block(TokenTypeIdx, Vec<Self>, usize, usize), 
}
impl TokenTreeNode {
    fn get_token_type(&self) -> TokenTypeIdx {
        match self {
            Self::PlainToken(typ, _) => *typ,
            Self::Block(typ, _, _, _) => *typ
        }
    }
    fn get_first_token_idx(&self) -> usize {
        match self {
            Self::PlainToken(_, pos) => *pos,
            Self::Block(_, _, first, _) => *first
        }
    }
    fn get_last_token_idx(&self) -> usize {
        match self {
            Self::PlainToken(_, pos) => *pos,
            Self::Block(_, _, _, last) => *last
        }
    }
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

struct AST_Parser_Context<'a> {
    file_text : &'a str,
    errors : Vec<ParsingError<'a>>,
    tokens : &'a [Token<'a>]
}

struct TokenStream<'it> {
    iter : Peekable<Iter<'it, TokenTreeNode>>,
    unexpected_eof_token : usize,
    pub last_idx : usize
}

impl<'it> TokenStream<'it> {
    // The given start idx should point to the first element in this block. unexpected_eof_token should point one past the last element
    fn new(list : &'it [TokenTreeNode], start_idx : usize, unexpected_eof_token : usize) -> TokenStream<'it> {
        TokenStream{iter : list.iter().peekable(), unexpected_eof_token : unexpected_eof_token, last_idx : start_idx}
    }
    fn next(&mut self) -> Option<&'it TokenTreeNode> {
        if let Some(found) = self.iter.next() {
            self.last_idx = found.get_last_token_idx();
            Some(found)
        } else {
            None
        }
    }
    fn peek(&mut self) -> Option<&'it TokenTreeNode> {
        if let Some(&found) = self.iter.peek() {
            Some(found)
        } else {
            None
        }
    }
    fn peek_is_plain(&mut self, expected : TokenTypeIdx) -> bool {
        if let Some(TokenTreeNode::PlainToken(typ, _place)) = self.iter.peek() {
            if *typ == expected {
                return true;
            }
        }
        false
    }
    fn skip_until(&mut self, end_type : TokenTypeIdx) {
        while let Some(found) = self.peek() {
            if found.get_token_type() == end_type {
                break;
            }
            self.next();
        }
    }
}
// type TokenIter<'it> = Iter<'it, TokenTreeNode>;

pub fn get_file_error_span<'a>(file_text : &'a str, tokens : &[Token<'a>], start_tok : usize, end_tok : usize) -> &'a str {
    let start_str = tokens[start_tok].text;
    let end_str = tokens[end_tok].text;

    let start = file_text.as_ptr() as usize - start_str.as_ptr() as usize;
    let end = file_text.as_ptr() as usize - end_str.as_ptr() as usize + end_str.len();

    file_text.get(start..end).unwrap()
}

impl<'a> AST_Parser_Context<'a> {
    fn eat_or_error<'it>(&mut self, token_stream : &mut TokenStream<'it>, error_reason : &str) -> Option<&'it TokenTreeNode> {
        if let Some(found) = token_stream.next() {
            Some(found)
        } else {
            self.errors.push(error_basic(self.tokens[token_stream.unexpected_eof_token].text, "Unexpected end of scope. ".to_owned() + error_reason));
            None
        }
    }
    fn eat_plain<'it>(&mut self, token_stream : &mut TokenStream<'it>, expected : TokenTypeIdx) -> Option<usize> {
        assert!(is_bracket(expected) == IsBracket::NotABracket);
        
        let tok_elem : &TokenTreeNode = self.eat_or_error(token_stream, &("Expected ".to_owned() + get_token_type_name(expected)))?;
        
        match tok_elem {
            &TokenTreeNode::PlainToken(typ, idx) => {
                if typ == expected {
                    Some(idx)
                } else {
                    self.errors.push(error_incorrect_token(&[expected], &self.tokens[idx], ""));
                    None
                }
            },
            &TokenTreeNode::Block(_, _, block_start, block_end) => {
                self.errors.push(error_basic(get_file_error_span(self.file_text, self.tokens, block_start, block_end), "Unexpected Code Block. Expected ".to_owned() + get_token_type_name(expected) + " but found Code Block"));
                None
            }
        }
    }
    fn eat_block<'it>(&mut self, iter : &mut TokenStream<'it>, expected_block_opener : TokenTypeIdx) -> Option<(&'it [TokenTreeNode], usize, usize)> {
        assert!(is_bracket(expected_block_opener) != IsBracket::NotABracket);
        
        let tok_elem : &TokenTreeNode = self.eat_or_error(iter, &("Expected Code Block ".to_owned() + get_token_type_name(expected_block_opener)))?;
        
        match tok_elem {
            TokenTreeNode::Block(opener_typ, contents, block_start, block_end) => {
                if *opener_typ == expected_block_opener {
                    Some((contents, *block_start, *block_end))
                } else {
                    let error_span = get_file_error_span(self.file_text, self.tokens, *block_start, *block_end);
                    self.errors.push(error_basic(error_span, "Unexpected Block of incorrect type. Expected a block starting with '".to_owned() + get_token_type_name(*opener_typ) + "'"));
                    None
                }
            },
            TokenTreeNode::PlainToken(typ, idx) => {
                self.errors.push(error_basic(self.tokens[*idx].text, "Unexpected token. Expected Code Block but found ".to_owned() + get_token_type_name(*typ)));
                None
            }
        }
    }

    fn parse_declaration_type(&mut self, token_stream : &mut TokenStream) -> Option<SpanSignalType> {
        let token_idx = self.eat_plain(token_stream, TOKEN_IDENTIFIER)?;
        Some((SignalType::Named(token_idx), Span(token_idx, token_idx)))
    }
    
    fn parse_signal_declaration(&mut self, token_stream : &mut TokenStream) -> Option<SignalDeclaration> {
        let sig_type = self.parse_declaration_type(token_stream)?;
        let name_token = self.eat_plain(token_stream, TOKEN_IDENTIFIER)?;
        Some(SignalDeclaration{span : Span(sig_type.1.0, token_stream.last_idx), typ : sig_type, name_token : name_token})
    }
    
    fn parse_bundle(&mut self, token_stream : &mut TokenStream) -> Bundle {
        let mut result : Bundle = Vec::new();
        while token_stream.peek_is_plain(TOKEN_IDENTIFIER) {
            if let Some(decl) = self.parse_signal_declaration(token_stream) {
                result.push(decl);
            } else {
                // Error during parsing signal decl. Skip till "," or end of scope
                token_stream.skip_until(kw(","));
            }
            
            if !token_stream.peek_is_plain(kw(",")) {
                break;
            }
            token_stream.next();
        }
        result
    }

    fn parse_interface(&mut self, token_stream : &mut TokenStream) -> Interface {
        let start_idx = token_stream.last_idx + 1;

        let inputs = self.parse_bundle(token_stream);
    
        let outputs = if token_stream.peek_is_plain(kw("->")) {
            token_stream.next();
            self.parse_bundle(token_stream)
        } else {
            Vec::new()
        };
        
        Interface{span : Span(start_idx, token_stream.last_idx), inputs : inputs, outputs : outputs}
    }

    fn parse_code_block(&mut self, token_stream : &mut TokenStream) -> Vec<Statement> {
        let statements : Vec<Statement> = Vec::new();
        
        /*while let Some(tok) = token_stream.next() {
            if let TokenTreeNode(typ, pos) = tok {
                
            } else {
                
            }

        }*/

        statements
    }

    fn parse_module(&mut self, token_stream : &mut TokenStream, declaration_start_idx : usize) -> Option<Module> {
        // done by caller 
        // self.eat_plain(token_stream, kw("module"));
        let module_name = self.eat_plain(token_stream, TOKEN_IDENTIFIER)?;
        self.eat_plain(token_stream, kw(":"))?;

        let module_interface = self.parse_interface(token_stream);

        let (block_tokens, start, end) = self.eat_block(token_stream, kw("{"))?;

        let module_code = self.parse_code_block(&mut TokenStream::new(block_tokens, start, end));

        Some(Module{span : Span(declaration_start_idx, token_stream.last_idx), name: module_name, interface : module_interface})
    }

    fn parse_ast(&mut self, outer_token_iter : &mut TokenStream) -> ASTRoot {
        let mut found_modules : Vec<Module> = Vec::new();

        while let Some(t) = outer_token_iter.next() {
            if let &TokenTreeNode::PlainToken(typ, module_kw_pos) = t {
                if typ == kw("module") {
                    if let Some(module) = self.parse_module(outer_token_iter, module_kw_pos) {
                        found_modules.push(module);
                    }
                } else {
                    // idk what to do with this. Continue until we recognise something?
                    self.errors.push(error_incorrect_token(&[kw("module")], &self.tokens[t.get_first_token_idx()], "while parsing outer scope"));
                }
            }
        }

        ASTRoot{modules : found_modules}
    }
}

pub fn parse<'a>(file_text : &'a str, tokens : &'a [Token<'a>]) -> (ASTRoot, Vec<TokenTreeNode>, Vec<ParsingError<'a>>) {
    let (token_hierarchy, mut hierarchy_errors) = to_token_hierarchy(tokens);

    let mut context = AST_Parser_Context{file_text : file_text, errors : Vec::new(), tokens : tokens};
    let mut token_stream = TokenStream::new(&token_hierarchy, 0, tokens.len() - 1);
    let ast_root : ASTRoot = context.parse_ast(&mut token_stream);
    
    if hierarchy_errors.is_empty() {
        hierarchy_errors.append(&mut context.errors);
    } else {
        hierarchy_errors = context.errors;
    }

    (ast_root, token_hierarchy, hierarchy_errors)
}


/*
use chumsky::prelude::*;
use chumsky::error::*;

pub fn parse_with_chumsky<'src>(tokens : &'src [TokenTypeIdx]) -> impl Parser<&'src [TokenTypeIdx], ASTRoot> {
    let identifier = just(TOKEN_IDENTIFIER);
    let value = just(TOKEN_NUMBER);
    

    let expression = recursive(|expression| 
        choice((
            identifier.map_with_span(|ident_token, span : Error::Span| Expression::Named(span.start)), // Just a named identifier
            expression
                .then(just(kw("+")))
                .then(expression)
                .map(|v| v)
        ))

    );

    let signal_type = identifier;
    let signal_decl = identifier.then(identifier);
    let interface = 
        signal_decl
        .separated_by(kw(","))
        .then(just(kw("->")))
        .then(signal_decl.separated_by(kw(",")));

    let statement = choice((
        identifier,
        signal_decl
    ))
        .then(kw("="))
        .ignore_then(expression)
        .then(kw(";"));

    let code_block = 
        statement
        .repeated()
        .delimited_by(
            just(kw("{")), 
            just(kw("}"))
        );

    let module = just(kw("module"))
        .ignore_then(identifier.labelled("module name"))
        .then(interface)
        .then(code_block);
    
        module.repeat()
}
*/
