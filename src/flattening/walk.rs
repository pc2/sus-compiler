use crate::{
    flattening::{GlobalReference, WrittenType},
    typing::template::TemplateKind,
};

use super::{ExpressionSource, WireReference, WireReferencePathElement, WireReferenceRoot};
use crate::prelude::*;

impl ExpressionSource {
    /// Enumerates all instructions that this instruction depends on. This includes (maybe compiletime) wires, and submodules.
    pub fn for_each_dependency(&self, collect: &mut impl FnMut(FlatID)) {
        self.for_each_input_wire(collect);
        if let ExpressionSource::WireRef(wire_ref) = self {
            wire_ref.for_each_generative_input_in_root(collect)
        }
    }

    /// Enumerates all input wires that are allowed to be non-generative
    ///
    /// So excludes template args
    ///
    /// Guarantees all results to be [SingleOutputExpression]
    pub fn for_each_input_wire(&self, collect: &mut impl FnMut(FlatID)) {
        match self {
            ExpressionSource::WireRef(wire_ref) => {
                wire_ref.for_each_input_wire_in_path(collect);
            }
            ExpressionSource::UnaryOp { right, .. } => collect(*right),
            ExpressionSource::BinaryOp { left, right, .. } => {
                collect(*left);
                collect(*right)
            }
            ExpressionSource::FuncCall(func_call) => {
                collect(func_call.func_wire_ref);
                for arg in &func_call.arguments {
                    collect(*arg)
                }
            }
            ExpressionSource::Literal(_) => {}
            ExpressionSource::ArrayConstruct(arr) => {
                for v in arr {
                    collect(*v);
                }
            }
        }
    }
}

impl WireReference {
    pub fn for_each_generative_input_in_root(&self, collect: &mut impl FnMut(FlatID)) {
        match &self.root {
            WireReferenceRoot::LocalDecl(decl_id) => collect(*decl_id),
            WireReferenceRoot::LocalSubmodule(submod_decl) => collect(*submod_decl),
            WireReferenceRoot::LocalInterface(interface_decl) => collect(*interface_decl),
            WireReferenceRoot::NamedConstant(cst) => {
                cst.for_each_generative_input(collect);
            }
            WireReferenceRoot::NamedModule(md) => {
                md.for_each_generative_input(collect);
            }
            WireReferenceRoot::Error => {}
        }
    }
    pub fn for_each_input_wire_in_path(&self, collect: &mut impl FnMut(FlatID)) {
        for p in &self.path {
            match p {
                WireReferencePathElement::FieldAccess { .. } => {}
                WireReferencePathElement::ArrayAccess { idx, .. } => collect(*idx),
                WireReferencePathElement::ArraySlice { from, to, .. } => {
                    if let Some(from) = from {
                        collect(*from);
                    }
                    if let Some(to) = to {
                        collect(*to);
                    }
                }
                WireReferencePathElement::ArrayPartSelect { from, width, .. } => {
                    collect(*from);
                    collect(*width);
                }
            }
        }
    }
}

impl WrittenType {
    pub fn for_each_generative_input(&self, f: &mut impl FnMut(FlatID)) {
        match self {
            WrittenType::Error(_) | WrittenType::TemplateVariable(_, _) => {}
            WrittenType::Named(name) => name.for_each_generative_input(f),
            WrittenType::Array(_span, arr_box) => {
                use std::ops::Deref;
                f(arr_box.deref().1)
            }
        }
    }
}

impl<ID> GlobalReference<ID> {
    pub fn for_each_generative_input(&self, f: &mut impl FnMut(FlatID)) {
        for t_arg in &self.template_args {
            match &t_arg.kind {
                None => {}
                Some(TemplateKind::Type(t)) => {
                    t.for_each_generative_input(f);
                }
                Some(TemplateKind::Value(v)) => f(*v),
            }
        }
    }
}
