use crate::typing::template::{TemplateArgKind, TemplateArgs};

use crate::prelude::*;

use super::{WireReferencePathElement, WireReferenceRoot, WireSource, WrittenType};


impl WireSource {
    /// Enumerates all instructions that this instruction depends on. This includes (maybe compiletime) wires, and submodules. 
    pub fn for_each_dependency<F : FnMut(FlatID)>(&self, mut func : F) {
        match self {
            WireSource::WireRef(wire_ref) => {
                match &wire_ref.root {
                    WireReferenceRoot::LocalDecl(decl_id, _) => func(*decl_id),
                    WireReferenceRoot::NamedConstant(_, _) => {}
                    WireReferenceRoot::SubModulePort(submod_port) => func(submod_port.submodule_decl),
                }
                for p in &wire_ref.path {
                    match p {
                        WireReferencePathElement::ArrayAccess { idx, bracket_span:_ } => func(*idx),
                    }
                }
            }
            &WireSource::UnaryOp { op:_, right } => {func(right)}
            &WireSource::BinaryOp { op:_, left, right } => {func(left); func(right)},
            WireSource::Constant(_) => {}
        }
    }
}

pub fn for_each_generative_input_in_template_args<F : FnMut(FlatID)>(template_args : &TemplateArgs, f : &mut F) {
    for (_id, t_arg) in template_args.iter_valids() {
        match &t_arg.kind {
            TemplateArgKind::Type(typ) => {
                typ.for_each_generative_input(f)
            }
            TemplateArgKind::Value(val) => {
                f(*val)
            }
        }
    }
}

impl WrittenType {
    pub fn for_each_generative_input<F : FnMut(FlatID)>(&self, f : &mut F) {
        match self {
            WrittenType::Error(_) | WrittenType::Template(_, _) => {}
            WrittenType::Named(name) => {
                for_each_generative_input_in_template_args(&name.template_args, f)
            }
            WrittenType::Array(_span, arr_box) => {
                use std::ops::Deref;
                f(arr_box.deref().1)
            }
        }
    }
}
