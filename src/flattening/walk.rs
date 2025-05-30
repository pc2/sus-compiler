use crate::typing::template::{TemplateArg, TemplateKind};

use super::{ExpressionSource, WireReferencePathElement, WireReferenceRoot};
use crate::prelude::*;

impl ExpressionSource {
    /// Enumerates all instructions that this instruction depends on. This includes (maybe compiletime) wires, and submodules.
    pub fn for_each_dependency(&self, collect: &mut impl FnMut(FlatID)) {
        match self {
            ExpressionSource::WireRef(wire_ref) => {
                match &wire_ref.root {
                    WireReferenceRoot::LocalDecl(decl_id) => collect(*decl_id),
                    WireReferenceRoot::NamedConstant(cst) => {
                        for (_id, arg) in &cst.template_args {
                            match arg {
                                TemplateKind::Type(TemplateArg::Provided {
                                    arg: wr_typ, ..
                                }) => wr_typ.for_each_generative_input(collect),
                                TemplateKind::Value(TemplateArg::Provided {
                                    arg: value_id,
                                    ..
                                }) => collect(*value_id),
                                TemplateKind::Type(TemplateArg::NotProvided { .. })
                                | TemplateKind::Value(TemplateArg::NotProvided { .. }) => {}
                            }
                        }
                    }
                    WireReferenceRoot::SubModulePort(submod_port) => {
                        collect(submod_port.submodule_decl)
                    }
                    WireReferenceRoot::Error => {}
                }
                WireReferencePathElement::for_each_dependency(&wire_ref.path, collect);
            }
            &ExpressionSource::UnaryOp { right, .. } => collect(right),
            &ExpressionSource::BinaryOp { left, right, .. } => {
                collect(left);
                collect(right)
            }
            ExpressionSource::FuncCall(func_call) => {
                collect(func_call.interface_reference.submodule_decl);
                for arg in &func_call.arguments {
                    collect(*arg)
                }
            }
            ExpressionSource::Constant(_) => {}
            ExpressionSource::ArrayConstruct(arr) => {
                for v in arr {
                    collect(*v);
                }
            }
        }
    }
}

impl WireReferencePathElement {
    pub fn for_each_dependency(path: &[WireReferencePathElement], mut f: impl FnMut(FlatID)) {
        for p in path {
            match p {
                WireReferencePathElement::ArrayAccess { idx, .. } => f(*idx),
            }
        }
    }
}
