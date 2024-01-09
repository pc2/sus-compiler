
use std::{error::Error, net::SocketAddr};
use lsp_types::{*, request::Request, notification::*};

use lsp_server::{Response, Message, Connection};

use lsp_types::notification::Notification;

use crate::{parser::perform_full_semantic_parse, dev_aid::syntax_highlighting::create_token_ide_info, ast::{IdentifierType, Span}, errors::{ErrorCollector, CompileError, ErrorLevel}, linker::{PreLinker, FileUUIDMarker, Linker, FileUUID, FileData, Links}, arena_alloc::ArenaVector};

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

pub fn lsp_main(port : u16) -> Result<(), Box<dyn Error + Sync + Send>> {
    // Note that  we must have our logging only write out to stderr.
    //println!("starting generic LSP server");

    println!("starting generic LSP server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    //let (connection, io_threads) = Connection::listen(SocketAddr::from(([127,0,0,1], 25000)))?;
    println!("Connecting on port {}...", port);
    let (connection, io_threads) = Connection::connect(SocketAddr::from(([127,0,0,1], port)))?;
    println!("connection established");
    
    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        definition_provider: Some(OneOf::Left(true)),
        /*document_highlight_provider: Some(OneOf::Right(
            DocumentHighlightOptions{
                work_done_progress_options: WorkDoneProgressOptions{
                    work_done_progress: Some(true)
                }
            }
        )),*/
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
    main_loop(connection, initialization_params)?;
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
        IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::Virtual)) => unreachable!(),
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

struct SemanticTokensDeltaAccumulator {
    prev : Position,
    semantic_tokens : Vec<SemanticToken>
}

impl SemanticTokensDeltaAccumulator {
    fn push(&mut self, position : Position, length : u32, typ : u32, mod_bits : u32) {
        let delta_line = position.line - self.prev.line;

        if delta_line != 0 {
            self.prev.character = 0;
        }

        let delta_col = position.character - self.prev.character;
        self.prev.character = position.character;
        self.prev.line = position.line;

        self.semantic_tokens.push(SemanticToken{
            delta_line: delta_line,
            delta_start: delta_col,
            length: length,
            token_type: typ,
            token_modifiers_bitset: mod_bits,
        });
    }
}

fn do_syntax_highlight(file_data : &FileData, links : &Links) -> (SemanticTokensResult, Vec<std::ops::Range<Position>>) {
    let file_text = &file_data.file_text;
    let ide_tokens = create_token_ide_info(&file_data, links);

    let mut semantic_tokens_acc = SemanticTokensDeltaAccumulator{prev : Position {line : 0, character : 0}, semantic_tokens : Vec::new()};
    semantic_tokens_acc.semantic_tokens.reserve(file_data.tokens.len());
    let mut positions : Vec<std::ops::Range<Position>> = Vec::new();
    positions.reserve(file_data.tokens.len());

    let mut cur_whitespace_start = 0;
    let mut cur_position = Position{line : 0, character : 0};
    for (tok_idx, ide_tok) in ide_tokens.iter().enumerate() {
        let typ = get_semantic_token_type_from_ide_token(ide_tok);
        let mod_bits = get_modifiers_for_token(ide_tok);

        let tok_range = file_data.tokens[tok_idx].get_range();
        let whitespace_text = &file_text[cur_whitespace_start..tok_range.start];
        cur_whitespace_start = tok_range.end;
        let token_text = &file_text[tok_range];

        // skip through whitespace
        for c in whitespace_text.chars() {
            if c == '\n' {
                cur_position.line += 1;
                cur_position.character = 0;
            } else {
                cur_position.character += 1;
            }
        }
        let real_token_start_position = cur_position;
        let mut part_start_position = cur_position;
        for c in token_text.chars() {
            if c == '\n' {
                semantic_tokens_acc.push(part_start_position, cur_position.character - part_start_position.character, typ, mod_bits);
                cur_position.line += 1;
                cur_position.character = 0;
                part_start_position = cur_position;
            } else {
                cur_position.character += 1;
            }
        }
        semantic_tokens_acc.push(part_start_position, cur_position.character - part_start_position.character, typ, mod_bits);
        positions.push(real_token_start_position..cur_position);
    }

    let eof_start = cur_position.clone();
    for c in file_text[cur_whitespace_start..].chars() {
        if c == '\n' {
            cur_position.line += 1;
            cur_position.character = 0;
        } else {
            cur_position.character += 1;
        }
    }
    positions.push(eof_start..cur_position);

    (SemanticTokensResult::Tokens(lsp_types::SemanticTokens {
        result_id: None,
        data: semantic_tokens_acc.semantic_tokens
    }), positions)
}

use lsp_types::Diagnostic;

fn cvt_span_to_lsp_range(ch_sp : Span, token_positions : &[std::ops::Range<Position>]) -> lsp_types::Range {
    Range {
        start: token_positions[ch_sp.0].start,
        end: token_positions[ch_sp.1].end
    }
}

// Requires that token_positions.len() == tokens.len() + 1 to include EOF token
fn convert_diagnostic(err : CompileError, token_positions : &[std::ops::Range<Position>], uris : &ArenaVector<Url, FileUUIDMarker>) -> Diagnostic {
    let error_pos = cvt_span_to_lsp_range(err.position, token_positions);

    let severity = match err.level {
        ErrorLevel::Error => DiagnosticSeverity::ERROR,
        ErrorLevel::Warning => DiagnosticSeverity::WARNING,
    };
    let mut related_info = Vec::new();
    for info in err.infos {
        let info_pos = cvt_span_to_lsp_range(info.position, token_positions);
        let location = Location{uri : uris[info.file].clone(), range : info_pos};
        related_info.push(DiagnosticRelatedInformation { location, message: info.info });
    }
    Diagnostic::new(error_pos, Some(severity), None, None, err.reason, Some(related_info), None)
}

// Requires that token_positions.len() == tokens.len() + 1 to include EOF token
fn send_errors_warnings(connection: &Connection, errors : ErrorCollector, token_positions : &[std::ops::Range<Position>], uris : &ArenaVector<Url, FileUUIDMarker>) -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut diag_vec : Vec<Diagnostic> = Vec::new();
    let (err_vec, file) = errors.get();
    for err in err_vec {
        diag_vec.push(convert_diagnostic(err, token_positions, uris));
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

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {

    let prelinker = PreLinker::new();
    let mut file_cache = LoadedFileCache::new(prelinker.link(), ArenaVector::new());

    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    println!("starting LSP main loop");
    for msg in &connection.receiver {
        println!("got msg: {msg:?}");
        match msg {
            Message::Request(req) => {
                match req.method.as_str() {
                    request::Shutdown::METHOD => {
                        println!("Shutdown request");
                        return Ok(());
                    }
                    request::GotoDefinition::METHOD => {
                        let params : GotoDefinitionParams = serde_json::from_value(req.params).expect("JSON Encoding Error while parsing params");
                        println!("got gotoDefinition request: {params:?}");

                        let result = Some(GotoDefinitionResponse::Array(Vec::new()));
                        let result = serde_json::to_value(&result).unwrap();
                        let resp = Response { id: req.id, result: Some(result), error: None };
                        connection.sender.send(Message::Response(resp))?;
                    },
                    request::SemanticTokensFullRequest::METHOD => {
                        let params : SemanticTokensParams = serde_json::from_value(req.params).expect("JSON Encoding Error while parsing params");
                        
                        println!("got fullSemanticTokens request: {params:?}");

                        let uuid = file_cache.ensure_contains_file(&params.text_document.uri);
                        
                        let file_data = &file_cache.linker.files[uuid];

                        let (syntax_highlight, token_positions) = do_syntax_highlight(file_data, &file_cache.linker.links);

                        let result = serde_json::to_value(&syntax_highlight).unwrap();
                        connection.sender.send(Message::Response(Response{
                            id: req.id, result: Some(result), error: None
                        }))?;

                        // println!("Flattening...");
                        file_cache.linker.recompile_all();

                        let mut errors = file_cache.linker.files[uuid].parsing_errors.clone();
                        file_cache.linker.get_all_errors_in_file(uuid, &mut errors);

                        // println!("Errors: {:?}", &errors);
                        send_errors_warnings(&connection, errors, &token_positions, &file_cache.uris)?;
                    },
                    // TODO ...
                    req => {
                        println!("Other request: {req:?}");
                    }
                }
            }
            Message::Response(resp) => {
                println!("got response: {resp:?}");
            }
            Message::Notification(not) => {
                match not.method.as_str() {
                    notification::DidChangeTextDocument::METHOD => {
                        let params : DidChangeTextDocumentParams = serde_json::from_value(not.params).expect("JSON Encoding Error while parsing params");
                        file_cache.update_text(params.text_document.uri, params.content_changes.into_iter().next().unwrap().text);
                    },
                    other => {
                        println!("got notification: {other:?}");
                    }
                }
            }
        }
    }
    Ok(())
}
