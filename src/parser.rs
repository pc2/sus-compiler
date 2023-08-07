
use num_bigint::BigUint;

use crate::{tokenizer::*, errors::*, ast::*};

use std::iter::Peekable;
use core::slice::Iter;

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

struct TokenHierarchyStackElem {
    open_bracket : TokenTypeIdx, 
    open_bracket_pos : usize,
    parent : Vec<TokenTreeNode>
}

pub fn to_token_hierarchy(tokens : &[Token]) -> (Vec<TokenTreeNode>, Vec<ParsingError<Span>>) {
    let mut cur_token_slab : Vec<TokenTreeNode> = Vec::new();
    let mut stack : Vec<TokenHierarchyStackElem> = Vec::new(); // Type of opening bracket, token position, Token Subtree
    let mut errors : Vec<ParsingError<Span>> = Vec::new();

    for (idx, &tok) in tokens.iter().enumerate() {
        let tok_typ = tok.get_type();
        if is_comment(tok_typ) { // At this stage the comments are filtered out
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
                                errors.push(error_unopened_bracket(idx, tok_typ, cur_block.open_bracket_pos));
                                stack.push(cur_block); // Push the previous bracket back onto bracket stack, as we disregarded erroneous closing bracket
                                break;
                            } else {
                                errors.push(error_unclosed_bracket(cur_block.open_bracket_pos, tokens[cur_block.open_bracket_pos].get_type(), idx));
                            }
                        }
                    } else {
                        // Too many close brackets
                        errors.push(error_basic_str(Span::from(idx), "A close bracket had no corresponding opening bracket."));
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
        errors.push(error_basic_str(Span::from(unclosed.open_bracket_pos), "Bracket was not closed before EOF!"))
    }

    (cur_token_slab, errors)
}

struct LocalVariableContext<'prev> {
    locals : Vec<(TokenExtraInfo, usize)>,
    prev : Option<&'prev LocalVariableContext<'prev>>
}

impl<'prev> LocalVariableContext<'prev> {
    pub fn get_declaration_for(&self, name : TokenExtraInfo) -> IdentifierIdx {
        for (decl_name, unique_id) in &self.locals {
            if *decl_name == name {
                return IdentifierIdx::new_local(*unique_id);
            }
        }
        if let Some(p) = self.prev {
            p.get_declaration_for(name)
        } else {
            IdentifierIdx::new_global(name)
        }
    }
    pub fn add_declaration(&mut self, new_local_name : TokenExtraInfo, new_local_unique_id : usize) -> Result<(), usize> { // Returns conflicting signal declaration
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

struct TokenStream<'it> {
    iter : Peekable<Iter<'it, TokenTreeNode>>,
    unexpected_eof_token : usize,
    pub last_idx : usize
}

impl<'it> TokenStream<'it> {
    // The given start idx should point to the first element in this block. unexpected_eof_token should point one past the last element
    fn new(list : &'it [TokenTreeNode], start_idx : usize, unexpected_eof_token : usize) -> TokenStream<'it> {
        TokenStream{iter : list.iter().peekable(), unexpected_eof_token : unexpected_eof_token, last_idx : start_idx}
    }
    fn next(&mut self) -> Option<&'it TokenTreeNode> {
        if let Some(found) = self.iter.next() {
            self.last_idx = found.get_span().1;
            Some(found)
        } else {
            None
        }
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
    fn peek_is_plain_one_of(&mut self, expecteds : &[TokenTypeIdx]) -> bool {
        if let Some(TokenTreeNode::PlainToken(tok, _place)) = self.iter.peek() {
            for ex in expecteds {
                if tok.get_type() == *ex {
                    return true;
                }
            }
        }
        false
    }
    fn peek_is_block(&mut self, expected : TokenTypeIdx) -> bool {
        if let Some(TokenTreeNode::Block(typ, _content, _span)) = self.iter.peek() {
            if *typ == expected {
                return true;
            }
        }
        false
    }
    fn skip_until(&mut self, end_type : TokenTypeIdx) {
        while let Some(found) = self.peek() {
            if found.get_token_type() == end_type {
                return;
            }
            self.next();
        }
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

struct ASTParserContext<'a> {
    errors : Vec<ParsingError<Span>>,
    numbers : &'a [BigUint]
}

impl<'a> ASTParserContext<'a> {
    fn eat_plain_internal(&mut self, token_stream : &mut TokenStream, expected : TokenTypeIdx, context : &str) -> Option<(usize, TokenExtraInfo)> {
        assert!(is_bracket(expected) == IsBracket::NotABracket);
        
        let tok_elem = token_stream.next();
        
        match tok_elem {
            Some(TokenTreeNode::PlainToken(tok, idx)) if tok.get_type() == expected => {
                Some((*idx, tok.get_info()))
            },
            other => {
                self.errors.push(error_unexpected_tree_node(&[expected], other, token_stream.unexpected_eof_token, context));
                None
            }
        }
    }
    fn eat_plain(&mut self, token_stream : &mut TokenStream, expected : TokenTypeIdx, context : &str) -> Option<usize> {
        Some(self.eat_plain_internal(token_stream, expected, context)?.0)
    }
    fn eat_identifier(&mut self, token_stream : &mut TokenStream, context : &str) -> Option<IdentifierToken> {
        let (position, name_idx) = self.eat_plain_internal(token_stream, TOKEN_IDENTIFIER, context)?;
        Some(IdentifierToken{name_idx, position})
    }
    fn eat_block<'it>(&mut self, token_stream : &mut TokenStream<'it>, expected_block_opener : TokenTypeIdx, context : &str) -> Option<(&'it [TokenTreeNode], Span)> {
        assert!(is_bracket(expected_block_opener) != IsBracket::NotABracket);
        
        let tok_elem = token_stream.next();
        
        match tok_elem {
            Some(TokenTreeNode::Block(opener_typ, contents, span)) if *opener_typ == expected_block_opener => {
                Some((contents, *span))
            },
            other => {
                self.errors.push(error_unexpected_tree_node(&[expected_block_opener], other, token_stream.unexpected_eof_token, context));
                None
            }
        }
    }

    fn add_declaration(&mut self, type_expr : SpanExpression, name : IdentifierToken, identifier_type : IdentifierType, declarations : &mut Vec<SignalDeclaration>, scope : &mut LocalVariableContext) -> usize {
        let span = Span(type_expr.1.0, name.position);
        let decl = SignalDeclaration{typ : type_expr, span, name_idx : name.name_idx, identifier_type};
        let decl_id = declarations.len();
        declarations.push(decl);
        if let Err(conflict) = scope.add_declaration(name.name_idx, decl_id) {
            self.errors.push(error_with_info(span, format!("This name was already declared previously"), vec![
                error_info_str(declarations[conflict].span, "Previous declaration")
            ]));
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
            Some(TokenTreeNode::PlainToken(tok, pos)) if is_identifier(tok.get_type()) => {
                (Expression::Named(scope.get_declaration_for(tok.get_info())), Span(*pos, *pos))
            },
            Some(TokenTreeNode::PlainToken(tok, pos)) if tok.get_type() == TOKEN_NUMBER => {
                let value = tok.get_info();
                (Expression::Constant(Value::Integer(BigUint::from(value))), Span(*pos, *pos))
            },
            Some(TokenTreeNode::PlainToken(tok, pos)) if tok.get_type() == TOKEN_BIG_INTEGER => {
                let idx = tok.get_info();
                (Expression::Constant(Value::Integer(self.numbers[idx as usize].clone())), Span(*pos, *pos))
            },
            Some(TokenTreeNode::PlainToken(tok, pos)) if tok.get_type() == kw("true") => {
                (Expression::Constant(Value::Bool(true)), Span(*pos, *pos))
            },
            Some(TokenTreeNode::PlainToken(tok, pos)) if tok.get_type() == kw("false") => {
                (Expression::Constant(Value::Bool(false)), Span(*pos, *pos))
            },
            Some(TokenTreeNode::Block(typ, contents, span)) if *typ == kw("(") => {
                let mut content_token_stream = TokenStream::new(contents, span.0, span.1);
                if let Some(result) = self.parse_expression(&mut content_token_stream, scope) {
                    if let Some(erroneous_found_token) = content_token_stream.peek() {
                        // The expression should cover the whole brackets! 
                        let infos = vec![
                            error_info_str(*span, "Expression should have ended with this scope"),
                            error_info_str(result.1, "But actually only stretches this far"),
                        ];
                        self.errors.push(error_with_info(erroneous_found_token.get_span(), "The expression should have ended at the end of the () brackets. But instead it ended here.".to_owned(), infos));
                        return None
                    } else {
                        result
                    }
                } else {
                    return None
                }
            }
            other => {
                self.errors.push(error_unexpected_tree_node(&[TOKEN_IDENTIFIER, TOKEN_NUMBER, kw("(")], other, token_stream.unexpected_eof_token, "unit expression"));
                return None;
            }
        };
        while let Some(TokenTreeNode::Block(typ, content, bracket_span)) = token_stream.peek() {
            if *typ == kw("[") || *typ == kw("(") {
                let start_at = base_expr.1.0;
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
                let total_span = Span(start_at, bracket_span.1);
                base_expr = (if *typ == kw("[") {Expression::Array(args)} else {Expression::FuncCall(args)}, total_span)
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

    fn parse_signal_declaration(&mut self, token_stream : &mut TokenStream, identifier_type : IdentifierType, declarations : &mut Vec<SignalDeclaration>, scope : &mut LocalVariableContext) -> Option<()> {
        let sig_type = self.parse_expression(token_stream, scope)?;
        let name = self.eat_identifier(token_stream, "signal declaration")?;
        self.add_declaration(sig_type, name, identifier_type, declarations, scope);
        Some(())
    }
    
    fn parse_bundle(&mut self, token_stream : &mut TokenStream, identifier_type : IdentifierType, declarations : &mut Vec<SignalDeclaration>, scope : &mut LocalVariableContext) {
        while token_stream.peek_is_plain(TOKEN_IDENTIFIER) {
            if let Some(()) = self.parse_signal_declaration(token_stream, identifier_type, declarations, scope) {

            } else {
                // Error during parsing signal decl. Skip till "," or end of scope
                token_stream.skip_until_one_of(&[kw(","), kw("->"), kw("{"), kw(";")]);
            }
            
            if !token_stream.peek_is_plain(kw(",")) {
                break;
            }
            token_stream.next();
        }
    }

    fn parse_interface(&mut self, token_stream : &mut TokenStream, declarations : &mut Vec<SignalDeclaration>, scope : &mut LocalVariableContext) {
        self.parse_bundle(token_stream, IdentifierType::Input, declarations, scope);
    
        if token_stream.peek_is_plain(kw("->")) {
            token_stream.next();
            self.parse_bundle(token_stream, IdentifierType::Output, declarations, scope);
        }
    }

    fn parse_statement(&mut self, token_stream : &mut TokenStream, declarations : &mut Vec<SignalDeclaration>, scope : &mut LocalVariableContext, statements : &mut Vec<SpanStatement>) -> Option<()> {
        let mut state_decl : Option<usize> = None;
        let start_at = if let Some(peek) = token_stream.peek() {
            peek.get_span().0
        } else {
            return None;
        };
        match token_stream.peek() {
            None => {
                return None;
            }
            Some(TokenTreeNode::PlainToken(tok, pos)) if tok.get_type() == kw("@") => {
                // Assignment
                token_stream.next();
                statements.push((Statement::PipelineStage(*pos), Span::from(*pos)));
                return Some(())
            }
            Some(TokenTreeNode::PlainToken(tok, pos)) if tok.get_type() == kw("#") => {
                // Assignment
                token_stream.next();
                statements.push((Statement::TimelineStage(*pos), Span::from(*pos)));
                return Some(())
            }
            Some(TokenTreeNode::PlainToken(tok, pos)) if tok.get_type() == kw("state") => {
                // Assignment
                token_stream.next();
                state_decl = Some(*pos);
            }
            _other => {}
        }
        let expr_first = self.parse_expression(token_stream, scope)?; // Error case
        let resulting_statement = match token_stream.peek() {
            // Regular assignment
            None => {
                if let Some(kw_pos) = state_decl {
                    self.errors.push(error_basic_str(Span::from(kw_pos), "Cannot attach 'state' keyword in mention"))
                }
                Statement::Mention(expr_first)
            },
            Some(TokenTreeNode::PlainToken(tok, _)) if tok.get_type() == kw(";") => {
                token_stream.next();
                if let Some(kw_pos) = state_decl {
                    self.errors.push(error_basic_str(Span::from(kw_pos), "Cannot attach 'state' keyword in mention"))
                }
                Statement::Mention(expr_first)
            },
            Some(TokenTreeNode::PlainToken(tok, eq_sign_pos)) if tok.get_type() == kw("=") => {
                if let Some(kw_pos) = state_decl {
                    self.errors.push(error_basic_str(Span::from(kw_pos), "Cannot attach 'state' keyword in assignment"))
                }
                // Assignment
                token_stream.next();
                let value = self.parse_expression(token_stream, scope)?;
                self.eat_plain(token_stream, kw(";"), "assignment");
                Statement::Assign(expr_first, value, *eq_sign_pos)
            },
            Some(_other) => {
                let declaration_type = if state_decl.is_some() {
                    IdentifierType::State
                } else {
                    IdentifierType::Local
                };
                // This is a declaration!
                let name = self.eat_identifier(token_stream, "declaration")?;
                match token_stream.next() {
                    Some(TokenTreeNode::PlainToken(tok, eq_sign_pos)) if tok.get_type() == kw("=") => {
                        // Parse set value expression before adding declaration. The variable name should not yet be in scope
                        let value = self.parse_expression(token_stream, scope)?;
                        self.eat_plain(token_stream, kw(";"), "declaration");
                        let id = self.add_declaration(expr_first, name, declaration_type, declarations, scope);
                        Statement::Assign((Expression::Named(IdentifierIdx::new_local(id)), Span::from(name.position)), value, *eq_sign_pos)
                    },
                    Some(TokenTreeNode::PlainToken(tok, _)) if tok.get_type() == kw(";") => {
                        self.add_declaration(expr_first, name, declaration_type, declarations, scope);
                        //Statement::Declare(self.to_signal_declaration(expr_first, name, declaration_type)?)
                        return Some(());
                    },
                    other => {
                        self.errors.push(error_unexpected_tree_node(&[kw(";"), kw("=")], other, token_stream.last_idx, "declaration")); // easy way to throw the End Of Scope error
                        return None;
                        // Statement::Declare(self.to_signal_declaration(expr_first, name)?)
                    }
                }
            }
        };

        statements.push((resulting_statement, Span(start_at, token_stream.last_idx)));
        return Some(())
    }
    fn parse_code_block(&mut self, block_tokens : &[TokenTreeNode], span : Span, declarations : &mut Vec<SignalDeclaration>, outer_scope : &LocalVariableContext) -> Vec<SpanStatement> {
        let mut token_stream = TokenStream::new(block_tokens, span.0, span.1);

        let mut statements : Vec<SpanStatement> = Vec::new();
        
        let mut inner_scope = LocalVariableContext::new_extend(outer_scope);

        while token_stream.peek().is_some() {
            if token_stream.peek_is_plain(kw(";")) {
                token_stream.next();
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
                token_stream.skip_until_one_of(&[kw(";"), kw("{")]);
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

        Some(Module{span : Span(declaration_start_idx, token_stream.last_idx), name, declarations, code})
    }

    fn parse_ast(&mut self, outer_token_iter : &mut TokenStream) -> ASTRoot {
        let mut modules : Vec<Module> = Vec::new();

        while let Some(t) = outer_token_iter.next() {
            match t {
                TokenTreeNode::PlainToken(tok, module_kw_pos) if tok.get_type() == kw("module") => {
                    if let Some(module) = self.parse_module(outer_token_iter, *module_kw_pos) {
                        modules.push(module);
                    }
                },
                other => {
                    self.errors.push(error_unexpected_tree_node(&[kw("module")], Some(other), outer_token_iter.unexpected_eof_token, "file scope"))
                }
            }
        }

        ASTRoot{modules}
    }
}

pub fn parse<'a>(token_hierarchy : &Vec<TokenTreeNode>, num_tokens : usize, numbers : &Vec<BigUint>) -> (ASTRoot, Vec<ParsingError<Span>>) {
    let mut context = ASTParserContext{errors : Vec::new(), numbers};
    let mut token_stream = TokenStream::new(&token_hierarchy, 0, num_tokens);
    let ast_root = context.parse_ast(&mut token_stream);
    
    (ast_root, context.errors)
}



pub struct FullParseResult<'txt> {
    pub tokens : TokenizerResult<'txt>,
    pub token_hierarchy : Vec<TokenTreeNode>,
    pub ast : ASTRoot
}

pub fn perform_full_semantic_parse<'txt>(file_text : &'txt str) -> (FullParseResult<'txt>, Vec<ParsingError<CharSpan>>) {
    let (tokens, mut errors) = tokenize(file_text);

    let (token_hierarchy, hierarchy_errors) = to_token_hierarchy(&tokens.tokens);
    for err in hierarchy_errors {
        errors.push(cvt_token_error_to_str_error(err, &tokens.token_spans));
    }

    let (ast, parse_errors) = parse(&token_hierarchy, tokens.token_spans.len(), &tokens.numbers);
    for err in parse_errors {
        errors.push(cvt_token_error_to_str_error(err, &tokens.token_spans));
    }

    (FullParseResult{
        tokens,
        token_hierarchy,
        ast,
    }, errors)
}
