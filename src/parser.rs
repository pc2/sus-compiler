
use sus_proc_macro::{field, kind, kw};
use tree_sitter::{Tree, TreeCursor};

use crate::{errors::*, file_position::{FileText, Span}};

use std::num::NonZeroU16;


fn print_current_node_indented(file_text : &FileText, cursor : &TreeCursor) -> String {
    let indent = "  ".repeat(cursor.depth() as usize);
    let n = cursor.node();
    let cursor_span = Span::from(n.byte_range());
    let node_name = if n.kind_id() == kind!("identifier") {
        format!("\"{}\"", &file_text[cursor_span])
    } else {
        n.kind().to_owned()
    };
    if let Some(field_name) = cursor.field_name() {
        println!("{indent} {field_name}: {node_name} [{cursor_span}]");
    } else {
        println!("{indent} {node_name} [{cursor_span}]");
    }
    node_name
}

pub fn report_all_tree_errors(file_text : &FileText, tree : &Tree, errors : &ErrorCollector) {
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
    gathered_comments : Vec<Span>,
    current_field_was_already_consumed : bool,
}

impl<'t> Cursor<'t> {
    pub fn new_at_root(tree : &'t Tree, file_text : &'t FileText) -> Self {
        Self{cursor : tree.walk(), file_text, gathered_comments : Vec::new(), current_field_was_already_consumed : false}
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
        // Broken due to https://github.com/tree-sitter/tree-sitter/issues/3270
        //let _ = cursor.goto_first_child_for_byte(span_range.start).unwrap();
        let start_node = cursor.node();
        assert_eq!(start_node.kind_id(), kind);
        assert_eq!(start_node.byte_range(), span_range);

        Self{cursor, file_text, gathered_comments : Vec::new(), current_field_was_already_consumed : false}
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
    #[must_use]
    pub fn optional_field(&mut self, field_id : NonZeroU16) -> bool {
        // If a previous call to field already found this field, then we must immediately skip it. 
        if self.current_field_was_already_consumed {
            if !self.cursor.goto_next_sibling() {
                self.current_field_was_already_consumed = false;
                return false;
            }
        }
        loop {
            if let Some(found) = self.cursor.field_id() {
                if found == field_id {
                    self.current_field_was_already_consumed = true;
                    return true;
                } else {
                    self.current_field_was_already_consumed = false;
                    return false; // Field found, but it's not this one. Stop here, because we've passed the possibly optional field
                }
            } else {
                self.maybe_add_comment();
                if !self.cursor.goto_next_sibling() {
                    self.current_field_was_already_consumed = false;
                    return false;
                }
            }
        }
    }

    /// The cursor advances to the next field and calls the given function. 
    /// 
    /// Panics if the next field doesn't exist or is not the requested field
    #[track_caller]
    pub fn field(&mut self, field_id : NonZeroU16) {
        if !self.optional_field(field_id) {
            self.print_stack();
            panic!("Did not find required field '{}'", tree_sitter_sus::language().field_name_for_id(field_id.into()).unwrap());
        }
    }

    #[track_caller]
    fn get_span_check_kind(&mut self, expected_kind : u16) -> Span {
        let node = self.cursor.node();
        let kind = node.kind_id();
        let span = node.byte_range().into();
        if kind != expected_kind {
            self.print_stack();
            panic!("Expected {}, Was {} instead", tree_sitter_sus::language().node_kind_for_id(expected_kind).unwrap(), node.kind());
        }
        span
    }

    #[track_caller]
    fn assert_is_kind(&mut self, expected_kind : u16) {
        let node = self.cursor.node();
        let kind = node.kind_id();
        if kind != expected_kind {
            self.print_stack();
            panic!("Expected {}, Was {} instead", tree_sitter_sus::language().node_kind_for_id(expected_kind).unwrap(), node.kind());
        }
    }

    #[track_caller]
    pub fn optional_field_span(&mut self, field_id : NonZeroU16, expected_kind : u16) -> Option<Span> {
        if self.optional_field(field_id) {
            Some(self.get_span_check_kind(expected_kind))
        } else {
            None
        }
    }

    #[track_caller]
    pub fn field_span(&mut self, field_id : NonZeroU16, expected_kind : u16) -> Span {
        self.field(field_id);

        self.get_span_check_kind(expected_kind)
    }

    #[track_caller]
    pub fn go_down<OT, F : FnOnce(&mut Self) -> OT>(&mut self, kind : u16, func : F) -> OT {
        self.assert_is_kind(kind);

        self.go_down_no_check(func)
    }

    pub fn go_down_no_check<OT, F : FnOnce(&mut Self) -> OT>(&mut self, func : F) -> OT {
        if !self.cursor.goto_first_child() {
            self.print_stack();
            panic!("Could not go down this node!");
        }
        self.current_field_was_already_consumed = false;
        let result = func(self);
        let r = self.cursor.goto_parent();
        assert!(r);

        self.current_field_was_already_consumed = true;

        result
    }

    // Some specialized functions for SUS Language

    /// Goes down the current node, checks it's kind, and then iterates through 'item' fields. 
    #[track_caller]
    pub fn list<F : FnMut(&mut Self)>(&mut self, parent_kind : u16, mut func : F) {
        self.go_down(parent_kind, |cursor| {
            loop {
                if let Some(found) = cursor.cursor.field_id() {
                    if found == field!("item") {
                        func(cursor);
                    } else {
                        cursor.print_stack();
                        panic!("List did not only contain 'item' fields, found field '{}' instead!", tree_sitter_sus::language().field_name_for_id(found.into()).unwrap());
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
            self2.field(field!("content"));
            func(self2)
        })
    }

    // Comment gathering
    
    fn maybe_add_comment(&mut self) {
        let node = self.cursor.node();
        let kind = node.kind_id();

        if kind == kind!("single_line_comment") || kind == kind!("multi_line_comment") {
            let mut range = node.byte_range();
            range.start += 2; // skip '/*' or '//'
            if kind == kind!("multi_line_comment") {
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
