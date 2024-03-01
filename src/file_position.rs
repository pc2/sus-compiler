use std::{fmt::Display, ops::{Index, Range}};

// Token span. Indices are INCLUSIVE
#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
pub struct Span(usize, usize);

impl Span {
    /// Only really used for having a span with the maximum size. 
    pub const MAX_POSSIBLE_SPAN : Span = Span(0, usize::MAX);

    pub fn new_overarching(left : Span, right : Span) -> Span {
        assert!(left.0 <= right.0);
        assert!(left.1 <= right.1);
        Span(left.0, right.1)
    }
    pub fn new_single_token(tok_idx : usize) -> Span {
        Span(tok_idx, tok_idx)
    }
    pub fn contains_token(&self, token_idx : usize) -> bool {
        token_idx >= self.0 && token_idx <= self.1
    }
    // Not really a useful quantity. Should only be used comparatively, find which is the nested-most span
    pub fn size(&self) -> usize {
        self.1 - self.0
    }
    pub fn difference_left(outer : Span, inner : Span) -> Span {
        assert!(outer.0 <= inner.0);
        assert!(outer.1 >= inner.1);

        Span(outer.0, inner.0 - 1) // temporary, because right now spans are still inclusive. 
        // Span(outer.0, inner.0)
    }
    pub fn difference_right(outer : Span, inner : Span) -> Span {
        assert!(outer.0 <= inner.0);
        assert!(outer.1 >= inner.1);

        Span(inner.1 + 1, outer.1) // temporary, because right now spans are still inclusive. 
        // Span(inner.1, outer.1)
    }
    pub fn into_single_char_span(self) -> SingleCharSpan {
        // todo assert(self.1 == self.0+1)
        SingleCharSpan{char_token: self.0}
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
        SingleCharSpan{char_token : self.0.0}
    }
    pub fn close_bracket(&self) -> SingleCharSpan {
        SingleCharSpan{char_token : self.0.1}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SingleCharSpan {
    pub char_token : usize
}

impl Into<Span> for SingleCharSpan {
    fn into(self) -> Span {
        Span(self.char_token, self.char_token)
    }
}

impl Into<Span> for &SingleCharSpan {
    fn into(self) -> Span {
        Span(self.char_token, self.char_token)
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
    // List of all boundaries. Starts with 0, in whitespace mode, and then alternatingly switch to being a token, switch to being whitespace, back and forth
    // The span of token i is given by token_boundaries[i*2+1..i*2+2]
    // Ends at the end of the file, with a final whitespace block
    token_boundaries : Vec<usize>,
    token_boundaries_as_char_lines : Vec<LineCol>,
    lines_start_at : Vec<usize>
}

impl FileText {
    pub fn new(file_text : String, token_boundaries : Vec<usize>) -> Self {
        let mut cur_position = LineCol{line: 0, col: 0};
        let mut start = 0;
        let token_boundaries_as_char_lines = token_boundaries.iter().map(|part_end| {
            for c in file_text[start..*part_end].chars() {
                if c == '\n' {
                    cur_position.line += 1;
                    cur_position.col = 0;
                } else {
                    cur_position.col += 1;
                }
            }
            start = *part_end;
            cur_position
        }).collect();

        let mut lines_start_at = Vec::new();

        lines_start_at.push(0);
        for (idx, c) in file_text.char_indices() {
            if c == '\n' {
                lines_start_at.push(idx + 1);
            }
        }
        lines_start_at.push(file_text.len());

        FileText{file_text, token_boundaries, token_boundaries_as_char_lines, lines_start_at}
    }
    
    pub fn num_tokens(&self) -> usize {
        (self.token_boundaries.len() - 2) / 2
    }
    pub fn get_span_range(&self, span : Span) -> Range<usize> {
        self.token_boundaries[span.0*2+1]..self.token_boundaries[span.1*2+2]
    }
    pub fn get_span_linecol_range(&self, span : Span) -> Range<LineCol> {
        self.token_boundaries_as_char_lines[span.0*2+1]..self.token_boundaries_as_char_lines[span.1*2+2]
    }

    pub fn byte_to_linecol(&self, byte_pos : usize) -> LineCol {
        assert!(byte_pos < self.file_text.len());
        let line = match self.lines_start_at.binary_search(&byte_pos) {
            Ok(exact_newline) => exact_newline,
            Err(after_newline) => after_newline
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

    pub fn get_token_on_or_left_of(&self, char_line : LineCol) -> usize {
        match self.token_boundaries_as_char_lines.binary_search(&char_line) {
            Ok(idx) | Err(idx) => {
                assert!(idx >= 1);
                return (idx - 1) / 2;
            }
        }
    }

    pub fn whole_file_span(&self) -> Span {
        Span(0, self.num_tokens() - 1)
    }

    pub fn is_span_valid(&self, span : Span) -> bool {
        span.1 < self.num_tokens()
    }
}

impl Index<Span> for FileText {
    type Output = str;

    fn index(&self, index: Span) -> &str {
        &self.file_text[self.get_span_range(index)]
    }
}
