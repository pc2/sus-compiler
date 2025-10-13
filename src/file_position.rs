use std::{
    fmt::Debug,
    ops::{Index, Range, RangeBounds},
};

use crate::prelude::FileUUID;

/// [Span] is defined as byte-byte idx. Start inclusive, end exclusive
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span(usize, usize);

impl From<Range<usize>> for Span {
    #[track_caller]
    fn from(value: Range<usize>) -> Self {
        assert!(value.end >= value.start);
        Span(value.start, value.end).debug()
    }
}

impl Span {
    /// Register that we have visited this span. Eases debugging when errors occur
    pub fn debug(&self) -> Span {
        crate::debug::add_debug_span(*self);
        *self
    }

    /// Only really used for having a span with the maximum size.
    pub const MAX_POSSIBLE_SPAN: Span = Span(0, usize::MAX);

    pub fn as_range(&self) -> Range<usize> {
        self.0..self.1
    }
    #[track_caller]
    pub fn new_overarching(left: Span, right: Span) -> Span {
        left.debug();
        right.debug();
        assert!(left.0 <= right.0);
        assert!(left.1 <= right.1);
        Span(left.0, right.1).debug()
    }
    pub fn contains_pos(&self, pos: usize) -> bool {
        self.debug();
        pos >= self.0 && pos <= self.1
    }
    // Not really a useful quantity. Should only be used comparatively, find which is the nested-most span
    pub fn size(&self) -> usize {
        self.debug();
        self.1 - self.0
    }
    pub fn empty_span_at_front(self) -> Span {
        self.debug();
        Span(self.0, self.0).debug()
    }
    pub fn empty_span_at_end(self) -> Span {
        self.debug();
        Span(self.1, self.1).debug()
    }
    pub fn sub_span<R: RangeBounds<usize>>(&self, bound: R) -> Span {
        let start = match bound.start_bound() {
            std::ops::Bound::Included(from) => self.0 + from,
            std::ops::Bound::Excluded(from) => self.0 + from + 1,
            std::ops::Bound::Unbounded => self.0,
        };
        let end = match bound.end_bound() {
            std::ops::Bound::Included(to) => self.0 + to + 1,
            std::ops::Bound::Excluded(to) => self.0 + to,
            std::ops::Bound::Unbounded => self.1,
        };
        Span(start, end)
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Span {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.debug();
        f.debug_tuple("Span").field(&self.0).field(&self.1).finish()
    }
}

/// A span for something that is between brackets. The assumption is that the brackets are 1 byte each.
///
/// This struct is provided to improve readability on using these spans
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BracketSpan(Span);

impl BracketSpan {
    pub fn from_outer(span: Span) -> Self {
        Self(span.debug())
    }
    pub fn inner_span(&self) -> Span {
        self.0.debug();
        Span(self.0.0 + 1, self.0.1 - 1).debug()
    }
    pub fn outer_span(&self) -> Span {
        self.0.debug();
        self.0
    }
    pub fn open_bracket(&self) -> Span {
        self.0.debug();
        Span(self.0.0, self.0.0 + 1).debug()
    }
    pub fn close_bracket(&self) -> Span {
        self.0.debug();
        Span(self.0.1 - 1, self.0.1).debug()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LineCol {
    pub line: usize,
    pub col: usize,
}
impl PartialOrd for LineCol {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for LineCol {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.line.cmp(&other.line).then(self.col.cmp(&other.col))
    }
}

pub type SpanFile = (Span, FileUUID);

pub struct FileText {
    pub file_text: String,
    lines_start_at: Vec<usize>,
}

impl FileText {
    pub fn new(file_text: String) -> Self {
        let mut lines_start_at = Vec::new();

        lines_start_at.push(0);
        for (idx, c) in file_text.char_indices() {
            if c == '\n' {
                lines_start_at.push(idx + 1);
            }
        }

        FileText {
            file_text,
            lines_start_at,
        }
    }
    /// Errors when byte is outside of file
    pub fn byte_to_linecol(&self, byte_pos: usize) -> LineCol {
        assert!(byte_pos <= self.file_text.len());
        let line = match self.lines_start_at.binary_search(&byte_pos) {
            Ok(exact_newline) => exact_newline,
            Err(before_newline) => before_newline - 1,
        };
        let text_before = &self.file_text[self.lines_start_at[line]..byte_pos];

        LineCol {
            line,
            col: text_before.chars().count(),
        }
    }
    /// Clamps the linecol to be within the file, so cannot error.
    pub fn linecol_to_byte_clamp(&self, linecol: LineCol) -> usize {
        let line_end = match (linecol.line + 1).cmp(&self.lines_start_at.len()) {
            std::cmp::Ordering::Less => self.lines_start_at[linecol.line + 1] - 1,
            std::cmp::Ordering::Equal => self.file_text.len(),
            std::cmp::Ordering::Greater => return self.file_text.len(),
        };
        let line_start = self.lines_start_at[linecol.line];
        let line_text = &self.file_text[line_start..line_end];

        let mut cols_left = linecol.col;
        let mut char_indices = line_text.char_indices();
        for (byte, _) in &mut char_indices {
            if cols_left == 0 {
                return line_start + byte;
            }
            cols_left -= 1;
        }
        line_end
    }
    pub fn get_span_linecol_range(&self, span: Span) -> Range<LineCol> {
        span.debug();
        self.byte_to_linecol(span.0)..self.byte_to_linecol(span.1)
    }

    pub fn is_span_valid(&self, span: Span) -> bool {
        span.debug();
        span.1 <= self.file_text.len()
    }

    pub fn len(&self) -> usize {
        self.file_text.len()
    }
}

impl Index<Span> for FileText {
    type Output = str;

    fn index(&self, index: Span) -> &str {
        index.debug();
        &self.file_text[index.as_range()]
    }
}
