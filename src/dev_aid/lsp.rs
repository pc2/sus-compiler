
use std::{error::Error, net::SocketAddr};
use lsp_types::{notification::*, request::Request, *};

use lsp_server::{Connection, Message, Response};

use lsp_types::notification::Notification;

use crate::{arena_alloc::ArenaVector, ast::{IdentifierType, Span}, dev_aid::syntax_highlighting::create_token_ide_info, errors::{CompileError, ErrorCollector, ErrorLevel}, flattening::{WireInstance, WireSource}, linker::{FileData, FileUUID, FileUUIDMarker, Linker, LocationInfo}, parser::perform_full_semantic_parse, tokenizer::{CharLine, TokenizeResult}};

use super::syntax_highlighting::{IDETokenType, IDEIdentifierType, IDEToken};

struct LoadedFileCache {
    linker : Linker,
    uris : ArenaVector<Url, FileUUIDMarker>
}

impl LoadedFileCache {
    fn new(linker : Linker, uris : ArenaVector<Url, FileUUIDMarker>) -> Self {
        Self{linker, uris}
    }
    fn find_uri(&self, uri : &Url) -> Option<FileUUID> {
        self.uris.iter()
            .find(|(_uuid, uri_found)| **uri_found == *uri)
            .map(|(uuid, _uri_found)| uuid)
    }
    fn update_text(&mut self, uri : Url, new_file_text : String) -> FileUUID {
        let found_opt = self.find_uri(&uri);
        let found_opt_was_none = found_opt.is_none();
        let file_uuid : FileUUID = found_opt.unwrap_or_else(|| self.linker.reserve_file());
        let full_parse = perform_full_semantic_parse(new_file_text, file_uuid);
        
        if found_opt_was_none {
            self.linker.add_reserved_file(file_uuid, full_parse);
            self.uris.insert(file_uuid, uri.clone());
        } else {
            self.linker.relink(file_uuid, full_parse);
        }
        self.linker.recompile_all();

        file_uuid
    }
    fn ensure_contains_file(&mut self, uri : &Url) -> FileUUID {
        if let Some(found) = self.find_uri(uri) {
            found
        } else {
            let file_uuid = self.linker.reserve_file();
            let file_text = std::fs::read_to_string(uri.to_file_path().unwrap()).unwrap();
            let full_parse = perform_full_semantic_parse(file_text, file_uuid);
            self.linker.add_reserved_file(file_uuid, full_parse);
            self.uris.insert(file_uuid, uri.clone());
            self.linker.recompile_all();
            file_uuid
        }
    }
}

pub fn lsp_main(port : u16, debug : bool) -> Result<(), Box<dyn Error + Sync + Send>> {
    println!("starting LSP server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    //let (connection, io_threads) = Connection::listen(SocketAddr::from(([127,0,0,1], 25000)))?;
    println!("Connecting on port {}...", port);
    let (connection, io_threads) = Connection::connect(SocketAddr::from(([127,0,0,1], port)))?;
    println!("connection established");
    
    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        definition_provider: Some(OneOf::Left(true)),
        document_highlight_provider: Some(OneOf::Right(
            DocumentHighlightOptions{
                work_done_progress_options: WorkDoneProgressOptions{
                    work_done_progress: Some(false)
                }
            }
        )),
        hover_provider : Some(HoverProviderCapability::Simple(true)),
        semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions{
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
        })),
        /*workspace: Some(WorkspaceClientCapabilities{
            did_change_watched_files : Some(DidChangeWatchedFilesClientCapabilities{

            }),
            ..Default::default()
        }),*/
        text_document_sync : Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::FULL
        )),
        ..Default::default()
    })
    .unwrap();
    let initialization_params = connection.initialize(server_capabilities)?;
    main_loop(connection, initialization_params, debug)?;
    io_threads.join()?;

    // Shut down gracefully.
    println!("shutting down server");
    Ok(())
}

fn get_semantic_token_type_from_ide_token(tok : &IDEToken) -> u32 {
    match &tok.typ {
        IDETokenType::Comment => 0,
        IDETokenType::Keyword => 1,
        IDETokenType::Operator => 2,
        IDETokenType::TimelineStage => 8,// EVENT seems to get a good colour
        IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::Input)) => 4,
        IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::Output)) => 4,
        IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::State)) => 3,
        IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::Local)) => 3,
        IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::Generative)) => 3,
        IDETokenType::Identifier(IDEIdentifierType::Constant) => 9, // make it 'OPERATOR'?
        IDETokenType::Identifier(IDEIdentifierType::Unknown) => 2, // make it 'OPERATOR'?
        IDETokenType::Identifier(IDEIdentifierType::Interface) => 7, // FUNCTION
        IDETokenType::Identifier(IDEIdentifierType::Type) => 5, // All others are 'TYPE'
        IDETokenType::Number => 6,
        IDETokenType::Invalid => 2, // make it 'OPERATOR'?
        IDETokenType::InvalidBracket => 2, // make it 'OPERATOR'?
        IDETokenType::OpenBracket(_) => 2,
        IDETokenType::CloseBracket(_) => 2,
    }
}

// Produces a bitset with 'modifier bits'
fn get_modifiers_for_token(tok : &IDEToken) -> u32 {
    match &tok.typ {
        IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::State)) => 1, // repurpose ASYNC for "State"
        IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::Generative)) => 8, // repurpose READONLY
        _other => 0
    }
}

fn from_position(pos : lsp_types::Position) -> CharLine {
    CharLine{line : pos.line as usize, character : pos.character as usize}
}
fn from_position_range(range : lsp_types::Range) -> std::ops::Range<CharLine> {
    std::ops::Range{start : from_position(range.start), end : from_position(range.end)}
}
fn to_position(char_line : CharLine) -> lsp_types::Position {
    lsp_types::Position{line : char_line.line as u32, character : char_line.character as u32}
}
fn to_position_range(range : std::ops::Range<CharLine>) -> lsp_types::Range {
    lsp_types::Range{start : to_position(range.start), end : to_position(range.end)}
}

fn do_syntax_highlight(file_data : &FileData, linker : &Linker) -> Vec<SemanticToken> {
    let ide_tokens = create_token_ide_info(&file_data, linker);

    let mut cursor = Position {line : 0, character : 0};
    let mut semantic_tokens = Vec::with_capacity(file_data.tokens.len());

    for (tok_idx, ide_tok) in ide_tokens.iter().enumerate() {
        let typ = get_semantic_token_type_from_ide_token(ide_tok);
        let mod_bits = get_modifiers_for_token(ide_tok);

        let tok_range = file_data.tokens.get_token_linechar_range(tok_idx);
        let start_pos = to_position(tok_range.start);
        let end_pos = to_position(tok_range.end);

        let delta_line = start_pos.line - cursor.line;

        if delta_line != 0 {
            cursor.character = 0;
        }

        let delta_col = start_pos.character - cursor.character;
        cursor = start_pos;

        semantic_tokens.push(SemanticToken{
            delta_line: delta_line,
            delta_start: delta_col,
            length: end_pos.character - start_pos.character,
            token_type: typ,
            token_modifiers_bitset: mod_bits,
        });
    }

    semantic_tokens
}

use lsp_types::Diagnostic;

fn cvt_span_to_lsp_range(ch_sp : Span, tokens : &TokenizeResult) -> lsp_types::Range {
    let rng = tokens.get_span_linechar_range(ch_sp);
    Range {
        start: Position{character : rng.start.character as u32, line : rng.start.line as u32},
        end: Position{character : rng.end.character as u32, line : rng.end.line as u32}
    }
}

// Requires that token_positions.len() == tokens.len() + 1 to include EOF token
fn convert_diagnostic(err : CompileError, tokens : &TokenizeResult, uris : &ArenaVector<Url, FileUUIDMarker>) -> Diagnostic {
    let error_pos = cvt_span_to_lsp_range(err.position, tokens);

    let severity = match err.level {
        ErrorLevel::Error => DiagnosticSeverity::ERROR,
        ErrorLevel::Warning => DiagnosticSeverity::WARNING,
    };
    let mut related_info = Vec::new();
    for info in err.infos {
        let info_pos = cvt_span_to_lsp_range(info.position, tokens);
        let location = Location{uri : uris[info.file].clone(), range : info_pos};
        related_info.push(DiagnosticRelatedInformation { location, message: info.info });
    }
    Diagnostic::new(error_pos, Some(severity), None, None, err.reason, Some(related_info), None)
}

// Requires that token_positions.len() == tokens.len() + 1 to include EOF token
fn send_errors_warnings(connection: &Connection, errors : ErrorCollector, token_boundaries : &TokenizeResult, uris : &ArenaVector<Url, FileUUIDMarker>) -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut diag_vec : Vec<Diagnostic> = Vec::new();
    let (err_vec, file) = errors.get();
    for err in err_vec {
        diag_vec.push(convert_diagnostic(err, token_boundaries, uris));
    }
    
    let params = &PublishDiagnosticsParams{
        uri: uris[file].clone(),
        diagnostics: diag_vec,
        version: None
    };
    let params_json = serde_json::to_value(params)?;

    connection.sender.send(Message::Notification(lsp_server::Notification{
        method: PublishDiagnostics::METHOD.to_owned(),
        params: params_json
    }))?;

    Ok(())
}

fn get_hover_info<'l>(file_cache : &'l LoadedFileCache, text_pos : &lsp_types::TextDocumentPositionParams) -> Option<(LocationInfo<'l>, Option<lsp_types::Range>)> {
    let uuid = file_cache.find_uri(&text_pos.text_document.uri).unwrap();
    
    let file_data = &file_cache.linker.files[uuid];

    let token_idx = file_data.tokens.get_token_on_or_left_of(from_position(text_pos.position));

    let (info, span) = file_cache.linker.get_info_about_source_location(token_idx, uuid)?;
    //let span = Span::new_single_token(token_idx);

    let char_line_range = file_data.tokens.get_span_linechar_range(span);
    Some((info, Some(to_position_range(char_line_range))))
}

fn main_loop(connection: Connection, params: serde_json::Value, debug : bool) -> Result<(), Box<dyn Error + Sync + Send>> {

    let mut file_cache = LoadedFileCache::new(Linker::new(), ArenaVector::new());

    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    println!("starting LSP main loop");
    for msg in &connection.receiver {
        println!("got msg: {msg:?}");
        match msg {
            Message::Request(req) => {
                let response_value = match req.method.as_str() {
                    request::Shutdown::METHOD => {
                        println!("Shutdown request");
                        return Ok(());
                    }
                    request::HoverRequest::METHOD => {
                        let params : HoverParams = serde_json::from_value(req.params).expect("JSON Encoding Error while parsing params");
                        println!("got hover request: {params:?}");

                        file_cache.ensure_contains_file(&params.text_document_position_params.text_document.uri);
                        serde_json::to_value(&if let Some((info, range)) = get_hover_info(&file_cache, &params.text_document_position_params) {
                            let mut hover_list : Vec<MarkedString> = Vec::new();
                            if debug {
                                hover_list.push(MarkedString::String(format!("{info:?}")))
                            } else {
                                match info {
                                    LocationInfo::WireRef(md, decl_id) => {
                                        let decl = md.flattened.instructions[decl_id].extract_wire_declaration();
                                        let typ_str = decl.typ.to_string(&file_cache.linker.types);
                                        let name_str = &decl.name;
                                        hover_list.push(MarkedString::String(format!("{typ_str} {name_str}")));
                                        
                                        md.instantiations.for_each_instance(|inst| {
                                            for (_id, wire) in &inst.wires {
                                                if wire.original_wire != decl_id {continue}
                                                let typ_str = wire.typ.to_string(&file_cache.linker.types);
                                                let name_str = &wire.name;
                                                let latency = wire.absolute_latency;
                                                hover_list.push(MarkedString::String(format!("{typ_str} {name_str}'{latency}")));
                                            }
                                        });
                                    }
                                    LocationInfo::Temporary(md, id, wire) => {
                                        let typ_str = wire.typ.to_string(&file_cache.linker.types);

                                        hover_list.push(MarkedString::String(format!("{typ_str}")));
                                        md.instantiations.for_each_instance(|inst| {
                                            for (_id, wire) in &inst.wires {
                                                if wire.original_wire != id {continue}
                                                let typ_str = wire.typ.to_string(&file_cache.linker.types);
                                                let name_str = &wire.name;
                                                let latency = wire.absolute_latency;
                                                hover_list.push(MarkedString::String(format!("{typ_str} {name_str}'{latency}")));
                                            }
                                        });
                                    }
                                    LocationInfo::Type(typ) => {
                                        hover_list.push(MarkedString::String(typ.to_type().to_string(&file_cache.linker.types)));
                                    }
                                    LocationInfo::Global(global) => {
                                        hover_list.push(MarkedString::String(file_cache.linker.get_full_name(global)));
                                    }
                                };
                            }
                            Hover{contents: HoverContents::Array(hover_list), range}
                        } else {
                            Hover{contents: HoverContents::Array(Vec::new()), range: None}
                        })
                    }
                    request::GotoDefinition::METHOD => {
                        let params : GotoDefinitionParams = serde_json::from_value(req.params).expect("JSON Encoding Error while parsing params");
                        println!("got gotoDefinition request: {params:?}");

                        file_cache.ensure_contains_file(&params.text_document_position_params.text_document.uri);
                        serde_json::to_value(&if let Some((info, range)) = get_hover_info(&file_cache, &params.text_document_position_params) {
                            
                            
                            GotoDefinitionResponse::Array(Vec::new())
                        } else {
                            GotoDefinitionResponse::Array(Vec::new())
                        })
                    }
                    request::SemanticTokensFullRequest::METHOD => {
                        let params : SemanticTokensParams = serde_json::from_value(req.params).expect("JSON Encoding Error while parsing params");
                        println!("got fullSemanticTokens request: {params:?}");

                        let uuid = file_cache.ensure_contains_file(&params.text_document.uri);
                        
                        let file_data = &file_cache.linker.files[uuid];

                        serde_json::to_value(&SemanticTokensResult::Tokens(lsp_types::SemanticTokens {result_id: None, data: do_syntax_highlight(file_data, &file_cache.linker)}))
                    }
                    // TODO ...
                    req => {
                        println!("Other request: {req:?}");
                        continue;
                    }
                };

                let result = response_value.unwrap();
                let response = Response { id : req.id, result: Some(result), error: None };
                connection.sender.send(Message::Response(response))?;
            }
            Message::Response(resp) => {
                println!("got response: {resp:?}");
            }
            Message::Notification(not) => {
                match not.method.as_str() {
                    notification::DidChangeTextDocument::METHOD => {
                        let params : DidChangeTextDocumentParams = serde_json::from_value(not.params).expect("JSON Encoding Error while parsing params");
                        let uuid = file_cache.update_text(params.text_document.uri, params.content_changes.into_iter().next().unwrap().text);

                        // println!("Flattening...");
                        file_cache.linker.recompile_all();

                        let file_data = &file_cache.linker.files[uuid]; // Have to grab it again because previous line mutates

                        let mut errors = file_data.parsing_errors.clone();
                        file_cache.linker.get_all_errors_in_file(uuid, &mut errors);

                        // println!("Errors: {:?}", &errors);
                        send_errors_warnings(&connection, errors, &file_data.tokens, &file_cache.uris)?;
                    }
                    other => {
                        println!("got notification: {other:?}");
                    }
                }
            }
        }
    }
    Ok(())
}
