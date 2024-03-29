use std::{iter::zip, ops::Deref};

use crate::{ast::Module, flattening::Instruction, instantiation::{ConnectToPathElem, InstantiatedModule, RealWire, RealWireDataSource, WireID}, linker::{get_builtin_type, TypeUUID}, typing::ConcreteType};

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

struct CodeGenerationContext<'g, 'out, Stream : std::fmt::Write> {
    md : &'g Module,
    instance : &'g InstantiatedModule,
    program_text : &'out mut Stream,

    use_latency : bool
}

fn wire_name_with_latency(wire : &RealWire, absolute_latency : i64, use_latency : bool) -> String {
    if use_latency && (wire.absolute_latency != absolute_latency) {
        format!("{}_D{}", wire.name, absolute_latency)
    } else {
        wire.name.to_string()
    }
}

fn wire_name_self_latency(wire : &RealWire, use_latency : bool) -> String {
    wire_name_with_latency(wire, wire.absolute_latency, use_latency)
}

impl<'g, 'out, Stream : std::fmt::Write> CodeGenerationContext<'g, 'out, Stream> { 
    fn wire_name(&self, wire_id : WireID, requested_latency : i64) -> String {
        let wire = &self.instance.wires[wire_id];
        wire_name_with_latency(wire, requested_latency, self.use_latency)
    }

    fn write_path_to_string(&self, path : &[ConnectToPathElem], absolute_latency : i64) -> String {
        let mut result = String::new();
        for path_elem in path {
            result.push_str(&match path_elem {
                ConnectToPathElem::MuxArrayWrite{idx_wire} => {
                    let idx_wire_name = self.wire_name(*idx_wire, absolute_latency);
                    format!("[{idx_wire_name}]")
                }
                ConnectToPathElem::ConstArrayWrite{idx} => {
                    format!("[{idx}]")
                }
            });
        }
        result
    }
    
    fn add_latency_registers(&mut self, w : &RealWire) -> Result<(), std::fmt::Error> {
        if self.use_latency {
            let type_str = typ_to_verilog_array(&w.typ);

            // Can do 0 iterations, when w.needed_until == w.absolute_latency. Meaning it's only needed this cycle
            for i in w.absolute_latency..w.needed_until {
                let from = wire_name_with_latency(w, i, self.use_latency);
                let to = wire_name_with_latency(w, i+1, self.use_latency);

                writeln!(self.program_text, "reg{type_str} {to}; always @(posedge clk) begin {to} <= {from}; end // Latency register")?;
            }
        }
        Ok(())
    }

    fn write_verilog_code(&mut self) -> Result<(), std::fmt::Error> {
        // First output the interface of the module
        writeln!(self.program_text, "module {}(", self.md.link_info.name)?;
        writeln!(self.program_text, "\tinput clk,")?;
        for (real_port, is_input) in self.instance.interface.iter() {
            let port_wire = &self.instance.wires[real_port];
            let input_or_output = if is_input {"input"} else {"output /*mux_wire*/ reg"};
            let wire_typ = typ_to_verilog_array(&port_wire.typ);
            let wire_name = wire_name_self_latency(port_wire, self.use_latency);
            writeln!(self.program_text, "\t{input_or_output}{wire_typ} {wire_name},")?;
        }
        writeln!(self.program_text, ");\n")?;
        for (real_port, _is_input) in self.instance.interface.iter() {
            let port_wire = &self.instance.wires[real_port];
            self.add_latency_registers(port_wire)?;
        }

        // Then output all declarations, and the wires we can already assign
        for (_id, w) in &self.instance.wires {
            if let Instruction::Declaration(wire_decl) = &self.md.flattened.instructions[w.original_wire] {
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

            let wire_name = wire_name_self_latency(w, self.use_latency);
            let type_str = typ_to_verilog_array(&w.typ);
            write!(self.program_text, "{wire_or_reg}{type_str} {wire_name}")?;

            match &w.source {
                RealWireDataSource::UnaryOp { op, right } => {
                    writeln!(self.program_text, " = {}{};", op.op_text(), self.wire_name(*right, w.absolute_latency))?;
                }
                RealWireDataSource::BinaryOp { op, left, right } => {
                    writeln!(self.program_text, " = {} {} {};", self.wire_name(*left, w.absolute_latency), op.op_text(), self.wire_name(*right, w.absolute_latency))?;
                }
                RealWireDataSource::ArrayAccess { arr, arr_idx } => {
                    writeln!(self.program_text, " = {}[{}];", self.wire_name(*arr, w.absolute_latency), self.wire_name(*arr_idx, w.absolute_latency))?;
                }
                RealWireDataSource::ConstArrayAccess { arr, arr_idx } => {
                    writeln!(self.program_text, " = {}[{arr_idx}];", self.wire_name(*arr, w.absolute_latency))?;
                }
                RealWireDataSource::Constant { value } => {
                    writeln!(self.program_text, " = {};", value.to_string())?;
                }
                RealWireDataSource::ReadOnly => {
                    writeln!(self.program_text, ";")?;
                }
                RealWireDataSource::Multiplexer{is_state, sources : _} => {
                    writeln!(self.program_text, ";")?;
                    if let Some(initial_value) = is_state {
                        if initial_value.is_valid() {
                            let initial_value_str = initial_value.to_string();
                            writeln!(self.program_text, "initial {wire_name} = {initial_value_str};")?;
                        }
                    }
                }
            }
            self.add_latency_registers(w)?;
        }
        
        // Output all submodules
        for (_id, sm) in &self.instance.submodules {
            let sm_instance_name = &sm.instance.name;
            let sm_name = &sm.name;
            writeln!(self.program_text, "{sm_instance_name} {sm_name}(")?;
            writeln!(self.program_text, "\t.clk(clk),")?;
            for (port, wire) in zip(sm.instance.interface.iter(), sm.wires.iter()) {
                let port_name = wire_name_self_latency(&sm.instance.wires[port.0], self.use_latency);
                let wire_name = wire_name_self_latency(&self.instance.wires[wire.0], self.use_latency);
                writeln!(self.program_text, "\t.{port_name}({wire_name}),")?;
            }
            writeln!(self.program_text, ");")?;
        }

        // For multiplexers, output 
        for (_id, w) in &self.instance.wires {
            match &w.source {
                RealWireDataSource::ReadOnly => {}
                RealWireDataSource::Multiplexer{is_state, sources} => {
                    let output_name = wire_name_self_latency(w, self.use_latency);
                    if is_state.is_some() {
                        writeln!(self.program_text, "/*always_ff*/ always @(posedge clk) begin")?;
                    } else {
                        writeln!(self.program_text, "/*always_comb*/ always @(*) begin")?;
                        writeln!(self.program_text, "\t{output_name} <= 1'bX; // Combinatorial wires are not defined when not valid")?;
                    }
                    
                    for s in sources {
                        let path = self.write_path_to_string(&s.path, w.absolute_latency);
                        let from_name = self.wire_name(s.from.from, w.absolute_latency);
                        if let Some(cond) = s.from.condition {
                            let cond_name = self.wire_name(cond, w.absolute_latency);
                            writeln!(self.program_text, "\tif({cond_name}) begin {output_name}{path} <= {from_name}; end")?;
                        } else {
                            writeln!(self.program_text, "\t{output_name}{path} <= {from_name};")?;
                        }
                    }
                    writeln!(self.program_text, "end")?;
                }
                RealWireDataSource::UnaryOp{op : _, right : _} => {}
                RealWireDataSource::BinaryOp{op : _, left : _, right : _} => {}
                RealWireDataSource::ArrayAccess{arr : _, arr_idx : _} => {}
                RealWireDataSource::ConstArrayAccess{arr : _, arr_idx : _} => {}
                RealWireDataSource::Constant{value : _} => {}
            }
        }

        writeln!(self.program_text, "endmodule")?;

        Ok(())
    }
}

pub fn gen_verilog_code(md : &Module, instance : &InstantiatedModule, use_latency : bool) -> String {
    let mut program_text = String::new();

    let mut ctx = CodeGenerationContext{md, instance, program_text: &mut program_text, use_latency};
    ctx.write_verilog_code().unwrap();

    program_text
}
