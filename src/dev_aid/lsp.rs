//! A minimal example LSP server that can only respond to the `gotoDefinition` request. To use
//! this example, execute it and then send an `initialize` request.
//!
//! ```no_run
//! Content-Length: 85
//!
//! {"jsonrpc": "2.0", "method": "initialize", "id": 1, "params": {"capabilities": {}}}
//! ```
//!
//! This will respond with a server response. Then send it a `initialized` notification which will
//! have no response.
//!
//! ```no_run
//! Content-Length: 59
//!
//! {"jsonrpc": "2.0", "method": "initialized", "params": {}}
//! ```
//!
//! Once these two are sent, then we enter the main loop of the server. The only request this
//! example can handle is `gotoDefinition`:
//!
//! ```no_run
//! Content-Length: 159
//!
//! {"jsonrpc": "2.0", "method": "textDocument/definition", "id": 2, "params": {"textDocument": {"uri": "file://temp"}, "position": {"line": 1, "character": 1}}}
//! ```
//!
//! To finish up without errors, send a shutdown request:
//!
//! ```no_run
//! Content-Length: 67
//!
//! {"jsonrpc": "2.0", "method": "shutdown", "id": 3, "params": null}
//! ```
//!
//! The server will exit the main loop and finally we send a `shutdown` notification to stop
//! the server.
//!
//! ```
//! Content-Length: 54
//!
//! {"jsonrpc": "2.0", "method": "exit", "params": null}
//! ```
use std::error::Error;
use std::fs::File;
use lsp_types::{*, request::Request, notification::*};

use lsp_server::{Response, Message, Connection, };

use lsp_types::notification::Notification;

use std::path::Path;

use crate::{tokenizer::tokenize, parser::{perform_full_semantic_parse, FullParseResult}, dev_aid::syntax_highlighting::create_token_ide_info, ast::{IdentifierType, CharSpan}, errors::ParsingError};

use super::syntax_highlighting::{IDETokenType, IDEIdentifierType, IDEToken};

thread_local!(static OUT_FILE: File = File::create("/home/lennart/lsp_out.txt").expect("Replacement terminal /home/lennart/lsp_out.txt could not be created"));

macro_rules! print {
    ($($arg:tt)*) => {{
        use std::io::Write;
        OUT_FILE.with(|mut file| {
            write!(file, $($arg)*).unwrap();
        })
    }};
}
macro_rules! println {
    ($($arg:tt)*) => {{
        use std::io::Write;
        OUT_FILE.with(|mut file| {
            write!(file, $($arg)*).unwrap();
            write!(file, "\n").unwrap();
        })
    }};
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
                    SemanticTokenType::FUNCTION
                ],
                token_modifiers: vec![
                    SemanticTokenModifier::DECLARATION,
                    SemanticTokenModifier::DEFINITION,
                    SemanticTokenModifier::READONLY
                ],
            },
            range: Some(false), // Don't support ranges yet
            full: Some(SemanticTokensFullOptions::Bool(true)), // TODO: Support delta updating for faster syntax highlighting, just do whole file for now
        })),
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
        IDETokenType::Symbol => 2,
        IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::Input)) => 4,
        IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::Output)) => 4,
        IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::State)) => 3, // TODO
        IDETokenType::Identifier(IDEIdentifierType::Value(IdentifierType::Local)) => 3,
        IDETokenType::Identifier(_) => 5, // All others are 'TYPE'
        IDETokenType::Number => 6,
        IDETokenType::Invalid => 2, // make it 'OPERATOR'?
        IDETokenType::InvalidBracket => 2, // make it 'OPERATOR'?
        IDETokenType::OpenBracket(_) => 2,
        IDETokenType::CloseBracket(_) => 2,
    }
}

fn do_syntax_highlight(file_text : &str, full_parse : &FullParseResult) -> SemanticTokensResult {
    let ide_tokens = create_token_ide_info(&file_text, &full_parse);

    let mut semantic_tokens : Vec<SemanticToken> = Vec::new();
    semantic_tokens.reserve(full_parse.token_spans.len());

    let mut prev_line : usize = 0;
    let mut prev_col : usize = 0;
    for (idx, tok) in ide_tokens.iter().enumerate() {
        let tok_file_pos = full_parse.token_spans[idx];

        let delta_line = tok_file_pos.file_pos.row - prev_line;

        if delta_line != 0 {
            prev_col = 0;
        }

        let delta_col = tok_file_pos.file_pos.col - prev_col;
        prev_col = tok_file_pos.file_pos.col;

        prev_line = tok_file_pos.file_pos.row;

        let typ = get_semantic_token_type_from_ide_token(tok);
        semantic_tokens.push(SemanticToken{
            delta_line: delta_line as u32,
            delta_start: delta_col as u32,
            length: tok_file_pos.length as u32,
            token_type: typ,
            token_modifiers_bitset: 0,
        });

        //println!("{}: typ={typ} {delta_line}:{delta_col}", file_text.get(tok_file_pos.as_range()).unwrap());
    }

    SemanticTokensResult::Tokens(lsp_types::SemanticTokens {
        result_id: None,
        data: semantic_tokens
    })
}

use lsp_types::Diagnostic;

fn cvt_char_span_to_lsp_range(ch_sp : CharSpan, file_text : &str) -> lsp_types::Range {
    let mut last_char_line = ch_sp.file_pos.row;
    let mut last_newline_idx = ch_sp.file_pos.char_idx - ch_sp.file_pos.col;
    let last_char_idx = ch_sp.file_pos.char_idx+ch_sp.length;
    for (i, c) in file_text.get(ch_sp.file_pos.char_idx..last_char_idx).unwrap().char_indices() {
        if c == '\n' {
            last_char_line += 1;
            last_newline_idx = i;
        }
    }
    let last_char_col = last_char_idx - last_newline_idx;
    Range{
        start : Position{
            line : ch_sp.file_pos.row as u32,
            character : ch_sp.file_pos.col as u32
        }, end : Position{
            line : last_char_line as u32,
            character : last_char_col as u32
        }
    }
}

fn send_errors_warnings(connection: &Connection, errs : Vec<ParsingError<CharSpan>>, file_uri: Url, file_text : &str) -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut diag_vec : Vec<Diagnostic> = Vec::new();
    for err in errs {
        diag_vec.push(Diagnostic::new_simple(cvt_char_span_to_lsp_range(err.error.position, file_text), err.error.reason));
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

                        let path = params.text_document.uri.to_file_path().unwrap();
                        let file_text = std::fs::read_to_string(path).unwrap();

                        let (full_parse, errors) = perform_full_semantic_parse(&file_text);
                        
                        let result = serde_json::to_value(&do_syntax_highlight(&file_text, &full_parse)).unwrap();
                        connection.sender.send(Message::Response(Response{
                            id: req.id, result: Some(result), error: None
                        }))?;

                        send_errors_warnings(&connection, errors, params.text_document.uri, &file_text)?;
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
                println!("got notification: {not:?}");
            }
        }
    }
    Ok(())
}
