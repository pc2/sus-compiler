
use crate::{ast::*, tokenizer::*, parser::*};

use console::{style, Style};


enum IDEIdentifierType {
    Value(IdentifierType),
    Type,
    Interface,
    Unknown
}

enum IDETokenType {
    Keyword,
    Symbol,
    Identifier(IDEIdentifierType),
    Number,
    Invalid,
    InvalidBracket,
    OpenBracket(usize), // Bracket depth
    CloseBracket(usize) // Bracket depth
}

struct IDEToken {
    typ : IDETokenType,
    attached_comments : Vec<usize> // Comment indices
}

fn pretty_print_chunk_with_whitespace(whitespace_start : usize, file_text : &str, text_chunk : &str, st : Style) -> usize /* next whitespace_start */ { 
    let whitespace_end = text_chunk.as_ptr() as usize - file_text.as_ptr() as usize;
    let whitespace_text = file_text.get(whitespace_start..whitespace_end).unwrap();
    let new_whitespace_start = text_chunk.as_ptr() as usize + text_chunk.len() - file_text.as_ptr() as usize;

    print!("{}{}", whitespace_text, st.apply_to(text_chunk));

    return new_whitespace_start;
}

fn print_tokens<'a>(file_text : &str, token_vec : &[Token<'a>]) {
    let mut whitespace_start : usize = 0;
    for (tok_idx, token) in token_vec.iter().enumerate() {
        let styles = [Style::new().magenta(), Style::new().yellow(), Style::new().blue()];
        let st = styles[tok_idx % styles.len()].clone().underlined();
        
        whitespace_start = pretty_print_chunk_with_whitespace(whitespace_start, file_text, token.text, st);
    }

    print!("{}\n", file_text.get(whitespace_start..file_text.len()).unwrap());
}

fn pretty_print(file_text : &str, tokens : &[Token], ide_infos : &[IDEToken], comments : &[CommentToken]) {
    let mut whitespace_start : usize = 0;

    let mut comment_iter = comments.iter().peekable();
    for (tok_idx, token) in ide_infos.iter().enumerate() {
        while let Some(comment) = comment_iter.peek() {
            if comment.token_idx <= tok_idx {
                whitespace_start = pretty_print_chunk_with_whitespace(whitespace_start, file_text, comment.text, Style::new().green().dim());
                comment_iter.next(); // Actually pop it
            } else {
                break;
            }
        }

        let bracket_styles = [Style::new().magenta(), Style::new().yellow(), Style::new().blue()];
        let st = match token.typ {
            IDETokenType::Keyword => Style::new().blue(),
            IDETokenType::Symbol => Style::new().white().bright(),
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
        
        whitespace_start = pretty_print_chunk_with_whitespace(whitespace_start, file_text, tokens[tok_idx].text, st);
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
    tokens : &'a [Token<'a>]
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
            if let Some(tok_decl) = context.get_declaration_for(tok_idx, self.tokens) {
                self.ide_token_info[tok_idx].typ = IDETokenType::Identifier(IDEIdentifierType::Value(tok_decl.identifier_type));
            }
        });
        //self.ide_token_info[decl.name_token].typ = IDETokenType::Identifier(IDEIdentifierType::Value(decl.identifier_type));
    }
    fn visit_assignment(&mut self, to : &SpanExpression, expr : &SpanExpression, context : &VariableContext) {}
}

fn walk_name_color(ast : &ASTRoot, tokens : &[Token], result : &mut [IDEToken]) {
    let mut walker = NameColoringWalker{ide_token_info : result, tokens : tokens};

    walk_ast(&mut walker, ast, tokens, &VariableContext::new_initial());
}

fn create_token_ide_info<'a>(tokens : &[Token<'a>], ast : &ASTRoot, token_hierarchy : &[TokenTreeNode], comments : &'a [CommentToken<'a>]) -> Vec<IDEToken> {
    let mut result : Vec<IDEToken> = Vec::new();

    for t in tokens {
        let initial_typ = if is_keyword(t.typ) {
            IDETokenType::Keyword
        } else if is_bracket(t.typ) != IsBracket::NotABracket {
            IDETokenType::InvalidBracket // Brackets are initially invalid. They should be overwritten by the token_hierarchy step. The ones that don't get overwritten are invalid
        } else if is_symbol(t.typ) {
            IDETokenType::Symbol
        } else if is_identifier(t.typ) {
            IDETokenType::Identifier(IDEIdentifierType::Unknown)
        } else if is_number(t.typ) {
            IDETokenType::Number
        } else {
            IDETokenType::Invalid
        };

        result.push(IDEToken{typ : initial_typ, attached_comments : vec![]})
    }

    add_ide_bracket_depths_recursive(&mut result, 0, token_hierarchy);

    walk_name_color(ast, &tokens, &mut result);

    result
}



pub fn syntax_highlight_file(file_path : &str) {
    match std::fs::read_to_string(file_path) {
        Err(err) => {
            println!("Could not open file {}: {}", style(file_path).yellow(), style(err.to_string()));
        },
        Ok(file_text) => {
            let (token_vec, comments, token_errors) = tokenize(&file_text);
            
            if !token_errors.is_empty() {
                for err in token_errors {
                    err.pretty_print_error(file_path, &file_text);
                }
            }

            let (token_hierarchy, hierarchy_errors) = to_token_hierarchy(&token_vec);
            if !hierarchy_errors.is_empty() {
                for err in hierarchy_errors {
                    err.pretty_print_error(file_path, &file_text, &token_vec);
                }
            }

            let (ast, parse_errors) = parse(&token_hierarchy, token_vec.len());

            if !parse_errors.is_empty() {
                for err in parse_errors {
                    err.pretty_print_error(file_path, &file_text, &token_vec);
                }
            }
            
            print_tokens(&file_text, &token_vec);

            let ide_tokens = create_token_ide_info(&token_vec, &ast, &token_hierarchy, &comments);
            
            
            pretty_print(&file_text, &token_vec, &ide_tokens, &comments);

        }
    }
}

