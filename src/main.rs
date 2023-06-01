
mod tokenizer;
mod parser;
mod errors;
use parser::parse;
use tokenizer::*;

use console::{style, Style};

fn pretty_print_chunk_with_whitespace(whitespace_start : usize, file_text : &str, text_chunk : &str, st : Style) -> usize /* next whitespace_start */ { 
    let whitespace_end = text_chunk.as_ptr() as usize - file_text.as_ptr() as usize;
    let whitespace_text = file_text.get(whitespace_start..whitespace_end).unwrap();
    let new_whitespace_start = text_chunk.as_ptr() as usize + text_chunk.len() - file_text.as_ptr() as usize;

    print!("{}{}", whitespace_text, st.apply_to(text_chunk));

    return new_whitespace_start;
}

fn pretty_print(file_text : &str, token_vec : &[Token], comments : &[CommentToken]) {
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

        let st = if is_keyword(token.typ) {
            Style::new().blue()
        } else if is_symbol(token.typ) {
            Style::new().cyan()
        } else if is_identifier(token.typ) {
            Style::new().white()
        } else if is_number(token.typ) {
            Style::new().green().bright()
        } else {
            Style::new().red().underlined()
        };
        whitespace_start = pretty_print_chunk_with_whitespace(whitespace_start, file_text, token.text, st);
    }

    print!("{}\n", file_text.get(whitespace_start..file_text.len()).unwrap());
}

fn find_line_starts(text : &str) -> Vec<usize> {
    let mut result : Vec<usize> = vec![0];
    for (i, ch) in text.char_indices() {
        if ch == '\n' {
            result.push(i+1);
        }
    }
    result
}

fn main() {
    let file_path = "multiply_add.sus";
    
    match std::fs::read_to_string(file_path) {
        Err(err) => {
            println!("Could not open file {}: {}", style(file_path).yellow(), style(err.to_string()));
            std::process::exit(1);
        },
        Ok(file_text) => {
            let (mut token_vec, comments, token_errors) = tokenize(&file_text);
        
            if !token_errors.is_empty() {
                let line_start_buffer = find_line_starts(&file_text);
                for err in token_errors {
                    err.pretty_print(&line_start_buffer, &file_text);
                }
                std::process::exit(1);
            }
            
            parse(&mut token_vec);
            pretty_print(&file_text, &token_vec, &comments);
        }
    }
}
