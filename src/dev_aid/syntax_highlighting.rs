
use std::{ops::Range, path::PathBuf};

use crate::{arena_alloc::ArenaVector, ast::*, errors::{CompileError, ErrorLevel}, file_position::{FileText, Span}, flattening::{Instruction, WireSource}, linker::{FileUUID, FileUUIDMarker, Linker, NameElem}, parser::*};

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
                                    let decl = module.flattened.instructions[from_wire].extract_wire_declaration();
                                    if !decl.is_declared_in_this_module {continue;} // Virtual wires don't appear in this program text
                                    result.push((IDEIdentifierType::Value(decl.identifier_type), w.span));
                                }
                                WireSource::UnaryOp { op:_, right:_ } => {}
                                WireSource::BinaryOp { op:_, left:_, right:_ } => {}
                                WireSource::ArrayAccess { arr:_, arr_idx:_ } => {}
                                WireSource::Constant(_) => {}
                                WireSource::NamedConstant(_name) => {
                                    result.push((IDEIdentifierType::Constant, w.span));
                                }
                            }
                        }
                        Instruction::Declaration(decl) => {
                            if !decl.is_declared_in_this_module {continue;} // Virtual wires don't appear in this program text
                            decl.typ_expr.for_each_located_type(&mut |_, span| {
                                result.push((IDEIdentifierType::Type, span));
                            });
                            result.push((IDEIdentifierType::Value(decl.identifier_type), Span::new_single_token(decl.name_token)));
                        }
                        Instruction::Write(conn) => {
                            let decl = module.flattened.instructions[conn.to.root].extract_wire_declaration();
                            if !decl.is_declared_in_this_module {continue;} // Virtual wires don't appear in this program text
                            result.push((IDEIdentifierType::Value(decl.identifier_type), conn.to.root_span));
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

// Outputs character_offsets.len() == tokens.len() + 1 to include EOF token
fn generate_character_offsets(file_text : &FileText) -> Vec<Range<usize>> {
    let mut character_offsets : Vec<Range<usize>> = Vec::new();
    character_offsets.reserve(file_text.num_tokens());
    
    let mut cur_char = 0;
    let mut whitespace_start = 0;
    for tok_idx in 0..file_text.num_tokens() {
        let tok_range = file_text.get_token_range(tok_idx);

        // whitespace
        cur_char += file_text.file_text[whitespace_start..tok_range.start].chars().count();
        let token_start_char = cur_char;
        
        // actual text
        cur_char += file_text.file_text[tok_range.clone()].chars().count();
        character_offsets.push(token_start_char..cur_char);
        whitespace_start = tok_range.end;
    }

    // Final char offset for EOF
    let num_chars_in_file = cur_char + file_text.file_text[whitespace_start..].chars().count();
    character_offsets.push(cur_char..num_chars_in_file);

    character_offsets
}

pub fn compile_all(file_paths : Vec<PathBuf>) -> (Linker, ArenaVector<(PathBuf, Source), FileUUIDMarker>) {
    let mut linker = Linker::new();
    let mut paths_arena : ArenaVector<(PathBuf, Source), FileUUIDMarker> = ArenaVector::new();
    for file_path in file_paths {
        let uuid = linker.reserve_file();
        let file_text = match std::fs::read_to_string(&file_path) {
            Ok(file_text) => file_text,
            Err(reason) => {
                let file_path_disp = file_path.display();
                panic!("Could not open file '{file_path_disp}' for syntax highlighting because {reason}")
            }
        };
        
        let full_parse = perform_full_semantic_parse(file_text, uuid);
        
        println!("{:?}", full_parse.ast);

        paths_arena.insert(uuid, (file_path, Source::from(full_parse.file_text.file_text.clone())));
        linker.add_reserved_file(uuid, full_parse);
    }

    linker.recompile_all();
    
    (linker, paths_arena)
}


struct CustomSpan {
    file : FileUUID,
    span : Range<usize>
}
impl ariadne::Span for CustomSpan {
    type SourceId = FileUUID;

    fn source(&self) -> &FileUUID { &self.file }
    fn start(&self) -> usize { self.span.start }
    fn end(&self) -> usize { self.span.end }
}

// Requires that character_ranges.len() == tokens.len() + 1 to include EOF token
pub fn pretty_print_error<AriadneCache : Cache<FileUUID>>(error : &CompileError, file : FileUUID, character_ranges : &[Range<usize>], file_cache : &mut AriadneCache) {
    // Generate & choose some colours for each of our elements
    let (err_color, report_kind) = match error.level {
        ErrorLevel::Error => (Color::Red, ReportKind::Error),
        ErrorLevel::Warning => (Color::Yellow, ReportKind::Warning),
    };
    let info_color = Color::Blue;

    let error_span = error.position.to_range(character_ranges);

    let mut report: ReportBuilder<'_, CustomSpan> = Report::build(report_kind, file, error_span.start);
    report = report
        .with_message(&error.reason)
        .with_label(
            Label::new(CustomSpan{file : file, span : error_span})
                .with_message(&error.reason)
                .with_color(err_color)
        );

    for info in &error.infos {
        let info_span = info.position.to_range(character_ranges);
        report = report.with_label(
            Label::new(CustomSpan{file : info.file, span : info_span})
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
        let token_offsets = generate_character_offsets(&f.file_text);

        let errors = linker.get_all_errors_in_file(file_uuid);

        for err in errors.get().0 {
            pretty_print_error(&err, f.parsing_errors.file, &token_offsets, paths_arena);
        }
    }
}
