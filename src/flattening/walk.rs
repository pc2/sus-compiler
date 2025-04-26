use crate::typing::template::{TVec, TemplateArg, TemplateArgKind};

use crate::prelude::*;

use super::{ExpressionSource, WireReferencePathElement, WireReferenceRoot, WrittenType};

impl ExpressionSource {
    /// Enumerates all instructions that this instruction depends on. This includes (maybe compiletime) wires, and submodules.
    pub fn for_each_dependency(&self, func: &mut impl FnMut(FlatID)) {
        match self {
            ExpressionSource::WireRef(wire_ref) => {
                match &wire_ref.root {
                    WireReferenceRoot::LocalDecl(decl_id, _) => func(*decl_id),
                    WireReferenceRoot::NamedConstant(cst) => {
                        for (_id, arg) in &cst.template_args {
                            let Some(arg) = arg else { continue };
                            match &arg.kind {
                                TemplateArgKind::Type(written_type) => {
                                    written_type.for_each_generative_input(func)
                                }
                                TemplateArgKind::Value(uuid) => func(*uuid),
                            }
                        }
                    }
                    WireReferenceRoot::SubModulePort(submod_port) => {
                        func(submod_port.submodule_decl)
                    }
                }
                WireReferencePathElement::for_each_dependency(&wire_ref.path, func);
            }
            &ExpressionSource::UnaryOp { right, .. } => func(right),
            &ExpressionSource::BinaryOp { left, right, .. } => {
                func(left);
                func(right)
            }
            ExpressionSource::Constant(_) => {}
            ExpressionSource::ArrayConstruct(arr) => {
                for v in arr {
                    func(*v);
                }
            }
        }
    }
}

impl WireReferencePathElement {
    pub fn for_each_dependency(path: &[WireReferencePathElement], mut f: impl FnMut(FlatID)) {
        for p in path {
            match p {
                WireReferencePathElement::ArrayAccess {
                    idx,
                    bracket_span: _,
                } => f(*idx),
            }
        }
    }
}

pub fn for_each_generative_input_in_template_args(
    template_args: &TVec<Option<TemplateArg>>,
    f: &mut impl FnMut(FlatID),
) {
    for (_id, t_arg) in template_args.iter_valids() {
        match &t_arg.kind {
            TemplateArgKind::Type(typ) => typ.for_each_generative_input(f),
            TemplateArgKind::Value(val) => f(*val),
        }
    }
}

impl WrittenType {
    pub fn for_each_generative_input(&self, f: &mut impl FnMut(FlatID)) {
        match self {
            WrittenType::Error(_) | WrittenType::TemplateVariable(_, _) => {}
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
