
use crate::ast::Span;
use crate::errors::*;

pub struct GlobalContext {
    pub errors : Vec<ParsingError<Span>>
}

impl<'a> GlobalContext {
    // Error reporting
    pub fn error(&mut self, err : ParsingError<Span>) {
        self.errors.push(err);
    }

    // Helpers for basic errors
    pub fn error_basic<S : Into<String>>(&mut self, position : Span, reason : S) {
        self.errors.push(ParsingError{error : error_info(position, reason), infos : Vec::new()});
    }
    
    pub fn error_with_info<S : Into<String>>(&mut self, position : Span, reason : S, infos : Vec<ErrorInfo<Span>>) {
        self.errors.push(ParsingError{error : error_info(position, reason), infos : infos});
    }
}
