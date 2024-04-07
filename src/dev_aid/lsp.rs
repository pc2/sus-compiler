
use std::{error::Error, ffi::OsStr, net::SocketAddr};
use lsp_types::{notification::*, request::Request, *};

use lsp_server::{Connection, Message, Response};

use lsp_types::notification::Notification;

use crate::{
    arena_alloc::ArenaVector, ast::{IdentifierType, Module}, errors::{CompileError, ErrorCollector, ErrorLevel}, file_position::{FileText, LineCol, Span}, flattening::{FlatID, Instruction}, instantiation::{SubModuleOrWire, CALCULATE_LATENCY_LATER}, linker::{FileData, FileUUID, FileUUIDMarker, Linker, LocationInfo}, parser::perform_full_semantic_parse, walk_name_color
};

use super::syntax_highlighting::IDEIdentifierType;

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
    fn update_text(&mut self, uri : Url, new_file_text : String) {
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
        completion_provider : Some(CompletionOptions{resolve_provider : Some(true), ..Default::default()}),
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

fn get_semantic_token_type_from_ide_token(tok : IDEIdentifierType) -> u32 {
    match tok {
        IDEIdentifierType::Value(IdentifierType::Input) => 4,
        IDEIdentifierType::Value(IdentifierType::Output) => 4,
        IDEIdentifierType::Value(IdentifierType::State) => 3,
        IDEIdentifierType::Value(IdentifierType::Local) => 3,
        IDEIdentifierType::Value(IdentifierType::Generative) => 3,
        IDEIdentifierType::Constant => 9, // make it 'OPERATOR'?
        IDEIdentifierType::Interface => 7, // FUNCTION
        IDEIdentifierType::Type => 5, // All others are 'TYPE'
    }
}

// Produces a bitset with 'modifier bits'
fn get_modifiers_for_token(tok : IDEIdentifierType) -> u32 {
    match tok {
        IDEIdentifierType::Value(IdentifierType::State) => 1, // repurpose ASYNC for "State"
        IDEIdentifierType::Value(IdentifierType::Generative) => 8, // repurpose READONLY
        _other => 0
    }
}

fn from_position(pos : lsp_types::Position) -> LineCol {
    LineCol{line : pos.line as usize, col : pos.character as usize}
}
fn from_position_range(range : lsp_types::Range) -> std::ops::Range<LineCol> {
    std::ops::Range{start : from_position(range.start), end : from_position(range.end)}
}
fn to_position(char_line : LineCol) -> lsp_types::Position {
    lsp_types::Position{line : char_line.line as u32, character : char_line.col as u32}
}
fn to_position_range(range : std::ops::Range<LineCol>) -> lsp_types::Range {
    lsp_types::Range{start : to_position(range.start), end : to_position(range.end)}
}

fn convert_to_semantic_tokens(file_data : &FileData, ide_tokens : &mut[(IDEIdentifierType, Span)]) -> Vec<SemanticToken> {
    ide_tokens.sort_by(|a, b| a.1.cmp(&b.1));
    
    let mut cursor = Position {line : 0, character : 0};

    ide_tokens.into_iter().map(|(ide_kind, span)| {
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

fn do_syntax_highlight(file_data : &FileData, linker : &Linker) -> Vec<SemanticToken> {
    let mut ide_tokens = walk_name_color(&file_data.associated_values, linker);

    convert_to_semantic_tokens(file_data, &mut ide_tokens)
}

use lsp_types::Diagnostic;

fn cvt_span_to_lsp_range(ch_sp : Span, file_text : &FileText) -> lsp_types::Range {
    let rng = file_text.get_span_linecol_range(ch_sp);
    Range {
        start: to_position(rng.start),
        end: to_position(rng.end)
    }
}

// Requires that token_positions.len() == tokens.len() + 1 to include EOF token
fn convert_diagnostic(err : CompileError, main_file_text : &FileText, linker : &Linker, uris : &ArenaVector<Url, FileUUIDMarker>) -> Diagnostic {
    assert!(main_file_text.is_span_valid(err.position), "bad error: {}", err.reason);
    let error_pos = cvt_span_to_lsp_range(err.position, main_file_text);

    let severity = match err.level {
        ErrorLevel::Error => DiagnosticSeverity::ERROR,
        ErrorLevel::Warning => DiagnosticSeverity::WARNING,
    };
    let mut related_info = Vec::new();
    for info in err.infos {
        let info_file_text = &linker.files[info.file].file_text;
        assert!(info_file_text.is_span_valid(info.position), "bad info: {}; in err: {}", info.info, err.reason);
        let info_pos = cvt_span_to_lsp_range(info.position, info_file_text);
        let location = Location{uri : uris[info.file].clone(), range : info_pos};
        related_info.push(DiagnosticRelatedInformation { location, message: info.info });
    }
    Diagnostic::new(error_pos, Some(severity), None, None, err.reason, Some(related_info), None)
}

// Requires that token_positions.len() == tokens.len() + 1 to include EOF token
fn send_errors_warnings(connection: &Connection, errors : ErrorCollector, main_file_text : &FileText, linker : &Linker, uris : &ArenaVector<Url, FileUUIDMarker>) -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut diag_vec : Vec<Diagnostic> = Vec::new();
    let (err_vec, file) = errors.get();
    for err in err_vec {
        diag_vec.push(convert_diagnostic(err, main_file_text, linker, uris));
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

fn get_hover_info<'l>(file_cache : &'l LoadedFileCache, text_pos : &lsp_types::TextDocumentPositionParams) -> Option<(LocationInfo<'l>, lsp_types::Range)> {
    let uuid = file_cache.find_uri(&text_pos.text_document.uri).unwrap();
    
    let file_data = &file_cache.linker.files[uuid];

    let byte_pos = file_data.file_text.linecol_to_byte_clamp(from_position(text_pos.position));

    let (info, span) = file_cache.linker.get_info_about_source_location(byte_pos, uuid)?;
    //let span = Span::new_single_token(token_idx);

    let char_line_range = file_data.file_text.get_span_linecol_range(span);
    Some((info, to_position_range(char_line_range)))
}

fn push_all_errors(connection: &Connection, file_cache : &LoadedFileCache) -> Result<(), Box<dyn Error + Sync + Send>> {
    for (uuid, file_data) in &file_cache.linker.files {
        let errors = file_cache.linker.get_all_errors_in_file(uuid);
    
        // println!("Errors: {:?}", &errors);
        send_errors_warnings(&connection, errors, &file_data.file_text, &file_cache.linker, &file_cache.uris)?;
    }
    Ok(())
}

fn initialize_all_files(init_params : &InitializeParams) -> LoadedFileCache {
    let mut linker = Linker::new();
    let mut uris = ArenaVector::new();

    if let Some(workspace_folder) = &init_params.workspace_folders {
        for folder in workspace_folder {
            let Ok(path) = folder.uri.to_file_path() else {continue};

            for file in std::fs::read_dir(path).unwrap() {
                let file_path = file.unwrap().path();
                if file_path.is_file() && file_path.extension() == Some(OsStr::new("sus")) {
                    let file_uuid = linker.reserve_file();
                    let file_text = std::fs::read_to_string(&file_path).unwrap();
                    let full_parse = perform_full_semantic_parse(file_text, file_uuid);
                    linker.add_reserved_file(file_uuid, full_parse);
                    uris.insert(file_uuid, Url::from_file_path(&file_path).unwrap());
                }
            }
        }
    }
    let mut result = LoadedFileCache::new(linker, uris);
    result.linker.recompile_all();
    result
}

fn gather_hover_infos(md: &Module, id: FlatID, is_generative : bool, file_cache: &LoadedFileCache, hover_list: &mut Vec<MarkedString>) {
    md.instantiations.for_each_instance(|inst| {
        if is_generative {
            let value_str = match &inst.generation_state[id] {
                SubModuleOrWire::SubModule(_) | SubModuleOrWire::Wire(_) => unreachable!(),
                SubModuleOrWire::CompileTimeValue(v) => format!(" = {}", v.to_string()),
                SubModuleOrWire::Unnasigned => format!("never assigned to"),
            };
            hover_list.push(MarkedString::String(value_str));
        } else {
            for (_id, wire) in &inst.wires {
                if wire.original_wire != id {continue}
                let typ_str = wire.typ.to_string(&file_cache.linker.types);
                let name_str = &wire.name;
                let latency_str = if wire.absolute_latency != CALCULATE_LATENCY_LATER {
                    format!("{}", wire.absolute_latency)
                } else {
                    "?".to_owned()
                };
                hover_list.push(MarkedString::String(format!("{typ_str} {name_str}'{latency_str}")));
            }
        }
    });
}

fn gather_completions(linker : &Linker, file_id : FileUUID, position : usize) -> Vec<CompletionItem> {
    let mut result = Vec::new();

    use crate::linker::Linkable;
    for (_, m) in &linker.modules {
        result.push(CompletionItem{label : m.link_info.name.to_string(), kind : Some(CompletionItemKind::FUNCTION), ..Default::default()});

        if m.link_info.file == file_id && m.link_info.span.contains_pos(position) {
            for (_id, v) in &m.flattened.instructions {
                if let Instruction::Declaration(d) = v {
                    result.push(CompletionItem{label : d.name.to_string(), kind : Some(CompletionItemKind::VARIABLE), ..Default::default()});
                }
            }
        }
    }
    for (_, c) in &linker.constants {
        result.push(CompletionItem{label : c.get_name().to_string(), kind : Some(CompletionItemKind::CONSTANT), ..Default::default()});
    }
    for (_, t) in &linker.types {
        result.push(CompletionItem{label : t.get_name().to_string(), kind : Some(CompletionItemKind::STRUCT), ..Default::default()});
    }

    result
}

fn handle_request(method : &str, params : serde_json::Value, file_cache : &mut LoadedFileCache, debug : bool) -> Result<serde_json::Value, serde_json::Error> {
    match method {
        request::HoverRequest::METHOD => {
            let params : HoverParams = serde_json::from_value(params).expect("JSON Encoding Error while parsing params");
            println!("HoverRequest");

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

                            let identifier_type_keyword = decl.identifier_type.get_keyword();
                            hover_list.push(MarkedString::String(format!("{identifier_type_keyword} {typ_str} {name_str}")));

                            gather_hover_infos(md, decl_id, decl.identifier_type.is_generative(), file_cache, &mut hover_list);
                        }
                        LocationInfo::Temporary(md, id, wire) => {
                            let typ_str = wire.typ.to_string(&file_cache.linker.types);

                            let gen_kw = if wire.is_compiletime {"gen "} else {""};
                            hover_list.push(MarkedString::String(format!("{gen_kw}{typ_str}")));
                            gather_hover_infos(md, id, wire.is_compiletime, file_cache, &mut hover_list);
                        }
                        LocationInfo::Type(typ) => {
                            hover_list.push(MarkedString::String(typ.to_type().to_string(&file_cache.linker.types)));
                        }
                        LocationInfo::Global(global) => {
                            hover_list.push(MarkedString::String(file_cache.linker.get_full_name(global)));
                        }
                    };
                }
                Hover{contents: HoverContents::Array(hover_list), range: Some(range)}
            } else {
                Hover{contents: HoverContents::Array(Vec::new()), range: None}
            })
        }
        request::GotoDefinition::METHOD => {
            let params : GotoDefinitionParams = serde_json::from_value(params).expect("JSON Encoding Error while parsing params");
            println!("GotoDefinition");

            file_cache.ensure_contains_file(&params.text_document_position_params.text_document.uri);
            serde_json::to_value(&if let Some((info, _range)) = get_hover_info(&file_cache, &params.text_document_position_params) {
                match info {
                    LocationInfo::WireRef(md, decl_id) => {
                        let uri = file_cache.uris[md.link_info.file].clone();
                        let decl = md.flattened.instructions[decl_id].extract_wire_declaration();
                        let range = to_position_range(file_cache.linker.files[md.link_info.file].file_text.get_span_linecol_range(decl.name_span));
                        GotoDefinitionResponse::Scalar(Location{uri, range})
                    }
                    LocationInfo::Temporary(_, _, _) => {
                        GotoDefinitionResponse::Array(Vec::new())
                    }
                    LocationInfo::Type(_) => {
                        GotoDefinitionResponse::Array(Vec::new())
                    }
                    LocationInfo::Global(id) => {
                        if let Some(link_info) = file_cache.linker.get_link_info(id) {
                            let uri = file_cache.uris[link_info.file].clone();
                            let range = to_position_range(file_cache.linker.files[link_info.file].file_text.get_span_linecol_range(link_info.name_span));
                            GotoDefinitionResponse::Scalar(Location{uri, range})
                        } else {
                            GotoDefinitionResponse::Array(Vec::new())
                        }
                    }
                }
            } else {
                GotoDefinitionResponse::Array(Vec::new())
            })
        }
        request::SemanticTokensFullRequest::METHOD => {
            let params : SemanticTokensParams = serde_json::from_value(params).expect("JSON Encoding Error while parsing params");
            println!("SemanticTokensFullRequest");

            let uuid = file_cache.ensure_contains_file(&params.text_document.uri);
            
            let file_data = &file_cache.linker.files[uuid];

            serde_json::to_value(&SemanticTokensResult::Tokens(lsp_types::SemanticTokens {result_id: None, data: do_syntax_highlight(file_data, &file_cache.linker)}))
        }
        /*request::DocumentHighlightRequest::METHOD => {
            let params : DocumentHighlightParams = serde_json::from_value(params).expect("JSON Encoding Error while parsing params");

            if let Some((hover_info, span)) = get_hover_info(file_cache, &params.text_document_position_params) {
                
            }

            todo!()
        }*/
        request::Completion::METHOD => {
            let params : CompletionParams = serde_json::from_value(params).expect("JSON Encoding Error while parsing params");
            println!("Completion");

            let uuid = file_cache.ensure_contains_file(&params.text_document_position.text_document.uri);
            
            let file_data = &file_cache.linker.files[uuid];

            let position = file_data.file_text.linecol_to_byte_clamp(from_position(params.text_document_position.position));

            serde_json::to_value(&CompletionResponse::Array(gather_completions(&file_cache.linker, uuid, position)))
        }
        req => {
            println!("Other request: {req:?}");
            Ok(serde_json::Value::Null)
        }
    }
}

fn handle_notification(connection: &Connection, notification : lsp_server::Notification, file_cache : &mut LoadedFileCache, initialize_params : &InitializeParams) -> Result<(), Box<dyn Error + Sync + Send>> {
    match notification.method.as_str() {
        notification::DidChangeTextDocument::METHOD => {
            println!("DidChangeTextDocument");
            let params : DidChangeTextDocumentParams = serde_json::from_value(notification.params).expect("JSON Encoding Error while parsing params");
            file_cache.update_text(params.text_document.uri, params.content_changes.into_iter().next().unwrap().text);

            push_all_errors(connection, &file_cache)?;
        }
        notification::DidChangeWatchedFiles::METHOD => {
            println!("Workspace Files modified");
            *file_cache = initialize_all_files(initialize_params);

            push_all_errors(&connection, &file_cache)?;
        }
        other => {
            println!("got notification: {other:?}");
        }
    }
    Ok(())
}

fn main_loop(connection: Connection, initialize_params: serde_json::Value, debug : bool) -> Result<(), Box<dyn Error + Sync + Send>> {
    let initialize_params: InitializeParams = serde_json::from_value(initialize_params).unwrap();

    let mut file_cache = initialize_all_files(&initialize_params);

    push_all_errors(&connection, &file_cache)?;

    println!("starting LSP main loop");
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if req.method.as_str() == request::Shutdown::METHOD {
                    println!("Shutdown request");
                    return Ok(());
                }

                let response_value = handle_request(&req.method, req.params, &mut file_cache, debug);

                let result = response_value.unwrap();
                let response = Response{id : req.id, result: Some(result), error: None};
                connection.sender.send(Message::Response(response))?;
            }
            Message::Response(resp) => {
                println!("got response: {resp:?}");
            }
            Message::Notification(notification) => {
                handle_notification(&connection, notification, &mut file_cache, &initialize_params)?;
            }
        }
    }
    Ok(())
}
