//! Implementes the Hindley-Milner algorithm for Type Inference. 

use itertools::Itertools;

use crate::prelude::*;

use crate::alloc::{UUIDMarker, UUID};

struct TypeVariableIDMarker;
impl UUIDMarker for TypeVariableIDMarker {
    const DISPLAY_NAME: &'static str = "type_variable_";
}
type TypeVariableID = UUID<TypeVariableIDMarker>;

#[derive(Debug, Clone)]
enum MyType {
    TypeFunc(TypeUUID, FlatAlloc<MyType, TemplateIDMarker>),
    TypeVar(TypeVariableID),
}

/// Returns false if the types couldn't be unified
fn unify<'t>(substitution_map: &mut FlatAlloc<Option<&'t MyType>, TypeVariableIDMarker>, a: &'t MyType, b: &'t MyType) -> Result<(), ()> {
    match a {
        MyType::TypeFunc(tf_a, args_a) => {
            match b {
                MyType::TypeFunc(tf_b, args_b) => {
                    if tf_a != tf_b {
                        return Err(());
                    }
                    for ((_, arg_a), (_, arg_b)) in args_a.iter().zip_eq(args_b.iter()) {
                        unify(substitution_map, arg_a, arg_b)?;
                    }
                    Ok(())
                }
                MyType::TypeVar(_) => unify(substitution_map, b, a)
            }
        }
        MyType::TypeVar(var) => {
            let typ_cell = &mut substitution_map[*var];
            if let Some(found) = typ_cell {
                let ff : &'t MyType = *found;
                unify(substitution_map, ff, b)
            } else {
                *typ_cell = Some(b);
                Ok(())
            }
        }
    }
}

fn fully_substitute(substitution_map: &mut FlatAlloc<Option<&MyType>, TypeVariableIDMarker>, a: &MyType) -> MyType {
    match a {
        MyType::TypeFunc(tf, tf_args) => MyType::TypeFunc(*tf, tf_args.map(|(_, arg)| fully_substitute(substitution_map, arg))),
        MyType::TypeVar(v) => {
            let substituted = substitution_map[*v].expect("This variable wasn't properly substituted");
            fully_substitute(substitution_map, substituted)
        }
    }
}
