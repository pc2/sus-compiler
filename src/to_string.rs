use crate::prelude::*;

use crate::typing::template::HowDoWeKnowTheTemplateArg;
use crate::{file_position::FileText, pretty_print_many_spans, value::Value};

use crate::flattening::{
    DomainInfo, Interface, InterfaceToDomainMap, Module, StructType, WrittenType,
};
use crate::linker::{FileData, LinkInfo};
use crate::typing::{
    abstract_type::{AbstractType, DomainType},
    concrete_type::ConcreteType,
    template::{ConcreteTemplateArg, ConcreteTemplateArgs, TemplateInputs},
};

use std::{
    fmt::{Display, Formatter},
    ops::Index,
};

use std::fmt::Write;
use std::ops::Deref;

pub fn map_to_type_names(template_inputs: &TemplateInputs) -> FlatAlloc<String, TemplateIDMarker> {
    template_inputs.map(|(_id, v)| v.name.clone())
}

pub trait TemplateNameGetter {
    fn get_template_name(&self, id: TemplateID) -> &str;
}

impl TemplateNameGetter for FlatAlloc<String, TemplateIDMarker> {
    fn get_template_name(&self, id: TemplateID) -> &str {
        &self[id]
    }
}
impl TemplateNameGetter for TemplateInputs {
    fn get_template_name(&self, id: TemplateID) -> &str {
        &self[id].name
    }
}

#[derive(Debug)]
struct WrittenTypeDisplay<
    'a,
    TypVec: Index<TypeUUID, Output = StructType>,
    TemplateVec: TemplateNameGetter,
> {
    written_type: &'a WrittenType,
    linker_types: &'a TypVec,
    template_names: &'a TemplateVec,
}

impl<'a, TypVec: Index<TypeUUID, Output = StructType>, TemplateVec: TemplateNameGetter> Display
    for WrittenTypeDisplay<'a, TypVec, TemplateVec>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.written_type {
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
    pub fn display<
        'a,
        TypVec: Index<TypeUUID, Output = StructType>,
        TemplateVec: TemplateNameGetter,
    >(
        &'a self,
        linker_types: &'a TypVec,
        template_names: &'a TemplateVec,
    ) -> impl Display + 'a {
        WrittenTypeDisplay {
            written_type: self,
            linker_types,
            template_names,
        }
    }
}

#[derive(Debug)]
struct AbstractTypeDisplay<
    'a,
    TypVec: Index<TypeUUID, Output = StructType>,
    TemplateVec: TemplateNameGetter,
> {
    abstract_type: &'a AbstractType,
    linker_types: &'a TypVec,
    template_names: &'a TemplateVec,
}

impl<'a, TypVec: Index<TypeUUID, Output = StructType>, TemplateVec: TemplateNameGetter> Display
    for AbstractTypeDisplay<'a, TypVec, TemplateVec>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.abstract_type {
            AbstractType::Unknown(id) => write!(f, "{id:?}"),
            AbstractType::Template(id) => f.write_str(self.template_names.get_template_name(*id)),
            AbstractType::Named(id) => {
                f.write_str(&self.linker_types[*id].link_info.get_full_name())
            }
            AbstractType::Array(sub) => write!(
                f,
                "{}[]",
                sub.deref().display(self.linker_types, self.template_names)
            ),
        }
    }
}

impl AbstractType {
    pub fn display<
        'a,
        TypVec: Index<TypeUUID, Output = StructType>,
        TemplateVec: TemplateNameGetter,
    >(
        &'a self,
        linker_types: &'a TypVec,
        template_names: &'a TemplateVec,
    ) -> impl Display + 'a {
        AbstractTypeDisplay {
            abstract_type: self,
            linker_types,
            template_names,
        }
    }
}

impl Display for HowDoWeKnowTheTemplateArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            HowDoWeKnowTheTemplateArg::Given => f.write_str("given"),
            HowDoWeKnowTheTemplateArg::Inferred => f.write_str("inferred"),
        }
    }
}

#[derive(Debug)]
struct ConcreteTypeDisplay<'a, TypVec: Index<TypeUUID, Output = StructType>> {
    concrete_type: &'a ConcreteType,
    linker_types: &'a TypVec,
}

impl<'a, TypVec: Index<TypeUUID, Output = StructType>> Display for ConcreteTypeDisplay<'a, TypVec> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.concrete_type {
            ConcreteType::Named(name) => {
                f.write_str(&self.linker_types[name.id].link_info.get_full_name())?;
                f.write_char('(')?;
                let mut template_args_it = name.template_args.iter().peekable();
                while let Some((_, template_arg)) = template_args_it.next() {
                    template_arg.display(self.linker_types).fmt(f)?;
                    if template_args_it.peek().is_some() {
                        f.write_str(", ")?;
                    }
                }
                f.write_char(')')
            }
            ConcreteType::Array(arr_box) => {
                let (elem_typ, arr_size) = arr_box.deref();
                write!(
                    f,
                    "{}[{}]",
                    elem_typ.display(self.linker_types),
                    arr_size.unwrap_value().unwrap_integer()
                )
            }
            ConcreteType::Value(v) => write!(f, "{{concrete_type_{v}}}"),
            ConcreteType::Unknown(u) => write!(f, "{{{u:?}}}"),
        }
    }
}

impl ConcreteType {
    pub fn display<'a, TypVec: Index<TypeUUID, Output = StructType>>(
        &'a self,
        linker_types: &'a TypVec,
    ) -> impl Display + 'a {
        ConcreteTypeDisplay {
            concrete_type: self,
            linker_types,
        }
    }
}

#[derive(Debug)]
struct ConcreteTemplateArgDisplay<'a, TypVec: Index<TypeUUID, Output = StructType>> {
    concrete_template_arg: &'a ConcreteTemplateArg,
    linker_types: &'a TypVec,
}

impl<'a, TypVec: Index<TypeUUID, Output = StructType>> Display
    for ConcreteTemplateArgDisplay<'a, TypVec>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}",
            self.concrete_template_arg.kind.display(self.linker_types),
            self.concrete_template_arg.source
        )
    }
}

impl ConcreteTemplateArg {
    pub fn display<'a, TypVec: Index<TypeUUID, Output = StructType>>(
        &'a self,
        linker_types: &'a TypVec,
    ) -> impl Display + 'a {
        ConcreteTemplateArgDisplay {
            concrete_template_arg: self,
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
            spans_print.push((format!("{id:?}"), span.into_range()));
        }
        pretty_print_many_spans(file_data, &spans_print);
    }
}

pub fn pretty_print_concrete_instance<TypVec>(
    target_link_info: &LinkInfo,
    given_template_args: &ConcreteTemplateArgs,
    linker_types: &TypVec,
) -> String
where
    TypVec: Index<TypeUUID, Output = StructType>,
{
    assert!(given_template_args.len() == target_link_info.template_arguments.len());
    let object_full_name = target_link_info.get_full_name();
    if given_template_args.len() == 0 {
        return format!("{object_full_name} #()");
    }

    let mut result = format!("{object_full_name} #(\n");
    for (id, arg) in given_template_args {
        let arg_in_target = &target_link_info.template_arguments[id];
        write!(
            result,
            "    {}: {}",
            arg_in_target.name,
            arg.display(linker_types)
        )
        .unwrap();
    }

    result.push(')');
    result
}
