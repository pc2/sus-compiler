use crate::prelude::*;

use crate::typing::abstract_type::{AbstractInnerType, PeanoType};
use crate::typing::template::{Parameter, TVec};
use crate::{file_position::FileText, pretty_print_many_spans, value::Value};

use crate::flattening::{
    DomainInfo, Interface, InterfaceToDomainMap, Module, StructType, WrittenType,
};
use crate::linker::FileData;
use crate::typing::{
    abstract_type::{AbstractRankedType, DomainType},
    concrete_type::ConcreteType,
};

use std::{
    fmt::{Display, Formatter},
    ops::Index,
};

use std::fmt::Write;
use std::ops::Deref;

pub fn map_to_type_names(parameters: &TVec<Parameter>) -> FlatAlloc<String, TemplateIDMarker> {
    parameters.map(|(_id, v)| v.name.clone())
}

/// For [Display::fmt] implementations on types: [ConcreteType], [WrittenType], [AbstractType]
pub trait TemplateNameGetter {
    fn get_template_name(&self, id: TemplateID) -> &str;
}

impl TemplateNameGetter for FlatAlloc<String, TemplateIDMarker> {
    fn get_template_name(&self, id: TemplateID) -> &str {
        &self[id]
    }
}
impl TemplateNameGetter for TVec<Parameter> {
    fn get_template_name(&self, id: TemplateID) -> &str {
        &self[id].name
    }
}

#[derive(Debug)]
pub struct WrittenTypeDisplay<
    'a,
    TypVec: Index<TypeUUID, Output = StructType>,
    TemplateVec: TemplateNameGetter,
> {
    inner: &'a WrittenType,
    linker_types: &'a TypVec,
    template_names: &'a TemplateVec,
}

impl<TypVec: Index<TypeUUID, Output = StructType>, TemplateVec: TemplateNameGetter> Display
    for WrittenTypeDisplay<'_, TypVec, TemplateVec>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.inner {
            WrittenType::Error(_) => f.write_str("{error"),
            WrittenType::TemplateVariable(_, id) => {
                f.write_str(self.template_names.get_template_name(*id))
            }
            WrittenType::Named(named_type) => {
                f.write_str(&self.linker_types[named_type.id].link_info.get_full_name())
            }
            WrittenType::Array(_, sub) => {
                write!(
                    f,
                    "{}[]",
                    sub.deref()
                        .0
                        .display(self.linker_types, self.template_names)
                )
            }
        }
    }
}

impl WrittenType {
    pub fn display<'a>(
        &'a self,
        linker_types: &'a impl Index<TypeUUID, Output = StructType>,
        template_names: &'a impl TemplateNameGetter,
    ) -> impl Display + 'a {
        WrittenTypeDisplay {
            inner: self,
            linker_types,
            template_names,
        }
    }
}

#[derive(Debug)]
pub struct AbstractRankedTypeDisplay<'a, TypVec, TemplateVec: TemplateNameGetter> {
    inner_typ: &'a AbstractInnerType,
    rank_typ: &'a PeanoType,
    linker_types: &'a TypVec,
    template_names: &'a TemplateVec,
}

impl<TypVec: Index<TypeUUID, Output = StructType>, TemplateVec: TemplateNameGetter> Display
    for AbstractRankedTypeDisplay<'_, TypVec, TemplateVec>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.inner_typ {
            AbstractInnerType::Unknown(id) => write!(f, "{id:?}"),
            AbstractInnerType::Template(id) => {
                f.write_str(self.template_names.get_template_name(*id))
            }
            AbstractInnerType::Named(id) => {
                f.write_str(&self.linker_types[*id].link_info.get_full_name())
            }
        }
        .and_then(|_| f.write_str(&self.rank_typ.to_string()))
    }
}

impl AbstractRankedType {
    pub fn display<'a>(
        &'a self,
        linker_types: &'a impl Index<TypeUUID, Output = StructType>,
        template_names: &'a impl TemplateNameGetter,
    ) -> impl Display + 'a {
        AbstractRankedTypeDisplay {
            inner_typ: &self.inner,
            rank_typ: &self.rank,
            linker_types,
            template_names,
        }
    }
}

#[derive(Debug)]
pub struct AbstractInnerTypeDisplay<'a, TypVec, TemplateVec: TemplateNameGetter> {
    inner_typ: &'a AbstractInnerType,
    linker_types: &'a TypVec,
    template_names: &'a TemplateVec,
}

impl<TypVec: Index<TypeUUID, Output = StructType>, TemplateVec: TemplateNameGetter> Display
    for AbstractInnerTypeDisplay<'_, TypVec, TemplateVec>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.inner_typ {
            AbstractInnerType::Unknown(id) => write!(f, "{id:?}"),
            AbstractInnerType::Template(id) => {
                f.write_str(self.template_names.get_template_name(*id))
            }
            AbstractInnerType::Named(id) => {
                f.write_str(&self.linker_types[*id].link_info.get_full_name())
            }
        }
    }
}

impl AbstractInnerType {
    pub fn display<'a>(
        &'a self,
        linker_types: &'a impl Index<TypeUUID, Output = StructType>,
        template_names: &'a impl TemplateNameGetter,
    ) -> impl Display + 'a {
        AbstractInnerTypeDisplay {
            inner_typ: self,
            linker_types,
            template_names,
        }
    }
}

#[derive(Debug)]
pub struct ConcreteTypeDisplay<'a, T: Index<TypeUUID, Output = StructType>> {
    inner: &'a ConcreteType,
    linker_types: &'a T,
}

impl<T: Index<TypeUUID, Output = StructType>> Display for ConcreteTypeDisplay<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.inner {
            ConcreteType::Named(name) => {
                f.write_str(&self.linker_types[name.id].link_info.get_full_name())
            }
            ConcreteType::Array(arr_box) => {
                let (elem_typ, arr_size) = arr_box.deref();
                write!(
                    f,
                    "{}[{}]",
                    elem_typ.display(self.linker_types),
                    arr_size.display(self.linker_types)
                )
            }
            ConcreteType::Value(v) => write!(f, "{v}"),
            ConcreteType::Unknown(u) => write!(f, "{{{u:?}}}"),
        }
    }
}

impl ConcreteType {
    pub fn display<'a>(
        &'a self,
        linker_types: &'a impl Index<TypeUUID, Output = StructType>,
    ) -> impl Display + 'a {
        ConcreteTypeDisplay {
            inner: self,
            linker_types,
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
                let mut iter = arr_box.iter();
                if let Some(v) = iter.next() {
                    v.fmt(f)?;

                    for v in iter {
                        f.write_str(", ")?;
                        v.fmt(f)?;
                    }
                }
                f.write_str("]")
            }
            Value::Unset => f.write_str("{value_unset}"),
            Value::Error => f.write_str("{value_error}"),
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
        interface: &Interface,
        file_text: &FileText,
        result: &mut String,
    ) {
        for port_id in interface.all_ports() {
            self.make_port_info_fmt(port_id, file_text, result);
        }
    }
    pub fn make_interface_info_string(
        &self,
        interface: &Interface,
        file_text: &FileText,
    ) -> String {
        let mut r = String::new();
        self.make_interface_info_fmt(interface, file_text, &mut r);
        r
    }

    pub fn make_all_ports_info_string(
        &self,
        file_text: &FileText,
        local_domains: Option<InterfaceToDomainMap>,
    ) -> String {
        let full_name_with_args = self.link_info.get_full_name_and_template_args(file_text);
        let mut result = format!("module {full_name_with_args}:\n");

        for (domain_id, domain) in &self.domains {
            if let Some(domain_map) = &local_domains {
                writeln!(
                    result,
                    "domain {}: {{{}}}",
                    &domain.name,
                    domain_map.local_domain_to_global_domain(domain_id).name
                )
                .unwrap();
            } else {
                writeln!(result, "domain {}:", &domain.name).unwrap();
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
            let span = self.get_instruction_span(id);
            spans_print.push((format!("{id:?}"), span.as_range()));
        }
        pretty_print_many_spans(file_data, &spans_print);
    }
}
