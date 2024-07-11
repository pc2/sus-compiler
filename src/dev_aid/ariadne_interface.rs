use std::{ops::Range, path::PathBuf};

use crate::prelude::*;

use crate::{
    alloc::ArenaVector,
    compiler_top::{add_file, recompile_all},
    config::config,
    errors::{CompileError, ErrorLevel},
};

use ariadne::*;

pub fn compile_all(
    file_paths: Vec<PathBuf>,
) -> (Linker, ArenaVector<(PathBuf, Source), FileUUIDMarker>) {
    let mut linker = Linker::new();
    let mut paths_arena: ArenaVector<(PathBuf, Source), FileUUIDMarker> = ArenaVector::new();
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

pub fn pretty_print_error<AriadneCache: Cache<FileUUID>>(
    error: &CompileError,
    file: FileUUID,
    linker: &Linker,
    file_cache: &mut AriadneCache,
) {
    // Generate & choose some colours for each of our elements
    let (err_color, report_kind) = match error.level {
        ErrorLevel::Error => (Color::Red, ReportKind::Error),
        ErrorLevel::Warning => (Color::Yellow, ReportKind::Warning),
    };
    let info_color = Color::Blue;

    // Assert that span is in file
    let _ = &linker.files[file].file_text[error.position];

    let error_span = error.position.into_range();

    let config = Config::default().with_index_type(IndexType::Byte);
    let mut report: ReportBuilder<'_, (FileUUID, Range<usize>)> =
        Report::build(report_kind, file, error_span.start).with_config(config);
    report = report.with_message(&error.reason).with_label(
        Label::new((file, error_span))
            .with_message(&error.reason)
            .with_color(err_color),
    );

    for info in &error.infos {
        let info_span = info.position.into_range();
        // Assert that span is in file
        let _ = &linker.files[info.file].file_text[info.position];
        report = report.with_label(
            Label::new((info.file, info_span))
                .with_message(&info.info)
                .with_color(info_color),
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
        let text: String = self[*id].0.to_string_lossy().into_owned();
        Some(Box::new(text))
    }
}

pub fn print_all_errors(
    linker: &Linker,
    paths_arena: &mut ArenaVector<(PathBuf, Source), FileUUIDMarker>,
) {
    for (file_uuid, _f) in &linker.files {
        linker.for_all_errors_in_file(file_uuid, |err| {
            pretty_print_error(err, file_uuid, linker, paths_arena);
        });
    }
}

pub fn pretty_print_spans_in_reverse_order(file_text: String, spans: Vec<Range<usize>>) {
    let text_len = file_text.len();
    let mut source = Source::from(file_text);

    for span in spans.into_iter().rev() {
        // If span not in file, just don't print it. This happens.
        if span.end > text_len {
            println!(
                "Span({}, {}) certainly does not correspond to this file. ",
                span.start, span.end
            );
            return;
        }

        let config = Config::default()
            .with_index_type(IndexType::Byte)
            .with_color(!config().use_lsp); // Disable color because LSP output doesn't support it

        let mut report: ReportBuilder<'_, Range<usize>> =
            Report::build(ReportKind::Advice, (), span.start).with_config(config);
        report = report.with_label(
            Label::new(span.clone())
                .with_message(format!("Span({}, {})", span.start, span.end))
                .with_color(Color::Blue),
        );

        report.finish().print(&mut source).unwrap();
    }
}

pub fn pretty_print_many_spans(file_text: String, spans: &[(String, Range<usize>)]) {
    let text_len = file_text.len();
    let mut source = Source::from(file_text);

    let config = Config::default()
        .with_index_type(IndexType::Byte)
        .with_color(!config().use_lsp); // Disable color because LSP output doesn't support it

    if spans.len() == 0 {
        return;
    }

    let mut report: ReportBuilder<'_, Range<usize>> =
        Report::build(ReportKind::Advice, (), spans[0].1.start).with_config(config);

    for (text, span) in spans.into_iter().rev() {
        // If span not in file, just don't print it. This happens.
        if span.end > text_len {
            println!(
                "Span({}, {}) certainly does not correspond to this file. ",
                span.start, span.end
            );
            return;
        }

        report = report.with_label(
            Label::new(span.clone())
                .with_message(text)
                .with_color(Color::Blue),
        );
    }
    report.finish().print(&mut source).unwrap();
}
