use std::ops::Deref;

use crate::{ast::{Value, Operator, TypeExpression, GlobalReference}, linker::{get_builtin_uuid, NamedUUID, Linker, Linkable}, tokenizer::kw};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Named(NamedUUID),
    Array(Box<Type>)
}

impl Type {
    pub fn to_string(&self, linker : &Linker) -> String {
        match self {
            Type::Named(n) => linker.links[*n].get_full_name(),
            Type::Array(sub) => sub.deref().to_string(linker) + "[]",
        }
    }
    pub fn get_root(&self) -> NamedUUID {
        match self {
            Type::Named(name) => *name,
            Type::Array(sub) => sub.get_root(),
        }
    }
}

impl Value {
    pub fn get_type(&self) -> Type {
        match self {
            Value::Bool(_) => Type::Named(get_builtin_uuid("bool")),
            Value::Integer(_) => Type::Named(get_builtin_uuid("int")),
        }
    }
}

impl TypeExpression {
    pub fn map_to_type(&self, global_references : &[GlobalReference]) -> Type {
        match self {
            TypeExpression::Named(n) => Type::Named(global_references[*n].1),
            TypeExpression::Array(b) => {
                let (sub, _idx) = b.deref();
                Type::Array(Box::new(sub.map_to_type(global_references)))
                // TODO gather bound constraints
            },
        }
    }
}


pub fn get_unary_operator_types(op : Operator) -> (Type, Type) {
    let bool = Type::Named(get_builtin_uuid("bool"));
    let int = Type::Named(get_builtin_uuid("int"));
    
    match op.op_typ {
        x if x == kw("!") => (bool.clone(), bool),
        x if x == kw("&") => (Type::Array(Box::new(bool.clone())), bool),
        x if x == kw("|") => (Type::Array(Box::new(bool.clone())), bool),
        x if x == kw("^") => (Type::Array(Box::new(bool.clone())), bool),
        x if x == kw("+") => (Type::Array(Box::new(int.clone())), int),
        x if x == kw("*") => (Type::Array(Box::new(int.clone())), int),
        _ => unreachable!()
    }
}
pub fn get_binary_operator_types(op : Operator) -> ((Type, Type), Type) {
    let bool = get_builtin_uuid("bool");
    let int = get_builtin_uuid("int");
    
    let (a, b, o) = match op.op_typ {
        x if x == kw("&") => (bool, bool, bool),
        x if x == kw("|") => (bool, bool, bool),
        x if x == kw("^") => (bool, bool, bool),
        x if x == kw("+") => (int, int, int),
        x if x == kw("-") => (int, int, int),
        x if x == kw("*") => (int, int, int),
        x if x == kw("/") => (int, int, int),
        x if x == kw("%") => (int, int, int),
        x if x == kw("==") => (int, int, bool),
        x if x == kw("!=") => (int, int, bool),
        x if x == kw(">=") => (int, int, bool),
        x if x == kw("<=") => (int, int, bool),
        x if x == kw(">") => (int, int, bool),
        x if x == kw("<") => (int, int, bool),
        _ => unreachable!()
    };
    ((Type::Named(a), Type::Named(b)), Type::Named(o))
}

