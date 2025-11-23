use colored::Colorize;
use ibig::IBig;
use sus_proc_macro::kw;

use crate::alloc::zip_eq;
use crate::flattening::typecheck::TyCell;
use crate::instantiation::{
    InferenceResult, InstantiatedModule, IsPort, ModuleTypingContext, MultiplexerSource,
    PartialBound, RealWire, RealWireDataSource, RealWirePathElem, SubModule, SubModuleOrWire,
};
use crate::latency::port_latency_inference::{
    InferenceCandidate, InferenceTarget, InferenceTargetPath, SubtypeInferencePathElem,
    ValueInferStrategy,
};
use crate::prelude::*;

use crate::typing::abstract_type::{AbstractGlobalReference, AbstractInnerType};
use crate::typing::concrete_type::{ConcreteGlobalReference, SubtypeRelation};
use crate::typing::domain_type::DomainType;
use crate::typing::set_unifier::Unifyable;
use crate::typing::template::{
    GenerativeParameterKind, Parameter, TVec, TemplateKind, TypeParameterKind,
};
use crate::typing::value_unifier::UnifyableValue;
use crate::value::Value;
use crate::{file_position::FileText, pretty_print_many_spans};

use crate::flattening::*;
use crate::linker::{FileData, GlobalUUID, IsExtern, LinkInfo, LinkerGlobals};
use crate::typing::{abstract_type::AbstractRankedType, concrete_type::ConcreteType};

use std::fmt::{Display, Formatter, Write};

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
            WrittenType::Named(named_type) => globals.types[named_type.id]
                .link_info
                .display_full_name()
                .fmt(f),
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
                    write!(f, "{}", name.display(globals, link_info))
                }
                AbstractInnerType::Interface(md_id, interface_id) => {
                    let md = &globals.modules[md_id.id];
                    write!(
                        f,
                        "Interface {} of {}",
                        md.interfaces[*interface_id].name,
                        md_id.display(globals, link_info)
                    )
                }
                AbstractInnerType::LocalInterface(local_interface) => write!(
                    f,
                    "Local Interface '{}'",
                    link_info.instructions[*local_interface]
                        .unwrap_interface()
                        .name,
                ),
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

impl LinkInfo {
    pub fn display_full_name(&self) -> impl Display {
        // Feelin iffy about namespaces, so just return self.name
        &self.name
        // format!("::{}", self.name)
    }
    pub fn display_full_name_and_args<'s>(&'s self, file_text: &'s FileText) -> impl Display + 's {
        self.display_with_template_args(&self.parameters, |f, (_, t)| match &t.kind {
            TemplateKind::Type(TypeParameterKind {}) => f.write_str(&t.name),
            TemplateKind::Value(GenerativeParameterKind {
                decl_span,
                declaration_instruction: _,
            }) => f.write_str(&file_text[*decl_span]),
        })
    }
    pub fn display_with_template_args<'s, T: 's, Iter: Iterator<Item = T> + Clone + 's>(
        &'s self,
        iter: impl IntoIterator<Item = T, IntoIter = Iter> + 's,
        func: impl Fn(&mut Formatter<'_>, T) -> std::fmt::Result + 's,
    ) -> impl Display + 's {
        let template_args = display_join(", ", iter, func);
        FmtWrapper(move |f| {
            let full_name = self.display_full_name();

            write!(f, "{full_name} #({template_args})")
        })
    }
}

impl<ID: Into<GlobalUUID> + Copy> GlobalReference<ID> {
    pub fn display<'a>(
        &'a self,
        globals: &'a LinkerGlobals,
        link_info: &'a LinkInfo,
    ) -> impl Display + 'a {
        let target_link_info: &LinkInfo = &globals[self.id.into()];
        target_link_info.display_with_template_args(
            &self.template_args,
            |f,
             WrittenTemplateArg {
                 name,
                 refers_to,
                 kind,
                 ..
             }| {
                write!(f, "{name} -> ")?;
                if let Some(found) = refers_to.get() {
                    write!(f, "{}: ", &target_link_info.parameters[*found].name)?;
                } else {
                    write!(f, "?: ")?;
                }
                match kind {
                    Some(TemplateKind::Type(wr_typ)) => {
                        write!(f, "type {}", wr_typ.display(globals, &link_info.parameters))
                    }
                    Some(TemplateKind::Value(v_id)) => write!(f, "{v_id:?}"),
                    None => write!(f, "INVALID"),
                }
            },
        )
    }
}
impl<ID: Into<GlobalUUID> + Copy> AbstractGlobalReference<ID> {
    pub fn display<'a>(
        &'a self,
        globals: &'a LinkerGlobals,
        link_info: &'a LinkInfo,
    ) -> impl Display + 'a {
        let target_link_info: &LinkInfo = &globals[self.id.into()];

        target_link_info.display_with_template_args(
            zip_eq(&self.template_arg_types, &target_link_info.parameters),
            |f, (_, typ, param)| {
                write!(f, "{}: ", &param.name)?;
                match typ {
                    TemplateKind::Type(typ) => {
                        write!(f, "type {}", typ.display(globals, link_info))
                    }
                    TemplateKind::Value(()) => write!(f, "_"),
                }
            },
        )
    }
}
impl<ID: Into<GlobalUUID> + Copy> ConcreteGlobalReference<ID> {
    pub fn display<'v>(&'v self, globals: &'v LinkerGlobals) -> impl Display + 'v {
        let target_link_info: &LinkInfo = &globals[self.id.into()];
        assert!(self.template_args.len() == target_link_info.parameters.len());

        target_link_info.display_with_template_args(
            zip_eq(&self.template_args, &target_link_info.parameters),
            |f, (_id, arg, arg_in_target)| {
                write!(f, "{}: ", &arg_in_target.name)?;
                match arg {
                    TemplateKind::Type(typ_arg) => {
                        write!(f, "type {}", typ_arg.display(globals))
                    }
                    TemplateKind::Value(Unifyable::Set(value)) => write!(f, "{value}"),
                    TemplateKind::Value(Unifyable::Unknown(_)) => write!(f, "?"),
                }
            },
        )
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
                    let decl_name = link_info.debug_name(globals, *decl_id);
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
                                let md_name = md.link_info.display_full_name();
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

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => b.fmt(f),
            Value::Integer(i) => i.fmt(f),
            Value::Float(fl32) => {
                let mut buf = dtoa::Buffer::new();
                write!(f, "{}", buf.format(f32::from(*fl32)))
            }
            Value::Double(fl64) => {
                let mut buf = dtoa::Buffer::new();
                write!(f, "{}", buf.format(f64::from(*fl64)))
            }
            Value::String(text) => {
                write!(f, "\"{}\"", text.escape_default())
            }
            Value::Array(elements) => {
                if elements.iter().all(|e| matches!(e, Value::Bool(_))) {
                    write!(f, "{}'b", elements.len())?;
                    for e in elements.iter().rev() {
                        f.write_char(if e.unwrap_bool() { '1' } else { '0' })?;
                    }
                } else {
                    let content = display_join(", ", elements.iter(), |f, v| v.fmt(f));
                    write!(f, "[{content}]")?;
                }
                Ok(())
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
                write!(f, "{{{}}}", physical_domain.name)
            } else {
                write!(f, "{{unnamed domain {}}}", self.get_hidden_value())
            }
        })
    }
}

impl Display for IsExtern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            IsExtern::Normal => "non-extern",
            IsExtern::Extern => "extern",
            IsExtern::Builtin => "__builtin__",
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
impl Display for InterfaceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            InterfaceKind::RegularInterface => "interface",
            InterfaceKind::Action(_) => "action",
            InterfaceKind::Trigger(_) => "trigger",
        })
    }
}

/// port: int#(MIN: {*})
impl InferenceTargetPath {
    pub fn display(&self, md: &Module, globals: &LinkerGlobals) -> impl Display {
        FmtWrapper(|f| {
            let port = &md.ports[self.port];
            let port_decl =
                md.link_info.instructions[port.declaration_instruction].unwrap_declaration();

            fn recurse_print(
                f: &mut std::fmt::Formatter<'_>,
                globals: &LinkerGlobals,
                li: &LinkInfo,
                typ: &WrittenType,
                path: &[SubtypeInferencePathElem],
            ) -> std::fmt::Result {
                if let Some((cur_elem, rest)) = path.split_first() {
                    match cur_elem {
                        SubtypeInferencePathElem::DownArray => {
                            let_unwrap!(WrittenType::Array(_, arr_box), typ);
                            let (content, _sz, _) = arr_box.deref();
                            recurse_print(f, globals, li, content, rest)?;
                            f.write_str("[]")
                        }
                        SubtypeInferencePathElem::ArraySize => {
                            let_unwrap!(WrittenType::Array(_, arr_box), typ);
                            let (content, _sz, _) = arr_box.deref();
                            recurse_print(f, globals, li, content, rest)?;
                            f.write_str("[{*}]")
                        }
                        SubtypeInferencePathElem::InNamed(arg_id) => {
                            let_unwrap!(WrittenType::Named(named), typ);
                            let named_type = &globals.types[named.id];
                            let named_name = &named_type.link_info.name;
                            let param_name = &named_type.link_info.parameters[*arg_id].name;

                            write!(f, "{named_name} #({param_name}: ")?;
                            match &named.get_arg_for(*arg_id).unwrap().kind.as_ref().unwrap() {
                                TemplateKind::Type(t) => recurse_print(f, globals, li, t, rest)?,
                                TemplateKind::Value(_) => {
                                    assert!(rest.is_empty());
                                    f.write_str("{*}")?
                                }
                            }
                            f.write_str(")")
                        }
                    }
                } else {
                    write!(f, "{}", typ.display(globals, &li.parameters))
                }
            }

            recurse_print(f, globals, &md.link_info, &port_decl.typ_expr, &self.path)?;
            write!(f, " {}", &port_decl.name)
        })
    }
}

impl InferenceCandidate {
    /// V * 5 + 3 <= {*} in int#(FROM: {*}) port
    /// V * 5 + 3 <= {t} - {f} in a'{t}, b'{f}
    pub fn display(
        &self,
        candidate_name: &str,
        md: &Module,
        globals: &LinkerGlobals,
    ) -> impl Display {
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
                    let path = path.display(md, globals);
                    write!(f, "{{*}} in {path}")
                }
                InferenceTarget::PortLatency { from, to } => {
                    let from = &md.ports[*from];
                    let to = &md.ports[*to];
                    write!(f, "{}'{{*}} - {}'{{*}}", to.name, from.name)
                }
            }
        })
    }
}

impl InferenceResult {
    fn display(
        &self,
        submodules: &FlatAlloc<SubModule, SubModuleIDMarker>,
        globals: &LinkerGlobals,
    ) -> impl Display {
        FmtWrapper(move |f| match self {
            InferenceResult::PortNotUsed => f.write_str("N/C"),
            InferenceResult::NotFound => f.write_str("?"),
            InferenceResult::LatencyBadProblem => f.write_str("? bad problem"),
            InferenceResult::LatencyNotReached => f.write_str("? not reached"),
            InferenceResult::LatencyPoison {
                submod,
                port_from,
                port_to,
            } => {
                let poison_sm = &submodules[*submod];
                let poison_submod_md = &globals.modules[poison_sm.refers_to.id];

                let poison_sm_name = &poison_sm.name;
                let from_port_name = &poison_submod_md.ports[*port_from].name;
                let to_port_name = &poison_submod_md.ports[*port_to].name;

                write!(
                    f,
                    "? poisoned by unknown latency {poison_sm_name}.{from_port_name} to {poison_sm_name}.{to_port_name}"
                )
            }
            InferenceResult::Found(v) => write!(f, "{v}"),
        })
    }
}

pub fn display_infer_param_info(
    globals: &LinkerGlobals,
    md: &Module,
    template_id: TemplateID,
    final_values: Option<(
        &Vec<InferenceResult>,
        &FlatAlloc<SubModule, SubModuleIDMarker>,
    )>,
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
                    let path = c.display(md, globals);
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
                    write!(f, "    {}", c.display(arg_name, md, globals))?;
                    if let Some((values_list, submodules)) = final_values
                        && let Some(final_value) = values_list.get(idx)
                    {
                        write!(
                            f,
                            "  ({{*}} = {})",
                            final_value.display(submodules, globals)
                        )?;
                    }
                    writeln!(f)?;
                }
                if !cant_infer.is_empty() {
                    writeln!(
                        f,
                        "The following constraints were found, but aren't used for inference here"
                    )?;
                    for c in cant_infer {
                        writeln!(f, "    {}", c.display(arg_name, md, globals))?;
                    }
                }
            }
        }
        Ok(())
    })
}

pub fn display_all_infer_params(
    globals: &LinkerGlobals,
    submodules: &FlatAlloc<SubModule, SubModuleIDMarker>,
    sm: &SubModule,
) -> impl Display {
    FmtWrapper(|f| {
        let md = &globals.modules[sm.refers_to.id];
        for (template_id, known_values) in sm.last_infer_values.borrow().iter() {
            display_infer_param_info(globals, md, template_id, Some((known_values, submodules)))
                .fmt(f)?;
        }
        Ok(())
    })
}

impl LinkInfo {
    fn debug_name<'s>(&'s self, globals: &'s LinkerGlobals, instr_id: FlatID) -> impl Display + 's {
        let name = self.get_instruction_name_best_effort(globals, instr_id);
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
    pub fn fmt_instructions(
        &self,
        f: &mut Formatter<'_>,
        domains: &FlatAlloc<DomainInfo, DomainIDMarker>,
        file_data: &FileData,
        globals: &LinkerGlobals,
    ) -> std::fmt::Result {
        let mut spans_print = Vec::new();
        for (id, instr) in &self.instructions {
            let domain = self.display_domain_of(id, domains);
            let span = self.get_instruction_span(id);
            spans_print.push((format!("{id:?} {domain}"), span));

            let parent = FmtWrapper(|f| {
                if let Some(p) = instr.get_parent_condition() {
                    let p_when = p.parent_when;
                    if p.is_else_branch {
                        write!(f, "parent: !{p_when:?}")?;
                    } else {
                        write!(f, "parent: {p_when:?}")?;
                    }
                } else {
                    f.write_str("no parent when")?;
                }
                Ok(())
            });
            write!(f, "{id:?}: {parent} {domain} ")?;
            match instr {
                Instruction::SubModule(SubModuleInstance {
                    module_ref,
                    name,
                    local_domain_map,
                    typ: _,
                    ..
                }) => {
                    let disp_md_ref = module_ref.display(globals, self);
                    let name = name.green();
                    write!(f, "{disp_md_ref} {name}")?;
                    let submod_domains = &globals[module_ref.id].domains;
                    if let Some(local_domain_map) = local_domain_map.get() {
                        let domain_map = display_join(
                            ", ",
                            local_domain_map,
                            |f, (submod_domain, domain_here)| {
                                let submod_domain = submod_domain.display(submod_domains);
                                let domain_here = domain_here.unwrap_physical();
                                let domain_here = domain_here.display(domains);
                                write!(f, ".{submod_domain} = {domain_here}")
                            },
                        );
                        write!(f, "[{domain_map}]")?;
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
                    write!(f, "{decl_kind:?} {typ_expr} ({typ}) {name}")?;
                    if let Some(lat_spec) = latency_specifier {
                        write!(f, "'{lat_spec:?}")?;
                    }
                }
                Instruction::Expression(Expression { source, output, .. }) => {
                    match output {
                        ExpressionOutput::SubExpression(typ) => {
                            let typ = typ.display(globals, self);
                            write!(f, "({typ})")?;
                        }
                        ExpressionOutput::MultiWrite(write_tos) => {
                            let write_tos = display_join(
                                ", ",
                                write_tos,
                                |f,
                                 WriteTo {
                                     to,
                                     write_modifiers,
                                     target_domain,
                                     ..
                                 }| {
                                    let target_domain = target_domain.get();
                                    let target_domain = target_domain.debug(domains);
                                    let typ_disp = to.output_typ.display(globals, self);
                                    let to = to.display(globals, self);
                                    write!(f, "{target_domain} ({typ_disp}) {write_modifiers} {to}")
                                },
                            );
                            write!(f, "({write_tos}) = ")?;
                        }
                    }
                    match source {
                        ExpressionSource::WireRef(wire_reference) => {
                            write!(f, "{}", wire_reference.display(globals, self))?;
                        }
                        ExpressionSource::FuncCall(FuncCall {
                            func_wire_ref,
                            arguments,
                            ..
                        }) => {
                            let args = display_join(", ", arguments, |f, arg| write!(f, "{arg:?}"));
                            write!(f, "{func_wire_ref:?}({args})")?;
                        }
                        ExpressionSource::UnaryOp { op, right, rank: _ } => {
                            write!(f, "{op}{right:?}")?;
                        }
                        ExpressionSource::BinaryOp {
                            op,
                            left,
                            right,
                            rank: _,
                        } => write!(f, "{left:?} {op} {right:?}")?,
                        ExpressionSource::ArrayConstruct(elements) => {
                            let arr_elems =
                                display_join(", ", elements, |f, elem| write!(f, "{elem:?}"));
                            write!(f, "[{arr_elems}]")?;
                        }
                        ExpressionSource::Literal(value) => {
                            write!(f, "literal {value}")?;
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
                    write!(f, "{is_local} {interface_kind:?} {interface_id:?} {name}")?;
                    if let Some(lat_spec) = latency_specifier {
                        write!(f, "'{lat_spec:?}")?;
                    }
                    if !inputs.is_empty() | !outputs.is_empty() {
                        write!(f, ": {inputs:?} -> {outputs:?}")?;
                    }
                    write!(f, " {{{then_block:?}}} else {{{else_block:?}}}")?;
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
                    write!(f, "{kw} {condition:?} ")?;
                    if !bindings_read_only.is_empty() | !bindings_writable.is_empty() {
                        write!(f, ": {bindings_read_only:?} -> {bindings_writable:?}")?;
                    }
                    write!(f, " {{{then_block:?}}} else {{{else_block:?}}}")?;
                }
                Instruction::ForStatement(ForStatement {
                    loop_var_decl,
                    start,
                    end,
                    loop_body,
                    ..
                }) => {
                    let loop_var_decl_name = self.debug_name(globals, *loop_var_decl);
                    write!(
                        f,
                        "for {loop_var_decl_name} in {start:?}..{end:?} {{{loop_body:?}}}"
                    )?;
                }
            }
            writeln!(f)?;
        }
        pretty_print_many_spans(file_data, &spans_print);
        Ok(())
    }
}

impl Module {
    fn display_latency(&self, lat_spec: &Option<FlatID>, file_text: &FileText) -> impl Display {
        display_maybe(lat_spec.as_ref(), |f, lat_spec| {
            let lat_spec_expr = self.link_info.instructions[*lat_spec].unwrap_expression();
            let lat_spec_text = &file_text[lat_spec_expr.span];
            write!(f, "'{lat_spec_text}")
        })
    }
    pub fn display_port_info(&self, decl: &Declaration, file_text: &FileText) -> impl Display {
        let_unwrap!(
            DeclarationKind::Port {
                direction,
                is_state,
                ..
            },
            decl.decl_kind
        );

        let state_kw = if is_state { "state " } else { "" };

        let written_typ = &file_text[decl.typ_expr.get_span()];
        let name = &decl.name;

        let lat_spec = self.display_latency(&decl.latency_specifier, file_text);
        FmtWrapper(move |f| write!(f, "{direction} {state_kw}{written_typ} {name}{lat_spec}"))
    }

    pub fn display_interface_info(
        &self,
        interface: &InterfaceDeclaration,
        file_text: &FileText,
        may_print_domain: bool,
    ) -> impl Display {
        let domain = display_if(may_print_domain, |f| {
            write!(
                f,
                "{{{}}} ",
                self.domains[interface.domain.unwrap_physical()].name
            )
        });
        let interface_kind = interface.interface_kind;
        let name = &file_text[interface.name_span];
        let lat_spec = self.display_latency(&interface.latency_specifier, file_text);
        FmtWrapper(move |f| {
            write!(f, "{domain}{interface_kind} {name}{lat_spec}:")?;
            for decl_id in &interface.inputs {
                let port_decl = self.link_info.instructions[*decl_id].unwrap_declaration();
                let port_info = self.display_port_info(port_decl, file_text);
                write!(f, "\n\t{port_info}")?;
            }
            if !interface.outputs.is_empty() {
                write!(f, "\n\t->")?;
                for decl_id in &interface.outputs {
                    let port_decl = self.link_info.instructions[*decl_id].unwrap_declaration();
                    let port_info = self.display_port_info(port_decl, file_text);
                    write!(f, "\n\t{port_info}")?;
                }
            }
            Ok(())
        })
    }

    pub fn display_all_ports_info<'s>(
        &'s self,
        file_text: &'s FileText,
        local_domains_used_in_parent_module: Option<InterfaceToDomainMap>,
    ) -> impl Display {
        let full_name_with_args = self.link_info.display_full_name_and_args(file_text);

        FmtWrapper(move |f| {
            write!(f, "module {full_name_with_args}:")?;

            for (domain_id, domain) in &self.domains {
                let name = &domain.name;
                if let Some(domain_map) = &local_domains_used_in_parent_module {
                    let submod_name = &self.link_info.name;
                    let name_in_parent =
                        domain_map.local_domain_map[domain_id].debug(domain_map.domains);
                    write!(f, "\ndomain {submod_name}.{name} = {name_in_parent}:")?;
                } else {
                    write!(f, "\ndomain {name}:")?;
                }

                for (_, interface) in &self.interfaces {
                    match interface.declaration_instruction {
                        Some(InterfaceDeclKind::Interface(decl_id)) => {
                            let interface = self.link_info.instructions[decl_id].unwrap_interface();
                            if interface.domain.unwrap_physical() == domain_id {
                                let info = self.display_interface_info(interface, file_text, false);
                                write!(f, "\n{info}")?;
                            }
                        }
                        Some(InterfaceDeclKind::SinglePort(decl_id)) => {
                            let single_port =
                                self.link_info.instructions[decl_id].unwrap_declaration();
                            if single_port.domain.get().unwrap_physical() == domain_id {
                                let info = self.display_port_info(single_port, file_text);
                                write!(f, "\n{info}")?;
                            }
                        }
                        None => {}
                    }
                }
            }
            Ok(())
        })
    }

    pub fn print_flattened_module(&self, file_data: &FileData, globals: &LinkerGlobals) {
        let disp = FmtWrapper(|f| {
            writeln!(f, "[[{}]]:", self.link_info.name)?;
            writeln!(f, "Interface:")?;
            writeln!(
                f,
                "{}",
                self.display_all_ports_info(&file_data.file_text, None)
            )?;
            writeln!(f, "Instructions:")?;
            self.link_info
                .fmt_instructions(f, &self.domains, file_data, globals)
        });

        eprintln!("{disp}");
    }
}

impl RealWire {
    pub fn display_decl(&self, globals: &LinkerGlobals) -> impl Display {
        FmtWrapper(|f| {
            let port_typ = self.typ.display(globals);
            let port_name = &self.name;
            let port_abs_lat = &self.absolute_latency;

            write!(f, "{port_typ} {port_name}'{port_abs_lat}")
        })
    }
}

impl InstantiatedModule {
    pub fn display_interface(&self, globals: &LinkerGlobals) -> impl Display {
        FmtWrapper(|f| {
            let md = &globals.modules[self.global_ref.id];

            writeln!(f, "module {}:", self.global_ref.display(globals))?;
            for (_, interf) in &md.interfaces {
                match interf.declaration_instruction {
                    Some(InterfaceDeclKind::Interface(interf_id)) => {
                        let interf = md.link_info.instructions[interf_id].unwrap_interface();
                        // If an execution error occurred, interface may only be half-finished. Just abort if any port is invalid
                        let interf_is_valid =
                            interf.inputs.iter().chain(interf.outputs.iter()).all(|id| {
                                matches!(&self.generation_state[*id], SubModuleOrWire::Wire(_))
                            });
                        if !interf_is_valid {
                            continue;
                        }
                        match interf.interface_kind {
                            InterfaceKind::RegularInterface => {
                                write!(f, "    interface {}", interf.name)?;
                            }
                            InterfaceKind::Action(_) => {
                                if let SubModuleOrWire::Wire(w) = &self.generation_state[interf_id]
                                {
                                    let w = &self.wires[*w];
                                    write!(f, "    action {}'{}", w.name, w.absolute_latency)?;
                                } else {
                                    continue;
                                }
                            }
                            InterfaceKind::Trigger(_) => {
                                if let SubModuleOrWire::Wire(w) = &self.generation_state[interf_id]
                                {
                                    let w = &self.wires[*w];
                                    write!(f, "    trigger {}'{}", w.name, w.absolute_latency)?;
                                } else {
                                    continue;
                                }
                            }
                        }
                        if !interf.inputs.is_empty() {
                            let inputs = display_join(", ", &interf.inputs, |f, i| {
                                let i_wire = self.generation_state[*i].unwrap_wire(); // Safely unwrap due to earlier check
                                let w = &self.wires[i_wire];

                                write!(f, "{}", w.display_decl(globals))
                            });
                            write!(f, ": {inputs}")?;
                        }
                        if !interf.outputs.is_empty() {
                            let outputs = display_join(", ", &interf.outputs, |f, i| {
                                let i_wire = self.generation_state[*i].unwrap_wire(); // Safely unwrap due to earlier check
                                let w = &self.wires[i_wire];

                                write!(f, "{}", w.display_decl(globals))
                            });
                            write!(f, " -> {outputs}")?;
                        }
                        writeln!(f)?;
                    }
                    Some(InterfaceDeclKind::SinglePort(port)) => {
                        if let SubModuleOrWire::Wire(w) = &self.generation_state[port] {
                            let port_w = &self.wires[*w];
                            let_unwrap!(IsPort::Port(_, port_dir), port_w.is_port);

                            writeln!(f, "    {port_dir} {}", port_w.display_decl(globals))?;
                        }
                    }
                    None => {}
                }
            }
            Ok(())
        })
    }
}

impl ModuleTypingContext<'_> {
    fn name(&self, wire_id: WireID) -> impl Display {
        self.wires[wire_id].name.green()
    }
    fn fmt_path(&self, f: &mut Formatter<'_>, path: &[RealWirePathElem]) -> std::fmt::Result {
        for p in path {
            match p {
                RealWirePathElem::Index { idx_wire, .. } => {
                    write!(f, "[{}]", self.name(*idx_wire))?;
                }
                RealWirePathElem::ConstIndex { idx, .. } => {
                    write!(f, "[{idx}]")?;
                }
                RealWirePathElem::Slice { bounds, .. } => match bounds {
                    PartialBound::Known(from, to) => write!(f, "[{from}:{to}]")?,
                    PartialBound::From(from) => write!(f, "[{from}:]")?,
                    PartialBound::To(to) => write!(f, "[:{to}]")?,
                    PartialBound::WholeSlice => write!(f, "[:]")?,
                },
                RealWirePathElem::PartSelect {
                    from_wire,
                    width,
                    direction,
                    ..
                } => {
                    let from = self.name(*from_wire);
                    write!(f, "[{from}{direction}{width}]")?;
                }
            }
        }
        Ok(())
    }
    fn fmt_rank(&self, f: &mut Formatter<'_>, rank: &[UnifyableValue]) -> std::fmt::Result {
        for r in rank {
            write!(f, "[{r}]")?;
        }
        Ok(())
    }

    fn fmt_instantiated_module(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Wires: ")?;
        for (
            _,
            RealWire {
                source,
                original_instruction,
                typ,
                name,
                domain,
                specified_latency: _,
                absolute_latency,
                is_port,
            },
        ) in &self.wires
        {
            let is_port_str = if let IsPort::Port(_, direction) = is_port {
                format!("{direction} ").purple()
            } else {
                "".purple()
            };
            let typ_str = typ.display(self.globals).to_string().red();
            let domain_name = domain.display(&self.md.domains);
            let name = name.green();
            write!(
                f,
                "{name}: {is_port_str}{typ_str}'{absolute_latency} {domain_name} [{original_instruction:?}]"
            )?;
            match source {
                RealWireDataSource::ReadOnly => writeln!(f, " = ReadOnly")?,
                RealWireDataSource::Multiplexer { is_state, sources } => {
                    write!(f, ": ")?;
                    if let Some(initial) = is_state {
                        write!(f, "state (initial {initial}) ")?;
                    }
                    writeln!(f, "Mux:")?;
                    for MultiplexerSource {
                        to_path,
                        num_regs,
                        from,
                        condition,
                        write_span: _,
                    } in sources
                    {
                        write!(f, "    ")?;
                        let mut is_first_condition = true;
                        for c in condition {
                            let if_or_and = if is_first_condition {
                                is_first_condition = false;
                                "if "
                            } else {
                                " & "
                            };
                            let invert = if c.inverse { "!" } else { "" };
                            write!(f, "{if_or_and}{invert}{}", self.name(c.condition_wire))?;
                        }
                        if *num_regs != 0 {
                            write!(f, ": reg({num_regs}) {name}")?;
                        } else {
                            write!(f, ": {name}")?;
                        }
                        self.fmt_path(f, to_path)?;
                        writeln!(f, " = {}", self.name(*from))?;
                    }
                }
                RealWireDataSource::UnaryOp { op, rank, right } => {
                    write!(f, " = {op}")?;
                    self.fmt_rank(f, rank)?;
                    writeln!(f, " {}", self.name(*right))?;
                }
                RealWireDataSource::BinaryOp {
                    op,
                    rank,
                    left,
                    right,
                } => {
                    write!(f, " = {} {op}", self.name(*left))?;
                    self.fmt_rank(f, rank)?;
                    writeln!(f, " {}", self.name(*right))?;
                }
                RealWireDataSource::Select { root, path } => {
                    write!(f, " = {}", self.name(*root))?;
                    self.fmt_path(f, path)?;
                    writeln!(f)?;
                }
                RealWireDataSource::ConstructArray { array_wires } => {
                    let s = display_join(", ", array_wires.iter(), |f, item| {
                        write!(f, "{}", self.name(*item))
                    });
                    write!(f, " = [{s}]")?;
                }
                RealWireDataSource::Constant { value } => writeln!(f, " = {value}")?,
            }
        }
        writeln!(f, "\nSubmodules: ")?;
        for (
            _,
            SubModule {
                original_instruction,
                instance,
                refers_to,
                last_infer_values: _,
                port_map,
                interface_call_sites: _,
                name,
            },
        ) in &self.submodules
        {
            let instance_md = &self.globals[refers_to.id];
            let refers_to = refers_to.display(self.globals);
            let instantiate_success = if instance.get().is_some() {
                "Instantiation Successful!".yellow()
            } else {
                "No Instance".red()
            };
            writeln!(
                f,
                "{name}: {refers_to}[{original_instruction:?}]: {instantiate_success}"
            )?;
            for (port_id, port, usage) in zip_eq(&instance_md.ports, port_map) {
                let local_name = if let Some(p) = usage {
                    self.name(p.maps_to_wire).to_string()
                } else {
                    "".into()
                };
                let remote_name = &port.name;
                let direction = port.direction.to_string().purple();
                write!(f, "    {direction} .{remote_name}({local_name})")?;
                if let Some(instance) = instance.get() {
                    let typ_str = if let Some(port) = &instance.interface_ports[port_id] {
                        port.typ.display(self.globals).to_string()
                    } else {
                        "/".into()
                    }
                    .red();
                    write!(f, ": {typ_str}")?;
                }
                writeln!(f)?;
            }
        }
        writeln!(f)
    }

    pub fn print_instantiated_module(&self) {
        eprintln!("{}", FmtWrapper(|f| { self.fmt_instantiated_module(f) }))
    }
}

impl SubModule {
    pub fn display_interface(&self, globals: &LinkerGlobals) -> impl Display {
        FmtWrapper(|f| {
            if let Some(instance) = self.instance.get() {
                instance.display_interface(globals).fmt(f)
            } else {
                write!(
                    f,
                    "module {}: /* Could not instantiate */",
                    self.refers_to.display(globals)
                )
            }
        })
    }
}

struct JoinDisplay<
    T,
    Iter: Iterator<Item = T> + Clone,
    F: Fn(&mut Formatter<'_>, T) -> std::fmt::Result,
> {
    sep: &'static str,
    iter: Iter,
    func: F,
}
impl<T, Iter: Iterator<Item = T> + Clone, F: Fn(&mut Formatter<'_>, T) -> std::fmt::Result> Display
    for JoinDisplay<T, Iter, F>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter_copy = self.iter.clone();
        if let Some(first) = iter_copy.next() {
            (self.func)(f, first)?;
            for item in iter_copy {
                f.write_str(self.sep)?;
                (self.func)(f, item)?;
            }
        }
        Ok(())
    }
}
/// Iterator should be clonable. This is usually the case for simple iterators
pub fn display_join<T, Iter: Iterator<Item = T> + Clone>(
    sep: &'static str,
    iter: impl IntoIterator<Item = T, IntoIter = Iter>,
    func: impl Fn(&mut Formatter<'_>, T) -> std::fmt::Result,
) -> impl Display {
    JoinDisplay {
        sep,
        iter: iter.into_iter(),
        func,
    }
}

pub fn display_maybe<T>(
    v: Option<&T>,
    func: impl Fn(&mut Formatter<'_>, &T) -> std::fmt::Result,
) -> impl Display {
    FmtWrapper(move |f| if let Some(v) = v { func(f, v) } else { Ok(()) })
}

pub fn display_if(b: bool, func: impl Fn(&mut Formatter<'_>) -> std::fmt::Result) -> impl Display {
    FmtWrapper(move |f| if b { func(f) } else { Ok(()) })
}

// Limit total folder/file name byte size to 255, which is the maximum on just about every platform
const MAX_FILENAME_LEN: usize = 255;

#[cfg(target_os = "linux")]
const INVALID_CHARS: &[char] = &['/'];
#[cfg(not(target_os = "linux"))]
const INVALID_CHARS: &[char] = &['\\', '/', ':', '*', '?', '"', '<', '>', '|']; // Mostly for windows. 

/// Shorten the total string (name + postfix) such that `format!("{name}{postfix}").len() <= MAX_FILENAME_LEN`
pub fn sanitize_filename(name: &str, postfix: &str) -> String {
    let max_len = MAX_FILENAME_LEN - postfix.len();
    if name.len() <= max_len {
        format!("{name}{postfix}")
    } else {
        let mut shortened = String::with_capacity(name.len() + postfix.len() + 1); // One for CString \0
        for c in name.chars() {
            if shortened.len() + c.len_utf8() >= max_len {
                break;
            }
            let new_c = if INVALID_CHARS.contains(&c) { ' ' } else { c };
            shortened.push(new_c);
        }
        let result = format!("{shortened}{postfix}");
        warn!(
            "Filename {name}{postfix} was shortened to {result} to avoid too long filenames on some platforms"
        );
        result
    }
}

pub struct FmtWrapper<F: Fn(&mut Formatter<'_>) -> std::fmt::Result>(pub F);

impl<F: Fn(&mut Formatter<'_>) -> std::fmt::Result> Display for FmtWrapper<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (self.0)(f)
    }
}
