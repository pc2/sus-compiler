
use crate::ast::{Span, CharSpan, cvt_span_to_char_span};
use crate::errors::*;

pub struct GlobalContext<'a> {
    pub errors : Vec<ParsingError<CharSpan>>,
    pub token_spans : &'a [CharSpan]
}

impl<'a> GlobalContext<'a> {
    // Error reporting
    pub fn error(&mut self, err : ParsingError<Span>) {
        let converted = cvt_token_error_to_str_error(err, &self.token_spans);
        self.errors.push(converted);
    }

    // Helpers for basic errors
    pub fn error_basic<S : Into<String>>(&mut self, position : Span, reason : S) {
        let cvt_position = cvt_span_to_char_span(position, &self.token_spans);
        self.errors.push(ParsingError{error : error_info(cvt_position, reason), infos : Vec::new()});
    }
    
    pub fn error_with_info<S : Into<String>, const N : usize>(&mut self, position : Span, reason : S, infos : [ErrorInfo<Span>; N]) {
        let cvt_position = cvt_span_to_char_span(position, &self.token_spans);
        let cvt_infos = infos.into_iter().map(|i| {
            cvt_token_err_info_to_str(i, &self.token_spans)
        }).collect();
        self.errors.push(ParsingError{error : error_info(cvt_position, reason), infos : cvt_infos});
    }
}
