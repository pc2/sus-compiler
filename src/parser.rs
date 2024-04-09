
use static_init::dynamic;
use tree_sitter::{Tree, TreeCursor};

use crate::{errors::*, file_position::{FileText, Span}, linker::FileUUID};

use std::num::NonZeroU16;


pub struct FullParseResult {
    pub file_text : FileText,
    pub errors : ErrorCollector,
    pub tree : tree_sitter::Tree
}

pub fn perform_full_semantic_parse(file_text : String, file : FileUUID) -> FullParseResult {
    let file_text = FileText::new(file_text);
    
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&SUS.language).unwrap();

    let tree = parser.parse(&file_text.file_text, None).unwrap();

    let errors = ErrorCollector::new(file, file_text.len());

    report_all_tree_errors(&file_text, &tree, &errors);

    FullParseResult{
        tree,
        file_text,
        errors
    }
}


fn print_current_node_indented(file_text : &FileText, cursor : &TreeCursor) -> String {
    let indent = "  ".repeat(cursor.depth() as usize);
    let n = cursor.node();
    let cursor_span = Span::from(n.byte_range());
    let node_name = if n.kind_id() == SUS.identifier_kind {format!("\"{}\"", &file_text[cursor_span])} else {n.kind().to_owned()};
    if let Some(field_name) = cursor.field_name() {
        println!("{indent} {field_name}: {node_name} [{cursor_span}]");
    } else {
        println!("{indent} {node_name} [{cursor_span}]");
    }
    node_name
}

fn report_all_tree_errors(file_text : &FileText, tree : &Tree, errors : &ErrorCollector) {
    let mut cursor = tree.walk();
    loop {
        let n = cursor.node();
        let span = Span::from(n.byte_range());
        let node_name = print_current_node_indented(file_text, &cursor);
        if n.is_error() || n.is_missing() {

            let of_name = if let Some(field) = cursor.field_name() {
                format!("in the field '{field}' of type '{node_name}'")
            } else {
                format!("in a node of type '{node_name}'")
            };
            let (error_type, parent_node) = if n.is_missing() {
                ("missing field", n.parent().unwrap().parent().unwrap()) // Weird workaround because MISSING nodes can't properly parent?
            } else {
                ("syntax error", n.parent().unwrap())
            };
            let parent_node_name = parent_node.kind();
            let parent_info = error_info(Span::from(parent_node.byte_range()), errors.file, format!("Parent node '{parent_node_name}'"));
            errors.error_with_info(span, format!("While parsing '{parent_node_name}', parser found a {error_type} {of_name}"), vec![parent_info]);
        } else {
            if cursor.goto_first_child() {
                continue;
            }
        }
        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                return;
            }
        }
    }
}


pub struct SusTreeSitterSingleton {
    pub language : tree_sitter::Language,

    pub source_file_kind : u16,
    pub module_kind : u16,
    pub interface_ports_kind : u16,
    pub identifier_kind : u16,
    pub number_kind : u16,
    pub global_identifier_kind : u16,
    pub array_type_kind : u16,
    pub declaration_kind : u16,
    pub declaration_list_kind : u16,
    pub latency_specifier_kind : u16,
    pub unary_op_kind : u16,
    pub binary_op_kind : u16,
    pub array_op_kind : u16,
    pub func_call_kind : u16,
    pub parenthesis_expression_kind : u16,
    pub parenthesis_expression_list_kind : u16,
    pub array_bracket_expression_kind : u16,
    pub block_kind : u16,
    pub decl_assign_statement_kind : u16,
    pub assign_left_side_kind : u16,
    pub assign_to_kind : u16,
    pub write_modifiers_kind : u16,
    pub if_statement_kind : u16,
    pub for_statement_kind : u16,

    pub single_line_comment_kind : u16,
    pub multi_line_comment_kind : u16,

    pub gen_kw : u16,
    pub state_kw : u16,
    pub reg_kw : u16,
    pub initial_kw : u16,

    pub name_field : NonZeroU16,
    pub inputs_field : NonZeroU16,
    pub outputs_field : NonZeroU16,
    pub block_field : NonZeroU16,
    pub interface_ports_field : NonZeroU16,
    pub type_field : NonZeroU16,
    pub latency_specifier_field : NonZeroU16,
    pub declaration_modifiers_field : NonZeroU16,
    pub left_field : NonZeroU16,
    pub right_field : NonZeroU16,
    pub operator_field : NonZeroU16,
    pub arr_field : NonZeroU16,
    pub arr_idx_field : NonZeroU16,
    pub arguments_field : NonZeroU16,
    pub from_field : NonZeroU16,
    pub write_modifiers_field : NonZeroU16,
    pub to_field : NonZeroU16,
    pub expr_or_decl_field : NonZeroU16,
    pub assign_left_field : NonZeroU16,
    pub assign_value_field : NonZeroU16,
    pub condition_field : NonZeroU16,
    pub then_block_field : NonZeroU16,
    pub else_block_field : NonZeroU16,
    pub for_decl_field : NonZeroU16,

    pub content_field : NonZeroU16,
    pub item_field : NonZeroU16
}

impl SusTreeSitterSingleton {
    fn new() -> Self {
        let language = tree_sitter_sus::language();
        let node_kind = |name : &str| -> u16 {
            let v = language.id_for_node_kind(name, true);
            assert!(v != 0, "{name}");
            v
        };
        let keyword_kind = |name : &str| -> u16 {
            let v = language.id_for_node_kind(name, false);
            assert!(v != 0, "{name}");
            v
        };
        let field = |name : &str| -> NonZeroU16 {
            language.field_id_for_name(name).expect(name)
        };
        SusTreeSitterSingleton {
            source_file_kind : node_kind("source_file"),
            module_kind : node_kind("module"),
            interface_ports_kind : node_kind("interface_ports"),
            identifier_kind : node_kind("identifier"),
            number_kind : node_kind("number"),
            global_identifier_kind : node_kind("global_identifier"),
            array_type_kind : node_kind("array_type"),
            declaration_kind : node_kind("declaration"),
            declaration_list_kind : node_kind("declaration_list"),
            latency_specifier_kind : node_kind("latency_specifier"),
            unary_op_kind : node_kind("unary_op"),
            binary_op_kind : node_kind("binary_op"),
            array_op_kind : node_kind("array_op"),
            func_call_kind : node_kind("func_call"),
            parenthesis_expression_kind : node_kind("parenthesis_expression"),
            parenthesis_expression_list_kind : node_kind("parenthesis_expression_list"),
            array_bracket_expression_kind : node_kind("array_bracket_expression"),
            block_kind : node_kind("block"),
            decl_assign_statement_kind : node_kind("decl_assign_statement"),
            assign_left_side_kind : node_kind("assign_left_side"),
            assign_to_kind : node_kind("assign_to"),
            write_modifiers_kind : node_kind("write_modifiers"),
            if_statement_kind : node_kind("if_statement"),
            for_statement_kind : node_kind("for_statement"),

            single_line_comment_kind : node_kind("single_line_comment"),
            multi_line_comment_kind : node_kind("multi_line_comment"),

            gen_kw : keyword_kind("gen"),
            state_kw : keyword_kind("state"),
            reg_kw : keyword_kind("reg"),
            initial_kw : keyword_kind("initial"),

            name_field : field("name"),
            inputs_field : field("inputs"),
            outputs_field : field("outputs"),
            block_field : field("block"),
            interface_ports_field : field("interface_ports"),
            type_field : field("type"),
            latency_specifier_field : field("latency_specifier"),
            declaration_modifiers_field : field("declaration_modifiers"),
            left_field : field("left"),
            right_field : field("right"),
            operator_field : field("operator"),
            arr_field : field("arr"),
            arr_idx_field : field("arr_idx"),
            arguments_field : field("arguments"),
            from_field : field("from"),
            to_field : field("to"),
            write_modifiers_field : field("write_modifiers"),
            expr_or_decl_field : field("expr_or_decl"),
            assign_left_field : field("assign_left"),
            assign_value_field : field("assign_value"),
            condition_field : field("condition"),
            then_block_field : field("then_block"),
            else_block_field : field("else_block"),
            for_decl_field : field("for_decl"),

            content_field : field("content"),
            item_field : field("item"),
                    
            language,
        }
    }
}

#[dynamic]
pub static SUS : SusTreeSitterSingleton = SusTreeSitterSingleton::new();

#[derive(Debug, Clone)]
pub struct Documentation {
    gathered : Box<[Span]>
}

impl Documentation {
    pub fn to_string(&self, file_text : &FileText) -> String {
        let mut total_length = self.gathered.len().saturating_sub(1);
        for s in self.gathered.iter() {
            total_length += s.size();
        }
        let mut result = String::with_capacity(total_length);
        for s in self.gathered.iter() {
            result.push_str(&file_text[*s]);
            result.push('\n');
        }
        result
    }
}

pub struct Cursor<'t> {
    cursor : TreeCursor<'t>,
    file_text : &'t FileText,
    gathered_comments : Vec<Span>
}

impl<'t> Cursor<'t> {
    pub fn new_at_root(tree : &'t Tree, file_text : &'t FileText) -> Self {
        Self{cursor : tree.walk(), file_text, gathered_comments : Vec::new()}
    }

    pub fn new_for_node(tree : &'t Tree, file_text : &'t FileText, span : Span, kind : u16) -> Self {
        let mut cursor = tree.walk();
        assert!(cursor.goto_first_child());

        let span_range = span.into_range();
        loop {
            let node = cursor.node();
            if node.byte_range() == span_range {break}
            assert!(cursor.goto_next_sibling());
        }
        //cursor.goto_parent();
        // https://github.com/tree-sitter/tree-sitter/issues/3270
        //let _ = cursor.goto_first_child_for_byte(span_range.start).unwrap();
        let start_node = cursor.node();
        assert_eq!(start_node.kind_id(), kind);
        assert_eq!(start_node.byte_range(), span_range);

        Self{cursor, file_text, gathered_comments : Vec::new()}
    }

    pub fn kind_span(&self) -> (u16, Span) {
        let node = self.cursor.node();
        (node.kind_id(), node.byte_range().into())
    }

    pub fn kind(&self) -> u16 {
        let node = self.cursor.node();
        node.kind_id()
    }

    pub fn span(&self) -> Span {
        let node = self.cursor.node();
        node.byte_range().into()
    }

    #[track_caller]
    pub fn print_stack(&mut self) {
        let this_node_kind = self.cursor.node().kind();
        let this_node_span = self.span();
        println!("Stack:");
        loop {
            print_current_node_indented(self.file_text, &self.cursor);
            if !self.cursor.goto_parent() {break;}
        }
        println!("Current node: {this_node_kind}, {this_node_span}");
    }

    #[track_caller]
    pub fn could_not_match(&mut self) -> ! {
        self.print_stack();
        panic!();
    }

    /// The cursor advances to the next field, regardless if it is the requested field. If the found field is the requested field, the function is called. 
    /// 
    /// If no more fields are available, the cursor lands at the end of the siblings, and None is returned
    /// 
    /// If the found field is incorrect, None is returned
    pub fn optional_field<OT, F : FnOnce(&mut Self) -> OT>(&mut self, field_id : NonZeroU16, func : F) -> Option<OT> {
        loop {
            if let Some(found) = self.cursor.field_id() {
                if found == field_id {
                    let result = func(self);
                    self.cursor.goto_next_sibling();
                    return Some(result);
                } else {
                    return None; // Field found, but it's not this one. Stop here, because we've passed the possibly optional field
                }
            } else {
                self.maybe_add_comment();
                if !self.cursor.goto_next_sibling() {
                    return None;
                }
            }
        }
    }

    /// The cursor advances to the next field and calls the given function. 
    /// 
    /// Panics if the next field doesn't exist or is not the requested field
    #[track_caller]
    pub fn field<OT, F : FnOnce(&mut Self) -> OT>(&mut self, field_id : NonZeroU16, func : F) -> OT {
        loop {
            if let Some(found) = self.cursor.field_id() {
                if found == field_id {
                    let result = func(self);
                    self.cursor.goto_next_sibling();
                    return result;
                } else {
                    self.print_stack();
                    panic!("Did not find required field '{}', found field '{}' instead!", SUS.language.field_name_for_id(field_id.into()).unwrap(), SUS.language.field_name_for_id(found.into()).unwrap());
                }
            } else {
                self.maybe_add_comment();
                if !self.cursor.goto_next_sibling() {
                    self.print_stack();
                    panic!("Reached the end of child nodes without finding field '{}'", SUS.language.field_name_for_id(field_id.into()).unwrap())
                }
            }
        }
    }

    pub fn optional_field_span(&mut self, field_id : NonZeroU16, expected_kind : u16) -> Option<Span> {
        self.optional_field(field_id, |cursor| {
            let (kind, span) = cursor.kind_span();
            if kind != expected_kind {
                cursor.print_stack();
                panic!("Expected {}, Was {} instead", SUS.language.node_kind_for_id(expected_kind).unwrap(), cursor.kind());
            }
            assert!(kind == expected_kind);
            span
        })
    }

    #[track_caller]
    pub fn field_span(&mut self, field_id : NonZeroU16, expected_kind : u16) -> Span {
        self.field(field_id, |cursor| {
            let (kind, span) = cursor.kind_span();
            if kind != expected_kind {
                cursor.print_stack();
                panic!("Expected {}, Was {} instead", SUS.language.node_kind_for_id(expected_kind).unwrap(), cursor.kind());
            }
            assert!(kind == expected_kind);
            span
        })
    }

    #[track_caller]
    pub fn field_span_no_check(&mut self, field_id : NonZeroU16) -> Span {
        self.field(field_id, |cursor| cursor.span())
    }

    #[track_caller]
    pub fn go_down<OT, F : FnOnce(&mut Self) -> OT>(&mut self, kind : u16, func : F) -> OT {
        let node = self.cursor.node();
        if node.kind_id() != kind {
            self.print_stack();
            panic!("Expected {}, Was {} instead", SUS.language.node_kind_for_id(kind).unwrap(), node.kind());
        }

        self.go_down_no_check(func)
    }

    pub fn go_down_no_check<OT, F : FnOnce(&mut Self) -> OT>(&mut self, func : F) -> OT {
        let r = self.cursor.goto_first_child();
        assert!(r);
        let result = func(self);
        let r = self.cursor.goto_parent();
        assert!(r);

        result
    }

    // Some specialized functions for SUS Language

    /// Goes down the current node, checks it's kind, and then iterates through 'item' fields. 
    #[track_caller]
    pub fn list<F : FnMut(&mut Self)>(&mut self, parent_kind : u16, mut func : F) {
        self.go_down(parent_kind, |cursor| {
            loop {
                if let Some(found) = cursor.cursor.field_id() {
                    if found == SUS.item_field {
                        func(cursor);
                    } else {
                        cursor.print_stack();
                        panic!("List did not only contain 'item' fields, found field '{}' instead!", SUS.language.field_name_for_id(found.into()).unwrap());
                    }
                } else {
                    cursor.maybe_add_comment();
                }
                if !cursor.cursor.goto_next_sibling() {
                    break;
                }
            }
        });
    }

    /// Goes down the current node, checks it's kind, and then iterates through 'item' fields. 
    /// 
    /// The function given should return OT, and from the valid outputs this function constructs a output list
    #[track_caller]
    pub fn collect_list<OT, F : FnMut(&mut Self) -> OT>(&mut self, parent_kind : u16, mut func : F) -> Vec<OT> {
        let mut result = Vec::new();

        self.list(parent_kind, |cursor| {
            let item = func(cursor);
            result.push(item);
        });

        result
    }

    /// Goes down the current node, checks it's kind, and then selects the 'content' field. Useful for constructs like seq('[', field('content', $.expr), ']')
    #[track_caller]
    pub fn go_down_content<OT, F : FnOnce(&mut Self) -> OT>(&mut self, parent_kind : u16, func : F) -> OT {
        self.go_down(parent_kind, |self2| {
            self2.field(SUS.content_field, |self3| func(self3))
        })
    }

    // Comment gathering
    
    fn maybe_add_comment(&mut self) {
        let node = self.cursor.node();
        let kind = node.kind_id();

        if kind == SUS.single_line_comment_kind || kind == SUS.multi_line_comment_kind {
            let mut range = node.byte_range();
            range.start += 2; // skip '/*' or '//'
            if kind == SUS.multi_line_comment_kind {
                range.end -= 2; // skip '*/'
            }
            self.gathered_comments.push(Span::from(range));
        }
    }

    pub fn extract_gathered_comments(&mut self) -> Documentation {
        let gathered = self.gathered_comments.clone().into_boxed_slice();
        self.gathered_comments.clear();
        Documentation{gathered}
    }
    pub fn clear_gathered_comments(&mut self) {
        self.gathered_comments.clear()
    }
}
