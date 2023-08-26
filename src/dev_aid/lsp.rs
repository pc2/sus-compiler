
use std::error::Error;
use std::fs::File;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use lsp_types::{*, request::Request, notification::*};

use lsp_server::{Response, Message, Connection};

use lsp_types::notification::Notification;

use crate::{parser::{perform_full_semantic_parse, FullParseResult}, dev_aid::syntax_highlighting::create_token_ide_info, ast::{IdentifierType, RowCol, Span}, errors::ParsingError, tokenizer::Token};

use super::syntax_highlighting::{IDETokenType, IDEIdentifierType, IDEToken};

static LSP_LOG_PATH : &str = if crate::tokenizer::const_eq_str(std::env::consts::OS, "windows") {
    "C:\\Users\\lenna\\lsp_out.txt"
} else {
    "/home/lennart/lsp_out.txt"
};

thread_local!(static LSP_LOG: File = File::create(LSP_LOG_PATH).expect("Replacement terminal /home/lennart/lsp_out.txt could not be created"));

macro_rules! println {
    ($($arg:tt)*) => {{
        use std::io::Write;
        LSP_LOG.with(|mut file| {
            write!(file, $($arg)*).unwrap();
            write!(file, "\n").unwrap();
        })
    }};
}
/*macro_rules! println {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
    }};
}*/

struct LoadedFile {
    file_text : String
}
struct LoadedFileCache {
    loaded_files : HashMap<PathBuf, Rc<LoadedFile>>
}

impl LoadedFileCache {
    fn new() -> LoadedFileCache {
        LoadedFileCache{loaded_files : HashMap::new()}
    }
    fn get(&mut self, path : &PathBuf) -> Rc<LoadedFile> {
        if let Some(found) = self.loaded_files.get(path) {
            found.clone()
        } else {
            self.update_from_disk(path.clone())
        }
    }
    fn update_text(&mut self, path : PathBuf, new_text : String) -> Rc<LoadedFile> {
        let result = Rc::new(LoadedFile{
            file_text: new_text
        });
        self.update(path, result.clone());
        result
    }
    fn update_from_disk(&mut self, path : PathBuf) -> Rc<LoadedFile> {
        let file_text = std::fs::read_to_string(&path).expect("Could not load file");
        self.update_text(path, file_text)
    }
    fn update(&mut self, path : PathBuf, new_val : Rc<LoadedFile>) {
        self.loaded_files.insert(path, new_val);
    }
}


pub fn lsp_main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // Note that  we must have our logging only write out to stderr.
    //println!("starting generic LSP server");

    println!("starting generic LSP server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    //let (connection, io_threads) = Connection::listen(SocketAddr::from(([127,0,0,1], 25000)))?;
    let (connection, io_threads) = Connection::stdio();
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
                    SemanticTokenType::EVENT
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
        IDETokenType::Identifier(IDEIdentifierType::Unknown) => 2, // make it 'OPERATOR'?
        IDETokenType::Identifier(IDEIdentifierType::Interface) => 7, // FUNCTION
        IDETokenType::Identifier(_) => 5, // All others are 'TYPE'
        IDETokenType::Number => 6,
        IDETokenType::Invalid => 2, // make it 'OPERATOR'?
        IDETokenType::InvalidBracket => 2, // make it 'OPERATOR'?
        IDETokenType::OpenBracket(_) => 2,
        IDETokenType::CloseBracket(_) => 2,
    }
}


fn get_modifiers_for_token(tok : &IDEToken) -> u32 {
    match &tok.typ {
        IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::State)) => 15, // repurpose ASYNC for "State"
        _other => 0
    }
}

struct SemanticTokensDeltaAccumulator {
    prev_line : usize,
    prev_col : usize,
    semantic_tokens : Vec<SemanticToken>
}

impl SemanticTokensDeltaAccumulator {
    fn push(&mut self, row_col : RowCol, length : usize, typ : u32, mod_bits : u32) {
        let delta_line = row_col.row - self.prev_line;

        if delta_line != 0 {
            self.prev_col = 0;
        }

        let delta_col = row_col.col - self.prev_col;
        self.prev_col = row_col.col;
        self.prev_line = row_col.row;

        self.semantic_tokens.push(SemanticToken{
            delta_line: delta_line as u32,
            delta_start: delta_col as u32,
            length: length as u32,
            token_type: typ,
            token_modifiers_bitset: mod_bits,
        });
    }
}

fn do_syntax_highlight(file_data : &LoadedFile, full_parse : &FullParseResult) -> SemanticTokensResult {
    let file_text = &file_data.file_text;
    let ide_tokens = create_token_ide_info(&full_parse);

    let mut semantic_tokens_acc = SemanticTokensDeltaAccumulator{prev_line : 0, prev_col : 0, semantic_tokens : Vec::new()};
    semantic_tokens_acc.semantic_tokens.reserve(full_parse.tokens.tokens.len());

    for (idx, tok) in ide_tokens.iter().enumerate() {
        let token_text = &file_text[full_parse.tokens.tokens[idx].get_range()];
        let char_iter = token_text.chars();
        let mut row_col = full_parse.tokens.token_row_cols[idx];

        let typ = get_semantic_token_type_from_ide_token(tok);
        let mod_bits = get_modifiers_for_token(tok);
        if tok.typ == IDETokenType::Comment {
            // Comments can be multiline, editor doesn't support this. Have to split them up myself. Eurgh
            let mut line_char_offset = 0;
            let mut length_in_chars : usize = 0;
            for (idx, c) in char_iter.enumerate() {
                length_in_chars += 1;
                if c == '\n' {
                    semantic_tokens_acc.push(row_col, idx - line_char_offset, typ, mod_bits);

                    line_char_offset = idx + 1;
                    row_col.row += 1;
                    row_col.col = 0;
                }
            }
            let leftover_length = length_in_chars - line_char_offset;
            if leftover_length > 0 {
                semantic_tokens_acc.push(row_col, leftover_length, typ, mod_bits);
            }
        } else {
            semantic_tokens_acc.push(row_col, char_iter.count(), typ, mod_bits);
        }

        //println!("{}: typ={typ} {delta_line}:{delta_col}", file_text[tok_file_pos.as_range()]);
    }

    SemanticTokensResult::Tokens(lsp_types::SemanticTokens {
        result_id: None,
        data: semantic_tokens_acc.semantic_tokens
    })
}

use lsp_types::Diagnostic;

fn cvt_span_to_lsp_range(ch_sp : Span, file_text : &str, tokens : &[Token], row_cols : &[RowCol]) -> lsp_types::Range {
    let start_row_col = row_cols[ch_sp.0];

    let mut last_row_col = row_cols[ch_sp.1];
    for c in file_text[tokens[ch_sp.1].get_range()].chars() {
        last_row_col.advance_char(c);
    }
    Range{
        start : Position{
            line : start_row_col.row as u32,
            character : start_row_col.col as u32
        }, end : Position{
            line : last_row_col.row as u32,
            character : last_row_col.col as u32
        }
    }
}

fn send_errors_warnings(connection: &Connection, errs : Vec<ParsingError<Span>>, file_uri: Url, file_text : &str, tokens : &[Token], token_row_cols : &[RowCol]) -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut diag_vec : Vec<Diagnostic> = Vec::new();
    for err in errs {
        diag_vec.push(Diagnostic::new_simple(cvt_span_to_lsp_range(err.error.position, file_text,tokens, token_row_cols), err.error.reason));
    }
    
    let params = &PublishDiagnosticsParams{
        uri: file_uri,
        diagnostics: diag_vec,
        version: None // TODO 
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

    let mut file_cache = LoadedFileCache::new();

    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    println!("starting example main loop");
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

                        let path : PathBuf = params.text_document.uri.to_file_path().unwrap();
                        let file_data : Rc<LoadedFile> = file_cache.get(&path);

                        let (full_parse, errors) = perform_full_semantic_parse(&file_data.file_text);
                        
                        let result = serde_json::to_value(&do_syntax_highlight(&file_data, &full_parse)).unwrap();
                        connection.sender.send(Message::Response(Response{
                            id: req.id, result: Some(result), error: None
                        }))?;

                        send_errors_warnings(&connection, errors, params.text_document.uri, &file_data.file_text, &full_parse.tokens.tokens, &full_parse.tokens.token_row_cols)?;
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
                        let path_to_update = params.text_document.uri.to_file_path().unwrap();
                        //let original_file_text = file_cache.get(&path_to_update).file_text;
                        let new_file_text = params.content_changes[0].text.clone();
                        file_cache.update_text(path_to_update, new_file_text);
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
