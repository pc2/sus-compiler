
use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operator {
	Plus,
	Minus,
	Times,
	Divide,
	Modulo
}

#[derive(Debug, PartialEq)]
enum Token {
	Named(String),
	Number(i64),
	
	Module,
	
	OpenScope,
	CloseScope,
	Semicolon,
	Colon,
	Comma,
	Assign,

	Operator(Operator),
	OutputMarker
}

fn tokenize(file_text : String) -> Vec<Token> {
	let mut tokens : Vec<Token> = Vec::new();
	let mut token_positions : Vec<usize> = Vec::new();
	let mut char_iter = file_text.chars().enumerate().peekable();
	while let Some((word_start, cur_char)) = char_iter.next() {
		token_positions.push(word_start);
		if cur_char.is_whitespace() {
			continue;
		} else if cur_char.is_alphabetic() || cur_char == '_' {
			loop {
				let &(next_idx, next_char) = char_iter.peek().unwrap();
				if !next_char.is_alphanumeric() && next_char != '_' {
					tokens.push(match file_text.get(word_start..next_idx).unwrap() {
						"module" => Token::Module,
						word => Token::Named(word.to_string())
					});
					break;
				} else {
					char_iter.next();
				}
			}
		} else if cur_char.is_digit(10) {
			loop {
				let &(next_idx, next_char) = char_iter.peek().unwrap();
				if !next_char.is_digit(10) {
					let num = file_text.get(word_start..next_idx).unwrap();
					tokens.push(Token::Number(num.parse::<i64>().unwrap()));
					break;
				} else {
					char_iter.next();
				}
			}
		} else {
			tokens.push(match (cur_char, char_iter.peek()) {
				('-', Some((_, '>'))) => {
					char_iter.next();
					Token::OutputMarker
				}
				('{', _) => Token::OpenScope,
				('}', _) => Token::CloseScope,
				(';', _) => Token::Semicolon,
				(':', _) => Token::Colon,
				(',', _) => Token::Comma,
				('=', _) => Token::Assign,
				('+', _) => Token::Operator(Operator::Plus),
				('-', _) => Token::Operator(Operator::Minus),
				('*', _) => Token::Operator(Operator::Times),
				('/', _) => Token::Operator(Operator::Divide),
				('%', _) => Token::Operator(Operator::Modulo),
				_ => {
					panic!("Unexpected char at position {}: {}", word_start, cur_char);
				}
			});
		}
	}
	return tokens;
}

#[derive(Debug, Clone)]
struct Type {
	name : String
}

#[derive(Debug, Clone)]
struct Connector {
	typ : Type,
	name : String
}

fn eat(tok : Token, it : &mut std::iter::Peekable<std::slice::Iter<Token>>) {
	match it.next() {
		Some(t) => {
			if tok != *t {
				panic!("Unexpected token! Expected {:?} but got {:?}", tok, t);
			}
		},
		None => {
			panic!("Unexpected end of token stream! Expected {:?}", tok);
		}
	}
}

fn eat_named(it : &mut std::iter::Peekable<std::slice::Iter<Token>>) -> String {
	match it.next() {
		Some(Token::Named(s)) => s.clone(),
		Some(other) => {
			panic!("Unexpected token! Expected Named but got {:?}", other);
		},
		_ => {
			panic!("Unexpected end of token stream! Expected Named");
		}
	}
}

fn parse_type(first_tok : &Token, token_iter : &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Type {
	match first_tok {
		Token::Named(s) => Type{name: s.clone()},
		other => {
			panic!("Unexpected token: {:?}. Expected type!", other);
		}
	}
}

fn parse_connector_from(first_tok : &Token, token_iter : &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Connector {
	let typ = parse_type(first_tok, token_iter);

	Connector{typ : typ, name: eat_named(token_iter)}
}

fn parse_connector(token_iter : &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Connector {
	let typ_first_tok = token_iter.next().unwrap();
	
	parse_connector_from(typ_first_tok, token_iter)
}

fn parse_list<T>(token_iter : &mut std::iter::Peekable<std::slice::Iter<Token>>, sep_token : Token, parse_elem: impl Fn(&mut std::iter::Peekable<std::slice::Iter<Token>>) -> T) -> Vec<T> {
	let mut result = Vec::new();
	
	loop {
		result.push(parse_elem(token_iter));
		if **token_iter.peek().unwrap() == sep_token {
			token_iter.next();
		} else {
			return result;
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum UnaryOperator {

}

#[derive(Debug, Clone)]
enum Expression {
	Named(String),
	Literal(i64),
	UnaryOperator(Operator, Box<Expression>),
	BinaryOperator(Operator, Box<Expression>, Box<Expression>)
}

fn parse_expression_term(token_iter : &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Expression {
	match token_iter.next().unwrap() {
		Token::Named(s) => {
			return Expression::Named(s.clone());
		},
		other => {
			panic!("Unexpected token: {:?}, expected expression term", other);
		}
	}
}

fn parse_expression(token_iter : &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Expression {
	let mut cur_expr = parse_expression_term(token_iter);
	loop {
		match token_iter.peek().unwrap() {
			Token::Operator(op) => {
				token_iter.next();
				cur_expr = Expression::BinaryOperator(*op, Box::new(cur_expr), Box::new(parse_expression_term(token_iter)));
			},
			_other => { // Unknown token, probably belongs to the outer scope, we can't handle it
				return cur_expr;
			}
		}
	}
}

fn list_names_in_expression<'a>(expr : &'a Expression) -> Vec<&'a str> {
	let mut found_names : Vec<&'a str> = Vec::new();

	list_names_in_expression_recursive(expr, &mut found_names);

	found_names
}

fn list_names_in_expression_recursive<'a>(expr : &'a Expression, found_so_far : &mut Vec<&'a str>) {
	match expr {
		Expression::Named(s) => {
			found_so_far.push(s);
		},
		Expression::UnaryOperator(_,a) => {
			list_names_in_expression_recursive(a, found_so_far);
		}
		Expression::BinaryOperator(_,a,b) => {
			list_names_in_expression_recursive(a, found_so_far);
			list_names_in_expression_recursive(b, found_so_far);
		}
		_other => {}
	}
}

#[derive(Debug, Clone)]
struct Block {
	declarations : Vec<Connector>,
	statements : Vec<Statement>
}

#[derive(Debug, Clone)]
enum Statement {
	Assignment(String, Expression),
	Block(Block)
}

fn parse_block(token_iter : &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Block {
	let mut statements : Vec<Statement> = Vec::new();
	let mut declarations : Vec<Connector> = Vec::new();
	
	eat(Token::OpenScope, token_iter);

	loop {
		match token_iter.next().unwrap() {
			Token::CloseScope => {
				return Block{statements : statements, declarations : declarations};
			}
			first_token_of_perhaps_type => {
				match token_iter.peek().unwrap() {
					Token::Assign => {
						match first_token_of_perhaps_type {
							Token::Named(name) => {
								token_iter.next();
								statements.push(Statement::Assignment(name.clone(), parse_expression(token_iter)));
							},
							_other => {
								panic!("Unexpected token {:?}", first_token_of_perhaps_type);
							}
						}
					},
					_other => {
						let decl = parse_connector_from(first_token_of_perhaps_type, token_iter);
						match token_iter.peek().unwrap() {
							Token::Assign => {
								token_iter.next();
								statements.push(Statement::Assignment(decl.name.clone(), parse_expression(token_iter)));
							},
							_other_other => {continue;} // End of declaration. 
						}
						declarations.push(decl)
					}
				}
			}
		}
	}
}

fn parse_statement(token_iter : &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Statement {
	match token_iter.peek().unwrap() {
		Token::OpenScope => Statement::Block(parse_block(token_iter)),
		other => {
			panic!("Unexpected token: {:?}", other);
		}
	}
}

#[derive(Debug)]
struct Module {
	name : String,
	inputs : Vec<Connector>,
	outputs : Vec<Connector>,
	content : Block
}

fn parse_module(token_iter : &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Module {
	eat(Token::Module, token_iter);
	let name = eat_named(token_iter);
	eat(Token::Colon, token_iter);
	let inputs = parse_list(token_iter, Token::Comma, &parse_connector);
	eat(Token::OutputMarker, token_iter);
	let outputs = parse_list(token_iter, Token::Comma, &parse_connector);
	let block = parse_block(token_iter);
	return Module{name: name, inputs: inputs, outputs: outputs, content: block};
}

struct CompiledModule {
	holders : Vec<Type>,
	inputs_end : usize,
	outputs_end : usize
}

fn block_declares_var(block : &Block, var_name : &str) -> bool {
	for d in &block.declarations {
		if d.name == var_name {
			return true;
		}
	}
	return false;
}

fn collect_assignments_to_var(block : &Block, var_name : &str) -> Option<Expression> {
	if block_declares_var(block, var_name) {
		None
	} else {
		let mut found_assignment : Option<Expression> = None;
		for stm in &block.statements {
			match stm {
				Statement::Assignment(name, expr) => {
					if name == var_name {
						if found_assignment.is_none() {
							panic!("Duplicate Assignment found for {}!", var_name);
						}
						found_assignment = Some(expr.clone());
					}
				},
				_other => {todo!();}
			}
		}
		found_assignment
	}
}

fn add_all_refs<'a, T>(to_copy : &'a [T], copy_into : &mut Vec<&'a T>) {
	for t in to_copy {
		copy_into.push(&t);
	}
}

enum HolderOperator {
	Input,
	Add,
	Multiply,

}

struct Holder {
	op : HolderOperator,
	v1 : usize,
	v2 : usize,
	v3 : usize
}

fn compile_expression_recursive() {

}

fn compile_module(m : &Module) {
	let mut all_connectors : Vec<&Connector> = Vec::new();
	let mut all_holders : Vec<Holder> = Vec::new();

	add_all_refs(&m.inputs, &mut all_connectors);
	add_all_refs(&m.outputs, &mut all_connectors);
	add_all_refs(&m.content.declarations, &mut all_connectors);
	
	for c in &m.inputs {
		all_connectors.push(&c);
		all_holders.push(Holder{op: HolderOperator::Input, v1: 0, v2: 0, v3: 0});
	}

	let mut to_assign : Vec<&Connector> = Vec::new();
	add_all_refs(&m.outputs, &mut to_assign);
	add_all_refs(&m.content.declarations, &mut to_assign);
	
	

	for var_to_assign in to_assign {
		let varAssign = collect_assignments_to_var(&m.content, &var_to_assign.name).unwrap(); // Error variable must be assigned

		
	}
}

enum VarType {
	Integer,
	Bool
}



enum VarOperation<'a> {
	Input,
	Operator(Operator, &'a Variable, &'a Variable)
}

struct Variable {
	name : String,
	typ : VarType
}

fn type_check_module(m : &mut Module) {
	let mut scope : Vec<Variable> = Vec::new();


}



struct FlattenedModule {
	
}

fn flatten_module(m : &Module) {

}

fn main() {
    println!("Hello, world!");
	
	let filename = "multiply_add.vpp";
	
	let contents = fs::read_to_string(filename).expect("Could not read file");
	
	println!("File text:\n{}", contents);

	let tokens = tokenize(contents);
	println!("Tokens:\n{:?}", &tokens);

	let module = parse_module(&mut tokens.iter().peekable());
	println!("Module:\n{:?}", &module);

	let flattened_module = flatten_module(module);
	println!("Flattened Module:\n{:?}", &flattened_module);
}
