

use crate::{errors::ErrorCollector, file_position::Span, flattening::FlattenedModule, instantiation::InstantiationList, linker::FileUUID};
use core::ops::Range;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum IdentifierType {
    Input,
    Output,
    Local,
    State,
    Generative
}

impl IdentifierType {
    pub fn get_keyword(&self) -> &'static str {
        match self {
            IdentifierType::Input => "input",
            IdentifierType::Output => "output",
            IdentifierType::Local => "",
            IdentifierType::State => "state",
            IdentifierType::Generative => "gen",
        }
    }
    pub fn is_generative(&self) -> bool {
        *self == IdentifierType::Generative
    }
    pub fn is_port(&self) -> bool {
        *self == IdentifierType::Input || *self == IdentifierType::Output
    }
}

#[derive(Debug)]
pub struct LinkInfo {
    pub file : FileUUID,
    pub name : Box<str>,
    pub name_span : Span,
    pub span : Span
}

impl LinkInfo {
    pub fn get_full_name(&self) -> String {
        format!("::{}", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct InterfacePorts<ID : Clone + Copy> {
    pub outputs_start : usize,
    pub ports : Box<[ID]>
}

impl<ID : Clone + Copy> InterfacePorts<ID> {
    pub fn empty() -> Self {
        InterfacePorts{outputs_start : 0, ports : Box::new([])}
    }

    // Todo, just treat all inputs and outputs as function call interface
    pub fn func_call_syntax_inputs(&self) -> Range<usize> {
        0..self.outputs_start
    }
    pub fn func_call_syntax_outputs(&self) -> Range<usize> {
        self.outputs_start..self.ports.len()
    }
    pub fn inputs(&self) -> &[ID] {
        &self.ports[..self.outputs_start]
    }
    pub fn outputs(&self) -> &[ID] {
        &self.ports[self.outputs_start..]
    }

    pub fn map<OtherID : Clone + Copy, MapFn : FnMut(ID, /*is_input : */bool) -> OtherID>(&self, f : &mut MapFn) -> InterfacePorts<OtherID> {
        InterfacePorts{
            ports : self.ports.iter().enumerate().map(|(idx, v)| f(*v, idx < self.outputs_start)).collect(),
            outputs_start : self.outputs_start
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (ID, /*is_input : */bool)> + '_ {
        self.ports.iter().enumerate().map(|(idx, v)| (*v, idx < self.outputs_start))
    }
}

#[derive(Debug)]
pub struct Module {
    pub link_info : LinkInfo,

    pub flattened : FlattenedModule,

    pub instantiations : InstantiationList
}

impl Module {
    pub fn print_flattened_module(&self) {
        println!("[[{}]]:", self.link_info.name);
        println!("Interface:");
        for (port, is_input) in self.flattened.interface_ports.iter() {
            let port_direction = if is_input {"input"} else {"output"};
            let port_name = &self.flattened.instructions[port].extract_wire_declaration().name;
            println!("    {port_direction} {port_name} -> {:?}", port);
        }
        println!("Instantiations:");
        for (id, inst) in &self.flattened.instructions {
            println!("    {:?}: {:?}", id, inst);
        }
    }
}

#[derive(Debug)]
pub struct ASTRoot {
    pub modules : Vec<Module>,
    pub errors : ErrorCollector
}
