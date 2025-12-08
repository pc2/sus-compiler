use crate::{linker::LinkerFiles, prelude::*};
use ariadne::*;
// disambiguate Span - it's ours, from ariadne's
use crate::prelude::Span;

use std::path::PathBuf;

use crate::{
    compiler_top::LinkerExtraFileInfoManager,
    config::config,
    errors::{CompileError, ErrorLevel},
};

impl Cache<FileUUID> for &LinkerFiles {
    type Storage = String;

    fn fetch(&mut self, file_id: &FileUUID) -> Result<&Source<String>, impl std::fmt::Debug> {
        let file_data = &self[*file_id];
        let result = file_data
            .ariadne_source
            .get_or_init(|| Source::from(file_data.file_text.file_text.clone()));

        Result::<&Source<String>, ()>::Ok(result)
    }
    fn display<'a>(&self, file_id: &'a FileUUID) -> Option<impl std::fmt::Display + 'a> {
        let file_data = &self[*file_id];
        if config().ci {
            let filename = file_data
                .file_identifier
                .rsplit("/")
                .next()
                .unwrap_or(file_data.file_identifier.as_str());
            Some(filename.to_string())
        } else {
            Some(file_data.file_identifier.clone())
        }
    }
}

impl ariadne::Span for Span {
    type SourceId = FileUUID;

    fn source(&self) -> &Self::SourceId {
        self.get_file_ref()
    }

    fn start(&self) -> usize {
        self.as_range().start
    }

    fn end(&self) -> usize {
        self.as_range().end
    }
}

pub fn compile_all(linker: &mut Linker, file_paths: Vec<PathBuf>) {
    linker.add_standard_library(&mut ());

    for file_path in file_paths {
        let file_text = match std::fs::read_to_string(&file_path) {
            Ok(file_text) => file_text,
            Err(reason) => {
                let file_path_disp = file_path.display();
                panic!(
                    "Could not open file '{file_path_disp}' for syntax highlighting because {reason}"
                )
            }
        };

        linker.add_file_text(().convert_filename(&file_path), file_text, &mut ());
    }

    linker.recompile_all();
}

fn ariadne_config() -> Config {
    Config::default()
        .with_index_type(IndexType::Byte)
        .with_color(config().use_color)
}

pub fn pretty_print_error<AriadneCache: Cache<FileUUID>>(
    error: CompileError,
    file_cache: &mut AriadneCache,
) {
    // Generate & choose some colours for each of our elements
    let (err_color, report_kind) = match error.level {
        ErrorLevel::Error => (Color::Red, ReportKind::Error),
        ErrorLevel::Warning => (Color::Yellow, ReportKind::Warning),
    };
    let info_color = Color::Blue;

    let config = ariadne_config();
    let mut report: ReportBuilder<'_, Span> =
        Report::build(report_kind, error.position).with_config(config);
    report = report.with_message(&error.reason).with_label(
        Label::new(error.position)
            .with_message(error.reason)
            .with_color(err_color),
    );

    for info in error.infos {
        report = report.with_label(
            Label::new(info.span)
                .with_message(info.info)
                .with_color(info_color),
        )
    }

    report.finish().eprint(file_cache).unwrap();
}

pub fn print_all_errors(linker: &Linker) {
    let errors = linker.collect_all_errors();
    for (_file_uuid, errs) in errors {
        for err in errs {
            pretty_print_error(err, &mut &linker.files);
        }
    }
}

pub fn pretty_print_span(linker_files: &LinkerFiles, span: Span, label: String) {
    pretty_print_many_spans(linker_files, [(span, label)].into_iter());
}

pub fn pretty_print_many_spans(
    mut linker_files: &LinkerFiles,
    mut spans_iter: impl Iterator<Item = (Span, String)>,
) {
    let config = ariadne_config();

    let Some(first_span) = spans_iter.next() else {
        return;
    };

    let mut report: ReportBuilder<'_, Span> =
        Report::build(ReportKind::Advice, first_span.0).with_config(config);

    for (span, label) in std::iter::once(first_span).chain(spans_iter) {
        report = report.with_label(
            Label::new(span)
                .with_message(label)
                .with_color(Color::Blue),
        );
    }
    report.finish().eprint(&mut linker_files).unwrap();
}
