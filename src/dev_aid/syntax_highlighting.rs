
use std::{ops::Range, path::PathBuf};

use crate::{ast::*, tokenizer::*, parser::*, linker::{PreLinker, FileData, Links, ValueUUID, NamedValue, Named, Linkable}, arena_alloc::ArenaVector};

use ariadne::FileCache;
use console::Style;


#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum IDEIdentifierType {
    Value(IdentifierType),
    Type,
    Interface,
    Constant,
    Unknown
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum IDETokenType {
    Comment,
    Keyword,
    Operator,
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

fn pretty_print_chunk_with_whitespace(whitespace_start : usize, file_text : &str, text_span : Range<usize>, st : Style) { 
    let whitespace_text = &file_text[whitespace_start..text_span.start];

    print!("{}{}", whitespace_text, st.apply_to(&file_text[text_span]));
}

fn print_tokens(file_text : &str, tokens : &[Token]) {
    let mut whitespace_start : usize = 0;
    for (tok_idx, token) in tokens.iter().enumerate() {
        let styles = [Style::new().magenta(), Style::new().yellow(), Style::new().blue()];
        let st = styles[tok_idx % styles.len()].clone().underlined();
        
        let token_range = token.get_range();
        pretty_print_chunk_with_whitespace(whitespace_start, file_text, token_range.clone(), st);
        whitespace_start = token_range.end;
    }

    print!("{}\n", &file_text[whitespace_start..file_text.len()]);
}

fn pretty_print(file_text : &str, tokens : &[Token], ide_infos : &[IDEToken]) {
    let mut whitespace_start : usize = 0;

    for (tok_idx, token) in ide_infos.iter().enumerate() {
        let bracket_styles = [Style::new().magenta(), Style::new().yellow(), Style::new().blue()];
        let st = match token.typ {
            IDETokenType::Comment => Style::new().green().dim(),
            IDETokenType::Keyword => Style::new().blue(),
            IDETokenType::Operator => Style::new().white().bright(),
            IDETokenType::TimelineStage => Style::new().red().bold(),
            IDETokenType::Identifier(IDEIdentifierType::Unknown) => Style::new().red().underlined(),
            IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::Local)) => Style::new().blue().bright(),
            IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::State)) => Style::new().blue().bright().underlined(),
            IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::Input)) => Style::new().blue().bright(),
            IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::Output)) => Style::new().blue().dim(),
            IDETokenType::Identifier(IDEIdentifierType::Constant) => Style::new().blue().bold(),
            IDETokenType::Identifier(IDEIdentifierType::Type) => Style::new().magenta().bright(),
            IDETokenType::Identifier(IDEIdentifierType::Interface) => Style::new().yellow(),
            IDETokenType::Number => Style::new().green().bright(),
            IDETokenType::Invalid | IDETokenType::InvalidBracket => Style::new().red().underlined(),
            IDETokenType::OpenBracket(depth) | IDETokenType::CloseBracket(depth) => {
                bracket_styles[depth % bracket_styles.len()].clone()
            }
        };
        
        let tok_span = tokens[tok_idx].get_range();
        pretty_print_chunk_with_whitespace(whitespace_start, file_text, tok_span.clone(), st);
        whitespace_start = tok_span.end;
    }

    print!("{}\n", &file_text[whitespace_start..file_text.len()]);
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

impl Named {
    fn get_ide_type(&self) -> IDEIdentifierType{
        match self {
            Named::Value(NamedValue::Module(_)) => IDEIdentifierType::Interface,
            Named::Value(NamedValue::Constant(_)) => IDEIdentifierType::Constant,
            Named::Type(_) => IDEIdentifierType::Type,
        }
    }
}

fn walk_name_color(all_objects : &[ValueUUID], links : &Links, result : &mut [IDEToken]) {
    for obj_uuid in all_objects {
        let object = &links.globals[*obj_uuid];
        match object {
            Named::Value(NamedValue::Module(module)) => {
                module.for_each_value(&mut |name, position| {
                    result[position].typ = IDETokenType::Identifier(if let LocalOrGlobal::Local(l) = name {
                        IDEIdentifierType::Value(module.declarations[l].identifier_type)
                    } else {
                        IDEIdentifierType::Unknown
                    });
                });       
            }
            _other => {}
        }
        
        let link_info = object.get_link_info().unwrap();
        result[link_info.name_token].typ = IDETokenType::Identifier(object.get_ide_type());
        for GlobalReference(reference_span, ref_uuid) in &link_info.global_references {
            let typ = if *ref_uuid != ValueUUID::INVALID {
                IDETokenType::Identifier(links.globals[*ref_uuid].get_ide_type())
            } else {
                IDETokenType::Invalid
            };
            for part_tok in *reference_span {
                result[part_tok].typ = typ;
            }
        }
    }
}

pub fn create_token_ide_info<'a>(parsed: &FileData, links : &Links) -> Vec<IDEToken> {
    let mut result : Vec<IDEToken> = Vec::new();
    result.reserve(parsed.tokens.len());

    for t in &parsed.tokens {
        let tok_typ = t.get_type();
        let initial_typ = if is_keyword(tok_typ) {
            IDETokenType::Keyword
        } else if is_bracket(tok_typ) != IsBracket::NotABracket {
            IDETokenType::InvalidBracket // Brackets are initially invalid. They should be overwritten by the token_hierarchy step. The ones that don't get overwritten are invalid
        } else if is_symbol(tok_typ) {
            if tok_typ == kw("#") {
                IDETokenType::TimelineStage
            } else {
                IDETokenType::Operator
            }
        } else if tok_typ == TOKEN_IDENTIFIER {
            IDETokenType::Identifier(IDEIdentifierType::Unknown)
        } else if tok_typ == TOKEN_NUMBER {
            IDETokenType::Number
        } else if tok_typ == TOKEN_COMMENT {
            IDETokenType::Comment
        } else {
            assert!(tok_typ == TOKEN_INVALID);
            IDETokenType::Invalid
        };

        result.push(IDEToken{typ : initial_typ})
    }

    add_ide_bracket_depths_recursive(&mut result, 0, &parsed.token_hierarchy);

    walk_name_color(&parsed.associated_values, links, &mut result);

    result
}

// Outputs character_offsets.len() == tokens.len() + 1 to include EOF token
fn generate_character_offsets(file_text : &str, tokens : &[Token]) -> Vec<Range<usize>> {
    let mut character_offsets : Vec<Range<usize>> = Vec::new();
    character_offsets.reserve(tokens.len());
    
    let mut cur_char = 0;
    let mut whitespace_start = 0;
    for tok in tokens {
        let tok_range = tok.get_range();

        // whitespace
        cur_char += file_text[whitespace_start..tok_range.start].chars().count();
        let token_start_char = cur_char;
        
        // actual text
        cur_char += file_text[tok_range.clone()].chars().count();
        character_offsets.push(token_start_char..cur_char);
        whitespace_start = tok_range.end;
    }

    // Final char offset for EOF
    let num_chars_in_file = cur_char + file_text[whitespace_start..].chars().count();
    character_offsets.push(cur_char..num_chars_in_file);

    character_offsets
}

pub fn syntax_highlight_file(file_paths : Vec<PathBuf>) {
    let mut prelinker : PreLinker = PreLinker::new();
    let mut paths_arena = ArenaVector::new();
    for file_path in file_paths {
        let uuid = prelinker.reserve_file();
        let file_text = match std::fs::read_to_string(&file_path) {
            Ok(file_text) => file_text,
            Err(reason) => {
                let file_path_disp = file_path.display();
                panic!("Could not open file '{file_path_disp}' for syntax highlighting because {reason}")
            }
        };
        
        let (full_parse, errors) = perform_full_semantic_parse(&file_text, uuid);
        
        println!("{:?}", full_parse.ast);

        prelinker.add_reserved_file(uuid, file_text, full_parse, errors);
        paths_arena.insert(uuid, file_path);
    }

    let linker = prelinker.link();

    let mut file_cache : FileCache = Default::default();
    
    for (file_uuid, f) in &linker.files {
        print_tokens(&f.file_text, &f.tokens);

        let ide_tokens = create_token_ide_info(f, &linker.links);
        pretty_print(&f.file_text, &f.tokens, &ide_tokens);

        let token_offsets = generate_character_offsets(&f.file_text, &f.tokens);

        let mut errors = f.parsing_errors.clone();
        linker.get_linking_errors(file_uuid, &mut errors);

        linker.flatten_all_modules_in_file(file_uuid, &mut errors);

        for err in errors.errors {
            err.pretty_print_error(f.parsing_errors.file, &token_offsets, &paths_arena, &mut file_cache);
        }
    }
}
