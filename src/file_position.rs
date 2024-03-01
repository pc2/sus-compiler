use std::{fmt::Display, ops::{Index, Range}};

// Span is defined as byte-byte idx. Start inclusive, end exclusive
#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
pub struct Span(usize, usize);

impl Span {
    /// Only really used for having a span with the maximum size. 
    pub const MAX_POSSIBLE_SPAN : Span = Span(0, usize::MAX);

    pub fn new_from_byte_range(rng : Range<usize>) -> Span {
        assert!(rng.end >= rng.start);
        Span(rng.start, rng.end)
    }
    pub fn into_range(&self) -> Range<usize> {
        self.0..self.1
    }
    pub fn new_overarching(left : Span, right : Span) -> Span {
        assert!(left.0 <= right.0);
        assert!(left.1 <= right.1);
        Span(left.0, right.1)
    }
    pub fn contains_pos(&self, pos : usize) -> bool {
        pos >= self.0 && pos <= self.1
    }
    // Not really a useful quantity. Should only be used comparatively, find which is the nested-most span
    pub fn size(&self) -> usize {
        self.1 - self.0
    }
    pub fn difference_left(outer : Span, inner : Span) -> Span {
        assert!(outer.0 <= inner.0);
        assert!(outer.1 >= inner.1);

        Span(outer.0, inner.0)
    }
    pub fn difference_right(outer : Span, inner : Span) -> Span {
        assert!(outer.0 <= inner.0);
        assert!(outer.1 >= inner.1);

        Span(inner.1, outer.1)
    }
    pub fn into_single_char_span(self) -> SingleCharSpan {
        assert!(self.1 == self.0+1);
        SingleCharSpan{char_idx: self.0}
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for Span {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}..{}", self.0, self.1))
    }
}

#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
pub struct BracketSpan(Span);

impl BracketSpan {
    pub fn from_outer(span : Span) -> Self {Self(span)}
    pub fn inner_span(&self) -> Span {
        Span(self.0.0 + 1, self.0.1 - 1)
    }
    pub fn outer_span(&self) -> Span {
        self.0
    }
    pub fn open_bracket(&self) -> SingleCharSpan {
        SingleCharSpan{char_idx : self.0.0}
    }
    pub fn close_bracket(&self) -> SingleCharSpan {
        SingleCharSpan{char_idx : self.0.1 - 1}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SingleCharSpan {
    pub char_idx : usize
}

impl Into<Span> for SingleCharSpan {
    fn into(self) -> Span {
        Span(self.char_idx, self.char_idx+1)
    }
}

impl Into<Span> for &SingleCharSpan {
    fn into(self) -> Span {
        Span(self.char_idx, self.char_idx+1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LineCol {
    pub line : usize,
    pub col : usize
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
    pub file_text : String,
    lines_start_at : Vec<usize>
}

impl FileText {
    pub fn new(file_text : String) -> Self {
        let mut lines_start_at = Vec::new();

        lines_start_at.push(0);
        for (idx, c) in file_text.char_indices() {
            if c == '\n' {
                lines_start_at.push(idx + 1);
            }
        }
        //lines_start_at.push(file_text.len());

        FileText{file_text, lines_start_at}
    }
    
    pub fn byte_to_linecol(&self, byte_pos : usize) -> LineCol {
        assert!(byte_pos < self.file_text.len());
        let line = match self.lines_start_at.binary_search(&byte_pos) {
            Ok(exact_newline) => exact_newline,
            Err(before_newline) => before_newline - 1
        };
        let text_before = &self.file_text[self.lines_start_at[line]..byte_pos];

        LineCol{line, col : text_before.chars().count()}
    }
    pub fn linecol_to_byte(&self, linecol : LineCol) -> usize {
        let line_start = self.lines_start_at[linecol.line];
        let line_text = &self.file_text[line_start..self.lines_start_at[linecol.line+1]];

        let mut cols_left = linecol.col;
        for (byte, _) in line_text.char_indices() {
            if cols_left == 0 {
                return line_start + byte;
            }
            cols_left -= 1;
        }
        unreachable!()
    }
    pub fn get_span_linecol_range(&self, span : Span) -> Range<LineCol> {
        self.byte_to_linecol(span.0)..self.byte_to_linecol(span.1)
    }

    pub fn whole_file_span(&self) -> Span {
        Span(0, self.file_text.len() - 1)
    }

    pub fn is_span_valid(&self, span : Span) -> bool {
        span.1 <= self.file_text.len()
    }
}

impl Index<Span> for FileText {
    type Output = str;

    fn index(&self, index: Span) -> &str {
        &self.file_text[index.into_range()]
    }
}
