
use num_bigint::BigUint;

use crate::{tokenizer::*, errors::*, ast::*, linker::{FileUUID, ValueUUID}};

use std::{iter::Peekable, str::FromStr, ops::Range};
use core::slice::Iter;

use std::mem::replace;

#[derive(Clone)]
struct TokenContent {
    position : usize,
    text : Range<usize> // File position
}

pub enum TokenTreeNode {
    PlainToken(Token, usize), // Has the index of the given token to the global Token array
    // Code between '{' and '}', '(' and ')', or '[' and ']' exclusive. Contains sublist of tokens, index of open, index of close bracket
    Block(TokenTypeIdx, Vec<Self>, Span), 
}
impl TokenTreeNode {
    fn get_token_type(&self) -> TokenTypeIdx {
        match self {
            Self::PlainToken(tok, _pos) => tok.get_type(),
            Self::Block(typ, _content, _span) => *typ
        }
    }
    fn get_span(&self) -> Span {
        match self {
            Self::PlainToken(_typ, pos) => Span::from(*pos),
            Self::Block(_typ, _content, span) => *span
        }
    }
}


fn error_unclosed_bracket(open_pos : usize, open_typ : TokenTypeIdx, close_before_pos : usize, errors : &mut ErrorCollector) {
    let open_name = get_token_type_name(open_typ);
    let reason = format!("Unclosed bracket {open_name}");
    let file_name = errors.file.clone();
    errors.error_with_info(Span::from(open_pos), reason, vec![error_info(Span(close_before_pos, close_before_pos), file_name, "must be closed before this")])
}
fn error_unopened_bracket(close_pos : usize, close_typ : TokenTypeIdx, open_after_pos : usize, errors : &mut ErrorCollector) {
    let close_name = get_token_type_name(close_typ);
    let reason = format!("Unopened bracket. Closing bracket {close_name} found but was not opened.");
    let file_name = errors.file.clone();
    errors.error_with_info(Span::from(close_pos), reason, vec![error_info(Span(open_after_pos, open_after_pos), file_name, "must be opened in scope after this")])
}
struct TokenHierarchyStackElem {
    open_bracket : TokenTypeIdx, 
    open_bracket_pos : usize,
    parent : Vec<TokenTreeNode>
}

pub fn to_token_hierarchy(tokens : &[Token], errors : &mut ErrorCollector) -> Vec<TokenTreeNode> {
    let mut cur_token_slab : Vec<TokenTreeNode> = Vec::new();
    let mut stack : Vec<TokenHierarchyStackElem> = Vec::new(); // Type of opening bracket, token position, Token Subtree

    for (idx, &tok) in tokens.iter().enumerate() {
        let tok_typ = tok.get_type();
        if tok_typ == TOKEN_COMMENT || tok_typ == TOKEN_INVALID { // At this stage the comments are filtered out
            continue;
        }
        match is_bracket(tok_typ) {
            IsBracket::Open => {
                stack.push(TokenHierarchyStackElem{open_bracket : tok_typ, open_bracket_pos : idx, parent : cur_token_slab});
                cur_token_slab = Vec::new();
            },
            IsBracket::Close => {
                loop { // Loop for bracket stack unrolling, for correct code only runs once
                    if let Some(cur_block) = stack.pop() {
                        if closes(cur_block.open_bracket, tok_typ) { // All is well. This bracket was closed properly. Happy path!
                            let mut parent_cur_token_slab = cur_block.parent;
                            parent_cur_token_slab.push(TokenTreeNode::Block(cur_block.open_bracket, cur_token_slab, Span(cur_block.open_bracket_pos, idx)));
                            cur_token_slab = parent_cur_token_slab;
                            break;
                        } else {
                            if !stack.iter().any(|prev_bracket| closes(prev_bracket.open_bracket, tok_typ)) { // Any bracket in the stack closes this?
                                error_unopened_bracket(idx, tok_typ, cur_block.open_bracket_pos, errors);
                                stack.push(cur_block); // Push the previous bracket back onto bracket stack, as we disregarded erroneous closing bracket
                                break;
                            } else {
                                error_unclosed_bracket(cur_block.open_bracket_pos, tokens[cur_block.open_bracket_pos].get_type(), idx, errors);
                            }
                        }
                    } else {
                        // Too many close brackets
                        errors.error_basic(Span::from(idx), "A close bracket had no corresponding opening bracket.");
                        break;
                    }
                }
            },
            IsBracket::NotABracket => {
                cur_token_slab.push(TokenTreeNode::PlainToken(tok, idx));
            }
        }
    }

    while let Some(unclosed) = stack.pop() {
        errors.error_basic(Span::from(unclosed.open_bracket_pos), "Bracket was not closed before EOF")
    }

    cur_token_slab
}

struct LocalVariableContext<'prev, 'file> {
    locals : Vec<(&'file str, usize)>,
    prev : Option<&'prev LocalVariableContext<'prev, 'file>>
}

impl<'prev, 'file> LocalVariableContext<'prev, 'file> {
    pub fn get_declaration_for(&self, name : &'file str) -> Option<usize> {
        for (decl_name, unique_id) in &self.locals {
            if *decl_name == name {
                return Some(*unique_id);
            }
        }
        if let Some(p) = self.prev {
            p.get_declaration_for(name)
        } else {
            None
        }
    }
    pub fn add_declaration(&mut self, new_local_name : &'file str, new_local_unique_id : usize) -> Result<(), usize> { // Returns conflicting signal declaration
        for (existing_local_name, existing_local_id) in &self.locals {
            if new_local_name == *existing_local_name {
                return Err(*existing_local_id)
            }
        }
        self.locals.push((new_local_name, new_local_unique_id));
        Ok(())
    }
    pub fn new_initial() -> Self {
        Self{locals : Vec::new(), prev : None}
    }
    pub fn new_extend(prev : &'prev Self) -> Self {
        Self{locals : Vec::new(), prev : Some(prev)}
    }
}

#[derive(Clone)]
struct TokenStream<'it> {
    iter : Peekable<Iter<'it, TokenTreeNode>>,
    unexpected_eof_token : usize,
    pub last_idx : usize
}

impl<'it> Iterator for TokenStream<'it> {
    type Item = &'it TokenTreeNode;

    fn next(&mut self) -> Option<&'it TokenTreeNode> {
        if let Some(found) = self.iter.next() {
            self.last_idx = found.get_span().1;
            Some(found)
        } else {
            None
        }
    }
}

impl<'it> TokenStream<'it> {
    // The given start idx should point to the first element in this block. unexpected_eof_token should point one past the last element
    fn new(list : &'it [TokenTreeNode], start_idx : usize, unexpected_eof_token : usize) -> TokenStream<'it> {
        TokenStream{iter : list.iter().peekable(), unexpected_eof_token : unexpected_eof_token, last_idx : start_idx}
    }
    fn peek(&mut self) -> Option<&'it TokenTreeNode> {
        if let Some(&found) = self.iter.peek() {
            Some(found)
        } else {
            None
        }
    }
    fn peek_is_plain(&mut self, expected : TokenTypeIdx) -> bool {
        if let Some(TokenTreeNode::PlainToken(tok, _place)) = self.iter.peek() {
            if tok.get_type() == expected {
                return true;
            }
        }
        false
    }
    fn eat_is_plain(&mut self, expected : TokenTypeIdx) -> Option<TokenContent> {
        if let Some(TokenTreeNode::PlainToken(tok, pos)) = self.peek() {
            if tok.get_type() == expected {
                self.next();
                return Some(TokenContent{position : *pos, text : tok.get_range()});
            }
        }
        None
    }
    fn eat_is_block(&mut self, expected : TokenTypeIdx) -> Option<(&Vec<TokenTreeNode>, Span)> {
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
}

struct ASTParserContext<'g, 'file> {
    errors : &'g mut ErrorCollector,
    file_text : &'file str,

    global_references : Vec<(GlobalReference, ValueUUID)>
}

struct ASTParserRollbackable {
    original_global_references_size : usize
}

impl<'g, 'file> ASTParserContext<'g, 'file> {
    fn add_global_reference(&mut self, name_span : GlobalReference) -> usize {
        let idx = self.global_references.len();
        self.global_references.push((name_span, ValueUUID::INVALID));
        idx
    }

    fn prepare_rollback(&self) -> ASTParserRollbackable {
        ASTParserRollbackable{original_global_references_size : self.global_references.len()}
    }

    fn rollback(&mut self, rollback_store : ASTParserRollbackable) {
        self.global_references.truncate(rollback_store.original_global_references_size);
    }
    
    fn error_unexpected_token(&mut self, expected : &[TokenTypeIdx], found : TokenTypeIdx, pos : usize, context : &str) {
        let expected_list_str = join_expected_list(expected);
        self.error_unexpected_token_str(&expected_list_str, found, pos, context);
    }
    
    fn error_unexpected_token_str(&mut self, expected_list_str : &str, found : TokenTypeIdx, pos : usize, context : &str) {
        let tok_typ_name = get_token_type_name(found);
        self.errors.error_basic(Span::from(pos), format!("Unexpected Token '{tok_typ_name}' while parsing {context}. Expected {expected_list_str}"));
    }
    
    fn error_unexpected_tree_node(&mut self, expected : &[TokenTypeIdx], found : Option<&TokenTreeNode>, unexpected_eof_idx : usize, context : &str) {
        let expected_list_str = join_expected_list(expected);
        self.error_unexpected_tree_node_str(&expected_list_str, found, unexpected_eof_idx, context);
    }
    
    fn error_unexpected_tree_node_str(&mut self, expected_list_str : &str, found : Option<&TokenTreeNode>, unexpected_eof_idx : usize, context : &str) {
        match found {
            None => {
                self.errors.error_basic(Span::from(unexpected_eof_idx), format!("Unexpected End of Scope while parsing {context}. Expected {expected_list_str}"))
            }
            Some(TokenTreeNode::PlainToken(tok, pos)) => {
                self.error_unexpected_token_str(expected_list_str, tok.get_type(), *pos, context);
            }
            Some(TokenTreeNode::Block(typ, _, span)) => {
                let tok_typ_name = get_token_type_name(*typ);
                self.errors.error_basic(*span, format!("Unexpected Block '{tok_typ_name}' while parsing {context}. Expected {expected_list_str}"))
            }
        }
    }


    fn eat_plain_internal(&mut self, token_stream : &mut TokenStream, expected : TokenTypeIdx, context : &str) -> Option<TokenContent> {
        assert!(is_bracket(expected) == IsBracket::NotABracket);
        
        match token_stream.next() {
            Some(TokenTreeNode::PlainToken(tok, idx)) if tok.get_type() == expected => {
                Some(TokenContent{position : *idx, text : tok.get_range()})
            },
            other => {
                self.error_unexpected_tree_node(&[expected], other, token_stream.unexpected_eof_token, context);
                None
            }
        }
    }
    fn eat_plain(&mut self, token_stream : &mut TokenStream, expected : TokenTypeIdx, context : &str) -> Option<usize> {
        Some(self.eat_plain_internal(token_stream, expected, context)?.position)
    }
    fn eat_identifier(&mut self, token_stream : &mut TokenStream, context : &str) -> Option<TokenContent> {
        self.eat_plain_internal(token_stream, TOKEN_IDENTIFIER, context)
    }
    fn eat_block<'it>(&mut self, token_stream : &mut TokenStream<'it>, expected_block_opener : TokenTypeIdx, context : &str) -> Option<(&'it [TokenTreeNode], Span)> {
        assert!(is_bracket(expected_block_opener) != IsBracket::NotABracket);
        
        let tok_elem = token_stream.next();
        
        match tok_elem {
            Some(TokenTreeNode::Block(opener_typ, contents, span)) if *opener_typ == expected_block_opener => {
                Some((contents, *span))
            },
            other => {
                self.error_unexpected_tree_node(&[expected_block_opener], other, token_stream.unexpected_eof_token, context);
                None
            }
        }
    }

    fn token_stream_should_be_finished(&mut self, mut token_stream : TokenStream, context : &str) {
        if let Some(bad_token) = token_stream.next() {
            let mut bad_tokens_span = bad_token.get_span();

            for tok in token_stream {
                bad_tokens_span.1 = tok.get_span().1;
            }

            self.errors.error_basic(bad_tokens_span, format!("More tokens found than expected while parsing {context}"))
        }
    }

    fn add_declaration(&mut self, type_expr : SpanTypeExpression, name : TokenContent, identifier_type : IdentifierType, declarations : &mut Vec<SignalDeclaration>, scope : &mut LocalVariableContext<'_, 'file>) -> usize {
        let span = Span(type_expr.1.0, name.position);
        let decl = SignalDeclaration{typ : type_expr, span, name : name.text.clone(), identifier_type};
        let decl_id = declarations.len();
        declarations.push(decl);
        if let Err(conflict) = scope.add_declaration(&self.file_text[name.text.clone()], decl_id) {
            self.errors.error_with_info(span, format!("This name was already declared previously"), vec![
                error_info(declarations[conflict].span, self.errors.file.clone(), "Previous declaration")
            ]);
        }
        decl_id
    }

    // For expression 
    fn parse_unit_expression(&mut self, token_stream : &mut TokenStream, scope : &LocalVariableContext) -> Option<SpanExpression> {
        let mut base_expr : (Expression, Span) = match token_stream.next() {
            Some(TokenTreeNode::PlainToken(tok, pos)) if is_unary_operator(tok.get_type()) => {
                let found_expr = self.parse_unit_expression(token_stream, scope)?;
                let new_span = Span(*pos, found_expr.1.1);
                return Some((Expression::UnaryOp(Box::new((Operator{op_typ : tok.get_type()}, *pos, found_expr))), new_span));
            },
            Some(TokenTreeNode::PlainToken(tok, pos)) if tok.get_type() == TOKEN_IDENTIFIER => {
                let ident_ref = if let Some(local_idx) = scope.get_declaration_for(&self.file_text[tok.get_range()]) {
                    LocalOrGlobal::Local(local_idx)
                } else {
                    // todo namespacing and shit
                    let global_ident = vec![*pos];
                    LocalOrGlobal::Global(self.add_global_reference(global_ident))
                };
                (Expression::Named(ident_ref), Span::from(*pos))
            },
            Some(TokenTreeNode::PlainToken(tok, pos)) if tok.get_type() == TOKEN_NUMBER => {
                let value = &self.file_text[tok.get_range()];
                (Expression::Constant(Value::Integer(BigUint::from_str(value).unwrap())), Span::from(*pos))
            },
            Some(TokenTreeNode::Block(typ, contents, span)) if *typ == kw("(") => {
                let mut content_token_stream = TokenStream::new(contents, span.0, span.1);
                if let Some(result) = self.parse_expression(&mut content_token_stream, scope) {
                    if let Some(erroneous_found_token) = content_token_stream.peek() {
                        // The expression should cover the whole brackets! 
                        let infos = vec![
                            error_info(*span, self.errors.file.clone(), "Expression should have ended with this scope"),
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
                self.error_unexpected_tree_node(&[TOKEN_IDENTIFIER, TOKEN_NUMBER, kw("(")], other, token_stream.unexpected_eof_token, "unit expression");
                return None;
            }
        };
        while let Some(TokenTreeNode::Block(typ, content, bracket_span)) = token_stream.peek() {
            let start_at = base_expr.1.0;
            let total_span = Span(start_at, bracket_span.1);
            if *typ == kw("(") {
                let mut args : Vec<SpanExpression> = Vec::new();
                args.push(base_expr);
                let mut content_tokens_iter = TokenStream::new(content, bracket_span.0, bracket_span.1);
                while content_tokens_iter.peek().is_some() {
                    if let Some(expr) = self.parse_expression(&mut content_tokens_iter, scope) {
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
                let mut arg_token_stream = TokenStream::new(content, bracket_span.0, bracket_span.1);
                let arg = self.parse_expression(&mut arg_token_stream, scope)?;
                base_expr = (Expression::Array(Box::new((base_expr, arg))), total_span)
            } else {
                break;
            }
            token_stream.next();
        };
        Some(base_expr)
    }

    fn parse_expression(&mut self, token_stream : &mut TokenStream, scope : &LocalVariableContext) -> Option<SpanExpression> {
        // Shunting-yard algorithm with single stack
        let mut stack : Vec<(SpanExpression, TokenTypeIdx, usize)> = Vec::new();
        loop {
            let mut grabbed_symbol = self.parse_unit_expression(token_stream, scope)?;
            match token_stream.peek() {
                Some(TokenTreeNode::PlainToken(tok, op_pos)) if is_operator(tok.get_type()) => {
                    //let operator_prescedence = get_binary_operator_prescedence(*typ);
                    while let Some((left_expr, stack_op, stack_op_pos)) = stack.pop() {
                        if get_binary_operator_prescedence(stack_op) >= get_binary_operator_prescedence(tok.get_type()) {
                            grabbed_symbol = Expression::new_binop(left_expr, Operator{op_typ : stack_op}, stack_op_pos, grabbed_symbol);
                        } else {
                            stack.push((left_expr, stack_op, stack_op_pos)); // oops, shouldn't have popped it
                            break;
                        }
                    }

                    token_stream.next(); // commit operator peek
                    stack.push((grabbed_symbol, tok.get_type(), *op_pos));
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

    fn parse_signal_declaration(&mut self, token_stream : &mut TokenStream, identifier_type : IdentifierType, declarations : &mut Vec<SignalDeclaration>, scope : &mut LocalVariableContext<'_, 'file>) -> Option<()> {
        let sig_type = self.try_parse_type(token_stream, scope)?;
        let name = self.eat_identifier(token_stream, "signal declaration")?;
        self.add_declaration(sig_type, name, identifier_type, declarations, scope);
        Some(())
    }
    
    fn try_parse_type(&mut self, token_stream : &mut TokenStream, scope : &LocalVariableContext) -> Option<SpanTypeExpression> {
        let first_token = token_stream.eat_is_plain(TOKEN_IDENTIFIER)?;
        // todo namespacing and shit
        let global_ident = vec![first_token.position];
        let mut cur_type = (TypeExpression::Named(self.add_global_reference(global_ident)), Span::from(first_token.position)); // TODO add more type info
        while let Some((content, block_span)) = token_stream.eat_is_block(kw("[")) {
            let mut array_index_token_stream = TokenStream::new(content, block_span.0, block_span.1);
            let expr = self.parse_expression(&mut array_index_token_stream, scope)?;
            self.token_stream_should_be_finished(array_index_token_stream, "type array index");
            cur_type = (TypeExpression::Array(Box::new((cur_type, expr))), Span(first_token.position, block_span.1));
        }
        Some(cur_type)
    }

    fn try_parse_declaration(&mut self, token_stream : &mut TokenStream, declarations : &mut Vec<SignalDeclaration>, scope : &mut LocalVariableContext<'_, 'file>) -> Option<(usize, Span)> {
        let identifier_type = if token_stream.eat_is_plain(kw("state")).is_some() {
            IdentifierType::State
        } else {
            IdentifierType::Local
        };
        
        let typ = self.try_parse_type(token_stream, scope)?;
        let name_token = token_stream.eat_is_plain(TOKEN_IDENTIFIER)?;
        let local_idx = self.add_declaration(typ, name_token.clone(), identifier_type, declarations, scope);
        Some((local_idx, Span::from(name_token.position)))
    }

    fn parse_bundle(&mut self, token_stream : &mut TokenStream, identifier_type : IdentifierType, declarations : &mut Vec<SignalDeclaration>, scope : &mut LocalVariableContext<'_, 'file>) {
        while token_stream.peek_is_plain(TOKEN_IDENTIFIER) {
            if let Some(_) = self.parse_signal_declaration(token_stream, identifier_type, declarations, scope) {

            } else {
                // Error during parsing signal decl. Skip till "," or end of scope
                token_stream.skip_until_one_of(&[kw(","), kw("->"), kw("{"), kw(";")]);
            }
            
            if !token_stream.eat_is_plain(kw(",")).is_some() {
                break;
            }
        }
    }

    fn parse_interface(&mut self, token_stream : &mut TokenStream, declarations : &mut Vec<SignalDeclaration>, scope : &mut LocalVariableContext<'_, 'file>) {
        self.parse_bundle(token_stream, IdentifierType::Input, declarations, scope);
    
        if token_stream.eat_is_plain(kw("->")).is_some() {
            self.parse_bundle(token_stream, IdentifierType::Output, declarations, scope);
        }
    }

    fn parse_statement(&mut self, token_stream : &mut TokenStream, declarations : &mut Vec<SignalDeclaration>, scope : &mut LocalVariableContext<'_, 'file>, statements : &mut Vec<SpanStatement>) -> Option<()> {
        let start_at = if let Some(peek) = token_stream.peek() {
            peek.get_span().0
        } else {
            return None;
        };
        if let Some(token) = token_stream.eat_is_plain(kw("#")) {
            statements.push((Statement::TimelineStage(token.position), Span::from(token.position)));
            return Some(());
        }
        
        let mut left_expressions : Vec<(SpanExpression, usize)> = Vec::new();
        let mut all_decls = true;
        loop { // Loop over a number of declarations possibly
            let mut reg_count : usize = 0;
            while let Some(_tok) = token_stream.eat_is_plain(kw("reg")) {
                reg_count += 1;
            }

            let mut tok_stream_copy = token_stream.clone();
            
            let rollback_ctx = self.prepare_rollback();
            if let Some((name, span)) = self.try_parse_declaration(&mut tok_stream_copy, declarations, scope) {
                // Maybe it's a declaration?
                *token_stream = tok_stream_copy;
                left_expressions.push(((Expression::Named(LocalOrGlobal::Local(name)), span), reg_count));

            } else {
                self.rollback(rollback_ctx);
                if let Some(sp_expr) = self.parse_expression(token_stream, scope) {
                    // It's an expression instead!
                    left_expressions.push((sp_expr, reg_count));
                    all_decls = false;
                } else {
                    // Also not, error then
                    //token_stream.skip_until_one_of(&[kw(","), kw("="), kw(";")]);
                }
            }
            match token_stream.next() {
                Some(TokenTreeNode::PlainToken(tok, _pos)) if tok.get_type() == kw(",") => {
                    continue; // parse next declaration
                }
                Some(TokenTreeNode::PlainToken(tok, assign_pos)) if tok.get_type() == kw("=") => {
                    // Ends the loop
                    // T a, T b = x(y);
                    return self.parse_statement_handle_equals(left_expressions, assign_pos, token_stream, scope, statements, start_at);
                }
                Some(TokenTreeNode::PlainToken(tok, _pos)) if tok.get_type() == kw(";") => {
                    // Ends the loop
                    return self.parse_statement_handle_end(left_expressions, all_decls, statements);
                }
                None => {
                    // Ends the loop
                    return self.parse_statement_handle_end(left_expressions, all_decls, statements);
                }
                other => {
                    self.error_unexpected_tree_node(&[kw(";"), kw("="), kw(",")], other, token_stream.unexpected_eof_token, "statement");
                    return None
                }
            }
        }
    }

    fn convert_expression_to_assignable_expression(&mut self, (expr, span) : SpanExpression, num_regs : usize) -> Option<SpanAssignableExpression> {
        match expr {
            Expression::Named(n) => {
                if let LocalOrGlobal::Local(local_idx) = n {
                    Some((AssignableExpression::Named{local_idx, num_regs}, span))
                } else {
                    self.errors.error_basic(span, "Can only assign to local variables");
                    None
                }
            },
            Expression::Array(b) => {
                let (arr, idx) = *b;
                let assignable_arr = self.convert_expression_to_assignable_expression(arr, num_regs)?;
                Some((AssignableExpression::ArrayIndex(Box::new((assignable_arr, idx))), span))
            },
            Expression::Constant(_) => {self.errors.error_basic(span, "Cannot assign to constant"); None},
            Expression::UnaryOp(_) => {self.errors.error_basic(span, "Cannot assign to the result of an operator"); None},
            Expression::BinOp(_) => {self.errors.error_basic(span, "Cannot assign to the result of an operator"); None},
            Expression::FuncCall(_) => {self.errors.error_basic(span, "Cannot assign to function call"); None},
        }
    }

    fn parse_statement_handle_equals(&mut self, left_expressions: Vec<(SpanExpression, usize)>, assign_pos: &usize, token_stream: &mut TokenStream<'_>, scope: &mut LocalVariableContext<'_, 'file>, statements: &mut Vec<(Statement, Span)>, start_at: usize) -> Option<()> {
        if left_expressions.len() == 0 {
            self.error_unexpected_token(&[TOKEN_IDENTIFIER], kw("="), *assign_pos, "statement");
            None
        } else if let Some(value) = self.parse_expression(token_stream, scope) {
            let converted_left : Vec<SpanAssignableExpression> = left_expressions.into_iter().filter_map(&mut |(expr, reg_count)| self.convert_expression_to_assignable_expression(expr, reg_count)).collect();
            let end_at = value.1.1;
            statements.push((Statement::Assign(converted_left, value), Span(start_at, end_at)));
            self.eat_plain(token_stream, kw(";"), "right-hand side of expression")?;
            Some(())
        } else {
            None
            // errors reported by self.parse_expression
        }
    }

    fn parse_statement_handle_end(&mut self, left_expressions: Vec<(SpanExpression, usize)>, all_decls: bool, statements: &mut Vec<(Statement, Span)>) -> Option<()> {
        // Declarations or single expression only
        // T a;
        // myFunc(x, y);
        if left_expressions.len() == 0 {
            return None
        } else if left_expressions.len() == 1 {
            // Is a single big expression, or a single declaration
            let (expr, _reg_count) = left_expressions.into_iter().next().unwrap();
            if all_decls {
                // decls have been taken care of during try_parse_declaration step
                return Some(());
            } else {
                let expr_span = expr.1;
                statements.push((Statement::Assign(Vec::new(), expr), expr_span));
                return None;
            }
        } else {
            self.errors.error_basic(Span(left_expressions[1].0.1.0, left_expressions[left_expressions.len()-1].0.1.1), "Multiple declarations are only allowed in function call syntax: int a, int b = f(x);");
            return None;
        }
    }
    fn parse_code_block(&mut self, block_tokens : &[TokenTreeNode], span : Span, declarations : &mut Vec<SignalDeclaration>, outer_scope : &LocalVariableContext<'_, 'file>) -> Vec<SpanStatement> {
        let mut token_stream = TokenStream::new(block_tokens, span.0, span.1);

        let mut statements : Vec<SpanStatement> = Vec::new();
        
        let mut inner_scope = LocalVariableContext::new_extend(outer_scope);


        while token_stream.peek().is_some() {
            // Allow empty statements
            if token_stream.eat_is_plain(kw(";")).is_some() {
                continue;
            }
            if let Some(TokenTreeNode::Block(typ, contents, block_span)) = token_stream.peek() {
                if *typ == kw("{") {
                    statements.push((Statement::Block(self.parse_code_block(contents, *block_span, declarations, &inner_scope)), *block_span));
                    token_stream.next();
                    continue; // Can't add condition to if let, so have to do some weird control flow here
                }
            }
            
            if self.parse_statement(&mut token_stream, declarations, &mut inner_scope, &mut statements).is_none() {
                // Error recovery. Find end of statement
                token_stream.next();
                //token_stream.skip_until_one_of(&[kw(";"), kw("{")]);
            }
        }

        statements
    }

    fn parse_module(&mut self, token_stream : &mut TokenStream, declaration_start_idx : usize) -> Option<Module> {
        // done by caller 
        // self.eat_plain(token_stream, kw("module"));
        let name = self.eat_identifier(token_stream, "module")?;
        self.eat_plain(token_stream, kw(":"), "module")?;

        let mut declarations : Vec<SignalDeclaration> = Vec::new();
        let mut scope = LocalVariableContext::new_initial();
        self.parse_interface(token_stream, &mut declarations, &mut scope);

        let (block_tokens, block_span) = self.eat_block(token_stream, kw("{"), "module")?;

        let code = self.parse_code_block(block_tokens, block_span, &mut declarations, &scope);

        let span = Span(declaration_start_idx, token_stream.last_idx);

        let link_info = LinkInfo{
            file : self.errors.file,
            name_token : name.position,
            span,
            global_references : replace(&mut self.global_references, Vec::new())
        };
        Some(Module{declarations, code, link_info})
    }

    fn parse_ast(mut self, outer_token_iter : &mut TokenStream) -> ASTRoot {
        let mut modules : Vec<Module> = Vec::new();

        while let Some(t) = outer_token_iter.next() {
            match t {
                TokenTreeNode::PlainToken(tok, module_kw_pos) if tok.get_type() == kw("module") => {
                    if let Some(module) = self.parse_module(outer_token_iter, *module_kw_pos) {
                        modules.push(module);
                    }
                },
                other => {
                    self.error_unexpected_tree_node(&[kw("module")], Some(other), outer_token_iter.unexpected_eof_token, "file scope");
                }
            }
        }

        ASTRoot{modules}
    }
}



pub fn parse<'nums, 'g, 'file>(token_hierarchy : &Vec<TokenTreeNode>, file_text : &'file str, num_tokens : usize, errors : &'g mut ErrorCollector) -> ASTRoot {
    let context = ASTParserContext{errors, file_text, global_references : Vec::new()};
    let mut token_stream = TokenStream::new(&token_hierarchy, 0, num_tokens);
    context.parse_ast(&mut token_stream)
}



pub struct FullParseResult {
    pub tokens : Vec<Token>,
    pub token_hierarchy : Vec<TokenTreeNode>,
    pub ast : ASTRoot
}

pub fn perform_full_semantic_parse<'txt>(file_text : &'txt str, file : FileUUID) -> (FullParseResult, ErrorCollector) {
    let mut errors = ErrorCollector::new(file);

    let tokens = tokenize(file_text, &mut errors);

    let token_hierarchy = to_token_hierarchy(&tokens, &mut errors);

    let ast = parse(&token_hierarchy, file_text, tokens.len(), &mut errors);

    (FullParseResult{
        tokens,
        token_hierarchy,
        ast,
    }, errors)
}
