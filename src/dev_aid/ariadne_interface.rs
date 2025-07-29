use std::path::Path;
use std::{ops::Range, path::PathBuf};

use crate::compiler_top::LinkerExtraFileInfoManager;
use crate::linker::FileData;
use crate::prelude::*;

use crate::{
    alloc::ArenaVector,
    config::config,
    errors::{CompileError, ErrorLevel},
};

use ariadne::*;

impl Cache<FileUUID> for (&Linker, &mut ArenaVector<Source<String>, FileUUIDMarker>) {
    type Storage = String;

    fn fetch(&mut self, id: &FileUUID) -> Result<&Source<String>, impl std::fmt::Debug> {
        Result::<&Source<String>, ()>::Ok(&self.1[*id])
    }
    fn display<'a>(&self, id: &'a FileUUID) -> Option<impl std::fmt::Display + 'a> {
        if config().ci {
            let filename = self.0.files[*id]
                .file_identifier
                .rsplit("/")
                .next()
                .unwrap_or(self.0.files[*id].file_identifier.as_str());
            Some(filename.to_string())
        } else {
            Some(self.0.files[*id].file_identifier.clone())
        }
    }
}

struct NamedSource<'s> {
    source: Source,
    name: &'s str,
}

impl Cache<()> for NamedSource<'_> {
    type Storage = String;

    fn fetch(&mut self, _id: &()) -> Result<&Source<String>, impl std::fmt::Debug> {
        Result::<&Source<String>, ()>::Ok(&self.source)
    }
    fn display<'a>(&self, _id: &'a ()) -> Option<impl std::fmt::Display + 'a> {
        Some(self.name.to_owned())
    }
}

pub struct FileSourcesManager {
    pub file_sources: ArenaVector<Source, FileUUIDMarker>,
}

impl LinkerExtraFileInfoManager for FileSourcesManager {
    fn convert_filename(&self, path: &Path) -> String {
        path.to_string_lossy().into_owned()
    }

    fn on_file_added(&mut self, file_id: FileUUID, linker: &Linker) {
        let source = Source::from(linker.files[file_id].file_text.file_text.clone());

        self.file_sources.insert(file_id, source);
    }

    fn on_file_updated(&mut self, file_id: FileUUID, linker: &Linker) {
        let source = Source::from(linker.files[file_id].file_text.file_text.clone());

        self.file_sources[file_id] = source;
    }

    fn before_file_remove(&mut self, file_id: FileUUID, _linker: &Linker) {
        self.file_sources.remove(file_id)
    }
}

pub fn compile_all(file_paths: Vec<PathBuf>) -> (Linker, FileSourcesManager) {
    let mut linker = Linker::new();
    let mut file_source_manager = FileSourcesManager {
        file_sources: ArenaVector::new(),
    };
    linker.add_standard_library(&mut file_source_manager);

    for file_path in file_paths {
        let file_text = match std::fs::read_to_string(&file_path) {
            Ok(file_text) => file_text,
            Err(reason) => {
                let file_path_disp = file_path.display();
                panic!("Could not open file '{file_path_disp}' for syntax highlighting because {reason}")
            }
        };

        linker.add_file_text(
            file_path.to_string_lossy().into_owned(),
            file_text,
            &mut file_source_manager,
        );
    }

    linker.recompile_all_report_panics();

    (linker, file_source_manager)
}

fn ariadne_config() -> Config {
    Config::default()
        .with_index_type(IndexType::Byte)
        .with_color(config().use_color)
}

pub fn pretty_print_error<AriadneCache: Cache<FileUUID>>(
    error: CompileError,
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

    let error_span = error.position.as_range();

    let config = ariadne_config();
    let mut report: ReportBuilder<'_, (FileUUID, Range<usize>)> =
        Report::build(report_kind, (file, error_span.clone())).with_config(config);
    report = report.with_message(&error.reason).with_label(
        Label::new((file, error_span))
            .with_message(error.reason)
            .with_color(err_color),
    );

    for info in error.infos {
        let info_span = info.position.as_range();
        // Assert that span is in file
        let _ = &linker.files[info.file].file_text[info.position];
        report = report.with_label(
            Label::new((info.file, info_span))
                .with_message(info.info)
                .with_color(info_color),
        )
    }

    report.finish().eprint(file_cache).unwrap();
}

pub fn print_all_errors(
    linker: &Linker,
    ariadne_sources: &mut ArenaVector<Source, FileUUIDMarker>,
) {
    let mut source_cache = (linker, ariadne_sources);
    let errors = linker.collect_all_errors();
    for (file_uuid, errs) in errors {
        for err in errs {
            pretty_print_error(err, file_uuid, linker, &mut source_cache);
        }
    }
}

pub fn pretty_print_spans_in_reverse_order(file_data: &FileData, spans: Vec<Range<usize>>) {
    let text_len = file_data.file_text.len();
    let mut source = NamedSource {
        source: Source::from(file_data.file_text.file_text.clone()),
        name: &file_data.file_identifier,
    };

    for span in spans.into_iter().rev() {
        // If span not in file, just don't print it. This happens.
        if span.end > text_len {
            println!(
                "Span({}, {}) certainly does not correspond to this file. ",
                span.start, span.end
            );
            return;
        }

        let config = ariadne_config();

        let mut report: ReportBuilder<'_, Range<usize>> =
            Report::build(ReportKind::Advice, span.clone()).with_config(config);
        report = report.with_label(
            Label::new(span.clone())
                .with_message(format!("Span({}, {})", span.start, span.end))
                .with_color(Color::Blue),
        );

        report.finish().print(&mut source).unwrap();
    }
}

pub fn pretty_print_span(file_data: &FileData, span: Range<usize>, label: impl ToString) {
    let text_len = file_data.file_text.len();
    let mut source = NamedSource {
        source: Source::from(file_data.file_text.file_text.clone()),
        name: &file_data.file_identifier,
    };

    // If span not in file, just don't print it. This happens.
    if span.end > text_len {
        println!(
            "Span({}, {}) certainly does not correspond to this file. ",
            span.start, span.end
        );
        return;
    }

    let config = ariadne_config();

    let mut report: ReportBuilder<'_, Range<usize>> =
        Report::build(ReportKind::Advice, span.clone()).with_config(config);
    report = report.with_label(
        Label::new(span.clone())
            .with_message(label)
            .with_color(Color::Blue),
    );

    report.finish().print(&mut source).unwrap();
}

pub fn pretty_print_many_spans(file_data: &FileData, spans: &[(String, Range<usize>)]) {
    let text_len = file_data.file_text.len();
    let mut source = NamedSource {
        source: Source::from(file_data.file_text.file_text.clone()),
        name: &file_data.file_identifier,
    };

    let config = ariadne_config();

    if spans.is_empty() {
        return;
    }

    let mut report: ReportBuilder<'_, Range<usize>> =
        Report::build(ReportKind::Advice, spans[0].1.clone()).with_config(config);

    for (text, span) in spans.iter().rev() {
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
