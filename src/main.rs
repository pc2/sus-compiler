
mod tokenizer;
mod parser;
use tokenizer::{tokenize,ParsingErr};

use console::{style, Style};

fn pretty_print_chunk_with_whitespace(whitespace_start : usize, file_text : &str, text_chunk : &str, st : Style) -> usize /* next whitespace_start */ { 
    let whitespace_end = text_chunk.as_ptr() as usize - file_text.as_ptr() as usize;
    let whitespace_text = file_text.get(whitespace_start..whitespace_end).unwrap();
    let new_whitespace_start = text_chunk.as_ptr() as usize + text_chunk.len() - file_text.as_ptr() as usize;

    print!("{}{}", whitespace_text, st.apply_to(text_chunk));

    return new_whitespace_start;
}

fn pretty_print(file_text : &str) {
    let (token_vec, comments, _token_errors) = tokenize(file_text);

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

        let st = if token.is_keyword() {
            Style::new().blue()
        } else if token.is_symbol() {
            Style::new().cyan()
        } else if token.is_identifier() {
            Style::new().white()
        } else if token.is_number() {
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

struct FilePosition {
    pub line : usize,
    pub col : usize,
}

fn to_file_position(line_start_buffer : &[usize], char_i : usize) -> FilePosition {
    let line = match line_start_buffer.binary_search_by(|probe| probe.cmp(&char_i)) {
        Ok(v) => v,
        Err(v) => v-1
    };
    let col = char_i - line_start_buffer[line];
    FilePosition{line : line, col : col}
}

fn pretty_print_error(line_start_buffer : &[usize], text : &str, err : &ParsingErr) {
    let total_lines = line_start_buffer.len();

    let part_start = err.position.as_ptr() as usize - text.as_ptr() as usize;
    let part_end = part_start + err.position.len();

    let err_start = to_file_position(line_start_buffer, part_start);
    let err_end = to_file_position(line_start_buffer, part_end);

    const LINES_BEFORE_MARGIN : usize = 3;
    const LINES_AFTER_MARGIN : usize = 3;

    let before_margin_line = if err_start.line < LINES_BEFORE_MARGIN {0} else {err_start.line - LINES_BEFORE_MARGIN};
    let after_margin_line = if err_end.line > total_lines - LINES_AFTER_MARGIN {total_lines} else {err_start.line + LINES_BEFORE_MARGIN};

    print!("{}", text.get(line_start_buffer[before_margin_line]..part_start).unwrap());
    print!("{}", style(err.position).red().underlined());
    print!("{}", text.get(part_end..line_start_buffer[err_end.line+1]).unwrap());
    print!("{}{}\n", " ".repeat(err_start.col), style("^ ".to_owned() + err.reason).red());
    print!("{}", text.get(line_start_buffer[err_end.line+1]..line_start_buffer[after_margin_line+1]).unwrap());
}

fn main() {
    let file_path = "multiply_add.vpp";
    
    match std::fs::read_to_string(file_path) {
        Ok(file_text) => {
            let (_token_vec, _comments, token_errors) = tokenize(&file_text);
        
            if !token_errors.is_empty() {
                let line_start_buffer = find_line_starts(&file_text);
                for err in token_errors {
                    pretty_print_error(&line_start_buffer, &file_text, &err);
                }
                std::process::exit(1);
            } else {
                pretty_print(&file_text);
                std::process::exit(0);
            }
        },
        Err(err) => {
            println!("Could not open file {}: {}", style(file_path).yellow(), style(err.to_string()));
            std::process::exit(1);
        }
    }
}
