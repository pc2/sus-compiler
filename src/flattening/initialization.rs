use sus_proc_macro::{field, kind, kw};

use crate::linker::IsExtern;
use crate::linker::passes::ResolvedGlobals;
use crate::prelude::*;

use crate::flattening::Module;
use crate::linker::{FileBuilder, LinkInfo};

use super::parser::Cursor;
use super::*;

pub fn gather_initial_file_data(mut builder: FileBuilder) {
    assert!(builder.file_data.associated_values.is_empty());

    let mut cursor = match Cursor::new_at_root(builder.file_id, builder.file_data) {
        Ok(cursor) => cursor,
        Err(file_span) => {
            builder
                .other_parsing_errors
                .error(file_span, "An ERROR node at the root of the syntax tree!");

            return;
        }
    };

    cursor.list_and_report_errors(
        kind!("source_file"),
        builder.other_parsing_errors,
        |cursor| {
            let whole_file_span =
                Span::from_range(0..builder.file_data.file_text.len(), builder.file_id);
            let parsing_errors = ErrorCollector::new_empty(whole_file_span, builder.files);
            cursor.report_all_decendant_errors(&parsing_errors);

            let span = cursor.span();
            cursor.go_down(kind!("global_object"), |cursor| {
                initialize_global_object(&mut builder, parsing_errors, span, cursor);
            });
        },
    );
}

enum GlobalObjectKind {
    Module,
    Const,
    Struct,
}

fn initialize_global_object(
    builder: &mut FileBuilder,
    parsing_errors: ErrorCollector,
    span: Span,
    cursor: &mut Cursor,
) {
    let is_extern = match cursor
        .optional_field(field!("extern_marker"))
        .then(|| cursor.kind())
    {
        None => IsExtern::Normal,
        Some(kw!("extern")) => IsExtern::Extern,
        Some(kw!("__builtin__")) => IsExtern::Builtin,
        Some(_) => cursor.could_not_match(),
    };

    cursor.field(field!("object_type"));
    let global_obj_kind = match cursor.kind() {
        kw!("module") => GlobalObjectKind::Module,
        kind!("const_and_type") => GlobalObjectKind::Const,
        kw!("struct") => GlobalObjectKind::Struct,
        _other => cursor.could_not_match(),
    };

    let (name_span, name) = cursor.field_to_string(field!("name"), kind!("identifier"));

    let link_info = LinkInfo {
        parameters: FlatAlloc::new(),
        instructions: FlatAlloc::new(),
        documentation: cursor.extract_gathered_docs(),
        name,
        name_span,
        span,
        errors: parsing_errors.into_storage(),
        is_extern,
        resolved_globals: ResolvedGlobals::default(),
        checkpoints: Vec::new(),
    };

    match global_obj_kind {
        GlobalObjectKind::Module => {
            builder.add_module(Module {
                link_info,
                ports: FlatAlloc::new(),
                inference_info: PortLatencyInferenceInfo::default(),
                domains: FlatAlloc::new(),
                interfaces: FlatAlloc::new(),
            });
        }
        GlobalObjectKind::Struct => {
            builder.add_type(StructType {
                link_info,
                fields: FlatAlloc::new(),
            });
        }
        GlobalObjectKind::Const => {
            builder.add_const(NamedConstant {
                link_info,
                output_decl: FlatID::PLACEHOLDER,
            });
        }
    }
}
