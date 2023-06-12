
mod tokenizer;
mod parser;
mod errors;
use parser::*;
use tokenizer::*;

use console::{style, Style};

fn pretty_print_chunk_with_whitespace(whitespace_start : usize, file_text : &str, text_chunk : &str, st : Style) -> usize /* next whitespace_start */ { 
    let whitespace_end = text_chunk.as_ptr() as usize - file_text.as_ptr() as usize;
    let whitespace_text = file_text.get(whitespace_start..whitespace_end).unwrap();
    let new_whitespace_start = text_chunk.as_ptr() as usize + text_chunk.len() - file_text.as_ptr() as usize;

    print!("{}{}", whitespace_text, st.apply_to(text_chunk));

    return new_whitespace_start;
}

fn pretty_print(file_text : &str, token_vec : &[IDEToken], comments : &[CommentToken]) {
    let mut whitespace_start : usize = 0;

    let mut comment_iter = comments.iter().peekable();
    for (tok_idx, token) in token_vec.iter().enumerate() {
        
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
            IDETokenType::Symbol => Style::new().cyan(),
            IDETokenType::Identifier => Style::new().white(),
            IDETokenType::Number => Style::new().green().bright(),
            IDETokenType::Invalid | IDETokenType::InvalidBracket => Style::new().red().underlined(),
            IDETokenType::OpenBracket(depth) | IDETokenType::CloseBracket(depth) => {
                bracket_styles[depth % bracket_styles.len()].clone()
            }
        };
        
        whitespace_start = pretty_print_chunk_with_whitespace(whitespace_start, file_text, token.text, st);
    }

    print!("{}\n", file_text.get(whitespace_start..file_text.len()).unwrap());
}

enum IDETokenType {
    Keyword,
    Symbol,
    Identifier,
    Number,
    Invalid,
    InvalidBracket,
    OpenBracket(usize), // Bracket depth
    CloseBracket(usize) // Bracket depth
}

struct IDEToken<'a> {
    text : &'a str,
    typ : IDETokenType,
    attached_comments : &'a [CommentToken<'a>]
}

fn add_ide_bracket_depths_recursive<'a>(result : &mut Vec<IDEToken<'a>>, current_depth : usize, token_hierarchy : &[TokenTreeNode]) {
    for tok in token_hierarchy {
        if let TokenTreeNode::Block(_, sub_block, left, right) = tok {
            result[*left].typ = IDETokenType::OpenBracket(current_depth);
            add_ide_bracket_depths_recursive(result, current_depth+1, sub_block);
            result[*right].typ = IDETokenType::CloseBracket(current_depth);
        }
    }
}

fn create_token_ide_info<'a>(tokens : &[Token<'a>], token_hierarchy : &[TokenTreeNode], comments : &'a [CommentToken<'a>]) -> Vec<IDEToken<'a>> {
    let mut result : Vec<IDEToken<'a>> = Vec::new();

    for t in tokens {
        let initial_typ = if is_keyword(t.typ) {
            IDETokenType::Keyword
        } else if is_bracket(t.typ) != IsBracket::NotABracket {
            IDETokenType::InvalidBracket // Brackets are initially invalid. They should be overwritten by the token_hierarchy step. The ones that don't get overwritten are invalid
        } else if is_symbol(t.typ) {
            IDETokenType::Symbol
        } else if is_identifier(t.typ) {
            IDETokenType::Identifier
        } else if is_number(t.typ) {
            IDETokenType::Number
        } else {
            IDETokenType::Invalid
        };

        result.push(IDEToken{text : t.text, typ : initial_typ, attached_comments : &[]})
    }

    add_ide_bracket_depths_recursive(&mut result, 0, token_hierarchy);

    result
}

fn main() {
    let file_path = "multiply_add.sus";
    
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
            
            let (ast, token_hierarchy, parse_errors) = parse(&file_text, &token_vec);

            if !parse_errors.is_empty() {
                for err in parse_errors {
                    err.pretty_print_error(file_path, &file_text);
                }
            }

            let ide_tokens = create_token_ide_info(&token_vec, &token_hierarchy, &comments);

            pretty_print(&file_text, &ide_tokens, &comments);

        }
    }
}
