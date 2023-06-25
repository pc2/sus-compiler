

use crate::ast::Span;
use crate::ast::CharSpan;
use crate::ast::cvt_span_to_char_span;
use ariadne::*;

use crate::tokenizer::{TokenTypeIdx, get_token_type_name};
use crate::parser::TokenTreeNode;

pub struct ErrorInfo<T> {
    position : T,
    reason : String
}

pub struct ParsingError<T> {
    error : ErrorInfo<T>,
    infos : Vec<ErrorInfo<T>>
}

impl<'a> ParsingError<CharSpan> {
    pub fn pretty_print_error(&self, file_name : &str, file_text : &str) {
        // Generate & choose some colours for each of our elements
        let err_color = Color::Red;
        let info_color = Color::Blue;

        let mut report = Report::build(ReportKind::Error, file_name, self.error.position.file_pos.char_idx)
            .with_message(&self.error.reason)
            .with_label(
                Label::new((file_name, self.error.position.as_range()))
                    .with_message(&self.error.reason)
                    .with_color(err_color)
            );

        for info in &self.infos {
            report = report.with_label(
                Label::new((file_name, info.position.as_range()))
                    .with_message(&info.reason)
                    .with_color(info_color)
            )
        }
            /*.with_note(format!(
                "Outputs of {} expressions must coerce to the same type",
                "match".fg(out)
            ))*/
        report.finish()
        .print((file_name, Source::from(file_text)))
        .unwrap();
    }
}

pub fn cvt_token_err_info_to_str(err : ErrorInfo<Span>, token_spans : &[CharSpan]) -> ErrorInfo<CharSpan> {
    ErrorInfo{position : cvt_span_to_char_span(err.position, token_spans), reason : err.reason}
}

pub fn cvt_token_error_to_str_error(err : ParsingError<Span>, token_spans : &[CharSpan]) -> ParsingError<CharSpan> {
    let mut info_vec : Vec<ErrorInfo<CharSpan>> = Vec::new();
    info_vec.reserve(err.infos.len());

    for i in err.infos {
        info_vec.push(cvt_token_err_info_to_str(i, token_spans));
    }

    ParsingError{error : cvt_token_err_info_to_str(err.error, token_spans), infos : info_vec}
}

pub fn error_info<T>(position : T, reason : String) -> ErrorInfo<T> {
    ErrorInfo{position : position, reason : reason}
}
pub fn error_info_str<T>(position : T, reason : &str) -> ErrorInfo<T> {
    ErrorInfo{position : position, reason : reason.to_owned()}
}

pub fn error_basic<T>(position : T, reason : String) -> ParsingError<T> {
    ParsingError{error : error_info(position, reason), infos : Vec::new()}
}

pub fn error_basic_str<T>(position : T, reason : &str) -> ParsingError<T> {
    ParsingError{error : error_info(position, reason.to_owned()), infos : Vec::new()}
}

pub fn error_with_info<T>(position : T, reason : String, infos : Vec<ErrorInfo<T>>) -> ParsingError<T> {
    ParsingError{error : error_info(position, reason), infos : infos}
}

pub fn join_expected_list(expected : &[TokenTypeIdx]) -> String {
    assert!(!expected.is_empty());
    let mut result = String::new();
    for exp in expected.get(..expected.len() - 1).unwrap() {
        result += "'";
        result += get_token_type_name(*exp);
        result += "',";
    }
    if expected.len() >= 2 {
        result += " or ";
    }
    result += "'";
    result += get_token_type_name(expected[expected.len() - 1]);
    result += "'";
    result
}

pub fn error_unclosed_bracket(open_pos : usize, open_typ : TokenTypeIdx, close_before_pos : usize) -> ParsingError<Span> {
    let open_name = get_token_type_name(open_typ);
    let reason = format!("Unclosed bracket {open_name}");
    error_with_info(Span::from(open_pos), reason, vec![error_info_str(Span(close_before_pos, close_before_pos), "must be closed before this")])
}
pub fn error_unopened_bracket(close_pos : usize, close_typ : TokenTypeIdx, open_after_pos : usize) -> ParsingError<Span> {
    let close_name = get_token_type_name(close_typ);
    let reason = format!("Unopened bracket. Closing bracket {close_name} found but was not opened.");
    error_with_info(Span::from(close_pos), reason, vec![error_info_str(Span(open_after_pos, open_after_pos), "must be opened in scope after this")])
}

pub fn error_unexpected_tree_node(expected : &[TokenTypeIdx], found : Option<&TokenTreeNode>, unexpected_eof_idx : usize, context : &str) -> ParsingError<Span> {
    let expected_list_str = join_expected_list(expected);
    match found {
        None => {
            error_basic(Span::from(unexpected_eof_idx), format!("Unexpected End of Scope while parsing {context}. Expected {expected_list_str}"))
        },
        Some(TokenTreeNode::PlainToken(typ, pos)) => {
            let tok_typ_name = get_token_type_name(*typ);
            error_basic(Span::from(*pos), format!("Unexpected Token '{tok_typ_name}' while parsing {context}. Expected {expected_list_str}"))
        },
        Some(TokenTreeNode::Block(typ, _, span)) => {
            let tok_typ_name = get_token_type_name(*typ);
            error_basic(*span, format!("Unexpected Block '{tok_typ_name}' while parsing {context}. Expected {expected_list_str}"))
        }
    }
}
