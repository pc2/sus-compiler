
mod tree_walk;

use std::{error::Error, ffi::OsStr, net::SocketAddr};
use lsp_types::{notification::*, request::Request, *};

use crate::{
    arena_alloc::ArenaVector, compiler_top::{add_file, recompile_all, update_file}, config::config, errors::{CompileError, ErrorLevel}, file_position::{FileText, LineCol, Span, SpanFile}, flattening::{FlatID, IdentifierType, Instruction, Module}, instantiation::{SubModuleOrWire, CALCULATE_LATENCY_LATER}, linker::{FileData, FileUUID, FileUUIDMarker, Linker, NameElem}
};

use tree_walk::{InModule, LocationInfo};



fn from_position(pos : lsp_types::Position) -> LineCol {
    LineCol{line : pos.line as usize, col : pos.character as usize}
}
fn to_position(char_line : LineCol) -> lsp_types::Position {
    lsp_types::Position{line : char_line.line as u32, character : char_line.col as u32}
}
fn span_to_lsp_range(file_text : &FileText, ch_sp : Span) -> lsp_types::Range {
    let rng = file_text.get_span_linecol_range(ch_sp);
    Range {
        start: to_position(rng.start),
        end: to_position(rng.end)
    }
}
fn cvt_location_list(location_vec: Vec<SpanFile>, file_cache: &LoadedFileCache) -> Vec<Location> {
    location_vec.into_iter().map(|(span, file)| {
        let uri = file_cache.uris[file].clone();
        let range = span_to_lsp_range(&file_cache.linker.files[file].file_text, span);
        Location{uri, range}
    }).collect()
}


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
        if let Some(found_file_uuid) = self.find_uri(&uri) {
            update_file(new_file_text, found_file_uuid, &mut self.linker);
        } else {
            let file_uuid = add_file(new_file_text, &mut self.linker);
            self.uris.insert(file_uuid, uri.clone());
        }

        recompile_all(&mut self.linker);
    }
    fn ensure_contains_file(&mut self, uri : &Url) -> FileUUID {
        if let Some(found) = self.find_uri(uri) {
            found
        } else {
            let file_text = std::fs::read_to_string(uri.to_file_path().unwrap()).unwrap();
            let file_uuid = add_file(file_text, &mut self.linker);
            recompile_all(&mut self.linker);
            file_uuid
        }
    }
    fn location_in_file(&mut self, text_pos : &lsp_types::TextDocumentPositionParams) -> (FileUUID, usize) {
        let file_id = self.ensure_contains_file(&text_pos.text_document.uri);
        let file_data = &self.linker.files[file_id];

        let position = file_data.file_text.linecol_to_byte_clamp(from_position(text_pos.position));

        (file_id, position)
    }
    fn get_selected_object(&self, file : FileUUID, byte_pos : usize) -> Option<(lsp_types::Range, LocationInfo)> {
        let file_data = &self.linker.files[file];

        let (span, info) = tree_walk::get_selected(&self.linker, file_data, byte_pos)?;

        Some((span_to_lsp_range(&file_data.file_text, span), info))
    }
}

pub fn lsp_main() -> Result<(), Box<dyn Error + Sync + Send>> {
    std::env::set_var("RUST_BACKTRACE", "1"); // Enable backtrace because I can't set it in Env vars
    
    println!("starting LSP server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    //let (connection, io_threads) = Connection::listen(SocketAddr::from(([127,0,0,1], 25000)))?;
    println!("Connecting on port {}...", config().lsp_port);
    let (connection, io_threads) = lsp_server::Connection::connect(SocketAddr::from(([127,0,0,1], config().lsp_port)))?;
    println!("connection established");
    
    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        definition_provider: Some(OneOf::Left(true)),
        document_highlight_provider: Some(OneOf::Left(true)),
        references_provider: Some(OneOf::Left(true)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        rename_provider: Some(OneOf::Left(true)),
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
pub enum IDEIdentifierType {
    Value(IdentifierType),
    Type,
    Interface,
    Constant
}

pub fn walk_name_color(file : &FileData, linker : &Linker) -> Vec<(Span, IDEIdentifierType)> {
    let mut result : Vec<(Span, IDEIdentifierType)> = Vec::new();

    tree_walk::visit_all(linker, file, |span, item| {
        result.push((span, match item {
            LocationInfo::InModule(_, _, InModule::NamedLocal(decl)) => {
                IDEIdentifierType::Value(decl.identifier_type)
            }
            LocationInfo::InModule(_, _, InModule::NamedSubmodule(_)) => {
                IDEIdentifierType::Interface
            }
            LocationInfo::InModule(_, _, InModule::Temporary(_)) => {return}
            LocationInfo::Type(_) => {return}
            LocationInfo::Global(g) => {
                match g {
                    NameElem::Module(_) => IDEIdentifierType::Interface,
                    NameElem::Type(_) => IDEIdentifierType::Type,
                    NameElem::Constant(_) => IDEIdentifierType::Constant,
                }
            }
            LocationInfo::Port(_, _, port) => {
                IDEIdentifierType::Value(port.identifier_type)
            }
        }));
    });

    result
}


fn do_syntax_highlight(file_data : &FileData, linker : &Linker) -> Vec<SemanticToken> {
    let mut ide_tokens = walk_name_color(file_data, linker);

    convert_to_semantic_tokens(file_data, &mut ide_tokens)
}

// Requires that token_positions.len() == tokens.len() + 1 to include EOF token
fn convert_diagnostic(err : &CompileError, main_file_text : &FileText, linker : &Linker, uris : &ArenaVector<Url, FileUUIDMarker>) -> Diagnostic {
    assert!(main_file_text.is_span_valid(err.position), "bad error: {}", err.reason);
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
        assert!(info_file_text.is_span_valid(info_span), "bad info in {file_name}:\n{}; in err: {}.\nSpan is {info_span}, but file length is {}", info.info, err.reason, info_file_text.len());
        let info_pos = span_to_lsp_range(info_file_text, info_span);
        let location = Location{uri : uris[info.file].clone(), range : info_pos};
        related_info.push(DiagnosticRelatedInformation { location, message: info.info.clone() });
    }
    Diagnostic::new(error_pos, Some(severity), None, None, err.reason.clone(), Some(related_info), None)
}

fn push_all_errors(connection: &lsp_server::Connection, file_cache : &LoadedFileCache) -> Result<(), Box<dyn Error + Sync + Send>> {
    for (file_id, file_data) in &file_cache.linker.files {
        let mut diag_vec : Vec<Diagnostic> = Vec::new();

        file_cache.linker.for_all_errors_in_file(file_id, |err| {
            diag_vec.push(convert_diagnostic(err, &file_data.file_text, &file_cache.linker, &file_cache.uris));
        });
        
        let params = &PublishDiagnosticsParams{
            uri: file_cache.uris[file_id].clone(),
            diagnostics: diag_vec,
            version: None
        };
        let params_json = serde_json::to_value(params)?;

        connection.sender.send(lsp_server::Message::Notification(lsp_server::Notification{
            method: PublishDiagnostics::METHOD.to_owned(),
            params: params_json
        }))?;

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

fn gather_hover_infos(md: &Module, id: FlatID, is_generative : bool, file_cache: &LoadedFileCache, hover_list: &mut Vec<MarkedString>) {
    md.instantiations.for_each_instance(|inst| {
        if is_generative {
            let value_str = match &inst.generation_state[id] {
                SubModuleOrWire::SubModule(_) | SubModuleOrWire::Wire(_) => unreachable!(),
                SubModuleOrWire::CompileTimeValue(v) => format!(" = {}", v.value.to_string()),
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
            for (_id, v) in &m.instructions {
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


fn for_each_reference_in_file<F : FnMut(Span)>(linker: &Linker, file_id: FileUUID, file_data: &FileData, hover_info: LocationInfo, mut f : F) {
    match hover_info {
        LocationInfo::InModule(md, id, _) => {
            if md.link_info.file == file_id {
                tree_walk::visit_all_in_module(&linker, md, |span, info| {
                    if let LocationInfo::InModule(_, id2, _) = info {
                        if id == id2 {
                            f(span);
                        }
                    }
                });
            }
        }
        LocationInfo::Type(_) => {}
        LocationInfo::Global(name_elem) => {
            tree_walk::visit_all(&linker, file_data, |span, info| {
                if let LocationInfo::Global(ne) = info {
                    if name_elem == ne {
                        f(span);
                    }
                }
            });
        }
        LocationInfo::Port(p_id, md_id, _) => {
            tree_walk::visit_all(&linker, file_data, |span, info| {
                if let LocationInfo::Port(p_id2, md_id2, _) = info {
                    if p_id == p_id2 && md_id == md_id2 {
                        f(span);
                    }
                }
            });
            let md = &linker.modules[md_id];
            if md.link_info.file == file_id {
                f(md.module_ports.ports[p_id].name_span);
            }
        }
    }
}

fn gather_all_references(file_cache: &LoadedFileCache, file_id: FileUUID, pos: usize) -> Vec<SpanFile> {
    let mut ref_locations = Vec::new();

    if let Some((_location, hover_info)) = file_cache.get_selected_object(file_id, pos) {
        for (other_file_id, other_file) in &file_cache.linker.files {
            for_each_reference_in_file(&file_cache.linker, other_file_id, other_file, hover_info, |span| {ref_locations.push((span, other_file_id))});
        }
    }
    ref_locations
}


fn handle_request(method : &str, params : serde_json::Value, file_cache : &mut LoadedFileCache) -> Result<serde_json::Value, serde_json::Error> {
    match method {
        request::HoverRequest::METHOD => {
            let params : HoverParams = serde_json::from_value(params).expect("JSON Encoding Error while parsing params");
            println!("HoverRequest");
            
            let (file_uuid, pos) = file_cache.location_in_file(&params.text_document_position_params);
            let file_data = &file_cache.linker.files[file_uuid];
            let mut hover_list : Vec<MarkedString> = Vec::new();
            
            let range = if let Some((location, info)) = file_cache.get_selected_object(file_uuid, pos) {
                if config().lsp_debug_mode {
                    hover_list.push(MarkedString::String(format!("{info:?}")))
                } else {
                    match info {
                        LocationInfo::InModule(md, decl_id, InModule::NamedLocal(decl)) => {
                            let typ_str = decl.typ.to_string(&file_cache.linker.types);
                            let name_str = &decl.name;

                            let identifier_type_keyword = decl.identifier_type.get_keyword();
                            hover_list.push(MarkedString::String(decl.documentation.to_string(&file_data.file_text)));
                            hover_list.push(MarkedString::String(format!("{identifier_type_keyword} {typ_str} {name_str}")));

                            gather_hover_infos(md, decl_id, decl.identifier_type.is_generative(), file_cache, &mut hover_list);
                        }
                        LocationInfo::InModule(_, _, InModule::NamedSubmodule(submod)) => {
                            let submodule = &file_cache.linker.modules[submod.module_uuid];
                            
                            // Declaration's documentation
                            hover_list.push(MarkedString::String(submod.documentation.to_string(&file_data.file_text)));

                            hover_list.push(MarkedString::String(format!("    {} {}", submodule.link_info.get_full_name(), submod.name.as_ref().expect("Impossible to select an unnamed submodule").0)));
                            hover_list.push(MarkedString::String(submodule.make_all_ports_info_string(&file_cache.linker.files[submodule.link_info.file].file_text)));
                            
                            // Module documentation
                            hover_list.push(MarkedString::String(submodule.link_info.documentation.to_string(&file_cache.linker.files[submodule.link_info.file].file_text)));
                        }
                        LocationInfo::InModule(md, id, InModule::Temporary(wire)) => {
                            let typ_str = wire.typ.to_string(&file_cache.linker.types);

                            let gen_kw = if wire.is_compiletime {"gen "} else {""};
                            hover_list.push(MarkedString::String(format!("{gen_kw}{typ_str}")));
                            gather_hover_infos(md, id, wire.is_compiletime, file_cache, &mut hover_list);
                        }
                        LocationInfo::Type(typ) => {
                            hover_list.push(MarkedString::String(typ.to_type().to_string(&file_cache.linker.types)));
                        }
                        LocationInfo::Global(global) => {
                            if let Some(link_info) = file_cache.linker.get_link_info(global) {
                                hover_list.push(MarkedString::String(link_info.documentation.to_string(&file_data.file_text)));
                            }
                            hover_list.push(MarkedString::String(format!("    {}", file_cache.linker.get_full_name(global))));
                            match global {
                                NameElem::Module(md_uuid) => {
                                    let md = &file_cache.linker.modules[md_uuid];
                                    hover_list.push(MarkedString::String(md.make_all_ports_info_string(&file_cache.linker.files[md.link_info.file].file_text)));
                                }
                                NameElem::Type(_) => {}
                                NameElem::Constant(_) => {}
                            }
                        }
                        LocationInfo::Port(port_id, md_uuid, _port) => {
                            let md = &file_cache.linker.modules[md_uuid];
                            hover_list.push(MarkedString::String(md.make_port_info_string(port_id, &file_cache.linker.files[md.link_info.file].file_text)));
                        }
                    };
                }
                Some(location)
            } else {
                None
            };
            serde_json::to_value(Hover{contents: HoverContents::Array(hover_list), range})
        }
        request::GotoDefinition::METHOD => {
            let params : GotoDefinitionParams = serde_json::from_value(params).expect("JSON Encoding Error while parsing params");
            println!("GotoDefinition");

            let (file_uuid, pos) = file_cache.location_in_file(&params.text_document_position_params);
            
            let mut goto_definition_list : Vec<SpanFile> = Vec::new();

            if let Some((_location, info)) = file_cache.get_selected_object(file_uuid, pos) {
                match info {
                    LocationInfo::InModule(md, _decl_id, InModule::NamedLocal(decl)) => {
                        goto_definition_list.push((decl.name_span, md.link_info.file));
                    }
                    LocationInfo::InModule(md, _decl_id, InModule::NamedSubmodule(submod_decl)) => {
                        goto_definition_list.push((submod_decl.module_name_span, md.link_info.file))
                    }
                    LocationInfo::InModule(_, _, InModule::Temporary(_)) => {}
                    LocationInfo::Type(_) => {}
                    LocationInfo::Global(id) => {
                        if let Some(link_info) = file_cache.linker.get_link_info(id) {
                            goto_definition_list.push((link_info.name_span, link_info.file));
                        }
                    }
                    LocationInfo::Port(_port_id, md_uuid, port) => {
                        let md = &file_cache.linker.modules[md_uuid];
                        goto_definition_list.push((port.name_span, md.link_info.file));
                    }
                }
            }

            serde_json::to_value(GotoDefinitionResponse::Array(cvt_location_list(goto_definition_list, file_cache)))
        }
        request::SemanticTokensFullRequest::METHOD => {
            let params : SemanticTokensParams = serde_json::from_value(params).expect("JSON Encoding Error while parsing params");
            println!("SemanticTokensFullRequest");

            let uuid = file_cache.ensure_contains_file(&params.text_document.uri);
            
            let file_data = &file_cache.linker.files[uuid];

            serde_json::to_value(SemanticTokensResult::Tokens(lsp_types::SemanticTokens {result_id: None, data: do_syntax_highlight(file_data, &file_cache.linker)}))
        }
        request::DocumentHighlightRequest::METHOD => {
            let params : DocumentHighlightParams = serde_json::from_value(params).expect("JSON Encoding Error while parsing params");

            let (file_id, pos) = file_cache.location_in_file(&params.text_document_position_params);
            let file_data = &file_cache.linker.files[file_id];

            let mut ref_locations = Vec::new();

            if let Some((_location, hover_info)) = file_cache.get_selected_object(file_id, pos) {
                for_each_reference_in_file(&file_cache.linker, file_id, file_data, hover_info, |span| {ref_locations.push(span)});
            }

            let result : Vec<DocumentHighlight> = ref_locations.into_iter().map(|sp| {
                DocumentHighlight{
                    range: span_to_lsp_range(&file_data.file_text, sp),
                    kind: None
                }
            }).collect();
            serde_json::to_value(result)
        }
        request::References::METHOD => {
            let params : ReferenceParams = serde_json::from_value(params).expect("JSON Encoding Error while parsing params");

            let (file_id, pos) = file_cache.location_in_file(&params.text_document_position);
  
            let ref_locations = gather_all_references(file_cache, file_id, pos);

            serde_json::to_value(cvt_location_list(ref_locations, file_cache))
        }
        request::Rename::METHOD => {
            let params : RenameParams = serde_json::from_value(params).expect("JSON Encoding Error while parsing params");

            todo!("RENAMING");
        }
        request::Completion::METHOD => {
            let params : CompletionParams = serde_json::from_value(params).expect("JSON Encoding Error while parsing params");
            println!("Completion");

            let (file_uuid, position) = file_cache.location_in_file(&params.text_document_position);
            
            serde_json::to_value(&CompletionResponse::Array(gather_completions(&file_cache.linker, file_uuid, position)))
        }
        req => {
            println!("Other request: {req:?}");
            Ok(serde_json::Value::Null)
        }
    }
}

fn handle_notification(connection: &lsp_server::Connection, notification : lsp_server::Notification, file_cache : &mut LoadedFileCache, initialize_params : &InitializeParams) -> Result<(), Box<dyn Error + Sync + Send>> {
    match notification.method.as_str() {
        notification::DidChangeTextDocument::METHOD => {
            println!("DidChangeTextDocument");
            let params : DidChangeTextDocumentParams = serde_json::from_value(notification.params).expect("JSON Encoding Error while parsing params");
            
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

fn main_loop(connection: lsp_server::Connection, initialize_params: serde_json::Value) -> Result<(), Box<dyn Error + Sync + Send>> {
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
                let response = lsp_server::Response{id : req.id, result: Some(result), error: None};
                connection.sender.send(lsp_server::Message::Response(response))?;
            }
            lsp_server::Message::Response(resp) => {
                println!("got response: {resp:?}");
            }
            lsp_server::Message::Notification(notification) => {
                handle_notification(&connection, notification, &mut file_cache, &initialize_params)?;
            }
        }
    }
    Ok(())
}
