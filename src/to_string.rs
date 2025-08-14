use colored::Colorize;
use ibig::IBig;
use sus_proc_macro::kw;

use crate::alloc::zip_eq;
use crate::flattening::typecheck::TyCell;
use crate::instantiation::{InferenceResult, SubModule};
use crate::latency::InferenceFailure;
use crate::latency::port_latency_inference::{
    InferenceCandidate, InferenceTarget, InferenceTargetPath, SubtypeInferencePathElem,
    ValueInferStrategy,
};
use crate::prelude::*;

use crate::typing::abstract_type::{AbstractGlobalReference, AbstractInnerType};
use crate::typing::concrete_type::{ConcreteGlobalReference, SubtypeRelation};
use crate::typing::domain_type::DomainType;
use crate::typing::set_unifier::Unifyable;
use crate::typing::template::{Parameter, TVec, TemplateKind};
use crate::value::Value;
use crate::{file_position::FileText, pretty_print_many_spans};

use crate::flattening::{
    Declaration, DeclarationKind, Direction, DomainInfo, Expression, ExpressionOutput,
    ExpressionSource, ForStatement, FuncCall, GlobalReference, IfStatement, Instruction,
    InterfaceDeclKind, InterfaceDeclaration, InterfaceKind, InterfaceToDomainMap, Module,
    PartSelectDirection, PathElemRefersTo, SliceType, SubModuleInstance, WireReference,
    WireReferencePathElement, WireReferenceRoot, WriteModifiers, WriteTo, WrittenTemplateArg,
    WrittenType,
};
use crate::linker::{FileData, GlobalUUID, LinkInfo, LinkerGlobals};
use crate::typing::{abstract_type::AbstractRankedType, concrete_type::ConcreteType};

use std::fmt::{Display, Formatter};

use std::fmt::Write;
use std::ops::Deref;

impl WrittenType {
    pub fn display<'a>(
        &'a self,
        globals: &'a LinkerGlobals,
        template_names: &'a TVec<Parameter>,
    ) -> impl Display + 'a {
        FmtWrapper(move |f| match self {
            WrittenType::Error(_) => f.write_str("{error}"),
            WrittenType::TemplateVariable(_, id) => f.write_str(&template_names[*id].name),
            WrittenType::Named(named_type) => {
                f.write_str(&globals.types[named_type.id].link_info.get_full_name())
            }
            WrittenType::Array(_, sub) => {
                write!(f, "{}[]", sub.deref().0.display(globals, template_names))
            }
        })
    }
}

impl AbstractRankedType {
    pub fn display<'a>(
        &'a self,
        globals: &'a LinkerGlobals,
        link_info: &'a LinkInfo,
    ) -> impl Display + 'a {
        FmtWrapper(move |f| {
            let res = match &self.inner {
                AbstractInnerType::Unknown(_) => write!(f, "?"),
                AbstractInnerType::Template(id) => f.write_str(&link_info.parameters[*id].name),
                AbstractInnerType::Named(name) => {
                    f.write_fmt(format_args!("{}", name.display(globals, link_info)))
                }
                AbstractInnerType::Interface(md_id, interface_id) => {
                    let md = &globals.modules[md_id.id];
                    f.write_fmt(format_args!(
                        "Interface {} of {}",
                        md.interfaces[*interface_id].name,
                        md_id.display(globals, link_info)
                    ))
                }
                AbstractInnerType::LocalInterface(local_interface) => f.write_fmt(format_args!(
                    "Local Interface '{}'",
                    link_info.instructions[*local_interface]
                        .unwrap_interface()
                        .name,
                )),
            };
            res?;
            // Print PeanoType rank using its custom Display impl
            write!(f, "{}", PeanoTypeDisplay(&self.rank))
        })
    }
}
impl TyCell<AbstractRankedType> {
    pub fn display<'a>(
        &'a self,
        globals: &'a LinkerGlobals,
        link_info: &'a LinkInfo,
    ) -> impl Display + 'a {
        FmtWrapper(move |f| {
            if let Some(slf) = self.get_maybe() {
                slf.display(globals, link_info).fmt(f)
            } else {
                f.write_str("?")
            }
        })
    }
}

// Helper wrapper for PeanoType display
struct PeanoTypeDisplay<'a>(&'a crate::typing::abstract_type::PeanoType);
impl<'a> std::fmt::Display for PeanoTypeDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cur = self.0;
        loop {
            match cur {
                crate::typing::abstract_type::PeanoType::Zero => return Ok(()),
                crate::typing::abstract_type::PeanoType::Succ(inner) => {
                    f.write_str("[]")?;
                    cur = inner;
                }
                crate::typing::abstract_type::PeanoType::Unknown(_) => {
                    write!(f, "[...]")?;
                    return Ok(());
                }
            }
        }
    }
}

impl<ID: Into<GlobalUUID> + Copy> AbstractGlobalReference<ID> {
    pub fn display<'a>(
        &'a self,
        globals: &'a LinkerGlobals,
        link_info: &'a LinkInfo,
    ) -> impl Display + 'a {
        FmtWrapper(move |f| {
            let target_link_info: &LinkInfo = &globals[self.id.into()];
            f.write_str(&target_link_info.name)?;
            if !self.template_arg_types.iter().any(|(_, t)| match t {
                TemplateKind::Type(_) => true,
                TemplateKind::Value(_) => false,
            }) {
                return Ok(());
            }
            f.write_str(" #(")?;
            let args_iter = zip_eq(&self.template_arg_types, &target_link_info.parameters);
            join_string_iter_formatter(", ", f, args_iter, |(_, typ, param), f| {
                write!(f, "{}: ", &param.name)?;
                match typ {
                    TemplateKind::Type(typ) => {
                        f.write_fmt(format_args!("type {}", typ.display(globals, link_info)))
                    }
                    TemplateKind::Value(()) => f.write_char('_'),
                }
            })?;
            f.write_str(")")
        })
    }
}

impl<ID: Into<GlobalUUID> + Copy> GlobalReference<ID> {
    pub fn display<'a>(
        &'a self,
        globals: &'a LinkerGlobals,
        link_info: &'a LinkInfo,
    ) -> impl Display + 'a {
        FmtWrapper(move |f| {
            let target_link_info: &LinkInfo = &globals[self.id.into()];
            f.write_str(&target_link_info.name)?;
            f.write_str(" #(")?;
            join_string_iter_formatter(
                ", ",
                f,
                &self.template_args,
                |WrittenTemplateArg {
                     name,
                     refers_to,
                     kind,
                     ..
                 },
                 f| {
                    write!(f, "{name} -> ")?;
                    if let Some(found) = refers_to.get() {
                        write!(f, "{}: ", &target_link_info.parameters[*found].name)?;
                    } else {
                        f.write_str("?: ")?;
                    }
                    match kind {
                        Some(TemplateKind::Type(wr_typ)) => {
                            write!(f, "type {}", wr_typ.display(globals, &link_info.parameters))
                        }
                        Some(TemplateKind::Value(v_id)) => write!(f, "{v_id:?}"),
                        None => f.write_str("INVALID"),
                    }
                },
            )?;
            f.write_str(")")
        })
    }
}

impl WireReference {
    pub fn display<'s>(
        &'s self,
        globals: &'s LinkerGlobals,
        link_info: &'s LinkInfo,
    ) -> impl Display + 's {
        FmtWrapper(|f| {
            match &self.root {
                WireReferenceRoot::LocalDecl(decl_id)
                | WireReferenceRoot::LocalSubmodule(decl_id)
                | WireReferenceRoot::LocalInterface(decl_id) => {
                    let decl_name = link_info.debug_name(*decl_id);
                    write!(f, "{decl_name}")?
                }
                WireReferenceRoot::NamedConstant(global_reference) => {
                    write!(f, "{}", global_reference.display(globals, link_info))?;
                }
                WireReferenceRoot::NamedModule(global_reference) => {
                    write!(f, "{}", global_reference.display(globals, link_info))?;
                }
                WireReferenceRoot::Error => write!(f, "{{error}}")?,
            }
            for p in &self.path {
                match p {
                    WireReferencePathElement::FieldAccess {
                        name, refers_to, ..
                    } => {
                        write!(f, ".{name}")?;
                        match refers_to.get() {
                            Some(PathElemRefersTo::Interface(md_id, interface)) => {
                                let md = &globals[*md_id];
                                let md_name = md.link_info.get_full_name();
                                if let Some(interface) = interface {
                                    let interf_name = &md.interfaces[*interface].name;
                                    write!(f, "({md_name}:{interf_name})")?;
                                } else {
                                    write!(f, "({md_name}:?)")?;
                                }
                            }
                            None => write!(f, "?")?,
                        }
                    }
                    WireReferencePathElement::ArrayAccess { idx, .. } => {
                        write!(f, "[{idx:?}]")?;
                    }
                    WireReferencePathElement::ArraySlice { from, to, .. } => match (from, to) {
                        (None, None) => write!(f, "[:]")?,
                        (None, Some(to)) => write!(f, "[:{to:?}]")?,
                        (Some(from), None) => write!(f, "[{from:?}:]")?,
                        (Some(from), Some(to)) => write!(f, "[{from:?}:{to:?}]")?,
                    },
                    WireReferencePathElement::ArrayPartSelect {
                        from,
                        width,
                        direction,
                        ..
                    } => {
                        write!(f, "[{from:?}{direction}{width:?}]")?;
                    }
                }
            }
            Ok(())
        })
    }
}

impl ConcreteType {
    pub fn display<'a>(&'a self, globals: &'a LinkerGlobals) -> impl Display + 'a {
        FmtWrapper(move |f| match self {
            ConcreteType::Named(global_ref) => {
                // Avoid ambiguity: call display() directly on ConcreteGlobalReference
                ConcreteGlobalReference::display(global_ref, globals).fmt(f)
            }
            ConcreteType::Array(arr_box) => {
                let (elem_typ, arr_size) = arr_box.deref();
                write!(f, "{}[", elem_typ.display(globals))?;
                // arr_size is Unifyable<Value, ...>, which implements Display for Unifyable, not Value
                match arr_size {
                    Unifyable::Set(val) => {
                        // Value does not implement Display, so use Debug
                        write!(f, "{val}")?;
                    }
                    Unifyable::Unknown(_) => {
                        write!(f, "_")?;
                    }
                }
                f.write_str("]")
            }
        })
    }
}

impl<ID: Into<GlobalUUID> + Copy> ConcreteGlobalReference<ID> {
    pub fn display<'v>(&'v self, globals: &'v LinkerGlobals) -> impl Display + 'v {
        let target_link_info = &globals[self.id.into()];
        FmtWrapper(move |f| {
            assert!(self.template_args.len() == target_link_info.parameters.len());
            let object_full_name = target_link_info.get_full_name();
            f.write_str(&object_full_name)?;
            if self.template_args.is_empty() {
                return f.write_str(" #()");
            } else {
                f.write_str(" #(")?;
            }
            let mut is_first = true;
            for (_id, arg, arg_in_target) in
                zip_eq(&self.template_args, &target_link_info.parameters)
            {
                if !is_first {
                    f.write_str(", ")?;
                }
                is_first = false;
                f.write_fmt(format_args!("{}: ", arg_in_target.name))?;
                match arg {
                    TemplateKind::Type(typ_arg) => {
                        f.write_fmt(format_args!("type {}", typ_arg.display(globals)))?;
                    }
                    TemplateKind::Value(v) => match v {
                        Unifyable::Set(value) => write!(f, "{value}")?,
                        Unifyable::Unknown(_) => f.write_char('?')?,
                    },
                }
            }
            f.write_char(')')
        })
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => b.fmt(f),
            Value::Integer(i) => i.fmt(f),
            Value::Array(arr_box) => {
                f.write_str("[")?;
                join_string_iter_formatter(", ", f, arr_box.iter(), |v, f| v.fmt(f))?;
                f.write_str("]")
            }
            Value::Unset => f.write_str("{value_unset}"),
        }
    }
}

impl DomainID {
    pub fn display<'d>(
        &'d self,
        domains: &'d FlatAlloc<DomainInfo, DomainIDMarker>,
    ) -> impl Display + 'd {
        FmtWrapper(move |f| {
            if let Some(physical_domain) = domains.get(*self) {
                f.write_fmt(format_args!("{{{}}}", physical_domain.name))
            } else {
                f.write_fmt(format_args!(
                    "{{unnamed domain {}}}",
                    self.get_hidden_value()
                ))
            }
        })
    }
}

impl DomainType {
    pub fn display<'d>(
        &'d self,
        domains: &'d FlatAlloc<DomainInfo, DomainIDMarker>,
    ) -> impl Display + 'd {
        FmtWrapper(move |f| match self {
            DomainType::Generative => f.write_str("gen"),
            DomainType::Physical(physical_id) => physical_id.display(domains).fmt(f),
            DomainType::Unknown(_) => unreachable!(),
        })
    }
    pub fn debug<'d>(
        &'d self,
        domains: &'d FlatAlloc<DomainInfo, DomainIDMarker>,
    ) -> impl Display + 'd {
        FmtWrapper(move |f| match self {
            DomainType::Generative => f.write_str("gen"),
            DomainType::Physical(physical_id) => write!(f, "{}", physical_id.display(domains)),
            DomainType::Unknown(unknown_id) => write!(f, "{{{unknown_id:?}}}"),
        })
    }
}

impl Display for WriteModifiers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteModifiers::Connection { num_regs, .. } => {
                for _ in 0..*num_regs {
                    f.write_str("reg ")?;
                }
                Ok(())
            }
            WriteModifiers::Initial { .. } => f.write_str("initial"),
        }
    }
}

impl core::fmt::Debug for DeclarationKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeclarationKind::RegularWire {
                is_state,
                read_only,
            } => {
                if *is_state {
                    f.write_str("state ")?;
                }
                if *read_only {
                    f.write_str("read_only ")?;
                }
                f.write_str("wire")
            }
            DeclarationKind::StructField(field_id) => write!(f, "field({field_id:?})"),
            DeclarationKind::Port {
                direction,
                is_state,
                port_id,
                parent_interface,
                is_standalone_port: _,
            } => {
                if *is_state {
                    f.write_str("state")?;
                }
                write!(f, "{direction}({port_id:?}@{parent_interface:?})")
            }
            DeclarationKind::ConditionalBinding {
                when_id,
                direction,
                is_state,
            } => {
                if *is_state {
                    f.write_str("state")?;
                }
                write!(f, "{direction} binding(in when {when_id:?})")
            }
            DeclarationKind::RegularGenerative { read_only } => {
                if *read_only {
                    f.write_str("read_only ")?;
                }
                write!(f, "gen")
            }
            DeclarationKind::TemplateParameter(template_id) => {
                write!(f, "gen param({template_id:?})")
            }
        }
    }
}
impl core::fmt::Debug for InterfaceKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RegularInterface => write!(f, "interface"),
            Self::Action(port) => write!(f, "action({port:?})"),
            Self::Trigger(port) => write!(f, "trigger({port:?})"),
        }
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Direction::Input => "input",
            Direction::Output => "output",
        })
    }
}
impl Display for PartSelectDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PartSelectDirection::Up => "+:",
            PartSelectDirection::Down => "-:",
        })
    }
}
impl SliceType {
    pub fn from_kind_id(kind_id: u16) -> Self {
        match kind_id {
            kw!(":") => SliceType::Normal,
            kw!("+:") => SliceType::PartSelect(PartSelectDirection::Up),
            kw!("-:") => SliceType::PartSelect(PartSelectDirection::Down),
            _ => unreachable!(),
        }
    }
}
impl Display for SliceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SliceType::Normal => ":",
            SliceType::PartSelect(PartSelectDirection::Up) => "+:",
            SliceType::PartSelect(PartSelectDirection::Down) => "-:",
        })
    }
}

/// port: int#(MIN: {*})
impl InferenceTargetPath {
    pub fn display(&self, md: &Module, linker: &Linker) -> impl Display {
        FmtWrapper(|f| {
            let port = &md.ports[self.port];
            let port_decl =
                md.link_info.instructions[port.declaration_instruction].unwrap_declaration();

            fn recurse_print(
                f: &mut std::fmt::Formatter<'_>,
                linker: &Linker,
                li: &LinkInfo,
                typ: &WrittenType,
                path: &[SubtypeInferencePathElem],
            ) -> std::fmt::Result {
                if let Some((cur_elem, rest)) = path.split_first() {
                    match cur_elem {
                        SubtypeInferencePathElem::DownArray => {
                            let_unwrap!(WrittenType::Array(_, arr_box), typ);
                            let (content, _sz, _) = arr_box.deref();
                            recurse_print(f, linker, li, content, rest)?;
                            f.write_str("[]")
                        }
                        SubtypeInferencePathElem::ArraySize => {
                            let_unwrap!(WrittenType::Array(_, arr_box), typ);
                            let (content, _sz, _) = arr_box.deref();
                            recurse_print(f, linker, li, content, rest)?;
                            f.write_str("[{*}]")
                        }
                        SubtypeInferencePathElem::InNamed(arg_id) => {
                            let_unwrap!(WrittenType::Named(named), typ);
                            let named_type = &linker.types[named.id];
                            let named_name = &named_type.link_info.name;
                            let param_name = &named_type.link_info.parameters[*arg_id].name;

                            write!(f, "{named_name} #({param_name}: ")?;
                            match &named.get_arg_for(*arg_id).unwrap().kind.as_ref().unwrap() {
                                TemplateKind::Type(t) => recurse_print(f, linker, li, t, rest)?,
                                TemplateKind::Value(_) => {
                                    assert!(rest.is_empty());
                                    f.write_str("{*}")?
                                }
                            }
                            f.write_str(")")
                        }
                    }
                } else {
                    write!(f, "{}", typ.display(&linker.globals, &li.parameters))
                }
            }

            recurse_print(f, linker, &md.link_info, &port_decl.typ_expr, &self.path)?;
            write!(f, " {}", &port_decl.name)
        })
    }
}

impl InferenceCandidate {
    /// V * 5 + 3 <= {*} in int#(FROM: {*}) port
    /// V * 5 + 3 <= {t} - {f} in a'{t}, b'{f}
    pub fn display(&self, candidate_name: &str, md: &Module, linker: &Linker) -> impl Display {
        FmtWrapper(|f| {
            let relation = match self.relation {
                SubtypeRelation::Exact => "==",
                SubtypeRelation::Min => "<=",
                SubtypeRelation::Max => ">=",
            };
            f.write_str(candidate_name)?;
            if self.mul_by != IBig::from(1) {
                write!(f, " * {}", self.mul_by)?;
            }
            if self.offset != IBig::from(0) {
                if self.offset < IBig::from(0) {
                    write!(f, " - {}", -&self.offset)?;
                } else {
                    write!(f, " + {}", self.offset)?;
                }
            }
            write!(f, " {relation} ")?;

            match &self.target {
                InferenceTarget::Subtype(path) => {
                    let path = path.display(md, linker);
                    write!(f, "{{*}} in {path}")
                }
                InferenceTarget::PortLatency { from, to } => {
                    let from = &md.ports[*from];
                    let to = &md.ports[*to];
                    write!(f, "{}'{{*}} - {}'{{*}}", from.name, to.name)
                }
            }
        })
    }
}

impl Display for InferenceResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InferenceResult::PortNotUsed => f.write_str("N/C"),
            InferenceResult::NotFound => f.write_str("?"),
            InferenceResult::LatencyError(InferenceFailure::BadProblem) => {
                f.write_str("? bad problem")
            }
            InferenceResult::LatencyError(InferenceFailure::NotReached) => {
                f.write_str("? not reached")
            }
            InferenceResult::LatencyError(InferenceFailure::Poison) => f.write_str("? poisoned"),
            InferenceResult::Found(v) => write!(f, "{v}"),
        }
    }
}

pub fn display_infer_param_info(
    linker: &Linker,
    md: &Module,
    template_id: TemplateID,
    final_values: Option<&Vec<InferenceResult>>,
) -> impl Display {
    FmtWrapper(move |f| {
        let arg_name = &md.link_info.parameters[template_id].name;
        match &md.inference_info.parameter_inference_candidates[template_id] {
            TemplateKind::Type(t_info) => {
                if t_info.candidates.is_empty() {
                    writeln!(f, "{arg_name} has no inference candidates")?;
                } else {
                    writeln!(f, "{arg_name} can be inferred from:")?;
                }
                for (idx, c) in t_info.candidates.iter().enumerate() {
                    let relation = if idx < t_info.num_inputs { "<:" } else { "=" };
                    let path = c.display(md, linker);
                    writeln!(f, "{{*}} {relation} {arg_name} in {path}")?;
                }
            }
            TemplateKind::Value(v_info) => {
                let (can_infer, cant_infer) =
                    v_info.candidates.split_at(v_info.total_inference_upto);
                if can_infer.is_empty() {
                    writeln!(f, "{arg_name} has no acceptable inference candidates")?;
                } else {
                    match v_info.total_inference_strategy {
                        ValueInferStrategy::Unify | ValueInferStrategy::Exact => writeln!(
                            f,
                            "{arg_name} can be inferred if at least one of the following constraint resolves:"
                        )?,
                        ValueInferStrategy::Min => writeln!(
                            f,
                            "{arg_name} can be inferred as an integer value that is as high as possible, without violating any of the following constraints:"
                        )?,
                        ValueInferStrategy::Max => writeln!(
                            f,
                            "{arg_name} can be inferred as an integer value that is as low as possible, without violating any of the following constraints:"
                        )?,
                    }
                }
                for (idx, c) in can_infer.iter().enumerate() {
                    write!(f, "- {}", c.display(arg_name, md, linker))?;
                    if let Some(values_list) = final_values
                        && let Some(final_value) = values_list.get(idx)
                    {
                        write!(f, "  ({{*}} = {final_value})")?;
                    }
                    writeln!(f)?;
                }
                if !cant_infer.is_empty() {
                    writeln!(
                        f,
                        "The following constraints were found, but aren't used for inference here"
                    )?;
                    for c in cant_infer {
                        writeln!(f, "- {}", c.display(arg_name, md, linker))?;
                    }
                }
            }
        }
        Ok(())
    })
}

pub fn display_all_infer_params(linker: &Linker, sm: &SubModule) -> impl Display {
    FmtWrapper(|f| {
        let md = &linker.modules[sm.refers_to.id];
        for (template_id, known_values) in sm.last_infer_values.borrow().iter() {
            display_infer_param_info(linker, md, template_id, Some(known_values)).fmt(f)?;
        }
        Ok(())
    })
}

impl LinkInfo {
    fn debug_name(&self, instr_id: FlatID) -> impl Display + '_ {
        let name = self.get_instruction_name(instr_id).unwrap();
        FmtWrapper(move |f| write!(f, "{instr_id:?}={name}"))
    }
    fn display_domain_of<'s>(
        &'s self,
        instr_id: FlatID,
        domains: &'s FlatAlloc<DomainInfo, DomainIDMarker>,
    ) -> impl Display + 's {
        let domain = self.get_instruction_domain(instr_id);
        FmtWrapper(move |f| {
            if let Some(domain) = domain {
                write!(f, "{}", domain.debug(domains))
            } else {
                Ok(())
            }
        })
    }
    pub fn print_instructions(
        &self,
        domains: &FlatAlloc<DomainInfo, DomainIDMarker>,
        file_data: &FileData,
        globals: &LinkerGlobals,
    ) {
        let mut spans_print = Vec::new();
        for (id, instr) in &self.instructions {
            let parent = FmtWrapper(|f| {
                if let Some(p) = instr.get_parent_condition() {
                    let p_when = p.parent_when;
                    if p.is_else_branch {
                        write!(f, "parent: !{p_when:?}")
                    } else {
                        write!(f, "parent: {p_when:?}")
                    }
                } else {
                    f.write_str("no parent when")
                }
            });
            let domain = self.display_domain_of(id, domains);
            print!("{id:?}: {parent} {domain} ");
            match instr {
                Instruction::SubModule(SubModuleInstance {
                    module_ref,
                    name,
                    local_domain_map,
                    typ: _,
                    ..
                }) => {
                    let name = name.green();
                    print!("{} {name}", module_ref.display(globals, self));
                    let submod_domains = &globals[module_ref.id].domains;
                    if let Some(local_domain_map) = local_domain_map.get() {
                        print!("[");
                        join_string_iter_print(
                            ", ",
                            local_domain_map,
                            |(submod_domain, domain_here)| {
                                let submod_domain = submod_domain.display(submod_domains);
                                let domain_here = domain_here.unwrap_physical();
                                let domain_here = domain_here.display(domains);
                                print!(".{submod_domain} = {domain_here}");
                            },
                        );
                        print!("]");
                    }
                }
                Instruction::Declaration(Declaration {
                    typ_expr,
                    typ,
                    name,
                    decl_kind,
                    latency_specifier,
                    ..
                }) => {
                    let typ_expr = typ_expr.display(globals, &self.parameters);
                    let name = name.green();
                    let typ = typ.display(globals, self);
                    print!("{decl_kind:?} {typ_expr} ({typ}) {name}");
                    if let Some(lat_spec) = latency_specifier {
                        print!("'{lat_spec:?}");
                    }
                }
                Instruction::Expression(Expression { source, output, .. }) => {
                    match output {
                        ExpressionOutput::SubExpression(typ) => {
                            let typ = typ.display(globals, self);
                            print!("({typ})")
                        }
                        ExpressionOutput::MultiWrite(write_tos) => {
                            print!("(");
                            join_string_iter_print(
                                ", ",
                                write_tos,
                                |WriteTo {
                                     to,
                                     write_modifiers,
                                     target_domain,
                                     ..
                                 }| {
                                    let target_domain = target_domain.get();
                                    let target_domain = target_domain.debug(domains);
                                    let to = to.display(globals, self);
                                    print!("{target_domain} {write_modifiers} {to}")
                                },
                            );
                            print!(") = ");
                        }
                    }
                    match source {
                        ExpressionSource::WireRef(wire_reference) => {
                            print!("{}", wire_reference.display(globals, self));
                        }
                        ExpressionSource::FuncCall(FuncCall {
                            func_wire_ref,
                            arguments,
                            ..
                        }) => {
                            print!("{func_wire_ref:?}(");
                            join_string_iter_print(", ", arguments, |arg| {
                                print!("{arg:?}");
                            });
                            print!(")");
                        }
                        ExpressionSource::UnaryOp { op, right, rank: _ } => {
                            print!("{op}{right:?}");
                        }
                        ExpressionSource::BinaryOp {
                            op,
                            left,
                            right,
                            rank: _,
                        } => print!("{left:?} {op} {right:?}"),
                        ExpressionSource::ArrayConstruct(elements) => {
                            print!("[");
                            join_string_iter_print(", ", elements, |elem| {
                                print!("{elem:?}");
                            });
                            print!("]");
                        }
                        ExpressionSource::Literal(value) => {
                            print!("literal {value}")
                        }
                    }
                }
                Instruction::Interface(InterfaceDeclaration {
                    name,
                    latency_specifier,
                    is_local,
                    interface_id,
                    interface_kind,
                    inputs,
                    outputs,
                    then_block,
                    else_block,
                    ..
                }) => {
                    let is_local = if *is_local { "local " } else { "" };
                    let name = name.green();
                    print!("{is_local} {interface_kind:?} {interface_id:?} {name}");
                    if let Some(lat_spec) = latency_specifier {
                        print!("'{lat_spec:?}");
                    }
                    if !inputs.is_empty() | !outputs.is_empty() {
                        print!(": {inputs:?} -> {outputs:?}");
                    }
                    print!(" {{{then_block:?}}} else {{{else_block:?}}}");
                }
                Instruction::IfStatement(IfStatement {
                    condition,
                    is_generative,
                    then_block,
                    else_block,
                    bindings_read_only,
                    bindings_writable,
                    ..
                }) => {
                    let kw = if *is_generative { "if" } else { "when" };
                    print!("{kw} {condition:?} ");
                    if !bindings_read_only.is_empty() | !bindings_writable.is_empty() {
                        print!(": {bindings_read_only:?} -> {bindings_writable:?}");
                    }
                    print!(" {{{then_block:?}}} else {{{else_block:?}}}");
                }
                Instruction::ForStatement(ForStatement {
                    loop_var_decl,
                    start,
                    end,
                    loop_body,
                    ..
                }) => {
                    let loop_var_decl_name = self.debug_name(*loop_var_decl);
                    print!("for {loop_var_decl_name} in {start:?}..{end:?} {{{loop_body:?}}}")
                }
            }
            println!();
            let span = self.get_instruction_span(id);
            spans_print.push((format!("{id:?} {domain}"), span));
        }
        pretty_print_many_spans(file_data, &spans_print);
    }
}

impl Module {
    pub fn make_port_info_fmt(
        &self,
        decl: &Declaration,
        file_text: &FileText,
        result: &mut String,
    ) {
        let_unwrap!(
            DeclarationKind::Port {
                direction,
                is_state,
                ..
            },
            decl.decl_kind
        );
        result.write_fmt(format_args!("{direction} ")).unwrap();
        if is_state {
            result.write_str("state ").unwrap();
        }

        result
            .write_str(&file_text[decl.typ_expr.get_span()])
            .unwrap();

        result.write_char(' ').unwrap();

        result.write_str(&decl.name).unwrap();

        if let Some(lat_spec) = decl.latency_specifier {
            result.write_char('\'').unwrap();

            let lat_spec_expr = self.link_info.instructions[lat_spec].unwrap_expression();
            result.write_str(&file_text[lat_spec_expr.span]).unwrap();
        }
        result.write_char('\n').unwrap();
    }

    pub fn make_interface_info_fmt(
        &self,
        interface: &InterfaceDeclaration,
        file_text: &FileText,
        may_print_domain: bool,
        result: &mut String,
    ) {
        if may_print_domain {
            result
                .write_fmt(format_args!(
                    "{{{}}} ",
                    self.domains[interface.domain.unwrap_physical()].name
                ))
                .unwrap();
        }
        result.write_str(&file_text[interface.decl_span]).unwrap();
        result.write_str(":\n").unwrap();
        for decl_id in &interface.inputs {
            let port_decl = self.link_info.instructions[*decl_id].unwrap_declaration();
            result.write_str("\t").unwrap();
            self.make_port_info_fmt(port_decl, file_text, result);
        }
        if !interface.outputs.is_empty() {
            result.write_str("\t->\n").unwrap();
            for decl_id in &interface.outputs {
                let port_decl = self.link_info.instructions[*decl_id].unwrap_declaration();
                result.write_str("\t").unwrap();
                self.make_port_info_fmt(port_decl, file_text, result);
            }
        }
    }

    pub fn make_all_ports_info_string(
        &self,
        file_text: &FileText,
        local_domains_used_in_parent_module: Option<InterfaceToDomainMap>,
    ) -> String {
        let full_name_with_args = self.link_info.get_full_name_and_template_args(file_text);
        let mut result = format!("module {full_name_with_args}:\n");

        for (domain_id, domain) in &self.domains {
            let name = &domain.name;
            if let Some(domain_map) = &local_domains_used_in_parent_module {
                let submod_name = &self.link_info.name;
                let name_in_parent =
                    domain_map.local_domain_map[domain_id].debug(domain_map.domains);
                writeln!(result, "domain {submod_name}.{name} = {name_in_parent}").unwrap();
            } else {
                writeln!(result, "domain {name}:").unwrap();
            }

            for (_, interface) in &self.interfaces {
                match interface.declaration_instruction {
                    Some(InterfaceDeclKind::Interface(decl_id)) => {
                        let interface = self.link_info.instructions[decl_id].unwrap_interface();
                        if interface.domain.unwrap_physical() == domain_id {
                            self.make_interface_info_fmt(interface, file_text, false, &mut result);
                        }
                    }
                    Some(InterfaceDeclKind::SinglePort(decl_id)) => {
                        let single_port = self.link_info.instructions[decl_id].unwrap_declaration();
                        if single_port.domain.get().unwrap_physical() == domain_id {
                            self.make_port_info_fmt(single_port, file_text, &mut result);
                        }
                    }
                    None => {}
                }
            }
        }

        result.pop().unwrap();

        result
    }

    pub fn print_flattened_module(&self, file_data: &FileData, globals: &LinkerGlobals) {
        println!("[[{}]]:", self.link_info.name);
        println!("Interface:");
        println!(
            "{}",
            self.make_all_ports_info_string(&file_data.file_text, None)
        );
        println!("Instructions:");
        self.link_info
            .print_instructions(&self.domains, file_data, globals);
    }
}

pub fn join_string_iter<T>(
    result: &mut String,
    sep: &'static str,
    iter: impl IntoIterator<Item = T>,
    mut f: impl FnMut(&mut String, T),
) {
    let mut iter = iter.into_iter();
    if let Some(first) = iter.next() {
        f(result, first);
        for item in iter {
            result.write_str(sep).unwrap();
            f(result, item);
        }
    }
}

pub fn join_string_iter_formatter<'fmt, T>(
    sep: &str,
    f: &mut Formatter<'fmt>,
    iter: impl IntoIterator<Item = T>,
    mut func: impl FnMut(T, &mut Formatter<'fmt>) -> std::fmt::Result,
) -> std::fmt::Result {
    let mut iter = iter.into_iter();
    if let Some(first) = iter.next() {
        func(first, f)?;
        for item in iter {
            f.write_str(sep)?;
            func(item, f)?;
        }
    }
    Ok(())
}

pub fn join_string_iter_print<T>(
    sep: &'static str,
    iter: impl IntoIterator<Item = T>,
    mut f: impl FnMut(T),
) {
    let mut iter = iter.into_iter();
    if let Some(first) = iter.next() {
        f(first);
        for item in iter {
            print!("{sep}");
            f(item);
        }
    }
}

pub fn trim_known_prefix<'a>(in_str: &'a str, prefix: &str) -> &'a str {
    assert_eq!(&in_str[..prefix.len()], prefix);
    &in_str[prefix.len()..]
}

pub struct FmtWrapper<F: Fn(&mut Formatter<'_>) -> std::fmt::Result>(pub F);

impl<F: Fn(&mut Formatter<'_>) -> std::fmt::Result> Display for FmtWrapper<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (self.0)(f)
    }
}
