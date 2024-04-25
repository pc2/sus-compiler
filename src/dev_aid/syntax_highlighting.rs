
use std::{ops::Range, path::PathBuf};

use crate::{
    arena_alloc::ArenaVector,
    compiler_top::{add_file, recompile_all},
    errors::{CompileError, ErrorLevel},
    file_position::Span,
    flattening::{ConnectionWriteRoot, IdentifierType, Instruction, WireSource},
    linker::{FileUUID, FileUUIDMarker, Linker, NameElem}
};

use ariadne::*;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum IDEIdentifierType {
    Value(IdentifierType),
    Type,
    Interface,
    Constant
}

pub struct SyntaxHighlightSettings {
    pub show_tokens : bool
}

pub fn walk_name_color(all_objects : &[NameElem], linker : &Linker) -> Vec<(IDEIdentifierType, Span)> {
    let mut result : Vec<(IDEIdentifierType, Span)> = Vec::new();
    for obj_uuid in all_objects {
        let (ide_typ, link_info) = match obj_uuid {
            NameElem::Module(id) => {
                let module = &linker.modules[*id];
                for (_id, item) in &module.flattened.instructions {
                    match item {
                        Instruction::Wire(w) => {
                            match &w.source {
                                &WireSource::WireRead(from_wire) => {
                                    let decl = module.flattened.instructions[from_wire].unwrap_wire_declaration();
                                    result.push((IDEIdentifierType::Value(decl.identifier_type), w.span));
                                }
                                WireSource::UnaryOp { op:_, right:_ } => {}
                                WireSource::BinaryOp { op:_, left:_, right:_ } => {}
                                WireSource::ArrayAccess { arr:_, arr_idx:_, bracket_span:_ } => {}
                                WireSource::Constant(_) => {}
                                WireSource::NamedConstant(_name) => {
                                    result.push((IDEIdentifierType::Constant, w.span));
                                }
                                WireSource::PortRead(info) => {
                                    if let Some(port_name_span) = &info.port_name_span {
                                        result.push((IDEIdentifierType::Value(info.port_identifier_typ), *port_name_span))
                                    }
                                }
                            }
                        }
                        Instruction::Declaration(decl) => {
                            decl.typ_expr.for_each_located_type(&mut |_, span| {
                                result.push((IDEIdentifierType::Type, span));
                            });
                            result.push((IDEIdentifierType::Value(decl.identifier_type), decl.name_span));
                        }
                        Instruction::Write(conn) => {
                            match conn.to.root {
                                ConnectionWriteRoot::LocalDecl(decl_id) => {
                                    let decl = module.flattened.instructions[decl_id].unwrap_wire_declaration();
                                    result.push((IDEIdentifierType::Value(decl.identifier_type), conn.to.root_span));
                                }
                                ConnectionWriteRoot::SubModulePort(port) => {
                                    if let Some(span) = port.port_name_span {
                                        todo!("Syntax highlight for named ports")
                                    }
                                }
                            }
                        }
                        Instruction::SubModule(sm) => {
                            result.push((IDEIdentifierType::Interface, sm.module_name_span));
                        }
                        Instruction::IfStatement(_) | Instruction::ForStatement(_) => {}
                    }
                }
                (IDEIdentifierType::Interface, &module.link_info)
            }
            _other => {todo!("Name Color for non-modules not implemented")}
        };
        
        result.push((ide_typ, link_info.name_span));
    }
    result
}

pub fn compile_all(file_paths : Vec<PathBuf>) -> (Linker, ArenaVector<(PathBuf, Source), FileUUIDMarker>) {
    let mut linker = Linker::new();
    let mut paths_arena : ArenaVector<(PathBuf, Source), FileUUIDMarker> = ArenaVector::new();
    for file_path in file_paths {
        let file_text = match std::fs::read_to_string(&file_path) {
            Ok(file_text) => file_text,
            Err(reason) => {
                let file_path_disp = file_path.display();
                panic!("Could not open file '{file_path_disp}' for syntax highlighting because {reason}")
            }
        };
        
        let source = Source::from(file_text.clone());
        let uuid = add_file(file_text, &mut linker);

        paths_arena.insert(uuid, (file_path, source));
    }

    recompile_all(&mut linker);
    
    (linker, paths_arena)
}

// Requires that character_ranges.len() == tokens.len() + 1 to include EOF token
pub fn pretty_print_error<AriadneCache : Cache<FileUUID>>(error : &CompileError, file : FileUUID, linker : &Linker, file_cache : &mut AriadneCache) {
    // Generate & choose some colours for each of our elements
    let (err_color, report_kind) = match error.level {
        ErrorLevel::Error => (Color::Red, ReportKind::Error),
        ErrorLevel::Warning => (Color::Yellow, ReportKind::Warning),
    };
    let info_color = Color::Blue;

    // Assert that span is in file
    let _ = &linker.files[file].file_text[error.position];

    let error_span = error.position.into_range();

    let config = 
        Config::default()
        .with_index_type(IndexType::Byte);
    let mut report: ReportBuilder<'_, (FileUUID, Range<usize>)> = Report::build(report_kind, file, error_span.start).with_config(config);
    report = report
        .with_message(&error.reason)
        .with_label(
            Label::new((file, error_span))
                .with_message(&error.reason)
                .with_color(err_color)
        );

    for info in &error.infos {
        let info_span = info.position.into_range();
        // Assert that span is in file
        let _ = &linker.files[info.file].file_text[info.position];
        report = report.with_label(
            Label::new((info.file, info_span))
                .with_message(&info.info)
                .with_color(info_color)
        )
    }

    report.finish().eprint(file_cache).unwrap();
}

impl Cache<FileUUID> for ArenaVector<(PathBuf, Source<String>), FileUUIDMarker> {
    type Storage = String;

    fn fetch(&mut self, id: &FileUUID) -> Result<&Source, Box<dyn std::fmt::Debug + '_>> {
        Ok(&self[*id].1)
    }
    fn display<'a>(&self, id: &'a FileUUID) -> Option<Box<dyn std::fmt::Display + 'a>> {
        let text : String = self[*id].0.to_string_lossy().into_owned();
        Some(Box::new(text))
    }
}

pub fn print_all_errors(linker : &Linker, paths_arena : &mut ArenaVector<(PathBuf, Source), FileUUIDMarker>) {
    for (file_uuid, f) in &linker.files {
        let errors = linker.get_all_errors_in_file(file_uuid);

        for err in errors.get().0 {
            pretty_print_error(&err, f.parsing_errors.file, linker, paths_arena);
        }
    }
}
