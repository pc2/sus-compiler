use crate::alloc::zip_eq;
use crate::prelude::*;

use crate::typing::abstract_type::{AbstractGlobalReference, AbstractInnerType, PeanoType};
use crate::typing::concrete_type::{ConcreteGlobalReference, ConcreteTemplateArg};
use crate::typing::domain_type::DomainType;
use crate::typing::set_unifier::Unifyable;
use crate::typing::template::{Parameter, TVec, TemplateKind};
use crate::{file_position::FileText, pretty_print_many_spans, value::Value};

use crate::flattening::{
    DomainInfo, InterfaceDeclaration, InterfaceToDomainMap, Module, WrittenType,
};
use crate::linker::{FileData, GlobalUUID, LinkInfo, LinkerGlobals};
use crate::typing::{abstract_type::AbstractRankedType, concrete_type::ConcreteType};

use std::fmt::{Display, Formatter};

use std::fmt::Write;
use std::ops::Deref;

pub struct WrittenTypeDisplay<'a> {
    inner: &'a WrittenType,
    globals: &'a LinkerGlobals,
    template_names: &'a TVec<Parameter>,
}

impl Display for WrittenTypeDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.inner {
            WrittenType::Error(_) => f.write_str("{error}"),
            WrittenType::TemplateVariable(_, id) => f.write_str(&self.template_names[*id].name),
            WrittenType::Named(named_type) => {
                f.write_str(&self.globals.types[named_type.id].link_info.get_full_name())
            }
            WrittenType::Array(_, sub) => {
                write!(
                    f,
                    "{}[]",
                    sub.deref().0.display(self.globals, self.template_names)
                )
            }
        }
    }
}

impl WrittenType {
    pub fn display<'a>(
        &'a self,
        globals: &'a LinkerGlobals,
        template_names: &'a TVec<Parameter>,
    ) -> impl Display + 'a {
        WrittenTypeDisplay {
            inner: self,
            globals,
            template_names,
        }
    }
}

pub struct AbstractRankedTypeDisplay<'a> {
    typ: &'a AbstractRankedType,
    globals: &'a LinkerGlobals,
    template_names: &'a TVec<Parameter>,
}

impl Display for AbstractRankedTypeDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.typ.inner {
            AbstractInnerType::Unknown(_) => write!(f, "?"),
            AbstractInnerType::Template(id) => f.write_str(&self.template_names[*id].name),
            AbstractInnerType::Named(id) => {
                f.write_str(&self.globals.types[*id].link_info.get_full_name())
            }
            AbstractInnerType::Interface(md_id, interface_id) => {
                let md = &self.globals.modules[md_id.id];
                f.write_fmt(format_args!(
                    "Interface {} of {}",
                    md.interfaces[*interface_id].name,
                    md_id.display(self.globals, self.template_names)
                ))
            }
        }
        .and_then(|_| f.write_fmt(format_args!("{}", &self.typ.rank)))
    }
}

pub struct AbstractGlobalReferenceDisplay<'a, ID> {
    typ: &'a AbstractGlobalReference<ID>,
    globals: &'a LinkerGlobals,
    template_names: &'a TVec<Parameter>,
}

impl<ID: Into<GlobalUUID> + Copy> Display for AbstractGlobalReferenceDisplay<'_, ID> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let target_link_info = &self.globals[self.typ.id.into()];

        f.write_str(&target_link_info.name)?;
        f.write_str(" #(")?;

        let args_iter = zip_eq(
            &self.typ.template_arg_types,
            &target_link_info.template_parameters,
        );

        join_string_iter_formatter(", ", f, args_iter, |(_, typ, param), f| {
            f.write_fmt(format_args!(
                "{}: {}",
                param.name,
                typ.display(self.globals, self.template_names)
            ))
        })?;
        f.write_str(")")
    }
}

impl<ID: Into<GlobalUUID> + Copy> AbstractGlobalReference<ID> {
    pub fn display<'a>(
        &'a self,
        globals: &'a LinkerGlobals,
        template_names: &'a TVec<Parameter>,
    ) -> impl Display + 'a {
        AbstractGlobalReferenceDisplay {
            typ: self,
            globals,
            template_names,
        }
    }
}

impl AbstractRankedType {
    pub fn display<'a>(
        &'a self,
        globals: &'a LinkerGlobals,
        template_names: &'a TVec<Parameter>,
    ) -> impl Display + 'a {
        AbstractRankedTypeDisplay {
            typ: self,
            globals,
            template_names,
        }
    }
}

impl Display for PeanoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut cur = self;
        loop {
            match cur {
                PeanoType::Zero => return Ok(()),
                PeanoType::Succ(inner) => {
                    f.write_str("[]")?;
                    cur = inner;
                }
                PeanoType::Unknown(_) => {
                    write!(f, "[...]")?;
                    return Ok(());
                }
            }
        }
    }
}

pub struct ConcreteTypeDisplay<'a> {
    inner: &'a ConcreteType,
    globals: &'a LinkerGlobals,
    use_newlines: bool,
}

impl Display for ConcreteTypeDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.inner {
            ConcreteType::Named(global_ref) => ConcreteGlobalReferenceDisplay {
                target_link_info: &self.globals.types[global_ref.id].link_info,
                template_args: &global_ref.template_args,
                globals: self.globals,
                use_newlines: self.use_newlines,
            }
            .fmt(f),
            ConcreteType::Array(arr_box) => {
                let (elem_typ, arr_size) = arr_box.deref();
                write!(
                    f,
                    "{}[{arr_size}]",
                    elem_typ.display(self.globals, self.use_newlines)
                )
            }
        }
    }
}

impl ConcreteType {
    pub fn display<'a>(
        &'a self,
        globals: &'a LinkerGlobals,
        use_newlines: bool,
    ) -> impl Display + 'a {
        ConcreteTypeDisplay {
            inner: self,
            globals,
            use_newlines,
        }
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

impl DomainType {
    pub fn physical_to_string(
        physical_id: DomainID,
        domains: &FlatAlloc<DomainInfo, DomainIDMarker>,
    ) -> String {
        if let Some(interf) = domains.get(physical_id) {
            format!("{{{}}}", interf.name)
        } else {
            format!("{{unnamed domain {}}}", physical_id.get_hidden_value())
        }
    }
}

impl Module {
    pub fn make_port_info_fmt(&self, port_id: PortID, file_text: &FileText, result: &mut String) {
        use std::fmt::Write;
        let port = &self.ports[port_id];
        let port_direction = if port.is_input { "input" } else { "output" };
        writeln!(result, "{port_direction} {}", &file_text[port.decl_span]).unwrap()
    }
    pub fn make_port_info_string(&self, port_id: PortID, file_text: &FileText) -> String {
        let mut r = String::new();
        self.make_port_info_fmt(port_id, file_text, &mut r);
        r
    }

    pub fn make_interface_info_fmt(
        &self,
        interface: &InterfaceDeclaration,
        file_text: &FileText,
        result: &mut String,
    ) {
        for port_id in interface.all_ports() {
            self.make_port_info_fmt(port_id, file_text, result);
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
                let domain_id_in_parent = domain_map.local_domain_map[domain_id].unwrap_physical();
                let name_in_parent =
                    DomainType::physical_to_string(domain_id_in_parent, domain_map.domains);
                writeln!(result, "domain {submod_name}.{name} = {name_in_parent}").unwrap();
            } else {
                writeln!(result, "domain {name}:").unwrap();
            }

            // TODO interfaces
            for (port_id, port) in &self.ports {
                if port.domain == domain_id {
                    self.make_port_info_fmt(port_id, file_text, &mut result);
                }
            }
        }

        result
    }

    pub fn print_flattened_module(&self, file_data: &FileData) {
        println!("[[{}]]:", self.link_info.name);
        println!("Interface:");
        for (port_id, port) in &self.ports {
            println!(
                "    {} -> {:?}",
                self.make_port_info_string(port_id, &file_data.file_text),
                port
            );
        }
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

pub struct ConcreteGlobalReferenceDisplay<'a> {
    template_args: &'a TVec<ConcreteTemplateArg>,
    target_link_info: &'a LinkInfo,
    globals: &'a LinkerGlobals,
    /// If there should be newlines: "\n", otherwise ""
    use_newlines: bool,
}

impl<'a> Display for ConcreteGlobalReferenceDisplay<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let nl = if self.use_newlines { "\n    " } else { "" };
        assert!(self.template_args.len() == self.target_link_info.template_parameters.len());
        let object_full_name = self.target_link_info.get_full_name();
        f.write_str(&object_full_name)?;
        if self.template_args.is_empty() {
            //return f.write_str(" #()");
            return Ok(());
        } else {
            f.write_str(" #(")?;
        }

        let mut is_first = true;
        for (_id, arg, arg_in_target) in zip_eq(
            self.template_args,
            &self.target_link_info.template_parameters,
        ) {
            if !is_first {
                f.write_str(", ")?;
            }
            is_first = false;
            f.write_fmt(format_args!("{nl}{}: ", arg_in_target.name))?;
            match arg {
                TemplateKind::Type(typ_arg) => {
                    f.write_fmt(format_args!(
                        "type {}",
                        typ_arg.display(self.globals, self.use_newlines)
                    ))?;
                }
                TemplateKind::Value(v) => match v {
                    Unifyable::Set(value) => f.write_fmt(format_args!("{value}"))?,
                    Unifyable::Unknown(_) => f.write_str("/* Could not infer */")?,
                },
            }
        }
        if self.use_newlines {
            f.write_str("\n")?;
        }
        f.write_char(')')
    }
}
impl<ID: Into<GlobalUUID> + Copy> ConcreteGlobalReference<ID> {
    pub fn display<'v>(
        &'v self,
        globals: &'v LinkerGlobals,
        use_newlines: bool,
    ) -> ConcreteGlobalReferenceDisplay<'v> {
        let target_link_info = &globals[self.id.into()];
        ConcreteGlobalReferenceDisplay {
            template_args: &self.template_args,
            target_link_info,
            globals,
            use_newlines,
        }
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
