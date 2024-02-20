use std::ops::Range;

use crate::tokenizer::TokenizeResult;



// Token span. Indices are INCLUSIVE
#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
pub struct Span(pub usize, pub usize);

impl Span {
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
    pub fn whole_file_span(tokens : &TokenizeResult) -> Span {
        Span(0, tokens.token_types.len())
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


