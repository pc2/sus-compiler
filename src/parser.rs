
use crate::tokenizer::*;

use crate::tokenizer::kw;

struct HardwareModule {
    name : String
}

pub struct ASTRoot {
    modules : Vec<HardwareModule>
}

/*struct SliceIterCanGoBack<'a, T> {
    slice : &'a [T],
    idx : usize
}
impl<'a, T> SliceIterCanGoBack<'a, T> {
    fn new(slice : &'a [T]) -> SliceIterCanGoBack<'a, T> {
        SliceIterCanGoBack{slice : slice, idx : 0}
    }
    fn back(&mut self) {
        assert_ne!(self.idx, 0);
        self.idx -= 1;
    }
}
impl<'a, T> Iterator for SliceIterCanGoBack<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.idx == self.slice.len() {
            None
        } else {
            let old_idx = self.idx;
            self.idx += 1;
            Some(&self.slice[old_idx])
        }
    }
}*/


pub fn parse(tokens : &[Token], comments : &[CommentToken]) -> ASTRoot {
    let mut token_iter = tokens.iter().peekable();

    let mut modules : Vec<HardwareModule> = Vec::new();
    loop {
        match token_iter.next() {
            None => return ASTRoot{modules : modules},
            Some(tok) => {
                let t = tok.typ;
                if t == kw("module") {
                    
                }
            }
        }
    }
}
