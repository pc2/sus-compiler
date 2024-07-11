
use crate::prelude::*;

use crate::{alloc::FlatAlloc, file_position::FileText, pretty_print_many_spans, value::Value};

use crate::flattening::{DomainInfo, Interface, InterfaceToDomainMap, Module, WrittenType};
use crate::linker::{LinkInfo, NamedType};
use crate::typing::{
    template::{ConcreteTemplateArg, ConcreteTemplateArgs, GenerativeTemplateInputKind, TemplateInputKind, TemplateInputs, TypeTemplateInputKind},
    abstract_type::{AbstractType, DomainType},
    concrete_type::ConcreteType,
};

use std::{fmt::{Display, Formatter}, ops::Index};

use std::fmt::Write;
use crate::linker::Linkable;
use std::ops::Deref;

pub fn map_to_type_names(template_inputs : &TemplateInputs) -> FlatAlloc<String, TemplateIDMarker> {
    template_inputs.map(|(_id, v)| v.name.clone())
}

pub trait TemplateNameGetter {
    fn get_template_name(&self, id : TemplateID) -> &str;
}

impl TemplateNameGetter for FlatAlloc<String, TemplateIDMarker> {
    fn get_template_name(&self, id : TemplateID) -> &str {
        &self[id]
    }
}
impl TemplateNameGetter for TemplateInputs {
    fn get_template_name(&self, id : TemplateID) -> &str {
        &self[id].name
    }
}

impl WrittenType {
    pub fn to_string<TypVec : Index<TypeUUID, Output = NamedType>, TemplateVec : TemplateNameGetter>(&self, linker_types : &TypVec, template_names : &TemplateVec) -> String {
        match self {
            WrittenType::Error(_) => {
                "{error}".to_owned()
            }
            WrittenType::Template(_, id) => {
                template_names.get_template_name(*id).to_owned()
            }
            WrittenType::Named(named_type) => {
                linker_types[named_type.id].get_full_name()
            }
            WrittenType::Array(_, sub) => sub.deref().0.to_string(linker_types, template_names) + "[]",
        }
    }
}

impl AbstractType {
    pub fn to_string<TypVec : Index<TypeUUID, Output = NamedType>, TemplateVec : TemplateNameGetter>(&self, linker_types : &TypVec, template_names : &TemplateVec) -> String {
        match self {
            AbstractType::Error => {
                "{error}".to_owned()
            }
            AbstractType::Unknown => {
                "{unknown}".to_owned()
            }
            AbstractType::Template(id) => {
                template_names.get_template_name(*id).to_owned()
            }
            AbstractType::Named(id) => {
                linker_types[*id].get_full_name()
            }
            AbstractType::Array(sub) => sub.deref().to_string(linker_types, template_names) + "[]",
        }
    }
}

impl ConcreteType {
    pub fn to_string<TypVec : Index<TypeUUID, Output = NamedType>>(&self, linker_types : &TypVec) -> String {
        match self {
            ConcreteType::Named(name) => linker_types[*name].get_full_name(),
            ConcreteType::Array(arr_box) => {
                let (elem_typ, arr_size) = arr_box.deref();
                format!("{}[{}]", elem_typ.to_string(linker_types), arr_size.unwrap_value().unwrap_integer())
            }
            ConcreteType::Value(v) => format!("{{concrete_type_{v}}}"),
            ConcreteType::Unknown => format!("{{concrete_type_unknown}}"),
            ConcreteType::Error => format!("{{concrete_type_error}}"),
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

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Bool(b) => if *b {"1'b1"} else {"1'b0"}.to_owned(),
            Value::Integer(v) => v.to_string(),
            Value::Array(arr_box) => {
                let mut result = "[".to_owned();
                for v in arr_box.iter() {
                    result.push_str(&v.to_string());
                    result.push_str(", ");
                }
                result.push(']');
                result
            }
            Value::Unset => "Value::Unset".to_owned(),
            Value::Error => "Value::Error".to_owned(),
        }
    }
}

impl DomainType {
    pub fn physical_to_string(physical_id : DomainID, domains : &FlatAlloc<DomainInfo, DomainIDMarker>) -> String {
        if let Some(interf) = domains.get(physical_id) {
            format!("{{{}}}", interf.name)
        } else {
            format!("{{unnamed domain {}}}", physical_id.get_hidden_value())
        }
    }
}

impl Module {
    pub fn make_port_info_fmt(&self, port_id : PortID, file_text : &FileText, result : &mut String) {
        use std::fmt::Write;
        let port = &self.ports[port_id];
        let port_direction = if port.is_input {"input"} else {"output"};
        writeln!(result, "{port_direction} {}", &file_text[port.decl_span]).unwrap()
    }
    pub fn make_port_info_string(&self, port_id : PortID, file_text : &FileText) -> String {
        let mut r = String::new(); self.make_port_info_fmt(port_id, file_text, &mut r); r
    }

    pub fn make_interface_info_fmt(&self, interface : &Interface, file_text : &FileText, result : &mut String) {
        for port_id in interface.all_ports() {
            self.make_port_info_fmt(port_id, file_text, result);
        }
    }
    pub fn make_interface_info_string(&self, interface : &Interface, file_text : &FileText) -> String {
        let mut r = String::new(); self.make_interface_info_fmt(interface, file_text, &mut r); r
    }

    pub fn make_all_ports_info_string(&self, file_text : &FileText, local_domains : Option<InterfaceToDomainMap>) -> String {
        use std::fmt::Write;

        let mut type_args : Vec<&str> = Vec::new();
        let mut temporary_gen_input_builder = String::new();
        for (_id, t) in &self.link_info.template_arguments {
            match &t.kind {
                TemplateInputKind::Type(TypeTemplateInputKind { default_value:_ }) => type_args.push(&t.name),
                TemplateInputKind::Generative(GenerativeTemplateInputKind { decl_span, declaration_instruction:_ }) => writeln!(temporary_gen_input_builder, "input gen {}", &file_text[*decl_span]).unwrap(), 
            }
        }

        let mut result = format!("module {}<{}>:\n", self.link_info.get_full_name(), type_args.join(", "));
        result.push_str(&temporary_gen_input_builder);

        for (domain_id, domain) in &self.domains {
            if let Some(domain_map) = &local_domains {
                writeln!(result, "domain {}: {{{}}}", &domain.name, domain_map.local_domain_to_global_domain(domain_id).name).unwrap();
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

    pub fn print_flattened_module(&self, file_text : &FileText) {
        println!("[[{}]]:", self.link_info.name);
        println!("Interface:");
        for (port_id, port) in &self.ports {
            println!("    {} -> {:?}", self.make_port_info_string(port_id, file_text), port);
        }
        println!("Instructions:");
        let mut spans_print = Vec::new();
        for (id, inst) in &self.instructions {
            println!("    {id:?}: {inst:?}");
            let span = self.get_instruction_span(id);
            spans_print.push((format!("{id:?}"), span.into_range()));
        }
        pretty_print_many_spans(file_text.file_text.clone(), &spans_print);
    }
}

pub fn pretty_print_concrete_instance(linker : &Linker, link_info : &LinkInfo, template_args : &ConcreteTemplateArgs) -> String {
    assert!(link_info.template_arguments.len() == template_args.len());

    let mut result = link_info.get_full_name();

    if !link_info.template_arguments.is_empty() {
        result.push_str("::<");

        for (id, input) in &link_info.template_arguments {
            let given_arg = &template_args[id];

            result.push_str(&input.name);
            match given_arg {
                ConcreteTemplateArg::Type(t) => {
                    write!(result, " = {}, ", t.to_string(&linker.types)).unwrap();
                }
                ConcreteTemplateArg::Value(v) => {
                    write!(result, " = {}, ", v.value).unwrap();
                }
                ConcreteTemplateArg::NotProvided => {
                    result.push_str(" not provided, ");
                }
            }
        }

        result.truncate(result.len() - 2);
        result.push_str(">");
    }

    result
}
