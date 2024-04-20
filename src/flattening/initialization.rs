use sus_proc_macro::{field, kind};

use crate::{
    errors::ErrorCollector,
    flattening::{FlattenedModule, Module},
    instantiation::InstantiationList,
    linker::{FileBuilder, LinkInfo},
    parser::Cursor
};


pub fn gather_initial_file_data(builder : &mut FileBuilder) {
    let mut walker = Cursor::new_at_root(builder.tree, builder.file_text);
    walker.list(kind!("source_file"), |cursor| {
        let (kind, span) = cursor.kind_span();
        assert!(kind == kind!("module"));
        let name_span = cursor.go_down_no_check(|cursor| cursor.field_span(field!("name"), kind!("identifier")));
        let md = Module{
            link_info: LinkInfo {
                documentation: cursor.extract_gathered_comments(),
                file: builder.file_id,
                name: builder.file_text[name_span].to_owned(),
                name_span,
                span
            },
            flattened: FlattenedModule::empty(ErrorCollector::new(builder.file_id, builder.file_text.len())),
            instantiations: InstantiationList::new()
        };

        builder.add_module(md);
    });
}
