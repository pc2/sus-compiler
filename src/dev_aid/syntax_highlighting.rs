
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

struct NameColoringWalker<'a> {
    ide_token_info : &'a mut [IDEToken],
    token_spans : &'a [CharSpan],
    file_text : &'a str
}

impl<'a> ASTWalker for NameColoringWalker<'a> {
    fn visit_module_name(&mut self, module_name : usize) {
        self.ide_token_info[module_name].typ = IDETokenType::Identifier(IDEIdentifierType::Interface);
    }
    fn visit_declaration(&mut self, decl : &SignalDeclaration, context : &VariableContext) {
        for_each_identifier_in_expression(&decl.typ.0, &mut |tok_idx| {
            self.ide_token_info[tok_idx].typ = IDETokenType::Identifier(IDEIdentifierType::Type);
        });
        self.ide_token_info[decl.name_token].typ = IDETokenType::Identifier(IDEIdentifierType::Value(decl.identifier_type));
    }
    fn visit_expression(&mut self, expr : &SpanExpression, context : &VariableContext) {
        for_each_identifier_in_expression(&expr.0, &mut |tok_idx| {
            if let Some(tok_decl) = context.get_declaration_for(tok_idx, self.token_spans, self.file_text) {
                self.ide_token_info[tok_idx].typ = IDETokenType::Identifier(IDEIdentifierType::Value(tok_decl.identifier_type));
            }
        });
        //self.ide_token_info[decl.name_token].typ = IDETokenType::Identifier(IDEIdentifierType::Value(decl.identifier_type));
    }
    fn visit_assignment(&mut self, to : &SpanExpression, expr : &SpanExpression, context : &VariableContext) {}
}

fn walk_name_color(ast : &ASTRoot, token_spans : &[CharSpan], file_text : &str, result : &mut [IDEToken]) {
    let mut walker = NameColoringWalker{ide_token_info : result, token_spans : token_spans, file_text : file_text};

    walk_ast(&mut walker, ast, token_spans, file_text, &VariableContext::new_initial());
}

pub fn create_token_ide_info<'a>(file_text : &str, parsed: &FullParseResult) -> Vec<IDEToken> {
    let mut result : Vec<IDEToken> = Vec::new();
    result.reserve(parsed.token_types.len());

    for &t in &parsed.token_types {
        let initial_typ = if is_keyword(t) {
            IDETokenType::Keyword
        } else if is_bracket(t) != IsBracket::NotABracket {
            IDETokenType::InvalidBracket // Brackets are initially invalid. They should be overwritten by the token_hierarchy step. The ones that don't get overwritten are invalid
        } else if is_symbol(t) {
            if t == kw("@") {
                IDETokenType::PipelineStage
            } else if t == kw("#") {
                IDETokenType::TimelineStage
            } else {
                IDETokenType::Operator
            }
        } else if is_identifier(t) {
            IDETokenType::Identifier(IDEIdentifierType::Unknown)
        } else if is_number(t) {
            IDETokenType::Number
        } else if is_comment(t) {
            IDETokenType::Comment
        } else {
            IDETokenType::Invalid
        };

        result.push(IDEToken{typ : initial_typ})
    }

    add_ide_bracket_depths_recursive(&mut result, 0, &parsed.token_hierarchy);

    walk_name_color(&parsed.ast, &parsed.token_spans, file_text, &mut result);

    result
}

pub fn syntax_highlight_file(file_path : &str) {
    let file_text = std::fs::read_to_string(file_path).expect("Could not open file!"); 
    
    let (full_parse, errors) = perform_full_semantic_parse(&file_text);

    for err in errors {
        err.pretty_print_error(&file_path, &file_text)
    }
    
    print_tokens(&file_text, &full_parse.token_spans);

    let ide_tokens = create_token_ide_info(&file_text, &full_parse);
    
    
    pretty_print(&file_text, &full_parse.token_spans, &ide_tokens);
    
    println!("{:?}", full_parse.ast);
}
