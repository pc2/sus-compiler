
use num::BigInt;
use static_init::dynamic;
use tree_sitter::{Tree, TreeCursor};

use crate::{ast::*, errors::*, file_position::{BracketSpan, FileText, SingleCharSpan, Span}, flattening::FlattenedModule, instantiation::InstantiationList, linker::FileUUID, tokenizer::*, value::Value};

use std::{iter::Peekable, num::NonZeroU16, str::FromStr};
use core::slice::Iter;

pub enum TokenTreeNode {
    PlainToken{tok_typ : TokenTypeIdx, span : Span}, // Has the index of the given token to the global Token array
    // Code between '{' and '}', '(' and ')', or '[' and ']' exclusive. Contains sublist of tokens, index of open, index of close bracket
    Block(TokenTypeIdx, Vec<Self>, BracketSpan), // attached span is outer span, inner span is defined as outer_span.to_inner_span_of_brackets();
}
impl TokenTreeNode {
    fn get_token_type(&self) -> TokenTypeIdx {
        match self {
            Self::PlainToken{tok_typ, span:_} => *tok_typ,
            Self::Block(typ, _content, _span) => *typ
        }
    }
    fn get_span(&self) -> Span {
        match self {
            Self::PlainToken{tok_typ: _, span} => *span,
            Self::Block(_typ, _content, span) => span.outer_span()
        }
    }
}


fn error_unclosed_bracket(open_pos : SingleCharSpan, open_typ : TokenTypeIdx, close_before_pos : SingleCharSpan, errors : &ErrorCollector) {
    let open_name = get_token_type_name(open_typ);
    let reason = format!("Unclosed bracket {open_name}");
    let file_name = errors.file.clone();
    errors.error_with_info(open_pos.into(), reason, vec![error_info(close_before_pos.into(), file_name, "must be closed before this")])
}
fn error_unopened_bracket(close_pos : SingleCharSpan, close_typ : TokenTypeIdx, open_after_pos : SingleCharSpan, errors : &ErrorCollector) {
    let close_name = get_token_type_name(close_typ);
    let reason = format!("Unopened bracket. Closing bracket {close_name} found but was not opened.");
    let file_name = errors.file.clone();
    errors.error_with_info(close_pos.into(), reason, vec![error_info(open_after_pos.into(), file_name, "must be opened in scope after this")])
}
struct TokenHierarchyStackElem {
    open_bracket : TokenTypeIdx, 
    open_bracket_span : SingleCharSpan,
    parent : Vec<TokenTreeNode>
}

pub fn to_token_hierarchy(token_types : &[TokenTypeIdx], token_spans : &[Span], errors : &ErrorCollector) -> Vec<TokenTreeNode> {
    let mut cur_token_slab : Vec<TokenTreeNode> = Vec::new();
    let mut stack : Vec<TokenHierarchyStackElem> = Vec::new(); // Type of opening bracket, token position, Token Subtree

    for (&tok_typ, &span) in std::iter::zip(token_types.iter(), token_spans.iter()) {
        if tok_typ == TOKEN_COMMENT { // At this stage the comments are filtered out
            continue;
        }
        match is_bracket(tok_typ) {
            IsBracket::Open => {
                let open_bracket_span = span.into_single_char_span();
                stack.push(TokenHierarchyStackElem{open_bracket : tok_typ, open_bracket_span, parent : cur_token_slab});
                cur_token_slab = Vec::new();
            },
            IsBracket::Close => {
                let close_bracket_span = span.into_single_char_span();
                loop { // Loop for bracket stack unrolling, for correct code only runs once
                    if let Some(cur_block) = stack.pop() {
                        if closes(cur_block.open_bracket, tok_typ) { // All is well. This bracket was closed properly. Happy path!
                            let mut parent_cur_token_slab = cur_block.parent;
                            parent_cur_token_slab.push(TokenTreeNode::Block(cur_block.open_bracket, cur_token_slab, BracketSpan::from_outer(Span::new_overarching(cur_block.open_bracket_span.into(), close_bracket_span.into()))));
                            cur_token_slab = parent_cur_token_slab;
                            break;
                        } else {
                            if !stack.iter().any(|prev_bracket| closes(prev_bracket.open_bracket, tok_typ)) { // Any bracket in the stack closes this?
                                error_unopened_bracket(close_bracket_span, tok_typ, cur_block.open_bracket_span, errors);
                                stack.push(cur_block); // Push the previous bracket back onto bracket stack, as we disregarded erroneous closing bracket
                                break;
                            } else {
                                error_unclosed_bracket(cur_block.open_bracket_span, cur_block.open_bracket, close_bracket_span, errors);
                            }
                        }
                    } else {
                        // Too many close brackets
                        errors.error_basic(close_bracket_span.into(), "A close bracket had no corresponding opening bracket.");
                        break;
                    }
                }
            },
            IsBracket::NotABracket => {
                cur_token_slab.push(TokenTreeNode::PlainToken{tok_typ, span});
            }
        }
    }

    while let Some(unclosed) = stack.pop() {
        errors.error_basic(unclosed.open_bracket_span.into(), "Bracket was not closed before EOF")
    }

    cur_token_slab
}

#[derive(Clone)]
struct TokenStream<'it> {
    iter : Peekable<Iter<'it, TokenTreeNode>>,
    remaining_span : Span
}

impl<'it> Iterator for TokenStream<'it> {
    type Item = &'it TokenTreeNode;

    fn next(&mut self) -> Option<&'it TokenTreeNode> {
        if let Some(found) = self.iter.next() {
            self.remaining_span = Span::difference_right(self.remaining_span, found.get_span());
            Some(found)
        } else {
            None
        }
    }
}

impl<'it> TokenStream<'it> {
    // The given start idx should point to the first element in this block. unexpected_eof_token should point one past the last element
    fn new(list : &'it [TokenTreeNode], span : Span) -> TokenStream<'it> {
        TokenStream{iter : list.iter().peekable(), remaining_span : span}
    }
    fn peek(&mut self) -> Option<&'it TokenTreeNode> {
        if let Some(&found) = self.iter.peek() {
            Some(found)
        } else {
            None
        }
    }
    fn peek_is_plain(&mut self, expected : TokenTypeIdx) -> bool {
        if let Some(TokenTreeNode::PlainToken{tok_typ, span : _}) = self.iter.peek() {
            if *tok_typ == expected {
                return true;
            }
        }
        false
    }
    fn eat_is_plain(&mut self, expected : TokenTypeIdx) -> Option<Span> {
        if let Some(TokenTreeNode::PlainToken{tok_typ, span}) = self.peek() {
            if *tok_typ == expected {
                self.next();
                return Some(*span);
            }
        }
        None
    }
    fn eat_is_block(&mut self, expected : TokenTypeIdx) -> Option<(&Vec<TokenTreeNode>, BracketSpan)> {
        if let Some(TokenTreeNode::Block(typ, content, span)) = self.peek() {
            if *typ == expected {
                self.next();
                return Some((content, *span));
            }
        }
        None
    }
    fn skip_until_one_of(&mut self, end_types : &[TokenTypeIdx]) {
        while let Some(found) = self.peek() {
            let found_tok_type = found.get_token_type();
            for et in end_types {
                if found_tok_type == *et {
                    return;
                }
            }
            self.next();
        }
    }
    fn skip_while_one_of_plain(&mut self, while_types : &[TokenTypeIdx], mut start_span : Span) -> Span {
        while let Some(TokenTreeNode::PlainToken{tok_typ, span}) = self.peek() {
            if !while_types.iter().any(|t| *t == *tok_typ) {
                break;
            }
            start_span = *span;
            self.next();
        }
        start_span
    }
}

struct ASTParserContext<'file> {
    errors : ErrorCollector,
    file_text : &'file FileText
}

impl<'file> ASTParserContext<'file> {
    fn error_unexpected_token(&mut self, expected : &[TokenTypeIdx], found : TokenTypeIdx, span : Span, context : &str) {
        let expected_list_str = join_expected_list(expected);
        self.error_unexpected_token_str(&expected_list_str, found, span, context);
    }
    
    fn error_unexpected_token_str(&mut self, expected_list_str : &str, found : TokenTypeIdx, span : Span, context : &str) {
        let tok_typ_name = get_token_type_name(found);
        self.errors.error_basic(span, format!("Unexpected Token '{tok_typ_name}' while parsing {context}. Expected {expected_list_str}"));
    }
    
    fn error_unexpected_tree_node(&mut self, expected : &[TokenTypeIdx], found : Option<&TokenTreeNode>, remaining_span : Span, context : &str) {
        let expected_list_str = join_expected_list(expected);
        self.error_unexpected_tree_node_str(&expected_list_str, found, remaining_span, context);
    }
    
    fn error_unexpected_tree_node_str(&mut self, expected_list_str : &str, found : Option<&TokenTreeNode>, remaining_span : Span, context : &str) {
        match found {
            None => {
                self.errors.error_basic(remaining_span, format!("Unexpected End of Scope while parsing {context}. Expected {expected_list_str}"))
            }
            Some(TokenTreeNode::PlainToken{tok_typ, span}) => {
                self.error_unexpected_token_str(expected_list_str, *tok_typ, *span, context);
            }
            Some(TokenTreeNode::Block(typ, _, span)) => {
                let tok_typ_name = get_token_type_name(*typ);
                self.errors.error_basic(span.outer_span(), format!("Unexpected Block '{tok_typ_name}' while parsing {context}. Expected {expected_list_str}"))
            }
        }
    }


    fn eat_plain(&mut self, token_stream : &mut TokenStream, expected : TokenTypeIdx, context : &str) -> Option<Span> {
        assert!(is_bracket(expected) == IsBracket::NotABracket);
        
        match token_stream.next() {
            Some(TokenTreeNode::PlainToken{tok_typ, span}) if *tok_typ == expected => {
                Some(*span)
            },
            other => {
                self.error_unexpected_tree_node(&[expected], other, token_stream.remaining_span, context);
                None
            }
        }
    }
    fn eat_identifier(&mut self, token_stream : &mut TokenStream, context : &str) -> Option<Span> {
        self.eat_plain(token_stream, TOKEN_IDENTIFIER, context)
    }
    fn eat_block<'it>(&mut self, token_stream : &mut TokenStream<'it>, expected_block_opener : TokenTypeIdx, context : &str) -> Option<(&'it [TokenTreeNode], BracketSpan)> {
        assert!(is_bracket(expected_block_opener) != IsBracket::NotABracket);
        
        let tok_elem = token_stream.next();
        
        match tok_elem {
            Some(TokenTreeNode::Block(opener_typ, contents, span)) if *opener_typ == expected_block_opener => {
                Some((contents, *span))
            },
            other => {
                self.error_unexpected_tree_node(&[expected_block_opener], other, token_stream.remaining_span, context);
                None
            }
        }
    }

    fn token_stream_should_be_finished(&mut self, mut token_stream : TokenStream, context : &str) {
        if let Some(bad_token) = token_stream.next() {
            let mut bad_tokens_span = bad_token.get_span();

            for tok in token_stream {
                bad_tokens_span = Span::new_overarching(bad_tokens_span, tok.get_span());
            }

            self.errors.error_basic(bad_tokens_span, format!("More tokens found than expected while parsing {context}"))
        }
    }

    fn make_declaration(&mut self, type_expr : SpanTypeExpression, name_span : Span, identifier_type : IdentifierType, latency_expr : Option<SpanExpression>) -> SignalDeclaration {
        SignalDeclaration{typ : type_expr, name_span, identifier_type, latency_expr}
    }

    fn parse_identifier(&mut self, start_token_idx : Span, token_stream : &mut TokenStream) -> Span {
        let span_end = token_stream.skip_while_one_of_plain(&[TOKEN_IDENTIFIER, kw("::")], start_token_idx);
        Span::new_overarching(start_token_idx, span_end)
        /*let (start_scope, first_identifier) : (StartScope, Box<str>) = if start_tok_typ == kw("::") {
            let identifier_token = self.eat_identifier(token_stream, "identifier path")?;
            span_end = identifier_token.tok_idx;
            (StartScope::Root, self.file_text[identifier_token.text].to_owned().into_boxed_str())
        } else if start_tok_typ == TOKEN_IDENTIFIER {
            (StartScope::Local, self.file_text[range].to_owned().into_boxed_str())
        } else {
            unreachable!()
        };
        let mut path = vec![first_identifier];
        while let Some(_) = token_stream.eat_is_plain(kw("::")) {
            let identifier_token = self.eat_identifier(token_stream, "identifier path")?;
            span_end = identifier_token.tok_idx;
            path.push(self.file_text[identifier_token.text].to_owned().into_boxed_str());
        }
        Some((Identifier{start_scope, path}, Span::new_across_tokens(start_token_idx, span_end)))*/
    }
    
    fn parse_unit_expression(&mut self, token_stream : &mut TokenStream) -> Option<SpanExpression> {
        let mut base_expr : (Expression, Span) = match token_stream.next() {
            Some(TokenTreeNode::PlainToken{tok_typ, span}) if is_unary_operator(*tok_typ) => {
                let found_expr = self.parse_unit_expression(token_stream)?;
                let new_span = Span::new_overarching(*span, found_expr.1);
                return Some((Expression::UnaryOp(Box::new((Operator{op_typ : *tok_typ}, *span, found_expr))), new_span));
            },
            Some(TokenTreeNode::PlainToken{tok_typ, span}) if *tok_typ == TOKEN_IDENTIFIER || *tok_typ == kw("::") => {
                let span = self.parse_identifier(*span, token_stream);
                (Expression::Named(Identifier{span}), span)
            },
            Some(TokenTreeNode::PlainToken{tok_typ, span}) if *tok_typ == TOKEN_NUMBER => {
                let value = &self.file_text[*span];
                (Expression::Constant(Value::Integer(BigInt::from_str(value).unwrap())), *span)
            },
            Some(TokenTreeNode::Block(typ, contents, span)) if *typ == kw("(") => {
                let mut content_token_stream = TokenStream::new(contents, span.inner_span());
                if let Some(result) = self.parse_expression(&mut content_token_stream) {
                    if let Some(erroneous_found_token) = content_token_stream.peek() {
                        // The expression should cover the whole brackets! 
                        let infos = vec![
                            error_info(span.inner_span(), self.errors.file.clone(), "Expression should have ended with this scope"),
                            error_info(result.1, self.errors.file.clone(), "But actually only stretches this far"),
                        ];
                        self.errors.error_with_info(erroneous_found_token.get_span(), "The expression should have ended at the end of the () brackets. But instead it ended here.".to_owned(), infos);
                        return None
                    } else {
                        result
                    }
                } else {
                    return None
                }
            }
            other => {
                self.error_unexpected_tree_node(&[TOKEN_IDENTIFIER, TOKEN_NUMBER, kw("(")], other, token_stream.remaining_span, "unit expression");
                return None;
            }
        };
        while let Some(TokenTreeNode::Block(typ, content, bracket_span)) = token_stream.peek() {
            let total_span = Span::new_overarching(base_expr.1, bracket_span.outer_span());
            if *typ == kw("(") {
                let mut args : Vec<SpanExpression> = Vec::new();
                args.push(base_expr);
                let mut content_tokens_iter = TokenStream::new(content, bracket_span.inner_span());
                while content_tokens_iter.peek().is_some() {
                    if let Some(expr) = self.parse_expression(&mut content_tokens_iter) {
                        args.push(expr);
                        if content_tokens_iter.peek().is_some() {
                            self.eat_plain(&mut content_tokens_iter, kw(","), if *typ == kw("[") {"array index arguments"} else {"function call arguments"});
                        } else {
                            break;
                        }
                    }
                }
                base_expr = (Expression::FuncCall(args), total_span)
            } else if *typ == kw("[") {
                let mut arg_token_stream = TokenStream::new(content, bracket_span.inner_span());
                let arg = self.parse_expression(&mut arg_token_stream)?;
                base_expr = (Expression::Array(Box::new((base_expr, arg, *bracket_span))), total_span)
            } else {
                break;
            }
            token_stream.next();
        };
        Some(base_expr)
    }

    fn parse_expression(&mut self, token_stream : &mut TokenStream) -> Option<SpanExpression> {
        // Shunting-yard algorithm with single stack
        let mut stack : Vec<(SpanExpression, TokenTypeIdx, Span)> = Vec::new();
        loop {
            let mut grabbed_symbol = self.parse_unit_expression(token_stream)?;
            match token_stream.peek() {
                Some(TokenTreeNode::PlainToken{tok_typ, span}) if is_operator(*tok_typ) => {
                    //let operator_prescedence = get_binary_operator_prescedence(*typ);
                    while let Some((left_expr, stack_op, stack_op_pos)) = stack.pop() {
                        if get_binary_operator_prescedence(stack_op) >= get_binary_operator_prescedence(*tok_typ) {
                            grabbed_symbol = Expression::new_binop(left_expr, Operator{op_typ : stack_op}, stack_op_pos, grabbed_symbol);
                        } else {
                            stack.push((left_expr, stack_op, stack_op_pos)); // oops, shouldn't have popped it
                            break;
                        }
                    }

                    token_stream.next(); // commit operator peek
                    stack.push((grabbed_symbol, *tok_typ, *span));
                },
                _other => {
                    while let Some((left_expr, stack_op, stack_op_pos)) = stack.pop() {
                        grabbed_symbol = Expression::new_binop(left_expr, Operator{op_typ : stack_op}, stack_op_pos, grabbed_symbol);
                    }
                    return Some(grabbed_symbol);
                }
            }
        }
    }

    fn parse_signal_declaration(&mut self, token_stream : &mut TokenStream, identifier_type : IdentifierType) -> Option<SignalDeclaration> {
        let sig_type = self.try_parse_type(token_stream)?;
        let name = self.eat_identifier(token_stream, "signal declaration")?;
        let latency_expr = self.parse_optional_latency_decl(token_stream);
        Some(self.make_declaration(sig_type, name, identifier_type, latency_expr))
    }
    
    fn try_parse_type(&mut self, token_stream : &mut TokenStream) -> Option<SpanTypeExpression> {
        let first_token = token_stream.eat_is_plain(TOKEN_IDENTIFIER)?;
        // todo namespacing and shit
        let mut cur_type = (TypeExpression::Named, first_token); // TODO add more type info
        while let Some((content, block_span)) = token_stream.eat_is_block(kw("[")) {
            let mut array_index_token_stream = TokenStream::new(content, block_span.inner_span());
            let expr = self.parse_expression(&mut array_index_token_stream)?;
            self.token_stream_should_be_finished(array_index_token_stream, "type array index");
            cur_type = (TypeExpression::Array(Box::new((cur_type, expr))), Span::new_overarching(first_token, block_span.outer_span()));
        }
        Some(cur_type)
    }

    fn parse_optional_latency_decl(&mut self, token_stream : &mut TokenStream) -> Option<SpanExpression> {
        let _acc_tok = token_stream.eat_is_plain(kw("'"))?;
        self.parse_expression(token_stream)
    }

    fn try_parse_declaration(&mut self, token_stream : &mut TokenStream) -> Option<(SignalDeclaration, Span)> {
        let mut state_kw = token_stream.eat_is_plain(kw("state"));
        let generative_kw = token_stream.eat_is_plain(kw("gen"));
        if state_kw.is_none() {
            state_kw = token_stream.eat_is_plain(kw("state")); // Catch any order
        }

        let identifier_type = match (generative_kw, state_kw) {
            (Some(_), None) => {
                IdentifierType::Generative
            }
            (None, Some(_)) => {
                IdentifierType::State
            }
            (None, None) => {
                IdentifierType::Local
            }
            (Some(generative_kw), Some(state_kw)) => {
                let gen_kw_info = error_info(generative_kw, self.errors.file, "Also declared as Generative here");
                self.errors.error_with_info(state_kw, "Cannot declare local as both State and Generative", vec![gen_kw_info]);
                IdentifierType::Generative // Fallback, statement is formatted reasonbly well enough
            }
        };
        
        let typ = self.try_parse_type(token_stream)?;
        let name_token = token_stream.eat_is_plain(TOKEN_IDENTIFIER)?;
        let latency_expr = self.parse_optional_latency_decl(token_stream);
        let local_idx = self.make_declaration(typ, name_token.clone(), identifier_type, latency_expr);
        Some((local_idx, name_token))
    }

    fn parse_bundle(&mut self, token_stream : &mut TokenStream, decls : &mut Vec<SignalDeclaration>, identifier_type : IdentifierType) {
        while token_stream.peek_is_plain(TOKEN_IDENTIFIER) {
            if let Some(id) = self.parse_signal_declaration(token_stream, identifier_type) {
                decls.push(id);// Current implementation happens to order inputs then outputs, but refactorings should ensure this remains the case
            } else {
                // Error during parsing signal decl. Skip till "," or end of scope
                token_stream.skip_until_one_of(&[kw(","), kw("->"), kw("{"), kw(";")]);
            }
            
            if !token_stream.eat_is_plain(kw(",")).is_some() {
                break;
            }
        }
    }

    fn parse_interface(&mut self, token_stream : &mut TokenStream) -> ParsedInterface {
        // Current implementation happens to order inputs then outputs, but refactorings should ensure this remains the case
        
        let mut ports = Vec::new();
        self.parse_bundle(token_stream, &mut ports, IdentifierType::Input);
        
        let outputs_start = ports.len();
        if token_stream.eat_is_plain(kw("->")).is_some() {
            self.parse_bundle(token_stream, &mut ports, IdentifierType::Output);
        }

        ParsedInterface{ports, outputs_start}
    }

    fn parse_assign_modifiers(&mut self, token_stream : &mut TokenStream) -> AssignableExpressionModifiers {
        let mut num_regs = 0;
        let mut regs_span : Option<Span> = None;
        while let Some(tok_span) = token_stream.eat_is_plain(kw("reg")) {
            if let Some(regs_span) = &mut regs_span {
                *regs_span = Span::new_overarching(*regs_span, tok_span)
            } else {
                regs_span = Some(tok_span);
            }
            num_regs += 1;
        }
        if num_regs >= 1 {
            return AssignableExpressionModifiers::LatencyAdding{num_regs, regs_span : regs_span.unwrap()};
        }
        
        // Initial value for state register
        if let Some(initial_token) = token_stream.eat_is_plain(kw("initial")) {
            return AssignableExpressionModifiers::Initial{initial_token}
        }

        AssignableExpressionModifiers::NoModifiers
    }

    fn parse_statement(&mut self, token_stream : &mut TokenStream) -> Option<SpanStatement> {
        let mut left_expressions : Vec<AssignableExpressionWithModifiers> = Vec::new();
        loop { // Loop over a number of declarations possibly
            let modifiers = self.parse_assign_modifiers(token_stream);

            let mut tok_stream_copy = token_stream.clone();
            
            if let Some((decl, span)) = self.try_parse_declaration(&mut tok_stream_copy) {
                // Maybe it's a declaration?
                *token_stream = tok_stream_copy;
                left_expressions.push(AssignableExpressionWithModifiers{expr : LeftExpression::Declaration(decl), span, modifiers});
            } else if let Some((expr, span)) = self.parse_expression(token_stream) {
                // It's an expression instead!
                left_expressions.push(AssignableExpressionWithModifiers{expr: LeftExpression::Assignable(expr), span, modifiers});
            } else {
                // Also not, error then
                //token_stream.skip_until_one_of(&[kw(","), kw("="), kw(";")]);
            }
            match token_stream.next() {
                Some(TokenTreeNode::PlainToken{tok_typ, span:_}) if *tok_typ == kw(",") => {
                    continue; // parse next declaration
                }
                Some(TokenTreeNode::PlainToken{tok_typ, span}) if *tok_typ == kw("=") => {
                    // T a, T b = x(y);
                    return Some(self.parse_statement_handle_assignment(left_expressions, span.into_single_char_span(), token_stream));
                }
                Some(TokenTreeNode::PlainToken{tok_typ, span:_}) if *tok_typ == kw(";") => {
                    // T a;
                    // f(3, a);
                    return self.parse_statement_no_assignment(left_expressions);
                }
                None => {
                    return self.parse_statement_no_assignment(left_expressions);
                }
                other => {
                    self.error_unexpected_tree_node(&[kw(";"), kw("="), kw(","), kw("if"), kw("for")], other, token_stream.remaining_span, "statement");
                    return None;
                }
            }
        }
    }

    fn parse_statement_handle_assignment(&mut self, left_expressions: Vec<AssignableExpressionWithModifiers>, assign_pos: SingleCharSpan, token_stream: &mut TokenStream<'_>) -> SpanStatement {
        let mut span = if let Some(first_left_expr) = left_expressions.first() {
            first_left_expr.span
        } else {
            self.error_unexpected_token(&[TOKEN_IDENTIFIER], kw("="), assign_pos.into(), "statement");
            assign_pos.into()
        };
        
        let expr = if let Some(assign_expr) = self.parse_expression(token_stream) {
            span = Span::new_overarching(span, assign_expr.1);
            let _ = self.eat_plain(token_stream, kw(";"), "right-hand side of expression"); // Error report handled by eat_plain

            Some(assign_expr)
        } else {
            None
            // errors reported by self.parse_expression
        };

        (Statement::Assign{to : left_expressions, eq_sign_position : Some(assign_pos), expr}, span)
    }

    // Declarations or single expression only
    // T a;
    // myFunc(x, y);
    fn parse_statement_no_assignment(&mut self, left_expressions: Vec<AssignableExpressionWithModifiers>) -> Option<(Statement, Span)> {
        if left_expressions.len() == 1 {
            // Is a single big expression
            let single_assignable = left_expressions.into_iter().next().unwrap();
            if let AssignableExpressionWithModifiers{expr : LeftExpression::Assignable(expr), span, modifiers : AssignableExpressionModifiers::NoModifiers} = single_assignable {
                Some((Statement::Assign{to : Vec::new(), eq_sign_position : None, expr : Some((expr, span))}, span))
            } else {
                let span = single_assignable.span;
                Some((Statement::Assign{to: vec![single_assignable], eq_sign_position: None, expr: None}, span))
            }
        } else if let (Some(first_elem), Some(last_elem)) = (left_expressions.first(), left_expressions.last()) {
            // Several declarations. Should never be assignable but this should be caught in Flattening
            let span = Span::new_overarching(first_elem.span, last_elem.span);
            Some((Statement::Assign{to: left_expressions, eq_sign_position: None, expr: None}, span))
        } else {
            // No statement, just a single semicolon
            None
        }
    }
    fn parse_range(&mut self, token_stream : &mut TokenStream) -> Option<RangeExpression> {
        let left_expr = self.parse_expression(token_stream)?;
        self.eat_plain(token_stream, kw(".."), "range")?;
        let right_expr = self.parse_expression(token_stream)?;
        Some(RangeExpression{from : left_expr, to : right_expr})
    }
    fn parse_if_statement(&mut self, token_stream : &mut TokenStream, if_token : Span) -> Option<SpanStatement> {
        let condition = self.parse_expression(token_stream)?;

        let (then_block, then_block_span) = self.eat_block(token_stream, kw("{"), "Then block of if statement")?;
        let then_content = self.parse_code_block(then_block, then_block_span);
        
        let (else_content, span) = if let Some(_else_tok) = token_stream.eat_is_plain(kw("else")) {
            if let Some(continuation_if) = token_stream.eat_is_plain(kw("if")) {
                if let Some(stmt) = self.parse_if_statement(token_stream, continuation_if) {
                    let span = stmt.1;
                    (Some(CodeBlock{statements : vec![stmt]}), Span::new_overarching(if_token, span))
                } else {
                    (Some(CodeBlock{statements : Vec::new()}), continuation_if)
                }
            } else {
                let (else_block, else_block_span) = self.eat_block(token_stream, kw("{"), "Else block of if statement")?;
                (Some(self.parse_code_block(else_block, else_block_span)), Span::new_overarching(if_token, else_block_span.outer_span()))
            }
        } else {
            (None, Span::new_overarching(if_token, then_block_span.outer_span()))
        };

        Some((Statement::If{condition, then: then_content, els: else_content }, span))
    }
    fn parse_for_loop(&mut self, token_stream : &mut TokenStream, for_token : Span) -> Option<SpanStatement> {
        let var = self.parse_signal_declaration(token_stream, IdentifierType::Generative)?;

        let _in_kw = self.eat_plain(token_stream, kw("in"), "for loop")?;

        let range = self.parse_range(token_stream)?;

        let (for_block, for_block_span) = self.eat_block(token_stream, kw("{"), "Block of for loop")?;
        let code = self.parse_code_block(for_block, for_block_span);

        Some((Statement::For{var, range, code}, Span::new_overarching(for_token, for_block_span.outer_span())))
    }

    fn parse_code_block(&mut self, block_tokens : &[TokenTreeNode], span : BracketSpan) -> CodeBlock {
        let mut token_stream = TokenStream::new(block_tokens, span.inner_span());

        let mut code_block = CodeBlock{statements : Vec::new()};
        
        while token_stream.peek().is_some() {
            // Allow empty statements
            if token_stream.eat_is_plain(kw(";")).is_some() {
                continue;
            }
            if let Some(TokenTreeNode::Block(typ, contents, block_span)) = token_stream.peek() {
                if *typ == kw("{") {
                    code_block.statements.push((Statement::Block(self.parse_code_block(contents, *block_span)), block_span.outer_span()));
                    token_stream.next();
                    continue; // Can't add condition to if let, so have to do some weird control flow here
                }
            }

            // If statements
            if let Some(if_token) = token_stream.eat_is_plain(kw("if")) {
                let Some(if_stmt) = self.parse_if_statement(&mut token_stream, if_token) else {continue;};
                code_block.statements.push(if_stmt);
            } else 

            // For loop
            if let Some(for_token) = token_stream.eat_is_plain(kw("for")) {
                let Some(for_loop_stmt) = self.parse_for_loop(&mut token_stream, for_token) else {continue;};
                code_block.statements.push(for_loop_stmt);
            } else 
            
            if let Some(stmt) = self.parse_statement(&mut token_stream) {
                code_block.statements.push(stmt);
            } else {
                // Error recovery. Find end of statement
                token_stream.next();
            }
        }

        code_block
    }

    fn parse_module(&mut self, token_stream : &mut TokenStream, module_token : Span) -> Option<Module> {
        // done by caller 
        // self.eat_plain(token_stream, kw("module"));
        let name_span = self.eat_identifier(token_stream, "module")?;
        self.eat_plain(token_stream, kw(":"), "module")?;

        let interface = self.parse_interface(token_stream);

        let (block_tokens, block_span) = self.eat_block(token_stream, kw("{"), "module")?;

        let code = self.parse_code_block(block_tokens, block_span);

        let span = Span::new_overarching(module_token, block_span.outer_span());
        
        let link_info = LinkInfo{
            file : self.errors.file,
            name : self.file_text[name_span].into(),
            name_span,
            span
        };

        Some(Module{interface, code, link_info, flattened : FlattenedModule::empty(self.errors.new_for_same_file()), instantiations : InstantiationList::new()})
    }

    fn parse_ast(mut self, outer_token_iter : &mut TokenStream) -> ASTRoot {
        let mut modules : Vec<Module> = Vec::new();

        while let Some(t) = outer_token_iter.next() {
            match t {
                TokenTreeNode::PlainToken{tok_typ, span} if *tok_typ == kw("module") => {
                    if let Some(module) = self.parse_module(outer_token_iter, *span) {
                        modules.push(module);
                    }
                },
                other => {
                    self.error_unexpected_tree_node(&[kw("module")], Some(other), outer_token_iter.remaining_span, "file scope");
                }
            }
        }

        ASTRoot{modules, errors : self.errors}
    }
}



pub fn parse<'nums, 'g, 'file>(token_hierarchy : &Vec<TokenTreeNode>, file_text : &'file FileText, errors : ErrorCollector) -> ASTRoot {
    let context = ASTParserContext{errors, file_text};
    let mut token_stream = TokenStream::new(&token_hierarchy, file_text.whole_file_span());
    context.parse_ast(&mut token_stream)
}



pub struct FullParseResult {
    pub file_text : FileText,
    pub tokens : Vec<TokenTypeIdx>,
    pub token_hierarchy : Vec<TokenTreeNode>,
    pub ast : ASTRoot,
    pub tree : tree_sitter::Tree
}

pub fn perform_full_semantic_parse(file_text : String, file : FileUUID) -> FullParseResult {
    let errors = ErrorCollector::new(file, file_text.len());

    let (tokens, token_spans) = tokenize(&file_text, &errors);

    let file_text = FileText::new(file_text);
    
    let token_hierarchy = to_token_hierarchy(&tokens, &token_spans, &errors);

    let ast = parse(&token_hierarchy, &file_text, errors);

    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&SUS.language).unwrap();

    let tree = parser.parse(&file_text.file_text, None).unwrap();

    report_all_tree_errors(&file_text, &tree, &ast.errors);

    FullParseResult{
        tree,
        file_text,
        tokens,
        token_hierarchy,
        ast,
    }
}


fn print_current_node_indented(file_text : &FileText, cursor : &TreeCursor) -> String {
    let indent = "  ".repeat(cursor.depth() as usize);
    let n = cursor.node();
    let cursor_span = Span::from(n.byte_range());
    let node_name = if n.kind_id() == SUS.identifier_kind {format!("\"{}\"", &file_text[cursor_span])} else {n.kind().to_owned()};
    if let Some(field_name) = cursor.field_name() {
        println!("{indent} {field_name}: {node_name} [{cursor_span}]");
    } else {
        println!("{indent} {node_name} [{cursor_span}]");
    }
    node_name
}

fn report_all_tree_errors(file_text : &FileText, tree : &Tree, errors : &ErrorCollector) {
    let mut cursor = tree.walk();
    loop {
        let n = cursor.node();
        if n.is_error() || n.is_missing() {
            let span = Span::from(n.byte_range());
            let node_name = print_current_node_indented(file_text, &cursor);

            let of_name = if let Some(field) = cursor.field_name() {
                format!("in the field '{field}' of type '{node_name}'")
            } else {
                format!("in a node of type '{node_name}'")
            };
            let (error_type, parent_node) = if n.is_missing() {
                ("missing field", n.parent().unwrap().parent().unwrap()) // Weird workaround because MISSING nodes can't properly parent?
            } else {
                ("syntax error", n.parent().unwrap())
            };
            let parent_node_name = parent_node.kind();
            let parent_info = error_info(Span::from(parent_node.byte_range()), errors.file, format!("Parent node '{parent_node_name}'"));
            errors.error_with_info(span, format!("While parsing '{parent_node_name}', parser found a {error_type} {of_name}"), vec![parent_info]);
        } else {
            if cursor.goto_first_child() {
                continue;
            }
        }
        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                return;
            }
        }
    }
}


pub struct SusTreeSitterSingleton {
    pub language : tree_sitter::Language,

    pub module_kind : u16,
    pub interface_ports_kind : u16,
    pub identifier_kind : u16,
    pub number_kind : u16,
    pub global_identifier_kind : u16,
    pub array_type_kind : u16,
    pub declaration_kind : u16,
    pub declaration_list_kind : u16,
    pub latency_specifier_kind : u16,
    pub unary_op_kind : u16,
    pub binary_op_kind : u16,
    pub array_op_kind : u16,
    pub func_call_kind : u16,
    pub parenthesis_expression_kind : u16,
    pub parenthesis_expression_list_kind : u16,
    pub array_bracket_expression_kind : u16,
    pub block_kind : u16,
    pub decl_assign_statement_kind : u16,
    pub assign_left_side_kind : u16,
    pub assign_to_kind : u16,
    pub write_modifiers_kind : u16,
    pub if_statement_kind : u16,
    pub for_statement_kind : u16,

    pub gen_kw : u16,
    pub state_kw : u16,
    pub reg_kw : u16,
    pub initial_kw : u16,

    pub name_field : NonZeroU16,
    pub inputs_field : NonZeroU16,
    pub outputs_field : NonZeroU16,
    pub block_field : NonZeroU16,
    pub interface_ports_field : NonZeroU16,
    pub type_field : NonZeroU16,
    pub latency_specifier_field : NonZeroU16,
    pub declaration_modifiers_field : NonZeroU16,
    pub left_field : NonZeroU16,
    pub right_field : NonZeroU16,
    pub operator_field : NonZeroU16,
    pub arr_field : NonZeroU16,
    pub arr_idx_field : NonZeroU16,
    pub arguments_field : NonZeroU16,
    pub from_field : NonZeroU16,
    pub write_modifiers_field : NonZeroU16,
    pub to_field : NonZeroU16,
    pub expr_or_decl_field : NonZeroU16,
    pub assign_left_field : NonZeroU16,
    pub assign_value_field : NonZeroU16,
    pub condition_field : NonZeroU16,
    pub then_block_field : NonZeroU16,
    pub else_block_field : NonZeroU16,
    pub for_decl_field : NonZeroU16,

    pub content_field : NonZeroU16,
    pub item_field : NonZeroU16
}

impl SusTreeSitterSingleton {
    fn new() -> Self {
        let language = tree_sitter_sus::language();
        let node_kind = |name : &str| -> u16 {
            let v = language.id_for_node_kind(name, true);
            assert!(v != 0, "{name}");
            v
        };
        let keyword_kind = |name : &str| -> u16 {
            let v = language.id_for_node_kind(name, false);
            assert!(v != 0, "{name}");
            v
        };
        let field = |name : &str| -> NonZeroU16 {
            language.field_id_for_name(name).expect(name)
        };
        SusTreeSitterSingleton {
            module_kind : node_kind("module"),
            interface_ports_kind : node_kind("interface_ports"),
            identifier_kind : node_kind("identifier"),
            number_kind : node_kind("number"),
            global_identifier_kind : node_kind("global_identifier"),
            array_type_kind : node_kind("array_type"),
            declaration_kind : node_kind("declaration"),
            declaration_list_kind : node_kind("declaration_list"),
            latency_specifier_kind : node_kind("latency_specifier"),
            unary_op_kind : node_kind("unary_op"),
            binary_op_kind : node_kind("binary_op"),
            array_op_kind : node_kind("array_op"),
            func_call_kind : node_kind("func_call"),
            parenthesis_expression_kind : node_kind("parenthesis_expression"),
            parenthesis_expression_list_kind : node_kind("parenthesis_expression_list"),
            array_bracket_expression_kind : node_kind("array_bracket_expression"),
            block_kind : node_kind("block"),
            decl_assign_statement_kind : node_kind("decl_assign_statement"),
            assign_left_side_kind : node_kind("assign_left_side"),
            assign_to_kind : node_kind("assign_to"),
            write_modifiers_kind : node_kind("write_modifiers"),
            if_statement_kind : node_kind("if_statement"),
            for_statement_kind : node_kind("for_statement"),

            gen_kw : keyword_kind("gen"),
            state_kw : keyword_kind("state"),
            reg_kw : keyword_kind("reg"),
            initial_kw : keyword_kind("initial"),

            name_field : field("name"),
            inputs_field : field("inputs"),
            outputs_field : field("outputs"),
            block_field : field("block"),
            interface_ports_field : field("interface_ports"),
            type_field : field("type"),
            latency_specifier_field : field("latency_specifier"),
            declaration_modifiers_field : field("declaration_modifiers"),
            left_field : field("left"),
            right_field : field("right"),
            operator_field : field("operator"),
            arr_field : field("arr"),
            arr_idx_field : field("arr_idx"),
            arguments_field : field("arguments"),
            from_field : field("from"),
            to_field : field("to"),
            write_modifiers_field : field("write_modifiers"),
            expr_or_decl_field : field("expr_or_decl"),
            assign_left_field : field("assign_left"),
            assign_value_field : field("assign_value"),
            condition_field : field("condition"),
            then_block_field : field("then_block"),
            else_block_field : field("else_block"),
            for_decl_field : field("for_decl"),

            content_field : field("content"),
            item_field : field("item"),
                    
            language,
        }
    }
}

#[dynamic]
pub static SUS : SusTreeSitterSingleton = SusTreeSitterSingleton::new();

pub struct Cursor<'t> {
    cursor : TreeCursor<'t>,
    file_text : &'t FileText
}

impl<'t> Cursor<'t> {
    pub fn new_for_node(tree : &'t Tree, file_text : &'t FileText, span : Span, kind : u16) -> Self {
        let mut cursor = tree.walk();
        let _ = cursor.goto_first_child_for_byte(span.into_range().start).unwrap();
        let start_node = cursor.node();
        assert_eq!(start_node.kind_id(), kind);
        // Temprarily comment out, because old parser and new parser are slightly different
        //assert_eq!(start_node.byte_range(), span.into_range());

        Self{cursor, file_text}
    }

    pub fn kind_span(&self) -> (u16, Span) {
        let node = self.cursor.node();
        (node.kind_id(), node.byte_range().into())
    }

    pub fn kind(&self) -> u16 {
        let node = self.cursor.node();
        node.kind_id()
    }

    pub fn span(&self) -> Span {
        let node = self.cursor.node();
        node.byte_range().into()
    }

    #[track_caller]
    pub fn print_stack(&mut self) {
        let this_node_kind = self.cursor.node().kind();
        let this_node_span = self.span();
        println!("Stack:");
        loop {
            print_current_node_indented(self.file_text, &self.cursor);
            if !self.cursor.goto_parent() {break;}
        }
        println!("Current node: {this_node_kind}, {this_node_span}");
    }

    #[track_caller]
    pub fn could_not_match(&mut self) -> ! {
        self.print_stack();
        panic!();
    }

    /// The cursor advances to the next field, regardless if it is the requested field. If the found field is the requested field, the function is called. 
    /// 
    /// If no more fields are available, the cursor lands at the end of the siblings, and None is returned
    /// 
    /// If the found field is incorrect, None is returned
    pub fn optional_field<OT, F : FnOnce(&mut Self) -> OT>(&mut self, field_id : NonZeroU16, func : F) -> Option<OT> {
        loop {
            if let Some(found) = self.cursor.field_id() {
                if found == field_id {
                    let result = func(self);
                    self.cursor.goto_next_sibling();
                    return Some(result);
                } else {
                    return None; // Field found, but it's not this one. Stop here, because we've passed the possibly optional field
                }
            } else {
                if !self.cursor.goto_next_sibling() {
                    return None;
                }
            }
        }
    }

    /// Pops off the keyword if it is found
    pub fn optional_keyword(&mut self, keyword_id : u16) -> Option<Span> {
        let node = self.cursor.node();
        let kind = node.kind_id();

        if kind == keyword_id {
            let span = node.byte_range().into();
            self.cursor.goto_next_sibling();
            Some(span)
        } else {
            None
        }
    }

    /// The cursor advances to the next field and calls the given function. 
    /// 
    /// Panics if the next field doesn't exist or is not the requested field
    #[track_caller]
    pub fn field<OT, F : FnOnce(&mut Self) -> OT>(&mut self, field_id : NonZeroU16, func : F) -> OT {
        loop {
            if let Some(found) = self.cursor.field_id() {
                if found == field_id {
                    let result = func(self);
                    self.cursor.goto_next_sibling();
                    return result;
                } else {
                    self.print_stack();
                    panic!("Did not find required field '{}', found field '{}' instead!", SUS.language.field_name_for_id(field_id.into()).unwrap(), SUS.language.field_name_for_id(found.into()).unwrap());
                }
            } else {
                if !self.cursor.goto_next_sibling() {
                    self.print_stack();
                    panic!("Reached the end of child nodes without finding field '{}'", SUS.language.field_name_for_id(field_id.into()).unwrap())
                }
            }
        }
    }

    pub fn optional_field_span(&mut self, field_id : NonZeroU16) -> Option<Span> {
        self.optional_field(field_id, |cursor| cursor.span())
    }

    #[track_caller]
    pub fn field_span(&mut self, field_id : NonZeroU16) -> Span {
        self.field(field_id, |cursor| cursor.span())
    }

    #[track_caller]
    pub fn go_down<OT, F : FnOnce(&mut Self) -> OT>(&mut self, kind : u16, func : F) -> OT {
        let node = self.cursor.node();
        if node.kind_id() != kind {
            self.print_stack();
            panic!("Expected {}, Was {} instead", SUS.language.node_kind_for_id(kind).unwrap(), node.kind());
        }

        self.go_down_no_check(func)
    }

    pub fn go_down_no_check<OT, F : FnOnce(&mut Self) -> OT>(&mut self, func : F) -> OT {
        let r = self.cursor.goto_first_child();
        assert!(r);
        let result = func(self);
        let r = self.cursor.goto_parent();
        assert!(r);

        result
    }

    // Some specialized functions for SUS Language

    /// Goes down the current node, checks it's kind, and then iterates through 'item' fields. 
    #[track_caller]
    pub fn list<F : FnMut(&mut Self)>(&mut self, parent_kind : u16, mut func : F) {
        self.go_down(parent_kind, |self2| {
            loop {
                if self2.cursor.field_id() == Some(SUS.item_field) {
                    func(self2);
                }

                if !self2.cursor.goto_next_sibling() {
                    break;
                }
            }
        });
    }

    /// Goes down the current node, checks it's kind, and then iterates through 'item' fields. 
    /// 
    /// The function given should return OT, and from the valid outputs this function constructs a output list
    #[track_caller]
    pub fn collect_list<OT, F : FnMut(&mut Self) -> OT>(&mut self, parent_kind : u16, mut func : F) -> Vec<OT> {
        let mut result = Vec::new();

        self.list(parent_kind, |cursor| {
            let item = func(cursor);
            result.push(item);
        });

        result
    }

    /// Goes down the current node, checks it's kind, and then selects the 'content' field. Useful for constructs like seq('[', field('content', $.expr), ']')
    #[track_caller]
    pub fn go_down_content<OT, F : FnOnce(&mut Self) -> OT>(&mut self, parent_kind : u16, func : F) -> OT {
        self.go_down(parent_kind, |self2| {
            self2.field(SUS.content_field, |self3| func(self3))
        })
    } 
}
