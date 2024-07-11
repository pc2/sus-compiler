use crate::{arena_alloc::{FlatAlloc, UUIDMarker, UUID}, errors::ErrorCollector, file_position::{BracketSpan, Span}, flattening::{FlatID, WrittenType}, linker::LinkInfo, value::TypedValue};
use super::concrete_type::ConcreteType;


pub struct TemplateIDMarker;
impl UUIDMarker for TemplateIDMarker {const DISPLAY_NAME : &'static str = "template_arg_";}
pub type TemplateID = UUID<TemplateIDMarker>;



#[derive(Debug)]
pub struct GlobalReference<ID> {
    pub span: Span,
    pub id: ID,
    pub template_args: TemplateArgs,
    pub template_span: Option<BracketSpan>
}


#[derive(Debug)]
pub struct TemplateInput {
    pub name : String,
    pub name_span : Span,
    pub kind : TemplateInputKind
}

#[derive(Debug)]
pub struct GenerativeTemplateInputKind {
    pub decl_span : Span,
    pub declaration_instruction : FlatID
}

#[derive(Debug)]
/// TODO this isn't quite right, because WrittenType requires access to the instructions, and ostensibly types get executed beforehand. Look into it
pub struct TypeTemplateInputKind {
    pub default_value : Option<WrittenType>
}

#[derive(Debug)]
pub enum TemplateInputKind {
    Type(TypeTemplateInputKind),
    Generative(GenerativeTemplateInputKind)
}

impl TemplateInputKind {
    #[track_caller]
    pub fn unwrap_type(&self) -> &TypeTemplateInputKind {
        let Self::Type(t) = self else {unreachable!("TemplateInputKind::unwrap_type on {self:?}")};
        t
    }
    #[track_caller]
    pub fn unwrap_value(&self) -> &GenerativeTemplateInputKind {
        let Self::Generative(v) = self else {unreachable!("TemplateInputKind::unwrap_value on {self:?}")};
        v
    }
}

#[derive(Debug)]
pub struct TemplateArg {
    pub name_specification : Option<Span>,
    pub whole_span : Span,
    pub kind : TemplateArgKind
}

#[derive(Debug)]
pub enum TemplateArgKind {
    Type(WrittenType),
    Value(FlatID)
}

impl TemplateArgKind {
    #[track_caller]
    pub fn unwrap_type(&self) -> &WrittenType {
        let Self::Type(t) = self else {unreachable!("TemplateArgKind::unwrap_type on {self:?}")};
        t
    }
    #[track_caller]
    pub fn unwrap_value(&self) -> FlatID {
        let Self::Value(v) = self else {unreachable!("TemplateArgKind::unwrap_value on {self:?}")};
        *v
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConcreteTemplateArg {
    Type(ConcreteType),
    Value(TypedValue),
    NotProvided
}

pub type TemplateArgs = FlatAlloc<Option<TemplateArg>, TemplateIDMarker>;
pub type TemplateInputs = FlatAlloc<TemplateInput, TemplateIDMarker>;
pub type ConcreteTemplateArgs = FlatAlloc<ConcreteTemplateArg, TemplateIDMarker>;

pub fn check_all_template_args_valid(errors : &ErrorCollector, span : Span, target_link_info : &LinkInfo, template_args : &ConcreteTemplateArgs) -> bool {
    let mut not_found_list : Vec<&TemplateInput> = Vec::new();
    for (id, arg) in &target_link_info.template_arguments {
        match &template_args[id] {
            ConcreteTemplateArg::Type(_) => {}
            ConcreteTemplateArg::Value(_) => {}
            ConcreteTemplateArg::NotProvided => {
                not_found_list.push(arg);
            }
        }
    }
    if !not_found_list.is_empty() {
        let mut uncovered_ports_list = String::new();
        for v in &not_found_list {
            use std::fmt::Write;
            write!(uncovered_ports_list, "'{}', ", v.name).unwrap();
        }
        uncovered_ports_list.truncate(uncovered_ports_list.len() - 2); // Cut off last comma
        let err_ref = errors.error(span, format!("Could not instantiate {} because the template arguments {uncovered_ports_list} were missing and no default was provided", target_link_info.get_full_name()));
        for v in &not_found_list {
            err_ref.info((v.name_span, target_link_info.file), format!("'{}' defined here", v.name));
        }
        false
    } else {true}
}
