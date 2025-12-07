mod hover_info;
mod semantic_tokens;
mod tree_walk;

use crate::{
    alloc::zip_eq,
    compiler_top::LinkerExtraFileInfoManager,
    config::{ConnectionMethod, lsp_config},
    dev_aid::ariadne_interface::{pretty_print_many_spans, pretty_print_span},
    linker::GlobalUUID,
    prelude::*,
    util::contains_duplicates,
};

use hover_info::hover;
use lsp_types::{notification::*, request::Request, *};
use semantic_tokens::{make_semantic_tokens, semantic_token_capabilities};
use std::{collections::HashMap, error::Error, net::SocketAddr, path::Path};

use crate::{
    config::config,
    errors::{CompileError, ErrorLevel},
    file_position::{FileText, LineCol},
    flattening::Instruction,
    linker::FileData,
};

use tree_walk::{InGlobal, LocationInfo, get_selected_object};

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
fn cvt_location_list(location_vec: Vec<Span>, linker: &Linker) -> Vec<Location> {
    location_vec
        .into_iter()
        .map(|span| {
            let file = &linker.files[span.get_file()];
            let uri = Url::parse(&file.file_identifier).unwrap();
            let range = span_to_lsp_range(&file.file_text, span);
            Location { uri, range }
        })
        .collect()
}
fn cvt_location_list_of_lists(
    location_vec: Vec<(FileUUID, Vec<Span>)>,
    linker: &Linker,
) -> Vec<Location> {
    let mut result_len = 0;
    for (_f, v) in &location_vec {
        result_len += v.len();
    }
    let mut result = Vec::with_capacity(result_len);
    for (file_id, vec) in location_vec {
        let file = &linker.files[file_id];
        let uri = Url::parse(&file.file_identifier).unwrap();
        for span in vec {
            let range = span_to_lsp_range(&file.file_text, span);
            result.push(Location {
                uri: uri.clone(),
                range,
            })
        }
    }
    result
}

impl Linker {
    fn find_uri(&self, uri: &Url) -> Option<FileUUID> {
        self.find_file(uri.as_str())
    }
    fn update_text(&mut self, uri: &Url, new_file_text: String, manager: &mut LSPFileManager) {
        self.add_or_update_file(uri.as_str(), new_file_text, manager);

        self.recompile_all();
    }
    fn ensure_contains_file(&mut self, uri: &Url, manager: &mut LSPFileManager) -> FileUUID {
        if let Some(found) = self.find_uri(uri) {
            found
        } else {
            let file_text = std::fs::read_to_string(uri.to_file_path().unwrap()).unwrap();

            let file_uuid = self.add_file_text(uri.to_string(), file_text, manager);
            self.recompile_all();
            file_uuid
        }
    }
    fn location_in_file(
        &mut self,
        text_pos: &lsp_types::TextDocumentPositionParams,
        manager: &mut LSPFileManager,
    ) -> (FileUUID, usize) {
        let file_id = self.ensure_contains_file(&text_pos.text_document.uri, manager);
        let file_data = &self.files[file_id];

        let position = file_data
            .file_text
            .linecol_to_byte_clamp(from_position(text_pos.position));

        (file_id, position)
    }
}

/// Requires that token_positions.len() == tokens.len() + 1 to include EOF token
fn convert_diagnostic(err: CompileError, main_file_text: &FileText, linker: &Linker) -> Diagnostic {
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
    for info in err.infos {
        let info_file = &linker.files[info.span.get_file()];
        let info_span = info.span;
        assert!(
            info_file.file_text.is_span_valid(info_span),
            "bad info in {}:\n{}; in err: {}.\nSpan is {info_span:?}, but file length is {}",
            info_file.file_identifier,
            info.info,
            err.reason,
            info_file.file_text.len()
        );
        let info_pos = span_to_lsp_range(&info_file.file_text, info_span);
        let location = Location {
            uri: Url::parse(&info_file.file_identifier).unwrap(),
            range: info_pos,
        };
        related_info.push(DiagnosticRelatedInformation {
            location,
            message: info.info,
        });
    }
    Diagnostic::new(
        error_pos,
        Some(severity),
        None,
        None,
        err.reason,
        Some(related_info),
        None,
    )
}

fn push_all_errors(
    connection: &lsp_server::Connection,
    linker: &Linker,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let errs = linker.collect_all_errors();

    for (_file_id, file_data, errs_for_file) in zip_eq(linker.files.iter(), errs.into_iter()) {
        let diag_vec: Vec<Diagnostic> = errs_for_file
            .into_iter()
            .map(|e| convert_diagnostic(e, &file_data.file_text, linker))
            .collect();

        let params = &PublishDiagnosticsParams {
            uri: Url::parse(&file_data.file_identifier).unwrap(),
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

struct LSPFileManager {}

impl LinkerExtraFileInfoManager for LSPFileManager {
    fn convert_filename(&self, path: &Path) -> String {
        Url::from_file_path(path).unwrap().into()
    }
}

fn initialize_all_files(linker: &mut Linker, init_params: &InitializeParams) -> LSPFileManager {
    let mut manager = LSPFileManager {};

    linker.add_standard_library(&mut manager);

    let files = &config().files;
    if !files.is_empty() {
        for f in files {
            let Ok(path) = f.canonicalize() else {
                warn!(
                    "Previously existing file {} no longer exists??? Ignoring",
                    f.to_string_lossy()
                );
                continue;
            };
            linker.add_file(&path, &mut manager);
        }
    } else if let Some(workspace_folder) = &init_params.workspace_folders {
        for folder in workspace_folder {
            let Ok(path) = folder.uri.to_file_path() else {
                continue;
            };

            linker.add_all_files_in_directory(&path, &mut manager);
        }
    }

    linker.recompile_all();
    manager
}

fn gather_completions(linker: &Linker, position: usize) -> Vec<CompletionItem> {
    let mut result = Vec::new();

    for (_, m) in &linker.modules {
        result.push(CompletionItem {
            label: m.link_info.name.to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        });

        if m.link_info.span.contains_pos(position) {
            for (_id, v) in &m.link_info.instructions {
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
            label: c.link_info.name.to_string(),
            kind: Some(CompletionItemKind::CONSTANT),
            ..Default::default()
        });
    }
    for (_, t) in &linker.types {
        result.push(CompletionItem {
            label: t.link_info.name.to_string(),
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

fn for_each_local_reference_in_global(
    linker: &Linker,
    obj_id: GlobalUUID,
    local: FlatID,
) -> Vec<Span> {
    let mut ref_locations = Vec::new();
    tree_walk::visit_all_in_module(linker, obj_id, |span, info| {
        if let LocationInfo::InGlobal(_, _, f_id, _) = info
            && local == f_id
        {
            ref_locations.push(span);
        }
    });
    ref_locations
}

fn gather_all_references_in_one_file(linker: &Linker, file_id: FileUUID, pos: usize) -> Vec<Span> {
    if let Some((_location, hover_info)) = get_selected_object(linker, file_id, pos) {
        let refers_to = RefersTo::from(hover_info);
        if refers_to.is_global() {
            gather_references_in_file(linker, &linker.files[file_id], refers_to)
        } else if let Some(local) = refers_to.local {
            for_each_local_reference_in_global(linker, local.0, local.1)
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

fn assert_all_refs_of_correct_length(
    location: Span,
    refs: &[Span],
    location_file: &FileData,
    other_spans_file: &FileData,
) {
    if refs.iter().any(|r| r.size() != location.size()) {
        let refs_vec: Vec<_> = refs.iter().map(|r| (String::new(), *r)).collect();
        pretty_print_span(location_file, location, "Original location Span");
        pretty_print_many_spans(other_spans_file, &refs_vec);
        panic!("One of the spans was not of the same size as the original span!")
    }
}

fn gather_all_references_across_all_files(
    linker: &Linker,
    file_id: FileUUID,
    pos: usize,
) -> Vec<(FileUUID, Vec<Span>)> {
    let mut ref_locations = Vec::new();

    let original_file = &linker.files[file_id];
    if let Some((location, hover_info)) = get_selected_object(linker, file_id, pos) {
        let refers_to = RefersTo::from(hover_info);
        if refers_to.is_global() {
            for (other_file_id, other_file) in &linker.files {
                let found_refs = gather_references_in_file(linker, other_file, refers_to);
                assert_all_refs_of_correct_length(location, &found_refs, original_file, other_file);
                if !found_refs.is_empty() {
                    ref_locations.push((other_file_id, found_refs))
                }
            }
        } else if let Some(local) = refers_to.local {
            let found_refs = for_each_local_reference_in_global(linker, local.0, local.1);
            assert_all_refs_of_correct_length(location, &found_refs, original_file, original_file);
            if !found_refs.is_empty() {
                ref_locations.push((file_id, found_refs))
            }
        }
    }
    for r in &ref_locations {
        assert!(!contains_duplicates(&r.1), "List: {:?}", &r.1);
    }

    ref_locations
}

fn goto_definition(linker: &mut Linker, file_uuid: FileUUID, pos: usize) -> Vec<Span> {
    let mut goto_definition_list: Vec<Span> = Vec::new();

    let Some((_location, info)) = get_selected_object(linker, file_uuid, pos) else {
        return Vec::new();
    };
    match info {
        LocationInfo::InGlobal(_obj_id, _link_info, _, InGlobal::NamedLocal(decl)) => {
            goto_definition_list.push(decl.name_span);
        }
        LocationInfo::InGlobal(_obj_id, _link_info, _, InGlobal::NamedSubmodule(submod_decl)) => {
            goto_definition_list.push(submod_decl.name_span);
        }
        LocationInfo::InGlobal(_obj_id, _link_info, _, InGlobal::LocalInterface(interface)) => {
            goto_definition_list.push(interface.name_span);
        }
        LocationInfo::InGlobal(_, _, _, InGlobal::Temporary(_)) => {}
        LocationInfo::Type(_, _) => {}
        LocationInfo::Parameter(_, _link_info, _, template_arg) => {
            goto_definition_list.push(template_arg.name_span);
        }
        LocationInfo::Global(id) => {
            let link_info = &linker.globals[id];
            goto_definition_list.push(link_info.name_span);
        }
        LocationInfo::Interface(_md_uuid, _md, _interface_id, interface) => {
            goto_definition_list.push(interface.name_span);
        }
    }

    goto_definition_list
}

fn handle_request(
    method: &str,
    params: serde_json::Value,
    linker: &mut Linker,
    manager: &mut LSPFileManager,
) -> Result<serde_json::Value, serde_json::Error> {
    match method {
        request::HoverRequest::METHOD => {
            let params: HoverParams =
                serde_json::from_value(params).expect("JSON Encoding Error while parsing params");
            info!("HoverRequest");

            let (file_uuid, pos) =
                linker.location_in_file(&params.text_document_position_params, manager);
            let file_data = &linker.files[file_uuid];
            let mut hover_list: Vec<MarkedString> = Vec::new();

            let range = if let Some((location, info)) = get_selected_object(linker, file_uuid, pos)
            {
                if crate::debug::is_enabled("lsp-debug") {
                    hover_list.push(MarkedString::String(format!("{info:?}")))
                } else {
                    hover_list = hover(info, linker, file_data);
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
            info!("GotoDefinition");

            let (file_uuid, pos) =
                linker.location_in_file(&params.text_document_position_params, manager);

            let goto_definition_list = goto_definition(linker, file_uuid, pos);

            serde_json::to_value(GotoDefinitionResponse::Array(cvt_location_list(
                goto_definition_list,
                linker,
            )))
        }
        request::SemanticTokensFullRequest::METHOD => {
            info!("SemanticTokensFullRequest: {params}");
            let params: SemanticTokensParams =
                serde_json::from_value(params).expect("JSON Encoding Error while parsing params");

            let uuid = linker.ensure_contains_file(&params.text_document.uri, manager);

            serde_json::to_value(SemanticTokensResult::Tokens(make_semantic_tokens(
                uuid, linker,
            )))
        }
        request::DocumentHighlightRequest::METHOD => {
            let params: DocumentHighlightParams =
                serde_json::from_value(params).expect("JSON Encoding Error while parsing params");
            info!("DocumentHighlight");

            let (file_id, pos) =
                linker.location_in_file(&params.text_document_position_params, manager);
            let file_data = &linker.files[file_id];

            let ref_locations = gather_all_references_in_one_file(linker, file_id, pos);

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
            info!("FindAllReferences");

            let (file_id, pos) = linker.location_in_file(&params.text_document_position, manager);

            let ref_locations = gather_all_references_across_all_files(linker, file_id, pos);

            serde_json::to_value(cvt_location_list_of_lists(ref_locations, linker))
        }
        request::Rename::METHOD => {
            let params: RenameParams =
                serde_json::from_value(params).expect("JSON Encoding Error while parsing params");
            info!("Rename");

            let (file_id, pos) = linker.location_in_file(&params.text_document_position, manager);

            let ref_locations_lists = gather_all_references_across_all_files(linker, file_id, pos);

            let changes: HashMap<_, _> = ref_locations_lists
                .into_iter()
                .map(|(file, spans)| {
                    let file_data = &linker.files[file];
                    (
                        Url::parse(&file_data.file_identifier).unwrap(),
                        spans
                            .into_iter()
                            .map(|span| TextEdit {
                                range: span_to_lsp_range(&file_data.file_text, span),
                                new_text: params.new_name.clone(),
                            })
                            .collect(),
                    )
                })
                .collect();

            debug!("{changes:?}");

            serde_json::to_value(WorkspaceEdit {
                changes: Some(changes),
                document_changes: None,
                change_annotations: None,
            })
        }
        request::Completion::METHOD => {
            let params: CompletionParams =
                serde_json::from_value(params).expect("JSON Encoding Error while parsing params");
            info!("Completion");

            let (_file_uuid, position) =
                linker.location_in_file(&params.text_document_position, manager);

            serde_json::to_value(CompletionResponse::Array(gather_completions(
                linker, position,
            )))
        }
        req => {
            info!("Other request: {req:?}");
            Ok(serde_json::Value::Null)
        }
    }
}

fn handle_notification(
    connection: &lsp_server::Connection,
    notification: lsp_server::Notification,
    linker: &mut Linker,
    manager: &mut LSPFileManager,
    initialize_params: &InitializeParams,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    match notification.method.as_str() {
        notification::DidChangeTextDocument::METHOD => {
            info!("DidChangeTextDocument");
            let params: DidChangeTextDocumentParams = serde_json::from_value(notification.params)
                .expect("JSON Encoding Error while parsing params");

            let mut content_change_iter = params.content_changes.into_iter();
            let only_change = content_change_iter.next().unwrap();
            assert!(content_change_iter.next().is_none());
            assert!(only_change.range.is_none());
            linker.update_text(&params.text_document.uri, only_change.text, manager);

            push_all_errors(connection, linker)?;
        }
        notification::DidChangeWatchedFiles::METHOD => {
            info!("Workspace Files modified");
            *linker = Linker::new();
            *manager = initialize_all_files(linker, initialize_params);

            push_all_errors(connection, linker)?;
        }
        other => {
            info!("got other notification: {other:?}");
        }
    }
    Ok(())
}

fn main_loop(
    connection: lsp_server::Connection,
    initialize_params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    info!("initialize_params: ");
    info!("{initialize_params}");

    let initialize_params: InitializeParams = serde_json::from_value(initialize_params).unwrap();

    let mut linker = Linker::new();
    crate::debug::create_dump_on_panic(&mut linker, |linker| {
        let mut manager = initialize_all_files(linker, &initialize_params);

        push_all_errors(&connection, &*linker)?;

        info!("starting LSP main loop");
        for msg in &connection.receiver {
            match msg {
                lsp_server::Message::Request(req) => {
                    if connection.handle_shutdown(&req)? {
                        info!("Shutdown request");
                        return Ok(());
                    }

                    let response_value =
                        handle_request(&req.method, req.params, linker, &mut manager);

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
                    info!("got response: {resp:?}");
                }
                lsp_server::Message::Notification(notification) => {
                    handle_notification(
                        &connection,
                        notification,
                        linker,
                        &mut manager,
                        &initialize_params,
                    )?;
                }
            }

            info!("All loaded files:");
            for (_id, file) in &linker.files {
                info!("File: {}", &file.file_identifier);
            }
        }
        Ok(())
    })
}

pub fn lsp_main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let cfg = lsp_config();

    info!("starting LSP server");

    // Create the transport.
    let (connection, io_threads) = match cfg.connection_method {
        ConnectionMethod::Stdio => {
            info!("LSP communicating over stdio");
            lsp_server::Connection::stdio()
        }
        ConnectionMethod::Tcp {
            port,
            should_listen,
        } => {
            let addr = SocketAddr::from(([127, 0, 0, 1], port));
            let result = if should_listen {
                info!("LSP Listening on {addr}");
                lsp_server::Connection::listen(addr)?
            } else {
                info!("LSP Attempting to connect on {addr}");
                lsp_server::Connection::connect(addr)?
            };

            info!("LSP socket connection established");
            result
        }
    };

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(ServerCapabilities {
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
    info!("shutting down server");
    Ok(())
}
