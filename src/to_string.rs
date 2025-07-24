use crate::alloc::zip_eq;
use crate::prelude::*;

use crate::typing::abstract_type::{AbstractGlobalReference, AbstractInnerType};
use crate::typing::concrete_type::ConcreteGlobalReference;
use crate::typing::domain_type::DomainType;
use crate::typing::set_unifier::Unifyable;
use crate::typing::template::{Parameter, TVec, TemplateKind};
use crate::value::Value;
use crate::{file_position::FileText, pretty_print_many_spans};

use crate::flattening::{
    Declaration, DeclarationKind, DomainInfo, InterfaceDeclKind, InterfaceDeclaration,
    InterfaceToDomainMap, Module, WrittenType,
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
                AbstractInnerType::Template(id) => {
                    f.write_str(&link_info.template_parameters[*id].name)
                }
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
            let args_iter = zip_eq(
                &self.template_arg_types,
                &target_link_info.template_parameters,
            );
            join_string_iter_formatter(", ", f, args_iter, |(_, typ, param), f| match typ {
                TemplateKind::Type(typ) => f.write_fmt(format_args!(
                    "{}: {}",
                    param.name,
                    typ.display(globals, link_info)
                )),
                TemplateKind::Value(()) => f.write_fmt(format_args!("{}: _", param.name)),
            })?;
            f.write_str(")")
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
            assert!(self.template_args.len() == target_link_info.template_parameters.len());
            let object_full_name = target_link_info.get_full_name();
            f.write_str(&object_full_name)?;
            if self.template_args.is_empty() {
                return f.write_str(" #()");
            } else {
                f.write_str(" #(")?;
            }
            let mut is_first = true;
            for (_id, arg, arg_in_target) in
                zip_eq(&self.template_args, &target_link_info.template_parameters)
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
                    domain_map.local_domain_map[domain_id].display(domain_map.domains);
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

    pub fn print_flattened_module(&self, file_data: &FileData) {
        println!("[[{}]]:", self.link_info.name);
        println!("Interface:");
        println!(
            "{}",
            self.make_all_ports_info_string(&file_data.file_text, None)
        );
        println!("Instructions:");
        let mut spans_print = Vec::new();
        for (id, inst) in &self.link_info.instructions {
            println!("    {id:?}: {inst:?}");
            let span = self.link_info.get_instruction_span(id);
            spans_print.push((format!("{id:?}"), span.as_range()));
        }
        pretty_print_many_spans(file_data, &spans_print);
    }
}

pub fn join_string_iter_formatter<'fmt, T>(
    sep: &str,
    f: &mut Formatter<'fmt>,
    mut iter: impl Iterator<Item = T>,
    mut func: impl FnMut(T, &mut Formatter<'fmt>) -> std::fmt::Result,
) -> std::fmt::Result {
    if let Some(first) = iter.next() {
        func(first, f)?;
        for item in iter {
            f.write_str(sep)?;
            func(item, f)?;
        }
    }
    Ok(())
}

pub fn join_string_iter<T>(
    result: &mut String,
    sep: &'static str,
    mut iter: impl Iterator<Item = T>,
    mut f: impl FnMut(&mut String, T),
) {
    if let Some(first) = iter.next() {
        f(result, first);
        for item in iter {
            result.write_str(sep).unwrap();
            f(result, item);
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
