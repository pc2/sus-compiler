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

pub fn gen_verilog_code(md : &Module, instance : &InstantiatedModule) -> String {
    let mut program_text : String = format!("module {}(\n\tinput clk, \n", md.link_info.name);
    for (real_port, is_input) in instance.interface.iter() {
        let wire = &instance.wires[real_port];
        program_text.push_str(if is_input {"\tinput"} else {"\toutput /*mux_wire*/ reg"});
        program_text.push_str(&typ_to_verilog_array(&wire.typ));
        program_text.push(' ');
        program_text.push_str(&wire.name);
        program_text.push_str(",\n");
    }
    program_text.push_str(");\n");

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
        program_text.push_str(wire_or_reg);
        program_text.push_str(&typ_to_verilog_array(&w.typ));
        program_text.push(' ');
        program_text.push_str(wire_name);

        match &w.source {
            RealWireDataSource::UnaryOp { op, right } => {
                program_text.push_str(&format!(" = {}{};\n", get_token_type_name(op.op_typ), instance.wires[*right].name));
            }
            RealWireDataSource::BinaryOp { op, left, right } => {
                program_text.push_str(&format!(" = {} {} {};\n", instance.wires[*left].name, get_token_type_name(op.op_typ), instance.wires[*right].name));
            }
            RealWireDataSource::ArrayAccess { arr, arr_idx } => {
                program_text.push_str(&format!(" = {}[{}];\n", instance.wires[*arr].name, instance.wires[*arr_idx].name));
            }
            RealWireDataSource::ConstArrayAccess { arr, arr_idx } => {
                program_text.push_str(&format!(" = {}[{arr_idx}];\n", instance.wires[*arr].name));
            }
            RealWireDataSource::Constant { value } => {
                program_text.push_str(&format!(" = {};\n", value.to_string()));
            }
            RealWireDataSource::ReadOnly => {
                program_text.push_str(";\n");
            }
            RealWireDataSource::Multiplexer{is_state, sources : _} => {
                program_text.push_str(";\n");
                if let Some(initial_value) = is_state {
                    if initial_value.is_valid() {
                        let initial_value_str = initial_value.to_string();
                        program_text.push_str(&format!("initial {wire_name} = {initial_value_str};\n"));
                    }
                }
            }
        }
    }
    
    for (_id, sm) in &instance.submodules {
        program_text.push_str(&sm.instance.name);
        program_text.push(' ');
        program_text.push_str(&sm.name);
        program_text.push_str("(\n.clk(clk)");
        for (port, wire) in zip(sm.instance.interface.iter(), sm.wires.iter()) {
            program_text.push_str(",\n.");
            program_text.push_str(&sm.instance.wires[port.0].name);
            program_text.push('(');
            program_text.push_str(&instance.wires[wire.0].name);
            program_text.push_str(")");
        }
        program_text.push_str("\n);\n");
    }

    for (_id, w) in &instance.wires {
        match &w.source {
            RealWireDataSource::ReadOnly => {}
            RealWireDataSource::Multiplexer{is_state, sources} => {
                let output_name = w.name.deref();
                if is_state.is_some() {
                    program_text.push_str(&format!("/*always_ff*/ always @(posedge clk) begin\n"));
                } else {
                    program_text.push_str(&format!("/*always_comb*/ always @(*) begin\n\t{output_name} <= 1'bX; // Combinatorial wires are not defined when not valid\n"));
                }
                
                for s in sources {
                    let path = write_path_to_string(instance, &s.path);
                    let from_name = instance.wires[s.from.from].name.deref();
                    if let Some(cond) = s.from.condition {
                        let cond = instance.wires[cond].name.deref();
                        program_text.push_str(&format!("\tif({cond}) begin {output_name}{path} <= {from_name}; end\n"));
                    } else {
                        program_text.push_str(&format!("\t{output_name}{path} <= {from_name};\n"));
                    }
                }
                program_text.push_str("end\n");
            }
            RealWireDataSource::UnaryOp{op : _, right : _} => {}
            RealWireDataSource::BinaryOp{op : _, left : _, right : _} => {}
            RealWireDataSource::ArrayAccess{arr : _, arr_idx : _} => {}
            RealWireDataSource::ConstArrayAccess{arr : _, arr_idx : _} => {}
            RealWireDataSource::Constant{value : _} => {}
        }
    }

    program_text.push_str("endmodule\n");

    program_text
}
