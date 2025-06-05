use crate::{
    flattening::WrittenType,
    typing::template::{GlobalReference, TemplateKind},
};

use super::{ExpressionSource, WireReference, WireReferencePathElement, WireReferenceRoot};
use crate::prelude::*;

impl ExpressionSource {
    /// Enumerates all instructions that this instruction depends on. This includes (maybe compiletime) wires, and submodules.
    pub fn for_each_dependency(&self, collect: &mut impl FnMut(FlatID)) {
        self.for_each_input_wire(collect);
        match self {
            ExpressionSource::WireRef(wire_ref) => match &wire_ref.root {
                WireReferenceRoot::LocalDecl(decl_id) => collect(*decl_id),
                WireReferenceRoot::NamedConstant(cst) => {
                    cst.for_each_generative_input(collect);
                }
                WireReferenceRoot::SubModulePort(submod_port) => {
                    collect(submod_port.submodule_decl)
                }
                WireReferenceRoot::Error => {}
            },
            ExpressionSource::FuncCall(func_call) => {
                collect(func_call.interface_reference.submodule_decl);
            }
            _ => {}
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
    pub fn for_each_generative_input(&self, collect: &mut impl FnMut(FlatID)) {
        match &self.root {
            WireReferenceRoot::LocalDecl(decl_id) => collect(*decl_id),
            WireReferenceRoot::NamedConstant(cst) => {
                cst.for_each_generative_input(collect);
            }
            WireReferenceRoot::SubModulePort(submod_port) => collect(submod_port.submodule_decl),
            WireReferenceRoot::Error => {}
        }
        self.for_each_input_wire_in_path(collect);
    }
    pub fn for_each_input_wire_in_path(&self, collect: &mut impl FnMut(FlatID)) {
        for p in &self.path {
            match p {
                WireReferencePathElement::ArrayAccess { idx, .. } => collect(*idx),
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
