use std::{fmt::Display, ops::{Index, Range}};

// Token span. Indices are INCLUSIVE
#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
pub struct Span(usize, usize);

impl Span {
    /// Only really used for having a span with the maximum size. 
    pub const MAX_POSSIBLE_SPAN : Span = Span(0, usize::MAX);

    pub fn to_range<T : Clone>(&self, tokens : &[Range<T>]) -> Range<T> {
        let min = tokens[self.0].start.clone();
        let max = tokens[self.1].end.clone();
        min..max
    }
    pub fn new_overarching(left : Span, right : Span) -> Span {
        assert!(left.0 <= right.0);
        assert!(left.1 <= right.1);
        Span(left.0, right.1)
    }
    pub fn new_single_token(tok_idx : usize) -> Span {
        Span(tok_idx, tok_idx)
    }
    pub fn new_extend_to_include_token(left : Span, tok_idx : usize) -> Span {
        Span::new_overarching(left, Span::new_single_token(tok_idx))
    }
    pub fn dont_include_last_token(self) -> Span {
        self
    }
    pub fn only_last_token(self) -> Span {
        Span(self.1, self.1)
    }
    pub fn new_extend_before(tok_idx : usize, right : Span) -> Span {
        Span::new_overarching(Span::new_single_token(tok_idx), right)
    }
    pub fn new_across_tokens(start_tok : usize, end_tok : usize) -> Span {
        assert!(start_tok <= end_tok);
        Span(start_tok, end_tok)
    }
    pub fn contains_token(&self, token_idx : usize) -> bool {
        token_idx >= self.0 && token_idx <= self.1
    }
    // Not really a useful quantity. Should only be used comparatively, find which is the nested-most span
    pub fn size(&self) -> usize {
        self.1 - self.0
    }
    #[track_caller]
    pub fn assert_is_single_token(&self) -> usize {
        assert!(self.1 == self.0, "Span is not singleton! {}..{}", self.0, self.1);
        self.0
    }
    pub fn is_single_token(&self) -> Option<usize> {
        if self.0 == self.1 {
            Some(self.0)
        } else {
            None
        }
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
}

impl IntoIterator for Span {
    type Item = usize;

    type IntoIter = <std::ops::Range<usize> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Range{start : self.0, end : self.1 + 1}.into_iter()
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CharLine {
    pub line : usize,
    pub character : usize
}
impl PartialOrd for CharLine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CharLine {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.line.cmp(&other.line).then(self.character.cmp(&other.character))
    }
}


pub struct FileText {
    pub file_text : String,
    // List of all boundaries. Starts with 0, in whitespace mode, and then alternatingly switch to being a token, switch to being whitespace, back and forth
    // The span of token i is given by token_boundaries[i*2+1..i*2+2]
    // Ends at the end of the file, with a final whitespace block
    token_boundaries : Vec<usize>,
    token_boundaries_as_char_lines : Vec<CharLine>
}

impl FileText {
    pub fn new(file_text : String, token_boundaries : Vec<usize>) -> Self {
        let mut cur_position = CharLine{line: 0, character: 0};
        let mut start = 0;
        let token_boundaries_as_char_lines = token_boundaries.iter().map(|part_end| {
            for c in file_text[start..*part_end].chars() {
                if c == '\n' {
                    cur_position.line += 1;
                    cur_position.character = 0;
                } else {
                    cur_position.character += 1;
                }
            }
            start = *part_end;
            cur_position
        }).collect();

        FileText{file_text, token_boundaries, token_boundaries_as_char_lines}
    }
    
    pub fn num_tokens(&self) -> usize {
        (self.token_boundaries.len() - 2) / 2
    }
    pub fn get_token_range(&self, token_idx : usize) -> Range<usize> {
        self.token_boundaries[token_idx*2+1]..self.token_boundaries[token_idx*2+2]
    }
    pub fn get_token_linechar_range(&self, token_idx : usize) -> Range<CharLine> {
        self.token_boundaries_as_char_lines[token_idx*2+1]..self.token_boundaries_as_char_lines[token_idx*2+2]
    }
    pub fn get_span_range(&self, span : Span) -> Range<usize> {
        self.token_boundaries[span.0*2+1]..self.token_boundaries[span.1*2+2]
    }
    pub fn get_span_linechar_range(&self, span : Span) -> Range<CharLine> {
        self.token_boundaries_as_char_lines[span.0*2+1]..self.token_boundaries_as_char_lines[span.1*2+2]
    }

    pub fn get_token_on_or_left_of(&self, char_line : CharLine) -> usize {
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
