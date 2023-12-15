use std::{iter::zip, ops::Deref};

use crate::{ast::{Module, Value}, instantiation::{InstantiatedModule, RealWireDataSource, StateInitialValue, ConnectToPathElem}, linker::{Linker, NamedUUID, get_builtin_uuid}, arena_alloc::UUID, typing::ConcreteType, tokenizer::get_token_type_name};

fn get_type_name_size(id : NamedUUID) -> u64 {
    if id == get_builtin_uuid("int") {
        32 // TODO concrete int sizes
    } else if id == get_builtin_uuid("bool") {
        1 // TODO concrete int sizes
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

pub fn value_to_str(value : &Value) -> String {
    match value {
        Value::Bool(b) => if *b {"1'b1"} else {"1'b0"}.to_owned(),
        Value::Integer(v) => v.to_string(),
        Value::Invalid => "INVALID".to_owned()
    }
}

pub fn gen_verilog_code(md : &Module, instance : &InstantiatedModule, linker : &Linker) -> Option<String> {
    let mut program_text : String = format!("module {}(\n\tinput clk, \n", md.link_info.name);
    for (port, real_port) in zip(&md.interface.interface_wires, &instance.interface) {
        if real_port.id == UUID::INVALID {return None;}
        let wire = &instance.wires[real_port.id];
        program_text.push_str(if port.is_input {"\tinput"} else {"\toutput"});
        program_text.push_str(&typ_to_verilog_array(&wire.typ));
        program_text.push(' ');
        program_text.push_str(&wire.name);
        program_text.push_str(",\n");
    }
    program_text.push_str(");\n");

    for (id, w) in &instance.wires {
        let wire_or_reg = if let RealWireDataSource::Multiplexer{is_state: initial_value, sources} = &w.source {
            if let StateInitialValue::NotState = initial_value {
                "/*mux_wire*/ reg"
            } else {
                "reg"
            }
        } else {"wire"};

        program_text.push_str(wire_or_reg);
        program_text.push_str(&typ_to_verilog_array(&w.typ));
        program_text.push(' ');
        program_text.push_str(&w.name);
        program_text.push_str(";\n");
    }
    
    for (id, sm) in &instance.submodules {
        program_text.push_str(&sm.instance.name);
        program_text.push(' ');
        program_text.push_str(&sm.name);
        program_text.push_str("(\n.clk(clk)");
        for (port, wire) in zip(&sm.instance.interface, &sm.wires) {
            program_text.push_str(",\n.");
            program_text.push_str(&sm.instance.wires[port.id].name);
            program_text.push('(');
            program_text.push_str(&instance.wires[*wire].name);
            program_text.push_str(")");
        }
        program_text.push_str("\n);\n");
    }

    for (id, w) in &instance.wires {
        match &w.source {
            RealWireDataSource::ReadOnly => {}
            RealWireDataSource::Multiplexer { is_state, sources } => {
                let output_name = w.name.deref();
                match is_state {
                    StateInitialValue::NotState => {
                        program_text.push_str(&format!("/*always_comb*/ always @(*) begin\n\t{output_name} <= 1'bX; // Not defined when not valid\n"));
                    }
                    StateInitialValue::State { initial_value } => {
                        program_text.push_str(&format!("/*always_ff*/ always @(posedge clk) begin\n"));
                    }
                }
                for s in sources {
                    let mut path = String::new();
                    for path_elem in &s.path {
                        match path_elem {
                            ConnectToPathElem::ArrayConnection { idx_wire } => {
                                path.push('[');
                                path.push_str(&instance.wires[*idx_wire].name);
                                path.push(']');
                            }
                        }
                    }
                    let from_name = instance.wires[s.from.from].name.deref();
                    if s.from.condition != UUID::INVALID {
                        let cond = instance.wires[s.from.condition].name.deref();
                        program_text.push_str(&format!("\tif({cond}) begin {output_name}{path} <= {from_name}; end\n"));
                    } else {
                        program_text.push_str(&format!("\t{output_name}{path} <= {from_name};\n"));
                    }
                }
                program_text.push_str("end\n");
            }
            RealWireDataSource::UnaryOp { op, right } => {
                program_text.push_str(&format!("assign {} = {}{};\n", w.name, get_token_type_name(op.op_typ), instance.wires[*right].name));
            }
            RealWireDataSource::BinaryOp { op, left, right } => {
                program_text.push_str(&format!("assign {} = {} {} {};\n", w.name, instance.wires[*left].name, get_token_type_name(op.op_typ), instance.wires[*right].name));
            }
            RealWireDataSource::ArrayAccess { arr, arr_idx } => {
                program_text.push_str(&format!("assign {} = {}[{}];\n", w.name, instance.wires[*arr].name, instance.wires[*arr_idx].name));
            }
            RealWireDataSource::Constant { value } => {
                program_text.push_str(&format!("assign {} = {};\n", w.name, value_to_str(value)));
            }
        }
    }

    program_text.push_str("endmodule\n");

    Some(program_text)
}
