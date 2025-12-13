use std::{
    fmt::Debug,
    ops::{Index, Range, RangeBounds},
};

use crate::prelude::FileUUID;

/// [Span] is defined as byte-byte idx. Start inclusive, end exclusive
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub file: FileUUID,
}

impl Span {
    pub fn from_range(range: Range<usize>, file: FileUUID) -> Span {
        assert!(range.end >= range.start);
        Span {
            start: range.start,
            end: range.end,
            file,
        }
        .debug()
    }
    pub fn to_range(self) -> Range<usize> {
        assert!(self.end >= self.start);
        self.debug();
        Range {
            start: self.start,
            end: self.end,
        }
    }
    /// Register that we have visited this span. Eases debugging when errors occur
    pub fn debug(self) -> Span {
        crate::debug::add_debug_span(self);
        self
    }

    pub const PLACEHOLDER: Span = Span {
        start: 0,
        end: usize::MAX,
        file: FileUUID::PLACEHOLDER,
    };

    /// Only really used for having a span with the maximum size.
    pub fn make_max_possible_span(file: FileUUID) -> Span {
        Span {
            start: 0,
            end: usize::MAX,
            file,
        }
    }

    #[track_caller]
    pub fn new_overarching(left: Span, right: Span) -> Span {
        left.debug();
        right.debug();
        assert!(left.start <= right.start);
        assert!(left.end <= right.end);
        assert_eq!(left.file, right.file);
        Span {
            start: left.start,
            end: right.end,
            file: left.file,
        }
        .debug()
    }
    pub fn contains(self, other: Span) -> bool {
        self.start <= other.start && self.end >= other.end && self.file == other.file
    }
    pub fn contains_pos(&self, pos: usize) -> bool {
        self.debug();
        pos >= self.start && pos <= self.end
    }
    // Not really a useful quantity. Should only be used comparatively, find which is the nested-most span
    pub fn size(self) -> usize {
        self.debug();
        self.end - self.start
    }
    pub fn empty_span_at_front(self) -> Span {
        self.debug();
        Span {
            start: self.start,
            end: self.start,
            file: self.file,
        }
        .debug()
    }
    pub fn empty_span_at_end(self) -> Span {
        self.debug();
        Span {
            start: self.end,
            end: self.end,
            file: self.file,
        }
        .debug()
    }
    pub fn sub_span<R: RangeBounds<usize>>(self, bound: R) -> Span {
        let start = match bound.start_bound() {
            std::ops::Bound::Included(from) => self.start + from,
            std::ops::Bound::Excluded(from) => self.start + from + 1,
            std::ops::Bound::Unbounded => self.start,
        };
        let end = match bound.end_bound() {
            std::ops::Bound::Included(to) => self.start + to + 1,
            std::ops::Bound::Excluded(to) => self.start + to,
            std::ops::Bound::Unbounded => self.end,
        };
        Span {
            start,
            end,
            file: self.file,
        }
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Span {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        assert_eq!(self.file, other.file);
        self.start.cmp(&other.start)
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.debug();
        f.debug_tuple("Span")
            .field(&self.start)
            .field(&self.end)
            .field(&self.file)
            .finish()
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
    pub fn inner_span(self) -> Span {
        self.0.debug();
        Span {
            start: self.0.start + 1,
            end: self.0.end - 1,
            file: self.0.file,
        }
        .debug()
    }
    pub fn outer_span(self) -> Span {
        self.0.debug();
        self.0
    }
    pub fn open_bracket(self) -> Span {
        self.0.debug();
        Span {
            start: self.0.start,
            end: self.0.start + 1,
            file: self.0.file,
        }
        .debug()
    }
    pub fn close_bracket(self) -> Span {
        self.0.debug();
        Span {
            start: self.0.end - 1,
            end: self.0.end,
            file: self.0.file,
        }
        .debug()
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
        self.byte_to_linecol(span.start)..self.byte_to_linecol(span.end)
    }

    pub fn is_span_valid(&self, span: Span) -> bool {
        span.debug();
        span.end <= self.file_text.len()
    }

    pub fn len(&self) -> usize {
        self.file_text.len()
    }
}

impl Index<Span> for FileText {
    type Output = str;

    fn index(&self, span: Span) -> &str {
        span.debug();
        &self.file_text[span.to_range()]
    }
}
