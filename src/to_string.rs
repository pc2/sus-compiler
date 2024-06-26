use crate::{abstract_type::AbstractType, arena_alloc::FlatAlloc, concrete_type::ConcreteType, flattening::WrittenType, linker::{LinkInfo, Linker, NamedType, TypeUUID}, template::{ConcreteTemplateArg, ConcreteTemplateArgs, TemplateIDMarker, TemplateInputs}, value::Value};

use std::{fmt::{Display, Formatter}, ops::Index};

use std::fmt::Write;
use crate::linker::Linkable;
use std::ops::Deref;

pub fn map_to_type_names(template_inputs : &TemplateInputs) -> FlatAlloc<String, TemplateIDMarker> {
    template_inputs.iter().map(|(_id, v)| v.name.clone()).collect()
}

impl WrittenType {
    pub fn to_string<TypVec : Index<TypeUUID, Output = NamedType>>(&self, linker_types : &TypVec, template_names : &FlatAlloc<String, TemplateIDMarker>) -> String {
        match self {
            WrittenType::Error(_) => {
                "{error}".to_owned()
            }
            WrittenType::Template(_, id) => {
                template_names[*id].clone()
            }
            WrittenType::Named(named_type) => {
                linker_types[named_type.id].get_full_name()
            }
            WrittenType::Array(_, sub) => sub.deref().0.to_string(linker_types, template_names) + "[]",
        }
    }
}

impl AbstractType {
    pub fn to_string<TypVec : Index<TypeUUID, Output = NamedType>>(&self, linker_types : &TypVec, template_names : &FlatAlloc<String, TemplateIDMarker>) -> String {
        match self {
            AbstractType::Error => {
                "{error}".to_owned()
            }
            AbstractType::Unknown => {
                "{unknown}".to_owned()
            }
            AbstractType::Template(id) => {
                template_names[*id].clone()
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
