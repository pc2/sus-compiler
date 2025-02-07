use crate::prelude::*;

use sus_proc_macro::{field, kind, kw};
use tree_sitter::{Tree, TreeCursor};

use crate::{file_position::FileText, linker::Documentation};

use std::num::NonZeroU16;

pub fn get_readable_node_name(file_text: &FileText, kind: u16, span: Span) -> &str {
    if kind == kind!("identifier") {
        &file_text[span]
    } else if kind == kw!("\n") {
        "\\n"
    } else {
        tree_sitter_sus::language().node_kind_for_id(kind).unwrap()
    }
}

fn print_current_node_indented<'ft>(file_text: &'ft FileText, cursor: &TreeCursor) -> &'ft str {
    let indent = "  ".repeat(cursor.depth() as usize);
    let n = cursor.node();
    let kind = n.kind_id();
    let cursor_span = Span::from(n.byte_range());
    let node_name = get_readable_node_name(file_text, kind, cursor_span);
    if let Some(field_name) = cursor.field_name() {
        println!("{indent} {field_name}: {node_name} [{cursor_span}]");
    } else {
        println!("{indent} {node_name} [{cursor_span}]");
    }
    node_name
}

/// Wraps the tree-sitter [TreeCursor] for a more functional-style interface.
///
/// Especially with regards to going up and down the syntax tree, this module provides [Self::go_down] and friends.
///
/// This module also handles documentation comment gathering
#[derive(Clone)]
pub struct Cursor<'t> {
    cursor: TreeCursor<'t>,
    file_text: &'t FileText,
    gathered_comments: Vec<Span>,
    current_field_was_already_consumed: bool,
}

impl<'t> Cursor<'t> {
    pub fn new_at_root(tree: &'t Tree, file_text: &'t FileText) -> Self {
        Self {
            cursor: tree.walk(),
            file_text,
            gathered_comments: Vec::new(),
            current_field_was_already_consumed: false,
        }
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
            if !self.cursor.goto_parent() {
                break;
            }
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
    /// If no more fields are available, the cursor lands at the end of the siblings, and false is returned
    ///
    /// If the found field is incorrect, false is returned
    #[must_use]
    pub fn optional_field(&mut self, field_id: NonZeroU16) -> bool {
        // If a previous call to field already found this field, then we must immediately skip it.
        if self.current_field_was_already_consumed && !self.cursor.goto_next_sibling() {
            self.current_field_was_already_consumed = false;
            return false;
        }
        loop {
            if let Some(found) = self.cursor.field_id() {
                if found == field_id {
                    self.current_field_was_already_consumed = true;
                    return true;
                } else {
                    self.current_field_was_already_consumed = false;
                    //println!("Optional field '{}' not found. Found '{}' instead", tree_sitter_sus::language().field_name_for_id(field_id.get()).unwrap(), tree_sitter_sus::language().field_name_for_id(found.get()).unwrap());
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
    pub fn field(&mut self, field_id: NonZeroU16) {
        if !self.optional_field(field_id) {
            self.print_stack();
            panic!(
                "Did not find required field '{}'",
                tree_sitter_sus::language()
                    .field_name_for_id(field_id.into())
                    .unwrap()
            );
        }
    }

    #[track_caller]
    fn get_span_check_kind(&mut self, expected_kind: u16) -> Span {
        let node = self.cursor.node();
        let kind = node.kind_id();
        let span = node.byte_range().into();
        if kind != expected_kind {
            self.print_stack();
            panic!(
                "Expected {}, Was {} instead",
                tree_sitter_sus::language()
                    .node_kind_for_id(expected_kind)
                    .unwrap(),
                node.kind()
            );
        }
        span
    }

    #[track_caller]
    fn assert_is_kind(&mut self, expected_kind: u16) {
        let node = self.cursor.node();
        let kind = node.kind_id();
        if kind != expected_kind {
            self.print_stack();
            panic!(
                "Expected {}, Was {} instead",
                tree_sitter_sus::language()
                    .node_kind_for_id(expected_kind)
                    .unwrap(),
                node.kind()
            );
        }
    }

    #[track_caller]
    pub fn optional_field_span(
        &mut self,
        field_id: NonZeroU16,
        expected_kind: u16,
    ) -> Option<Span> {
        if self.optional_field(field_id) {
            Some(self.get_span_check_kind(expected_kind))
        } else {
            None
        }
    }

    #[track_caller]
    pub fn field_span(&mut self, field_id: NonZeroU16, expected_kind: u16) -> Span {
        self.field(field_id);

        self.get_span_check_kind(expected_kind)
    }

    #[track_caller]
    pub fn go_down<OT>(&mut self, kind: u16, func: impl FnOnce(&mut Self) -> OT) -> OT {
        self.assert_is_kind(kind);

        self.go_down_no_check(func)
    }

    pub fn go_down_no_check<OT>(&mut self, func: impl FnOnce(&mut Self) -> OT) -> OT {
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
    pub fn list(&mut self, parent_kind: u16, mut func: impl FnMut(&mut Self)) {
        self.assert_is_kind(parent_kind);

        if self.cursor.goto_first_child() {
            loop {
                if let Some(found) = self.cursor.field_id() {
                    if found == field!("item") {
                        func(self);
                    } else {
                        self.print_stack();
                        panic!(
                            "List did not only contain 'item' fields, found field '{}' instead!",
                            tree_sitter_sus::language()
                                .field_name_for_id(found.into())
                                .unwrap()
                        );
                    }
                } else {
                    self.maybe_add_comment();
                }
                if !self.cursor.goto_next_sibling() {
                    break;
                }
            }
            assert!(self.cursor.goto_parent());
        }
    }

    /// Goes down the current node, checks it's kind, and then iterates through 'item' fields.
    ///
    /// The function given should return OT, and from the valid outputs this function constructs a output list
    #[track_caller]
    pub fn collect_list<OT>(
        &mut self,
        parent_kind: u16,
        mut func: impl FnMut(&mut Self) -> OT,
    ) -> Vec<OT> {
        let mut result = Vec::new();

        self.list(parent_kind, |cursor| {
            let item = func(cursor);
            result.push(item);
        });

        result
    }

    /// Goes down the current node, checks it's kind, and then selects the 'content' field. Useful for constructs like seq('[', field('content', $.expr), ']')
    #[track_caller]
    pub fn go_down_content<OT>(
        &mut self,
        parent_kind: u16,
        func: impl FnOnce(&mut Self) -> OT,
    ) -> OT {
        self.go_down(parent_kind, |self2| {
            self2.field(field!("content"));
            func(self2)
        })
    }

    // Comment gathering

    fn maybe_add_comment(&mut self) {
        let node = self.cursor.node();
        let kind = node.kind_id();

        if kind == kind!("doc_comment") {
            let mut range = node.byte_range();
            range.start += 3; // skip '///'
            self.gathered_comments.push(Span::from(range));
        } else if kind == kind!("single_line_comment") || kind == kind!("multi_line_comment") {
            self.clear_gathered_comments();
        }
    }

    pub fn extract_gathered_comments(&mut self) -> Documentation {
        let gathered = self.gathered_comments.clone().into_boxed_slice();
        self.gathered_comments.clear();
        Documentation { gathered }
    }
    pub fn clear_gathered_comments(&mut self) {
        self.gathered_comments.clear()
    }

    // Error reporting

    pub fn push_potential_node_error(&mut self, errors: &ErrorCollector) -> bool {
        let node = self.cursor.node();
        let is_error = node.is_error() || node.is_missing();
        if is_error {
            let node_name = node.kind();
            let span = Span::from(node.byte_range());
            let of_name = if let Some(field) = self.cursor.field_name() {
                format!("in the field '{field}' of type '{node_name}'")
            } else {
                format!("in a node of type '{node_name}'")
            };
            let (error_type, parent_node) = if node.is_missing() {
                ("missing field", node.parent().unwrap().parent().unwrap()) // Weird workaround because MISSING nodes can't properly parent?
            } else {
                ("syntax error", node.parent().unwrap())
            };
            let parent_node_name = parent_node.kind();
            errors
                .error(
                    span,
                    format!(
                        "While parsing '{parent_node_name}', parser found a {error_type} {of_name}"
                    ),
                )
                .info_same_file(
                    Span::from(parent_node.byte_range()),
                    format!("Parent node '{parent_node_name}'"),
                );
        }
        is_error
    }

    pub fn report_all_decendant_errors(&mut self, errors: &ErrorCollector) {
        let mut depth = 0;
        assert!(self.cursor.goto_first_child());
        loop {
            if !self.push_potential_node_error(errors) && self.cursor.goto_first_child() {
                depth += 1;
                continue;
            }
            while !self.cursor.goto_next_sibling() {
                assert!(self.cursor.goto_parent());
                if depth == 0 {
                    return;
                } else {
                    depth -= 1;
                }
            }
        }
    }

    /// Goes down the current node, checks it's kind, and then iterates through 'item' fields.
    #[track_caller]
    pub fn list_and_report_errors(
        &mut self,
        parent_kind: u16,
        errors: &ErrorCollector,
        mut func: impl FnMut(&mut Self),
    ) {
        self.assert_is_kind(parent_kind);
        if self.cursor.goto_first_child() {
            loop {
                self.push_potential_node_error(errors);
                if let Some(found) = self.cursor.field_id() {
                    if found == field!("item") {
                        func(self);
                    } else {
                        self.print_stack();
                        panic!(
                            "List did not only contain 'item' fields, found field '{}' instead!",
                            tree_sitter_sus::language()
                                .field_name_for_id(found.into())
                                .unwrap()
                        );
                    }
                } else {
                    self.maybe_add_comment();
                }
                if !self.cursor.goto_next_sibling() {
                    break;
                }
            }
            assert!(self.cursor.goto_parent());
        }
    }
}
