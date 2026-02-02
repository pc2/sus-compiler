use crate::{flattening::InterfaceDeclKind, prelude::*, typing::template::TemplateKind};

use lsp_types::{
    Position, SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokensFullOptions,
    SemanticTokensLegend, SemanticTokensOptions, SemanticTokensServerCapabilities,
    WorkDoneProgressOptions,
};

use crate::{
    dev_aid::lsp::to_position,
    linker::{FileData, GlobalUUID},
};

use super::tree_walk::{self, InGlobal, LocationInfo};

const NUM_INTERFACE_DISTINGUISHERS: u32 = 5;
const TOKEN_TYPES: [SemanticTokenType; 8] = [
    SemanticTokenType::VARIABLE, // These are all for distinguishing interfaces
    SemanticTokenType::STRING,
    SemanticTokenType::NUMBER,
    SemanticTokenType::REGEXP,
    SemanticTokenType::COMMENT,
    SemanticTokenType::ENUM_MEMBER,
    SemanticTokenType::FUNCTION,
    SemanticTokenType::TYPE,
];

fn get_semantic_token_type_from_ide_token(tok: IDEIdentifierType) -> u32 {
    match tok {
        IDEIdentifierType::Local {
            is_state: _,
            domain: interface,
        } => interface % NUM_INTERFACE_DISTINGUISHERS,
        IDEIdentifierType::Generative => NUM_INTERFACE_DISTINGUISHERS, // ENUM_MEMBER
        IDEIdentifierType::Constant => NUM_INTERFACE_DISTINGUISHERS,   // ENUM_MEMBER
        IDEIdentifierType::Interface => NUM_INTERFACE_DISTINGUISHERS + 1, // FUNCTION
        IDEIdentifierType::Type => NUM_INTERFACE_DISTINGUISHERS + 2,   // TYPE
    }
}

const TOKEN_MODIFIERS: [SemanticTokenModifier; 2] = [
    SemanticTokenModifier::ASYNC,        // "State"
    SemanticTokenModifier::MODIFICATION, // "Generative"
];
// Produces a bitset with 'modifier bits'
fn get_modifiers_for_token(tok: IDEIdentifierType) -> u32 {
    match tok {
        IDEIdentifierType::Local {
            is_state: true,
            domain: _,
        } => 1, // ASYNC
        IDEIdentifierType::Generative => 2, // MODIFICATION
        _other => 0,
    }
}

pub fn semantic_token_capabilities() -> SemanticTokensServerCapabilities {
    SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
        work_done_progress_options: WorkDoneProgressOptions {
            work_done_progress: Some(false),
        },
        legend: SemanticTokensLegend {
            token_types: Vec::from(TOKEN_TYPES),
            token_modifiers: Vec::from(TOKEN_MODIFIERS),
        },
        range: Some(false), // Don't support ranges yet
        full: Some(SemanticTokensFullOptions::Bool(true)), // TODO: Support delta updating for faster syntax highlighting, just do whole file for now
    })
}

fn convert_to_semantic_tokens(
    file_data: &FileData,
    ide_tokens: &mut [(Span, IDEIdentifierType)],
) -> Vec<SemanticToken> {
    ide_tokens.sort_by(|a, b| a.0.cmp(&b.0));

    let mut cursor = Position {
        line: 0,
        character: 0,
    };

    ide_tokens
        .iter_mut()
        .map(|(span, ide_kind)| {
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

            SemanticToken {
                delta_line,
                delta_start: delta_col,
                length: end_pos.character - start_pos.character,
                token_type: typ,
                token_modifiers_bitset: mod_bits,
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IDEIdentifierType {
    Local { is_state: bool, domain: u32 },
    Generative,
    Type,
    Interface,
    Constant,
}

impl IDEIdentifierType {
    fn make_local(is_state: bool, domain: ClockID) -> IDEIdentifierType {
        IDEIdentifierType::Local {
            is_state,
            domain: domain.get_hidden_value() as u32,
        }
    }
}

fn walk_name_color(file: &FileData, linker: &Linker) -> Vec<(Span, IDEIdentifierType)> {
    let mut result: Vec<(Span, IDEIdentifierType)> = Vec::new();

    tree_walk::visit_all(linker, file, |span, item| {
        result.push((
            span,
            match item {
                LocationInfo::InGlobal(_md_id, _md, _, InGlobal::NamedLocal(decl)) => {
                    if decl.decl_kind.is_generative() {
                        IDEIdentifierType::Generative
                    } else {
                        IDEIdentifierType::make_local(
                            decl.decl_kind.is_state(),
                            decl.clock_domain.unwrap_physical(),
                        )
                    }
                }
                LocationInfo::InGlobal(_md_id, _, _, InGlobal::NamedSubmodule(_)) => {
                    IDEIdentifierType::Interface
                }
                LocationInfo::InGlobal(_md_id, _, _, InGlobal::LocalInterface(_)) => {
                    IDEIdentifierType::Interface
                }
                LocationInfo::InGlobal(_md_id, _, _, InGlobal::Temporary(_)) => return,
                LocationInfo::Type(_, _) => return,
                LocationInfo::Parameter(_id, _link_info, _, template_arg) => {
                    match &template_arg.kind {
                        TemplateKind::Type(_) => IDEIdentifierType::Type,
                        TemplateKind::Value(_) => IDEIdentifierType::Generative,
                    }
                }
                LocationInfo::Global(g) => match g {
                    GlobalUUID::Module(_) => IDEIdentifierType::Interface,
                    GlobalUUID::Type(_) => IDEIdentifierType::Type,
                    GlobalUUID::Constant(_) => IDEIdentifierType::Constant,
                },
                LocationInfo::Interface(_, md, _, i) => match i.declaration_instruction.unwrap() {
                    InterfaceDeclKind::SinglePort(decl_id) => {
                        let domain = md.link_info.instructions[decl_id]
                            .unwrap_declaration()
                            .clock_domain
                            .unwrap_physical();
                        IDEIdentifierType::make_local(false, domain)
                    }
                    InterfaceDeclKind::Interface(_decl_id) => IDEIdentifierType::Interface,
                },
            },
        ));
    });

    result
}

pub fn make_semantic_tokens(uuid: FileUUID, linker: &Linker) -> lsp_types::SemanticTokens {
    let file_data = &linker.files[uuid];

    let mut ide_tokens = walk_name_color(file_data, linker);

    let data = convert_to_semantic_tokens(file_data, &mut ide_tokens);

    lsp_types::SemanticTokens {
        result_id: None,
        data,
    }
}
