
use crate::{tokenizer::*, errors::*, ast::*};

use std::iter::Peekable;
use core::slice::Iter;

pub enum TokenTreeNode {
    PlainToken(TokenTypeIdx, usize), // Has the index of the given token to the global Token array
    // Code between '{' and '}', '(' and ')', or '[' and ']' exclusive. Contains sublist of tokens, index of open, index of close bracket
    Block(TokenTypeIdx, Vec<Self>, Span), 
}
impl TokenTreeNode {
    fn get_token_type(&self) -> TokenTypeIdx {
        match self {
            Self::PlainToken(typ, _) => *typ,
            Self::Block(typ, _, _) => *typ
        }
    }
    fn get_span(&self) -> Span {
        match self {
            Self::PlainToken(_, pos) => Span::from(*pos),
            Self::Block(_, _, span) => *span
        }
    }
}

struct TokenHierarchyStackElem {
    open_bracket : TokenTypeIdx, 
    open_bracket_pos : usize,
    parent : Vec<TokenTreeNode>
}

pub fn to_token_hierarchy(token_types : &[TokenTypeIdx]) -> (Vec<TokenTreeNode>, Vec<ParsingError<Span>>) {
    let mut cur_token_slab : Vec<TokenTreeNode> = Vec::new();
    let mut stack : Vec<TokenHierarchyStackElem> = Vec::new(); // Type of opening bracket, token position, Token Subtree
    let mut errors : Vec<ParsingError<Span>> = Vec::new();

    for (idx, &tok) in token_types.iter().enumerate() {
        if is_comment(tok) { // At this stage the comments are filtered out
            continue;
        }
        match is_bracket(tok) {
            IsBracket::Open => {
                stack.push(TokenHierarchyStackElem{open_bracket : tok, open_bracket_pos : idx, parent : cur_token_slab});
                cur_token_slab = Vec::new();
            },
            IsBracket::Close => {
                loop { // Loop for bracket stack unrolling, for correct code only runs once
                    if let Some(cur_block) = stack.pop() {
                        if closes(cur_block.open_bracket, tok) { // All is well. This bracket was closed properly. Happy path!
                            let mut parent_cur_token_slab = cur_block.parent;
                            parent_cur_token_slab.push(TokenTreeNode::Block(cur_block.open_bracket, cur_token_slab, Span(cur_block.open_bracket_pos, idx)));
                            cur_token_slab = parent_cur_token_slab;
                            break;
                        } else {
                            if !stack.iter().any(|prev_bracket| closes(prev_bracket.open_bracket, tok)) { // Any bracket in the stack closes this?
                                errors.push(error_unopened_bracket(idx, tok, cur_block.open_bracket_pos));
                                stack.push(cur_block); // Push the previous bracket back onto bracket stack, as we disregarded erroneous closing bracket
                                break;
                            } else {
                                errors.push(error_unclosed_bracket(cur_block.open_bracket_pos, token_types[cur_block.open_bracket_pos], idx));
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

struct ASTParserContext {
    errors : Vec<ParsingError<Span>>
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
        if let Some(TokenTreeNode::PlainToken(typ, _place)) = self.iter.peek() {
            if *typ == expected {
                return true;
            }
        }
        false
    }
    fn peek_is_plain_one_of(&mut self, expecteds : &[TokenTypeIdx]) -> bool {
        if let Some(TokenTreeNode::PlainToken(typ, _place)) = self.iter.peek() {
            for ex in expecteds {
                if *typ == *ex {
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

impl ASTParserContext {
    fn eat_plain(&mut self, token_stream : &mut TokenStream, expected : TokenTypeIdx, context : &str) -> Option<usize> {
        assert!(is_bracket(expected) == IsBracket::NotABracket);
        
        let tok_elem = token_stream.next();
        
        match tok_elem {
            Some(TokenTreeNode::PlainToken(typ, idx)) if *typ == expected => {
                Some(*idx)
            },
            other => {
                self.errors.push(error_unexpected_tree_node(&[expected], other, token_stream.unexpected_eof_token, context));
                None
            }
        }
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

    // For expression 
    fn parse_unit_expression(&mut self ,token_stream : &mut TokenStream) -> Option<SpanExpression> {
        match token_stream.next() {
            Some(TokenTreeNode::PlainToken(typ, pos)) if *typ == TOKEN_IDENTIFIER => {
                Some((Expression::Named(*pos), Span(*pos, *pos)))
            },
            Some(TokenTreeNode::PlainToken(typ, pos)) if *typ == TOKEN_NUMBER => {
                Some((Expression::Constant(*pos), Span(*pos, *pos)))
            },
            Some(TokenTreeNode::Block(typ, contents, span)) if *typ == kw("(") => {
                let mut content_token_stream = TokenStream::new(contents, span.0, span.1);
                if let Some(result) = self.parse_expression(&mut content_token_stream) {
                    if let Some(erroneous_found_token) = content_token_stream.peek() {
                        // The expression should cover the whole brackets! 
                        let infos = vec![
                            error_info_str(*span, "Expression should have ended with this scope"),
                            error_info_str(result.1, "But actually only stretches this far"),
                        ];
                        self.errors.push(error_with_info(erroneous_found_token.get_span(), "The expression should have ended at the end of the () brackets. But instead it ended here.".to_owned(), infos));
                        None
                    } else {
                        Some(result)
                    }
                } else {
                    None
                }
            }
            other => {
                self.errors.push(error_unexpected_tree_node(&[TOKEN_IDENTIFIER, TOKEN_NUMBER, kw("(")], other, token_stream.unexpected_eof_token, "unit expression"));
                None
            }
        }
    }
    fn parse_expression(&mut self, token_stream : &mut TokenStream) -> Option<SpanExpression> {
        // Shunting-yard algorithm with single stack
        let mut stack : Vec<(SpanExpression, TokenTypeIdx, usize)> = Vec::new();
        loop {
            let mut grabbed_symbol = self.parse_unit_expression(token_stream)?;
            match token_stream.peek() {
                Some(TokenTreeNode::PlainToken(typ, op_pos)) if is_operator(*typ) => {
                    //let operator_prescedence = get_binary_operator_prescedence(*typ);
                    while let Some((left_expr, stack_op, stack_op_pos)) = stack.pop() {
                        if get_binary_operator_prescedence(stack_op) >= get_binary_operator_prescedence(*typ) {
                            grabbed_symbol = Expression::new_binop(left_expr, stack_op, stack_op_pos, grabbed_symbol);
                        } else {
                            stack.push((left_expr, stack_op, stack_op_pos)); // oops, shouldn't have popped it
                            break;
                        }
                    }

                    token_stream.next(); // commit operator peek
                    stack.push((grabbed_symbol, *typ, *op_pos));
                },
                _other => {
                    while let Some((left_expr, stack_op, stack_op_pos)) = stack.pop() {
                        grabbed_symbol = Expression::new_binop(left_expr, stack_op, stack_op_pos, grabbed_symbol);
                    }
                    return Some(grabbed_symbol);
                }
            }
        }
    }

    fn parse_signal_declaration(&mut self, token_stream : &mut TokenStream, identifier_type : IdentifierType) -> Option<SignalDeclaration> {
        let sig_type = self.parse_expression(token_stream)?;
        let name_token = self.eat_plain(token_stream, TOKEN_IDENTIFIER, "signal declaration")?;
        Some(SignalDeclaration{span : Span(sig_type.1.0, token_stream.last_idx), typ : sig_type, name_token : name_token, identifier_type : identifier_type})
    }
    
    fn parse_bundle(&mut self, token_stream : &mut TokenStream, identifier_type : IdentifierType) -> Vec<SignalDeclaration> {
        let mut result : Vec<SignalDeclaration> = Vec::new();
        while token_stream.peek_is_plain(TOKEN_IDENTIFIER) {
            if let Some(decl) = self.parse_signal_declaration(token_stream, identifier_type) {
                result.push(decl);
            } else {
                // Error during parsing signal decl. Skip till "," or end of scope
                token_stream.skip_until(kw(","));
            }
            
            if !token_stream.peek_is_plain(kw(",")) {
                break;
            }
            token_stream.next();
        }
        result
    }

    fn parse_interface(&mut self, token_stream : &mut TokenStream) -> Interface {
        let start_idx = token_stream.last_idx + 1;

        let inputs = self.parse_bundle(token_stream, IdentifierType::Input);
    
        let outputs = if token_stream.peek_is_plain(kw("->")) {
            token_stream.next();
            self.parse_bundle(token_stream, IdentifierType::Output)
        } else {
            Vec::new()
        };
        
        Interface{span : Span(start_idx, token_stream.last_idx), inputs : inputs, outputs : outputs}
    }

    fn to_signal_declaration(&mut self, type_expr : SpanExpression, name_token_idx : usize, identifier_type : IdentifierType) -> Option<SignalDeclaration> {
        let decl_span = Span(type_expr.1.0, name_token_idx);
        Some(SignalDeclaration{typ : type_expr, span : decl_span, name_token : name_token_idx, identifier_type : identifier_type})
    }

    fn parse_statement(&mut self, token_stream : &mut TokenStream) -> Option<SpanStatement> {
        let expr_first = self.parse_expression(token_stream)?; // Error case
        let start_at = expr_first.1.0;
        let resulting_statement = match token_stream.peek() {
            // Regular assignment
            None => {
                Statement::Mention(expr_first)
            },
            Some(TokenTreeNode::PlainToken(typ, _)) if *typ == kw(";") => {
                token_stream.next();
                Statement::Mention(expr_first)
            },
            Some(TokenTreeNode::PlainToken(typ, _)) if *typ == kw("=") => {
                // Assignment
                token_stream.next();
                let value = self.parse_expression(token_stream)?;
                self.eat_plain(token_stream, kw(";"), "assignment");
                Statement::Assign(expr_first, value)
            },
            Some(_other) => {
                // This is a declaration!
                let name = self.eat_plain(token_stream, TOKEN_IDENTIFIER, "declaration")?;
                match token_stream.next() {
                    Some(TokenTreeNode::PlainToken(typ, _)) if *typ == kw("=") => {
                        let value = self.parse_expression(token_stream)?;
                        self.eat_plain(token_stream, kw(";"), "declaration");
                        Statement::DeclareAssign(self.to_signal_declaration(expr_first, name, IdentifierType::Local)?, value)
                    },
                    Some(TokenTreeNode::PlainToken(typ, _)) if *typ == kw(";") => {
                        Statement::Declare(self.to_signal_declaration(expr_first, name, IdentifierType::Local)?)
                    },
                    other => {
                        self.errors.push(error_unexpected_tree_node(&[kw(";"), kw("=")], other, token_stream.last_idx, "declaration")); // easy way to throw the End Of Scope error
                        return None;
                        // Statement::Declare(self.to_signal_declaration(expr_first, name)?)
                    }
                }
            }
        };

        Some((resulting_statement, Span(start_at, token_stream.last_idx)))
    }
    fn parse_code_block(&mut self, block_tokens : &[TokenTreeNode], span : Span) -> Vec<SpanStatement> {
        let mut token_stream = TokenStream::new(block_tokens, span.0, span.1);

        let mut statements : Vec<SpanStatement> = Vec::new();
        
        while token_stream.peek().is_some() {
            if token_stream.peek_is_plain(kw(";")) {
                token_stream.next();
                continue;
            }
            if let Some(TokenTreeNode::Block(typ, contents, block_span)) = token_stream.peek() {
                if *typ == kw("{") {
                    statements.push((Statement::Block(self.parse_code_block(contents, *block_span)), *block_span));
                    token_stream.next();
                    continue; // Can't add condition to if let, so have to do some weird control flow here
                }
            }
            if let Some(stmt) = self.parse_statement(&mut token_stream) {
                statements.push(stmt);
            } else {
                // Error recovery. Find end of statement
                token_stream.skip_until_one_of(&[kw(";"), kw("{")]);
            }
        }

        statements
    }

    fn parse_module(&mut self, token_stream : &mut TokenStream, declaration_start_idx : usize) -> Option<Module> {
        // done by caller 
        // self.eat_plain(token_stream, kw("module"));
        let module_name = self.eat_plain(token_stream, TOKEN_IDENTIFIER, "module")?;
        self.eat_plain(token_stream, kw(":"), "module")?;

        let module_interface = self.parse_interface(token_stream);

        let (block_tokens, block_span) = self.eat_block(token_stream, kw("{"), "module")?;

        let module_code = self.parse_code_block(block_tokens, block_span);

        Some(Module{span : Span(declaration_start_idx, token_stream.last_idx), name: module_name, interface : module_interface, code : module_code})
    }

    fn parse_ast(&mut self, outer_token_iter : &mut TokenStream) -> ASTRoot {
        let mut found_modules : Vec<Module> = Vec::new();

        while let Some(t) = outer_token_iter.next() {
            match t {
                TokenTreeNode::PlainToken(typ, module_kw_pos) if *typ == kw("module") => {
                    if let Some(module) = self.parse_module(outer_token_iter, *module_kw_pos) {
                        found_modules.push(module);
                    }
                },
                other => {
                    self.errors.push(error_unexpected_tree_node(&[kw("module")], Some(other), outer_token_iter.unexpected_eof_token, "file scope"))
                }
            }
        }

        ASTRoot{modules : found_modules}
    }
}

pub fn parse<'a>(token_hierarchy : &Vec<TokenTreeNode>, num_tokens : usize) -> (ASTRoot, Vec<ParsingError<Span>>) {
    let mut context = ASTParserContext{errors : Vec::new()};
    let mut token_stream = TokenStream::new(&token_hierarchy, 0, num_tokens);
    let ast_root = context.parse_ast(&mut token_stream);
    
    (ast_root, context.errors)
}
