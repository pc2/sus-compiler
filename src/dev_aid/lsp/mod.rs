mod hover_info;
mod semantic_tokens;
mod tree_walk;

use crate::prelude::*;

use hover_info::hover;
use lsp_types::{notification::*, request::Request, *};
use semantic_tokens::{make_semantic_tokens, semantic_token_capabilities};
use std::{collections::HashMap, error::Error, ffi::OsStr, net::SocketAddr};

use crate::{
    alloc::ArenaVector,
    compiler_top::{add_file, recompile_all, update_file},
    config::config,
    errors::{CompileError, ErrorLevel},
    file_position::{FileText, LineCol},
    flattening::Instruction,
    linker::FileData,
};

use tree_walk::{get_selected_object, InModule, LocationInfo};

use self::tree_walk::RefersTo;

fn from_position(pos: lsp_types::Position) -> LineCol {
    LineCol {
        line: pos.line as usize,
        col: pos.character as usize,
    }
}
fn to_position(char_line: LineCol) -> lsp_types::Position {
    lsp_types::Position {
        line: char_line.line as u32,
        character: char_line.col as u32,
    }
}
fn span_to_lsp_range(file_text: &FileText, ch_sp: Span) -> lsp_types::Range {
    let rng = file_text.get_span_linecol_range(ch_sp);
    Range {
        start: to_position(rng.start),
        end: to_position(rng.end),
    }
}
fn cvt_location_list(location_vec: Vec<SpanFile>, file_cache: &LoadedFileCache) -> Vec<Location> {
    location_vec
        .into_iter()
        .map(|(span, file)| {
            let uri = file_cache.uris[file].clone();
            let range = span_to_lsp_range(&file_cache.linker.files[file].file_text, span);
            Location { uri, range }
        })
        .collect()
}
fn cvt_location_list_of_lists(
    location_vec: Vec<(FileUUID, Vec<Span>)>,
    file_cache: &LoadedFileCache,
) -> Vec<Location> {
    let mut result_len = 0;
    for (_f, v) in &location_vec {
        result_len += v.len();
    }
    let mut result = Vec::with_capacity(result_len);
    for (file, vec) in location_vec {
        let uri = &file_cache.uris[file];
        for span in vec {
            let range = span_to_lsp_range(&file_cache.linker.files[file].file_text, span);
            result.push(Location {
                uri: uri.clone(),
                range,
            })
        }
    }
    result
}

struct LoadedFileCache {
    linker: Linker,
    uris: ArenaVector<Url, FileUUIDMarker>,
}

impl LoadedFileCache {
    fn new(linker: Linker, uris: ArenaVector<Url, FileUUIDMarker>) -> Self {
        Self { linker, uris }
    }
    fn find_uri(&self, uri: &Url) -> Option<FileUUID> {
        self.uris.find(|_uuid, uri_found| uri_found == uri)
    }
    fn update_text(&mut self, uri: Url, new_file_text: String) {
        if let Some(found_file_uuid) = self.find_uri(&uri) {
            update_file(new_file_text, found_file_uuid, &mut self.linker);
        } else {
            let file_uuid = add_file(new_file_text, &mut self.linker);
            self.uris.insert(file_uuid, uri.clone());
        }

        recompile_all(&mut self.linker);
    }
    fn ensure_contains_file(&mut self, uri: &Url) -> FileUUID {
        if let Some(found) = self.find_uri(uri) {
            found
        } else {
            let file_text = std::fs::read_to_string(uri.to_file_path().unwrap()).unwrap();
            let file_uuid = add_file(file_text, &mut self.linker);
            recompile_all(&mut self.linker);
            file_uuid
        }
    }
    fn location_in_file(
        &mut self,
        text_pos: &lsp_types::TextDocumentPositionParams,
    ) -> (FileUUID, usize) {
        let file_id = self.ensure_contains_file(&text_pos.text_document.uri);
        let file_data = &self.linker.files[file_id];

        let position = file_data
            .file_text
            .linecol_to_byte_clamp(from_position(text_pos.position));

        (file_id, position)
    }
}

// Requires that token_positions.len() == tokens.len() + 1 to include EOF token
fn convert_diagnostic(
    err: &CompileError,
    main_file_text: &FileText,
    linker: &Linker,
    uris: &ArenaVector<Url, FileUUIDMarker>,
) -> Diagnostic {
    assert!(
        main_file_text.is_span_valid(err.position),
        "bad error: {}",
        err.reason
    );
    let error_pos = span_to_lsp_range(main_file_text, err.position);

    let severity = match err.level {
        ErrorLevel::Error => DiagnosticSeverity::ERROR,
        ErrorLevel::Warning => DiagnosticSeverity::WARNING,
    };
    let mut related_info = Vec::new();
    for info in &err.infos {
        let info_file_text = &linker.files[info.file].file_text;
        let file_name = uris[info.file].to_string();
        let info_span = info.position;
        assert!(
            info_file_text.is_span_valid(info_span),
            "bad info in {file_name}:\n{}; in err: {}.\nSpan is {info_span}, but file length is {}",
            info.info,
            err.reason,
            info_file_text.len()
        );
        let info_pos = span_to_lsp_range(info_file_text, info_span);
        let location = Location {
            uri: uris[info.file].clone(),
            range: info_pos,
        };
        related_info.push(DiagnosticRelatedInformation {
            location,
            message: info.info.clone(),
        });
    }
    Diagnostic::new(
        error_pos,
        Some(severity),
        None,
        None,
        err.reason.clone(),
        Some(related_info),
        None,
    )
}

fn push_all_errors(
    connection: &lsp_server::Connection,
    file_cache: &LoadedFileCache,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    for (file_id, file_data) in &file_cache.linker.files {
        let mut diag_vec: Vec<Diagnostic> = Vec::new();

        file_cache.linker.for_all_errors_in_file(file_id, |err| {
            diag_vec.push(convert_diagnostic(
                err,
                &file_data.file_text,
                &file_cache.linker,
                &file_cache.uris,
            ));
        });

        let params = &PublishDiagnosticsParams {
            uri: file_cache.uris[file_id].clone(),
            diagnostics: diag_vec,
            version: None,
        };
        let params_json = serde_json::to_value(params)?;

        connection.sender.send(lsp_server::Message::Notification(
            lsp_server::Notification {
                method: PublishDiagnostics::METHOD.to_owned(),
                params: params_json,
            },
        ))?;
    }
    Ok(())
}

fn initialize_all_files(init_params: &InitializeParams) -> LoadedFileCache {
    let mut linker = Linker::new();
    let mut uris = ArenaVector::new();

    if let Some(workspace_folder) = &init_params.workspace_folders {
        for folder in workspace_folder {
            let Ok(path) = folder.uri.to_file_path() else {
                continue;
            };

            for file in std::fs::read_dir(path).unwrap() {
                let file_path = file.unwrap().path();
                if file_path.is_file() && file_path.extension() == Some(OsStr::new("sus")) {
                    let file_text = std::fs::read_to_string(&file_path).unwrap();
                    let file_uuid = add_file(file_text, &mut linker);
                    uris.insert(file_uuid, Url::from_file_path(&file_path).unwrap());
                }
            }
        }
    }
    let mut result = LoadedFileCache::new(linker, uris);
    recompile_all(&mut result.linker);
    result
}

fn gather_completions(linker: &Linker, file_id: FileUUID, position: usize) -> Vec<CompletionItem> {
    let mut result = Vec::new();

    use crate::linker::Linkable;
    for (_, m) in &linker.modules {
        result.push(CompletionItem {
            label: m.link_info.name.to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        });

        if m.link_info.file == file_id && m.link_info.span.contains_pos(position) {
            for (_id, v) in &m.instructions {
                if let Instruction::Declaration(d) = v {
                    result.push(CompletionItem {
                        label: d.name.to_string(),
                        kind: Some(CompletionItemKind::VARIABLE),
                        ..Default::default()
                    });
                }
            }
        }
    }
    for (_, c) in &linker.constants {
        result.push(CompletionItem {
            label: c.get_name().to_string(),
            kind: Some(CompletionItemKind::CONSTANT),
            ..Default::default()
        });
    }
    for (_, t) in &linker.types {
        result.push(CompletionItem {
            label: t.get_name().to_string(),
            kind: Some(CompletionItemKind::STRUCT),
            ..Default::default()
        });
    }

    result
}

fn gather_references_in_file(
    linker: &Linker,
    file_data: &FileData,
    refers_to: RefersTo,
) -> Vec<Span> {
    let mut ref_locations = Vec::new();
    tree_walk::visit_all(linker, file_data, |span, info| {
        if refers_to.refers_to_same_as(info) {
            ref_locations.push(span);
        }
    });
    ref_locations
}

fn for_each_local_reference_in_module(
    linker: &Linker,
    md_id: ModuleUUID,
    local: FlatID,
) -> Vec<Span> {
    let mut ref_locations = Vec::new();
    tree_walk::visit_all_in_module(linker, md_id, |span, info| {
        if let LocationInfo::InModule(_, _, f_id, _) = info {
            if local == f_id {
                ref_locations.push(span);
            }
        }
    });
    ref_locations
}

fn gather_all_references_in_one_file(linker: &Linker, file_id: FileUUID, pos: usize) -> Vec<Span> {
    if let Some((_location, hover_info)) = get_selected_object(linker, file_id, pos) {
        let refers_to = RefersTo::from(hover_info);
        if refers_to.is_global() {
            gather_references_in_file(&linker, &linker.files[file_id], refers_to)
        } else if let Some(local) = refers_to.local {
            for_each_local_reference_in_module(&linker, local.0, local.1)
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

fn gather_all_references_across_all_files(
    linker: &Linker,
    file_id: FileUUID,
    pos: usize,
) -> Vec<(FileUUID, Vec<Span>)> {
    let mut ref_locations = Vec::new();

    if let Some((location, hover_info)) = get_selected_object(linker, file_id, pos) {
        let refers_to = RefersTo::from(hover_info);
        if refers_to.is_global() {
            for (other_file_id, other_file) in &linker.files {
                let found_refs = gather_references_in_file(&linker, other_file, refers_to);
                for r in &found_refs {
                    assert!(location.size() == r.size())
                }
                if found_refs.len() > 0 {
                    ref_locations.push((other_file_id, found_refs))
                }
            }
        } else if let Some(local) = refers_to.local {
            let found_refs = for_each_local_reference_in_module(&linker, local.0, local.1);
            for r in &found_refs {
                assert!(location.size() == r.size())
            }
            if found_refs.len() > 0 {
                ref_locations.push((file_id, found_refs))
            }
        }
    }
    ref_locations
}

fn handle_request(
    method: &str,
    params: serde_json::Value,
    file_cache: &mut LoadedFileCache,
) -> Result<serde_json::Value, serde_json::Error> {
    match method {
        request::HoverRequest::METHOD => {
            let params: HoverParams =
                serde_json::from_value(params).expect("JSON Encoding Error while parsing params");
            println!("HoverRequest");

            let (file_uuid, pos) =
                file_cache.location_in_file(&params.text_document_position_params);
            let file_data = &file_cache.linker.files[file_uuid];
            let mut hover_list: Vec<MarkedString> = Vec::new();

            let range = if let Some((location, info)) =
                get_selected_object(&file_cache.linker, file_uuid, pos)
            {
                if config().lsp_debug_mode {
                    hover_list.push(MarkedString::String(format!("{info:?}")))
                } else {
                    hover_list = hover(info, &file_cache.linker, file_data);
                }
                Some(span_to_lsp_range(&file_data.file_text, location))
            } else {
                None
            };
            serde_json::to_value(Hover {
                contents: HoverContents::Array(hover_list),
                range,
            })
        }
        request::GotoDefinition::METHOD => {
            let params: GotoDefinitionParams =
                serde_json::from_value(params).expect("JSON Encoding Error while parsing params");
            println!("GotoDefinition");

            let (file_uuid, pos) =
                file_cache.location_in_file(&params.text_document_position_params);

            let mut goto_definition_list: Vec<SpanFile> = Vec::new();

            if let Some((_location, info)) = get_selected_object(&file_cache.linker, file_uuid, pos)
            {
                match info {
                    LocationInfo::InModule(_md_id, md, _decl_id, InModule::NamedLocal(decl)) => {
                        goto_definition_list.push((decl.name_span, md.link_info.file));
                    }
                    LocationInfo::InModule(
                        _md_id,
                        md,
                        _decl_id,
                        InModule::NamedSubmodule(submod_decl),
                    ) => goto_definition_list
                        .push((submod_decl.name.as_ref().unwrap().1, md.link_info.file)),
                    LocationInfo::InModule(_, _, _, InModule::Temporary(_)) => {}
                    LocationInfo::Type(_, _) => {}
                    LocationInfo::TemplateInput(_, link_info, _, template_arg) => {
                        goto_definition_list.push((template_arg.name_span, link_info.file))
                    }
                    LocationInfo::Global(id) => {
                        if let Some(link_info) = file_cache.linker.get_link_info(id) {
                            goto_definition_list.push((link_info.name_span, link_info.file));
                        }
                    }
                    LocationInfo::Port(_sm, md, port_id) => {
                        goto_definition_list.push((md.ports[port_id].name_span, md.link_info.file));
                    }
                    LocationInfo::Interface(_md_uuid, md, _interface_id, interface) => {
                        goto_definition_list.push((interface.name_span, md.link_info.file));
                    }
                }
            }

            serde_json::to_value(GotoDefinitionResponse::Array(cvt_location_list(
                goto_definition_list,
                file_cache,
            )))
        }
        request::SemanticTokensFullRequest::METHOD => {
            let params: SemanticTokensParams =
                serde_json::from_value(params).expect("JSON Encoding Error while parsing params");
            println!("SemanticTokensFullRequest");

            let uuid = file_cache.ensure_contains_file(&params.text_document.uri);

            serde_json::to_value(SemanticTokensResult::Tokens(make_semantic_tokens(
                uuid,
                &file_cache.linker,
            )))
        }
        request::DocumentHighlightRequest::METHOD => {
            let params: DocumentHighlightParams =
                serde_json::from_value(params).expect("JSON Encoding Error while parsing params");
            println!("DocumentHighlight");

            let (file_id, pos) = file_cache.location_in_file(&params.text_document_position_params);
            let file_data = &file_cache.linker.files[file_id];

            let ref_locations = gather_all_references_in_one_file(&file_cache.linker, file_id, pos);

            let result: Vec<DocumentHighlight> = ref_locations
                .into_iter()
                .map(|sp| DocumentHighlight {
                    range: span_to_lsp_range(&file_data.file_text, sp),
                    kind: None,
                })
                .collect();
            serde_json::to_value(result)
        }
        request::References::METHOD => {
            let params: ReferenceParams =
                serde_json::from_value(params).expect("JSON Encoding Error while parsing params");
            println!("FindAllReferences");

            let (file_id, pos) = file_cache.location_in_file(&params.text_document_position);

            let ref_locations =
                gather_all_references_across_all_files(&file_cache.linker, file_id, pos);

            serde_json::to_value(cvt_location_list_of_lists(ref_locations, file_cache))
        }
        request::Rename::METHOD => {
            let params: RenameParams =
                serde_json::from_value(params).expect("JSON Encoding Error while parsing params");
            println!("Rename");

            let (file_id, pos) = file_cache.location_in_file(&params.text_document_position);

            let ref_locations_lists =
                gather_all_references_across_all_files(&file_cache.linker, file_id, pos);

            let changes: HashMap<_, _> = ref_locations_lists
                .into_iter()
                .map(|(file, spans)| {
                    let file_text = &file_cache.linker.files[file].file_text;
                    (
                        file_cache.uris[file].clone(),
                        spans
                            .into_iter()
                            .map(|span| TextEdit {
                                range: span_to_lsp_range(file_text, span),
                                new_text: params.new_name.clone(),
                            })
                            .collect(),
                    )
                })
                .collect();

            println!("{changes:?}");

            serde_json::to_value(WorkspaceEdit {
                changes: Some(changes),
                document_changes: None,
                change_annotations: None,
            })
        }
        request::Completion::METHOD => {
            let params: CompletionParams =
                serde_json::from_value(params).expect("JSON Encoding Error while parsing params");
            println!("Completion");

            let (file_uuid, position) = file_cache.location_in_file(&params.text_document_position);

            serde_json::to_value(&CompletionResponse::Array(gather_completions(
                &file_cache.linker,
                file_uuid,
                position,
            )))
        }
        req => {
            println!("Other request: {req:?}");
            Ok(serde_json::Value::Null)
        }
    }
}

fn handle_notification(
    connection: &lsp_server::Connection,
    notification: lsp_server::Notification,
    file_cache: &mut LoadedFileCache,
    initialize_params: &InitializeParams,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    match notification.method.as_str() {
        notification::DidChangeTextDocument::METHOD => {
            println!("DidChangeTextDocument");
            let params: DidChangeTextDocumentParams = serde_json::from_value(notification.params)
                .expect("JSON Encoding Error while parsing params");

            let mut content_change_iter = params.content_changes.into_iter();
            let only_change = content_change_iter.next().unwrap();
            assert!(content_change_iter.next().is_none());
            assert!(only_change.range.is_none());
            file_cache.update_text(params.text_document.uri, only_change.text);

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

fn main_loop(
    connection: lsp_server::Connection,
    initialize_params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    println!("initialize_params: ");
    println!("{initialize_params}");

    let initialize_params: InitializeParams = serde_json::from_value(initialize_params).unwrap();

    let mut file_cache = initialize_all_files(&initialize_params);

    push_all_errors(&connection, &file_cache)?;

    println!("starting LSP main loop");
    for msg in &connection.receiver {
        match msg {
            lsp_server::Message::Request(req) => {
                if req.method.as_str() == request::Shutdown::METHOD {
                    println!("Shutdown request");
                    return Ok(());
                }

                let response_value = handle_request(&req.method, req.params, &mut file_cache);

                let result = response_value.unwrap();
                let response = lsp_server::Response {
                    id: req.id,
                    result: Some(result),
                    error: None,
                };
                connection
                    .sender
                    .send(lsp_server::Message::Response(response))?;
            }
            lsp_server::Message::Response(resp) => {
                println!("got response: {resp:?}");
            }
            lsp_server::Message::Notification(notification) => {
                handle_notification(
                    &connection,
                    notification,
                    &mut file_cache,
                    &initialize_params,
                )?;
            }
        }
    }
    Ok(())
}

pub fn lsp_main() -> Result<(), Box<dyn Error + Sync + Send>> {
    std::env::set_var("RUST_BACKTRACE", "1"); // Enable backtrace because I can't set it in Env vars

    println!("starting LSP server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    //let (connection, io_threads) = Connection::listen(SocketAddr::from(([127,0,0,1], 25000)))?;
    println!("Connecting on port {}...", config().lsp_port);
    let (connection, io_threads) =
        lsp_server::Connection::connect(SocketAddr::from(([127, 0, 0, 1], config().lsp_port)))?;
    println!("connection established");

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        definition_provider: Some(OneOf::Left(true)),
        document_highlight_provider: Some(OneOf::Left(true)),
        references_provider: Some(OneOf::Left(true)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        rename_provider: Some(OneOf::Left(true)),
        semantic_tokens_provider: Some(semantic_token_capabilities()),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(true),
            ..Default::default()
        }),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
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
