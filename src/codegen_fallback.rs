use std::{iter::zip, ops::Deref};

use crate::{ast::Module, instantiation::{ConnectToPathElem, InstantiatedModule, RealWireDataSource}, linker::{get_builtin_type, TypeUUID}, typing::ConcreteType, tokenizer::get_token_type_name, flattening::Instruction};

fn get_type_name_size(id : TypeUUID) -> u64 {
    if id == get_builtin_type("int") {
        32 // TODO concrete int sizes
    } else if id == get_builtin_type("bool") {
        1
    } else {
        println!("TODO Named Structs Size");
        1 // todo!() // Named structs are not implemented yet
    }
}

fn arr_str(sz : u64) -> String {
    format!("[{}:0]", sz - 1)
}

fn typ_to_verilog_array(typ : &ConcreteType) -> String {
    match typ {
        ConcreteType::Named(id) => {
            let sz = get_type_name_size(*id);
            if sz == 1 {
                String::new()
            } else {
                arr_str(sz)
            }
        }
        ConcreteType::Array(arr) => {
            let (sub_typ, size) = arr.deref();
            typ_to_verilog_array(sub_typ) + &arr_str(*size)
        }
    }
}

pub fn write_path_to_string(instance : &InstantiatedModule, path : &[ConnectToPathElem]) -> String {
    let mut result = String::new();
    for path_elem in path {
        match path_elem {
            ConnectToPathElem::MuxArrayWrite{idx_wire} => {
                result.push('[');
                result.push_str(&instance.wires[*idx_wire].name);
                result.push(']');
            }
            ConnectToPathElem::ConstArrayWrite{idx} => {
                result.push_str(&format!("[{idx}]"));
            }
        }
    }
    result
}

pub fn write_verilog_code<Stream : std::fmt::Write>(md : &Module, instance : &InstantiatedModule, program_text : &mut Stream) -> Result<(), std::fmt::Error> {
    // First output the interface of the module
    writeln!(program_text, "module {}(", md.link_info.name)?;
    writeln!(program_text, "\tinput clk,")?;
    for (real_port, is_input) in instance.interface.iter() {
        let wire = &instance.wires[real_port];
        let input_or_output = if is_input {"input"} else {"output /*mux_wire*/ reg"};
        let wire_typ = typ_to_verilog_array(&wire.typ);
        let wire_name = &wire.name;
        writeln!(program_text, "\t{input_or_output}{wire_typ} {wire_name},")?;
    }
    writeln!(program_text, ");\n")?;

    // Then output all declarations, and the wires we can already assign
    for (_id, w) in &instance.wires {
        if let Instruction::Declaration(wire_decl) = &md.flattened.instructions[w.original_wire] {
            // Don't print named inputs and outputs, already did that in interface
            if wire_decl.identifier_type.is_port() {
                continue;
            }
        }
        let wire_or_reg = if let RealWireDataSource::Multiplexer{is_state, sources: _} = &w.source {
            if is_state.is_some() {
                "reg"
            } else {
                "/*mux_wire*/ reg"
            }
        } else {"wire"};

        let wire_name = &w.name;
        let type_str = typ_to_verilog_array(&w.typ);
        write!(program_text, "{wire_or_reg}{type_str} {wire_name}")?;

        match &w.source {
            RealWireDataSource::UnaryOp { op, right } => {
                writeln!(program_text, " = {}{};", get_token_type_name(op.op_typ), instance.wires[*right].name)?;
            }
            RealWireDataSource::BinaryOp { op, left, right } => {
                writeln!(program_text, " = {} {} {};", instance.wires[*left].name, get_token_type_name(op.op_typ), instance.wires[*right].name)?;
            }
            RealWireDataSource::ArrayAccess { arr, arr_idx } => {
                writeln!(program_text, " = {}[{}];", instance.wires[*arr].name, instance.wires[*arr_idx].name)?;
            }
            RealWireDataSource::ConstArrayAccess { arr, arr_idx } => {
                writeln!(program_text, " = {}[{arr_idx}];", instance.wires[*arr].name)?;
            }
            RealWireDataSource::Constant { value } => {
                writeln!(program_text, " = {};", value.to_string())?;
            }
            RealWireDataSource::ReadOnly => {
                writeln!(program_text, ";")?;
            }
            RealWireDataSource::Multiplexer{is_state, sources : _} => {
                writeln!(program_text, ";")?;
                if let Some(initial_value) = is_state {
                    if initial_value.is_valid() {
                        let initial_value_str = initial_value.to_string();
                        writeln!(program_text, "initial {wire_name} = {initial_value_str};")?;
                    }
                }
            }
        }
    }
    
    // Output all submodules
    for (_id, sm) in &instance.submodules {
        let sm_instance_name = &sm.instance.name;
        let sm_name = &sm.name;
        writeln!(program_text, "{sm_instance_name} {sm_name}(")?;
        writeln!(program_text, ".clk(clk),")?;
        for (port, wire) in zip(sm.instance.interface.iter(), sm.wires.iter()) {
            let port_name = &sm.instance.wires[port.0].name;
            let wire_name = &instance.wires[wire.0].name;
            writeln!(program_text, "\t.{port_name}({wire_name}),")?;
        }
        writeln!(program_text, ");")?;
    }

    // For multiplexers, output 
    for (_id, w) in &instance.wires {
        match &w.source {
            RealWireDataSource::ReadOnly => {}
            RealWireDataSource::Multiplexer{is_state, sources} => {
                let output_name = w.name.deref();
                if is_state.is_some() {
                    writeln!(program_text, "/*always_ff*/ always @(posedge clk) begin")?;
                } else {
                    writeln!(program_text, "/*always_comb*/ always @(*) begin")?;
                    writeln!(program_text, "\t{output_name} <= 1'bX; // Combinatorial wires are not defined when not valid")?;
                }
                
                for s in sources {
                    let path = write_path_to_string(instance, &s.path);
                    let from_name = instance.wires[s.from.from].name.deref();
                    if let Some(cond) = s.from.condition {
                        let cond = instance.wires[cond].name.deref();
                        writeln!(program_text, "\tif({cond}) begin {output_name}{path} <= {from_name}; end")?;
                    } else {
                        writeln!(program_text, "\t{output_name}{path} <= {from_name};")?;
                    }
                }
                writeln!(program_text, "end")?;
            }
            RealWireDataSource::UnaryOp{op : _, right : _} => {}
            RealWireDataSource::BinaryOp{op : _, left : _, right : _} => {}
            RealWireDataSource::ArrayAccess{arr : _, arr_idx : _} => {}
            RealWireDataSource::ConstArrayAccess{arr : _, arr_idx : _} => {}
            RealWireDataSource::Constant{value : _} => {}
        }
    }

    writeln!(program_text, "endmodule")?;

    Ok(())
}

pub fn gen_verilog_code(md : &Module, instance : &InstantiatedModule) -> String {
    let mut program_text = String::new();

    write_verilog_code(md, instance, &mut program_text).unwrap();

    program_text
}
