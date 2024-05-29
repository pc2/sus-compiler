
use lsp_types::{Position, SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions, SemanticTokensServerCapabilities, WorkDoneProgressOptions};

use crate::{dev_aid::lsp::to_position, file_position::Span, flattening::IdentifierType, linker::{FileData, FileUUID, Linker, NameElem}};

use super::tree_walk::{self, InModule, LocationInfo};


pub fn semantic_token_capabilities() -> SemanticTokensServerCapabilities {
    SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions{
        work_done_progress_options: WorkDoneProgressOptions {
            work_done_progress: Some(false)
        },
        legend: SemanticTokensLegend{
            token_types: vec![
                SemanticTokenType::COMMENT, // When updating, see ['get_semantic_token_type_from_ide_token']
                SemanticTokenType::KEYWORD,
                SemanticTokenType::OPERATOR,
                SemanticTokenType::VARIABLE,
                SemanticTokenType::PARAMETER,
                SemanticTokenType::TYPE,
                SemanticTokenType::NUMBER,
                SemanticTokenType::FUNCTION,
                SemanticTokenType::EVENT,
                SemanticTokenType::ENUM_MEMBER
            ],
            token_modifiers: vec![
                SemanticTokenModifier::ASYNC, // repurpose ASYNC for "State"
                SemanticTokenModifier::DECLARATION,
                SemanticTokenModifier::DEFINITION,
                SemanticTokenModifier::READONLY
            ],
        },
        range: Some(false), // Don't support ranges yet
        full: Some(SemanticTokensFullOptions::Bool(true)), // TODO: Support delta updating for faster syntax highlighting, just do whole file for now
    })
}


fn get_semantic_token_type_from_ide_token(tok : IDEIdentifierType) -> u32 {
    match tok {
        IDEIdentifierType::Local { is_state:false } => 4,
        IDEIdentifierType::Local { is_state:true } => 3,
        IDEIdentifierType::Generative => 3,
        IDEIdentifierType::Constant => 9, // make it 'OPERATOR'?
        IDEIdentifierType::Interface => 7, // FUNCTION
        IDEIdentifierType::Type => 5, // All others are 'TYPE'
    }
}

// Produces a bitset with 'modifier bits'
fn get_modifiers_for_token(tok : IDEIdentifierType) -> u32 {
    match tok {
        IDEIdentifierType::Local { is_state: true } => 1, // repurpose ASYNC for "State"
        IDEIdentifierType::Generative => 8, // repurpose READONLY
        _other => 0
    }
}


fn convert_to_semantic_tokens(file_data : &FileData, ide_tokens : &mut[(Span, IDEIdentifierType)]) -> Vec<SemanticToken> {
    ide_tokens.sort_by(|a, b| a.0.cmp(&b.0));
    
    let mut cursor = Position {line : 0, character : 0};

    ide_tokens.into_iter().map(|(span, ide_kind)| {
        let typ = get_semantic_token_type_from_ide_token(*ide_kind);
        let mod_bits = get_modifiers_for_token(*ide_kind);

        let tok_range = file_data.file_text.get_span_linecol_range(*span);
        let start_pos = to_position(tok_range.start);
        let end_pos = to_position(tok_range.end);

        assert!(end_pos.line == start_pos.line);

        let delta_line = start_pos.line - cursor.line;

        if delta_line != 0 {
            cursor.character = 0;
        }

        let delta_col = start_pos.character - cursor.character;
        cursor = start_pos;

        SemanticToken{
            delta_line: delta_line,
            delta_start: delta_col,
            length: end_pos.character - start_pos.character,
            token_type: typ,
            token_modifiers_bitset: mod_bits,
        }
    }).collect()
}


#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum IDEIdentifierType {
    Local{is_state : bool},
    Generative,
    Type,
    Interface,
    Constant
}

fn ide_from_identifier_typ(t : IdentifierType) -> IDEIdentifierType {
    match t {
        IdentifierType::Input | IdentifierType::Output | IdentifierType::Local => IDEIdentifierType::Local { is_state: false },
        IdentifierType::State =>  IDEIdentifierType::Local { is_state: true },
        IdentifierType::Generative => IDEIdentifierType::Generative
    }
}

fn walk_name_color(file : &FileData, linker : &Linker) -> Vec<(Span, IDEIdentifierType)> {
    let mut result : Vec<(Span, IDEIdentifierType)> = Vec::new();

    tree_walk::visit_all(linker, file, |span, item| {
        result.push((span, match item {
            LocationInfo::InModule(_, _md_id, _, InModule::NamedLocal(decl)) => {
                ide_from_identifier_typ(decl.identifier_type)
            }
            LocationInfo::InModule(_, _md_id, _, InModule::NamedSubmodule(_)) => {
                IDEIdentifierType::Interface
            }
            LocationInfo::InModule(_, _md_id, _, InModule::Temporary(_)) => {return}
            LocationInfo::Type(_) => {return}
            LocationInfo::Global(g) => {
                match g {
                    NameElem::Module(_) => IDEIdentifierType::Interface,
                    NameElem::Type(_) => IDEIdentifierType::Type,
                    NameElem::Constant(_) => IDEIdentifierType::Constant,
                }
            }
            LocationInfo::Port(_, _, _, port) => {
                ide_from_identifier_typ(port.identifier_type)
            }
            LocationInfo::Interface(_, _, _, _) => {
                IDEIdentifierType::Interface
            }
        }));
    });

    result
}

pub fn make_semantic_tokens(uuid : FileUUID, linker : &Linker) -> lsp_types::SemanticTokens {
    let file_data = &linker.files[uuid];

    let mut ide_tokens = walk_name_color(file_data, linker);

    let data = convert_to_semantic_tokens(file_data, &mut ide_tokens);

    lsp_types::SemanticTokens { result_id: None, data }
}
