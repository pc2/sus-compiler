
use crate::{ast::*, tokenizer::*, parser::*};

use console::Style;


#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum IDEIdentifierType {
    Value(IdentifierType),
    Type,
    Interface,
    Unknown
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum IDETokenType {
    Comment,
    Keyword,
    Operator,
    PipelineStage,
    TimelineStage,
    Identifier(IDEIdentifierType),
    Number,
    Invalid,
    InvalidBracket,
    OpenBracket(usize), // Bracket depth
    CloseBracket(usize) // Bracket depth
}

#[derive(Debug,Clone,Copy)]
pub struct IDEToken {
    pub typ : IDETokenType
}

fn pretty_print_chunk_with_whitespace(whitespace_start : usize, file_text : &str, text_span : CharSpan, st : Style) { 
    let whitespace_text = file_text.get(whitespace_start..text_span.file_pos.char_idx).unwrap();

    print!("{}{}", whitespace_text, st.apply_to(file_text.get(text_span.as_range()).unwrap()));
}

fn print_tokens(file_text : &str, token_spans : &[CharSpan]) {
    let mut whitespace_start : usize = 0;
    for (tok_idx, tok_span) in token_spans.iter().enumerate() {
        let styles = [Style::new().magenta(), Style::new().yellow(), Style::new().blue()];
        let st = styles[tok_idx % styles.len()].clone().underlined();
        
        pretty_print_chunk_with_whitespace(whitespace_start, file_text, *tok_span, st);
        whitespace_start = tok_span.end_pos();
    }

    print!("{}\n", file_text.get(whitespace_start..file_text.len()).unwrap());
}

fn pretty_print(file_text : &str, token_spans : &[CharSpan], ide_infos : &[IDEToken]) {
    let mut whitespace_start : usize = 0;

    for (tok_idx, token) in ide_infos.iter().enumerate() {
        let bracket_styles = [Style::new().magenta(), Style::new().yellow(), Style::new().blue()];
        let st = match token.typ {
            IDETokenType::Comment => Style::new().green().dim(),
            IDETokenType::Keyword => Style::new().blue(),
            IDETokenType::Operator => Style::new().white().bright(),
            IDETokenType::PipelineStage => Style::new().red().bold(),
            IDETokenType::TimelineStage => Style::new().red().bold(),
            IDETokenType::Identifier(IDEIdentifierType::Unknown) => Style::new().red().underlined(),
            IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::Local)) => Style::new().blue().bright(),
            IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::State)) => Style::new().blue().bright().underlined(),
            IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::Input)) => Style::new().blue().bright(),
            IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::Output)) => Style::new().blue().dim(),
            IDETokenType::Identifier(IDEIdentifierType::Type) => Style::new().magenta().bright(),
            IDETokenType::Identifier(IDEIdentifierType::Interface) => Style::new().magenta().dim(),
            IDETokenType::Number => Style::new().green().bright(),
            IDETokenType::Invalid | IDETokenType::InvalidBracket => Style::new().red().underlined(),
            IDETokenType::OpenBracket(depth) | IDETokenType::CloseBracket(depth) => {
                bracket_styles[depth % bracket_styles.len()].clone()
            }
        };
        
        let tok_span = token_spans[tok_idx];
        pretty_print_chunk_with_whitespace(whitespace_start, file_text, tok_span, st);
        whitespace_start = tok_span.end_pos();
    }

    print!("{}\n", file_text.get(whitespace_start..file_text.len()).unwrap());
}

fn add_ide_bracket_depths_recursive<'a>(result : &mut [IDEToken], current_depth : usize, token_hierarchy : &[TokenTreeNode]) {
    for tok in token_hierarchy {
        if let TokenTreeNode::Block(_, sub_block, Span(left, right)) = tok {
            result[*left].typ = IDETokenType::OpenBracket(current_depth);
            add_ide_bracket_depths_recursive(result, current_depth+1, sub_block);
            result[*right].typ = IDETokenType::CloseBracket(current_depth);
        }
    }
}

fn walk_name_color(ast : &ASTRoot, result : &mut [IDEToken]) {
    for module in &ast.modules {
        for decl in &module.declarations {
            for_each_identifier_in_expression(&decl.typ, &mut |_name, position| {
                result[position].typ = IDETokenType::Identifier(IDEIdentifierType::Type);
            });
            //result[decl.name.position].typ = IDETokenType::Identifier(IDEIdentifierType::Value(decl.identifier_type));
        }

        for_each_expression_in_module(&module, &mut |expr| {
            for_each_identifier_in_expression(expr, &mut |name, position| {
                result[position].typ = IDETokenType::Identifier(if let Some(l) = name.get_local() {
                    IDEIdentifierType::Value(module.declarations[l].identifier_type)
                } else {
                    IDEIdentifierType::Unknown
                });
            });
        });
    }
}

pub fn create_token_ide_info<'a>(parsed: &FullParseResult) -> Vec<IDEToken> {
    let mut result : Vec<IDEToken> = Vec::new();
    result.reserve(parsed.tokens.len());

    for t in &parsed.tokens.tokens {
        let tok_typ = t.get_type();
        let initial_typ = if is_keyword(tok_typ) {
            IDETokenType::Keyword
        } else if is_bracket(tok_typ) != IsBracket::NotABracket {
            IDETokenType::InvalidBracket // Brackets are initially invalid. They should be overwritten by the token_hierarchy step. The ones that don't get overwritten are invalid
        } else if is_symbol(tok_typ) {
            if tok_typ == kw("@") {
                IDETokenType::PipelineStage
            } else if tok_typ == kw("#") {
                IDETokenType::TimelineStage
            } else {
                IDETokenType::Operator
            }
        } else if is_identifier(tok_typ) {
            IDETokenType::Identifier(IDEIdentifierType::Unknown)
        } else if is_number(tok_typ) {
            IDETokenType::Number
        } else if is_comment(tok_typ) {
            IDETokenType::Comment
        } else {
            IDETokenType::Invalid
        };

        result.push(IDEToken{typ : initial_typ})
    }

    add_ide_bracket_depths_recursive(&mut result, 0, &parsed.token_hierarchy);

    walk_name_color(&parsed.ast, &mut result);

    result
}

pub fn syntax_highlight_file(file_path : &str) {
    let file_text = match std::fs::read_to_string(file_path) {
        Ok(file_text) => file_text,
        Err(reason) => panic!("Could not open file '{file_path}' for syntax highlighting because {reason}")
    };
    
    let (full_parse, errors) = perform_full_semantic_parse(&file_text);

    for err in errors {
        err.pretty_print_error(&file_path, &file_text)
    }
    
    print_tokens(&file_text, &full_parse.tokens.token_spans);

    let ide_tokens = create_token_ide_info(&full_parse);
    
    
    pretty_print(&file_text, &full_parse.tokens.token_spans, &ide_tokens);
    
    println!("{:?}", full_parse.ast);
}
