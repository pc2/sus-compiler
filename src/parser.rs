
use num::BigInt;

use crate::{ast::*, errors::*, file_position::{BracketSpan, FileText, Span}, flattening::FlattenedModule, instantiation::InstantiationList, linker::FileUUID, tokenizer::*, value::Value};

use std::{iter::Peekable, ops::Range, str::FromStr};
use core::slice::Iter;

#[derive(Clone)]
struct TokenContent {
    tok_idx : usize,
    text : Range<usize> // File position
}

pub enum TokenTreeNode {
    PlainToken{tok_typ : TokenTypeIdx, range : Range<usize>, tok_idx : usize}, // Has the index of the given token to the global Token array
    // Code between '{' and '}', '(' and ')', or '[' and ']' exclusive. Contains sublist of tokens, index of open, index of close bracket
    Block(TokenTypeIdx, Vec<Self>, BracketSpan), // attached span is outer span, inner span is defined as outer_span.to_inner_span_of_brackets();
}
impl TokenTreeNode {
    fn get_token_type(&self) -> TokenTypeIdx {
        match self {
            Self::PlainToken{tok_typ, range : _, tok_idx : _} => *tok_typ,
            Self::Block(typ, _content, _span) => *typ
        }
    }
    fn get_span(&self) -> Span {
        match self {
            Self::PlainToken{tok_typ: _, range : _, tok_idx} => Span::new_single_token(*tok_idx),
            Self::Block(_typ, _content, span) => span.outer_span()
        }
    }
}


fn error_unclosed_bracket(open_pos : usize, open_typ : TokenTypeIdx, close_before_pos : usize, errors : &ErrorCollector) {
    let open_name = get_token_type_name(open_typ);
    let reason = format!("Unclosed bracket {open_name}");
    let file_name = errors.file.clone();
    errors.error_with_info(Span::new_single_token(open_pos), reason, vec![error_info(Span::new_single_token(close_before_pos), file_name, "must be closed before this")])
}
fn error_unopened_bracket(close_pos : usize, close_typ : TokenTypeIdx, open_after_pos : usize, errors : &ErrorCollector) {
    let close_name = get_token_type_name(close_typ);
    let reason = format!("Unopened bracket. Closing bracket {close_name} found but was not opened.");
    let file_name = errors.file.clone();
    errors.error_with_info(Span::new_single_token(close_pos), reason, vec![error_info(Span::new_single_token(open_after_pos), file_name, "must be opened in scope after this")])
}
struct TokenHierarchyStackElem {
    open_bracket : TokenTypeIdx, 
    open_bracket_pos : usize,
    parent : Vec<TokenTreeNode>
}

pub fn to_token_hierarchy(token_types : &[TokenTypeIdx], file_text : &FileText, errors : &ErrorCollector) -> Vec<TokenTreeNode> {
    let mut cur_token_slab : Vec<TokenTreeNode> = Vec::new();
    let mut stack : Vec<TokenHierarchyStackElem> = Vec::new(); // Type of opening bracket, token position, Token Subtree

    for (tok_idx, &tok_typ) in token_types.iter().enumerate() {
        if tok_typ == TOKEN_COMMENT || tok_typ == TOKEN_INVALID { // At this stage the comments are filtered out
            continue;
        }
        match is_bracket(tok_typ) {
            IsBracket::Open => {
                stack.push(TokenHierarchyStackElem{open_bracket : tok_typ, open_bracket_pos : tok_idx, parent : cur_token_slab});
                cur_token_slab = Vec::new();
            },
            IsBracket::Close => {
                loop { // Loop for bracket stack unrolling, for correct code only runs once
                    if let Some(cur_block) = stack.pop() {
                        if closes(cur_block.open_bracket, tok_typ) { // All is well. This bracket was closed properly. Happy path!
                            let mut parent_cur_token_slab = cur_block.parent;
                            parent_cur_token_slab.push(TokenTreeNode::Block(cur_block.open_bracket, cur_token_slab, BracketSpan::from_outer(Span::new_across_tokens(cur_block.open_bracket_pos, tok_idx))));
                            cur_token_slab = parent_cur_token_slab;
                            break;
                        } else {
                            if !stack.iter().any(|prev_bracket| closes(prev_bracket.open_bracket, tok_typ)) { // Any bracket in the stack closes this?
                                error_unopened_bracket(tok_idx, tok_typ, cur_block.open_bracket_pos, errors);
                                stack.push(cur_block); // Push the previous bracket back onto bracket stack, as we disregarded erroneous closing bracket
                                break;
                            } else {
                                error_unclosed_bracket(cur_block.open_bracket_pos, token_types[cur_block.open_bracket_pos], tok_idx, errors);
                            }
                        }
                    } else {
                        // Too many close brackets
                        errors.error_basic(Span::new_single_token(tok_idx), "A close bracket had no corresponding opening bracket.");
                        break;
                    }
                }
            },
            IsBracket::NotABracket => {
                cur_token_slab.push(TokenTreeNode::PlainToken{tok_typ, range : file_text.get_token_range(tok_idx), tok_idx});
            }
        }
    }

    while let Some(unclosed) = stack.pop() {
        errors.error_basic(Span::new_single_token(unclosed.open_bracket_pos), "Bracket was not closed before EOF")
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
        if let Some(TokenTreeNode::PlainToken{tok_typ, range : _, tok_idx : _}) = self.iter.peek() {
            if *tok_typ == expected {
                return true;
            }
        }
        false
    }
    fn eat_is_plain(&mut self, expected : TokenTypeIdx) -> Option<TokenContent> {
        if let Some(TokenTreeNode::PlainToken{tok_typ, range, tok_idx}) = self.peek() {
            if *tok_typ == expected {
                self.next();
                return Some(TokenContent{tok_idx : *tok_idx, text : range.clone()});
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
    fn skip_while_one_of_plain(&mut self, while_types : &[TokenTypeIdx], mut start_idx : usize) -> usize {
        while let Some(TokenTreeNode::PlainToken{tok_typ, range:_, tok_idx}) = self.peek() {
            if !while_types.iter().any(|t| *t == *tok_typ) {
                break;
            }
            start_idx = *tok_idx;
            self.next();
        }
        start_idx
    }
}

struct ASTParserContext<'file> {
    errors : ErrorCollector,
    file_text : &'file str
}

impl<'file> ASTParserContext<'file> {
    fn error_unexpected_token(&mut self, expected : &[TokenTypeIdx], found : TokenTypeIdx, pos : usize, context : &str) {
        let expected_list_str = join_expected_list(expected);
        self.error_unexpected_token_str(&expected_list_str, found, pos, context);
    }
    
    fn error_unexpected_token_str(&mut self, expected_list_str : &str, found : TokenTypeIdx, pos : usize, context : &str) {
        let tok_typ_name = get_token_type_name(found);
        self.errors.error_basic(Span::new_single_token(pos), format!("Unexpected Token '{tok_typ_name}' while parsing {context}. Expected {expected_list_str}"));
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
            Some(TokenTreeNode::PlainToken{tok_typ, range: _, tok_idx}) => {
                self.error_unexpected_token_str(expected_list_str, *tok_typ, *tok_idx, context);
            }
            Some(TokenTreeNode::Block(typ, _, span)) => {
                let tok_typ_name = get_token_type_name(*typ);
                self.errors.error_basic(span.outer_span(), format!("Unexpected Block '{tok_typ_name}' while parsing {context}. Expected {expected_list_str}"))
            }
        }
    }


    fn eat_plain_internal(&mut self, token_stream : &mut TokenStream, expected : TokenTypeIdx, context : &str) -> Option<TokenContent> {
        assert!(is_bracket(expected) == IsBracket::NotABracket);
        
        match token_stream.next() {
            Some(TokenTreeNode::PlainToken{tok_typ, range, tok_idx}) if *tok_typ == expected => {
                Some(TokenContent{tok_idx : *tok_idx, text : range.clone()})
            },
            other => {
                self.error_unexpected_tree_node(&[expected], other, token_stream.remaining_span, context);
                None
            }
        }
    }
    fn eat_plain(&mut self, token_stream : &mut TokenStream, expected : TokenTypeIdx, context : &str) -> Option<usize> {
        Some(self.eat_plain_internal(token_stream, expected, context)?.tok_idx)
    }
    fn eat_identifier(&mut self, token_stream : &mut TokenStream, context : &str) -> Option<TokenContent> {
        self.eat_plain_internal(token_stream, TOKEN_IDENTIFIER, context)
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

    fn make_declaration(&mut self, type_expr : SpanTypeExpression, name : TokenContent, identifier_type : IdentifierType, latency_expr : Option<SpanExpression>) -> SignalDeclaration {
        SignalDeclaration{typ : type_expr, name_span : Span::new_single_token(name.tok_idx), identifier_type, latency_expr}
    }

    fn parse_identifier(&mut self, start_token_idx : usize, token_stream : &mut TokenStream) -> Span {
        let span_end = token_stream.skip_while_one_of_plain(&[TOKEN_IDENTIFIER, kw("::")], start_token_idx);
        Span::new_across_tokens(start_token_idx, span_end)
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
            Some(TokenTreeNode::PlainToken{tok_typ, range: _, tok_idx}) if is_unary_operator(*tok_typ) => {
                let found_expr = self.parse_unit_expression(token_stream)?;
                let new_span = Span::new_extend_before(*tok_idx, found_expr.1);
                return Some((Expression::UnaryOp(Box::new((Operator{op_typ : *tok_typ}, *tok_idx, found_expr))), new_span));
            },
            Some(TokenTreeNode::PlainToken{tok_typ, range:_, tok_idx}) if *tok_typ == TOKEN_IDENTIFIER || *tok_typ == kw("::") => {
                let span = self.parse_identifier(*tok_idx, token_stream);
                (Expression::Named(Identifier{span}), span)
            },
            Some(TokenTreeNode::PlainToken{tok_typ, range, tok_idx}) if *tok_typ == TOKEN_NUMBER => {
                let value = &self.file_text[range.clone()];
                (Expression::Constant(Value::Integer(BigInt::from_str(value).unwrap())), Span::new_single_token(*tok_idx))
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
        let mut stack : Vec<(SpanExpression, TokenTypeIdx, usize)> = Vec::new();
        loop {
            let mut grabbed_symbol = self.parse_unit_expression(token_stream)?;
            match token_stream.peek() {
                Some(TokenTreeNode::PlainToken{tok_typ, range: _, tok_idx}) if is_operator(*tok_typ) => {
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
                    stack.push((grabbed_symbol, *tok_typ, *tok_idx));
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
        let mut cur_type = (TypeExpression::Named, Span::new_single_token(first_token.tok_idx)); // TODO add more type info
        while let Some((content, block_span)) = token_stream.eat_is_block(kw("[")) {
            let mut array_index_token_stream = TokenStream::new(content, block_span.inner_span());
            let expr = self.parse_expression(&mut array_index_token_stream)?;
            self.token_stream_should_be_finished(array_index_token_stream, "type array index");
            cur_type = (TypeExpression::Array(Box::new((cur_type, expr))), Span::new_extend_before(first_token.tok_idx, block_span.outer_span()));
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
            (Some(gen), Some(st)) => {
                let gen_kw_info = error_info(Span::new_single_token(gen.tok_idx), self.errors.file, "Also declared as Generative here");
                self.errors.error_with_info(Span::new_single_token(st.tok_idx), "Cannot declare local as both State and Generative", vec![gen_kw_info]);
                IdentifierType::Generative // Fallback, statement is formatted reasonbly well enough
            }
        };
        
        let typ = self.try_parse_type(token_stream)?;
        let name_token = token_stream.eat_is_plain(TOKEN_IDENTIFIER)?;
        let latency_expr = self.parse_optional_latency_decl(token_stream);
        let local_idx = self.make_declaration(typ, name_token.clone(), identifier_type, latency_expr);
        Some((local_idx, Span::new_single_token(name_token.tok_idx)))
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
        while let Some(tok) = token_stream.eat_is_plain(kw("reg")) {
            if let Some(regs_span) = &mut regs_span {
                *regs_span = Span::new_overarching(*regs_span, Span::new_single_token(tok.tok_idx))
            } else {
                regs_span = Some(Span::new_single_token(tok.tok_idx));
            }
            num_regs += 1;
        }
        if num_regs >= 1 {
            return AssignableExpressionModifiers::LatencyAdding{num_regs, regs_span : regs_span.unwrap()};
        }
        
        // Initial value for state register
        if let Some(initial_token) = token_stream.eat_is_plain(kw("initial")) {
            return AssignableExpressionModifiers::Initial{initial_token : initial_token.tok_idx}
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
                Some(TokenTreeNode::PlainToken{tok_typ, range:_, tok_idx:_}) if *tok_typ == kw(",") => {
                    continue; // parse next declaration
                }
                Some(TokenTreeNode::PlainToken{tok_typ, range:_, tok_idx : eq_sign_pos}) if *tok_typ == kw("=") => {
                    // T a, T b = x(y);
                    return Some(self.parse_statement_handle_assignment(left_expressions, *eq_sign_pos, token_stream));
                }
                Some(TokenTreeNode::PlainToken{tok_typ, range:_, tok_idx:_}) if *tok_typ == kw(";") => {
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

    fn parse_statement_handle_assignment(&mut self, left_expressions: Vec<AssignableExpressionWithModifiers>, assign_pos: usize, token_stream: &mut TokenStream<'_>) -> SpanStatement {
        let mut span = if let Some(first_left_expr) = left_expressions.first() {
            first_left_expr.span
        } else {
            self.error_unexpected_token(&[TOKEN_IDENTIFIER], kw("="), assign_pos, "statement");
            Span::new_single_token(assign_pos)
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
    fn parse_if_statement(&mut self, token_stream : &mut TokenStream, if_token : TokenContent) -> Option<SpanStatement> {
        let condition = self.parse_expression(token_stream)?;

        let (then_block, then_block_span) = self.eat_block(token_stream, kw("{"), "Then block of if statement")?;
        let then_content = self.parse_code_block(then_block, then_block_span);
        
        let (else_content, span) = if let Some(_else_tok) = token_stream.eat_is_plain(kw("else")) {
            if let Some(continuation_if) = token_stream.eat_is_plain(kw("if")) {
                let cont_if_pos = continuation_if.tok_idx;
                if let Some(stmt) = self.parse_if_statement(token_stream, continuation_if) {
                    let span = stmt.1;
                    (Some(CodeBlock{statements : vec![stmt]}), Span::new_extend_before(if_token.tok_idx, span))
                } else {
                    (Some(CodeBlock{statements : Vec::new()}), Span::new_single_token(cont_if_pos))
                }
            } else {
                let (else_block, else_block_span) = self.eat_block(token_stream, kw("{"), "Else block of if statement")?;
                (Some(self.parse_code_block(else_block, else_block_span)), Span::new_extend_before(if_token.tok_idx, else_block_span.outer_span()))
            }
        } else {
            (None, Span::new_extend_before(if_token.tok_idx, then_block_span.outer_span()))
        };

        Some((Statement::If{condition, then: then_content, els: else_content }, span))
    }
    fn parse_for_loop(&mut self, token_stream : &mut TokenStream, for_token : TokenContent) -> Option<SpanStatement> {
        let var = self.parse_signal_declaration(token_stream, IdentifierType::Generative)?;

        let _in_kw = self.eat_plain(token_stream, kw("in"), "for loop")?;

        let range = self.parse_range(token_stream)?;

        let (for_block, for_block_span) = self.eat_block(token_stream, kw("{"), "Block of for loop")?;
        let code = self.parse_code_block(for_block, for_block_span);

        Some((Statement::For{var, range, code}, Span::new_extend_before(for_token.tok_idx, for_block_span.outer_span())))
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

    fn parse_module(&mut self, token_stream : &mut TokenStream, declaration_start_idx : usize) -> Option<Module> {
        // done by caller 
        // self.eat_plain(token_stream, kw("module"));
        let name = self.eat_identifier(token_stream, "module")?;
        self.eat_plain(token_stream, kw(":"), "module")?;

        let interface = self.parse_interface(token_stream);

        let (block_tokens, block_span) = self.eat_block(token_stream, kw("{"), "module")?;

        let code = self.parse_code_block(block_tokens, block_span);

        let span = Span::new_extend_before(declaration_start_idx, block_span.outer_span());
        
        let link_info = LinkInfo{
            file : self.errors.file,
            name : self.file_text[name.text].into(),
            name_span : Span::new_single_token(name.tok_idx),
            span
        };

        Some(Module{interface, code, link_info, flattened : FlattenedModule::empty(self.errors.file), instantiations : InstantiationList::new()})
    }

    fn parse_ast(mut self, outer_token_iter : &mut TokenStream) -> ASTRoot {
        let mut modules : Vec<Module> = Vec::new();

        while let Some(t) = outer_token_iter.next() {
            match t {
                TokenTreeNode::PlainToken{tok_typ, range:_, tok_idx} if *tok_typ == kw("module") => {
                    if let Some(module) = self.parse_module(outer_token_iter, *tok_idx) {
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



pub fn parse<'nums, 'g, 'file>(token_hierarchy : &Vec<TokenTreeNode>, file_text : &'file str, whole_file_span : Span, errors : ErrorCollector) -> ASTRoot {
    let context = ASTParserContext{errors, file_text};
    let mut token_stream = TokenStream::new(&token_hierarchy, whole_file_span);
    context.parse_ast(&mut token_stream)
}



pub struct FullParseResult {
    pub file_text : FileText,
    pub tokens : Vec<TokenTypeIdx>,
    pub token_hierarchy : Vec<TokenTreeNode>,
    pub ast : ASTRoot
}

pub fn perform_full_semantic_parse<'txt>(file_text : String, file : FileUUID) -> FullParseResult {
    let errors = ErrorCollector::new(file);

    let (tokens, token_boundaries) = tokenize(&file_text, &errors);

    let file_text = FileText::new(file_text, token_boundaries);
    
    let token_hierarchy = to_token_hierarchy(&tokens, &file_text, &errors);

    let ast = parse(&token_hierarchy, &file_text.file_text, file_text.whole_file_span(), errors);

    FullParseResult{
        file_text,
        tokens,
        token_hierarchy,
        ast,
    }
}
