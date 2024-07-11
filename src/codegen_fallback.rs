use std::ops::Deref;

use crate::prelude::*;

use crate::flattening::{DeclarationPortInfo, Instruction, Module};
use crate::instantiation::{
    InstantiatedModule, RealWire, RealWireDataSource, RealWirePathElem, CALCULATE_LATENCY_LATER,
};
use crate::{linker::get_builtin_type, typing::concrete_type::ConcreteType, value::Value};

fn get_type_name_size(id: TypeUUID) -> u64 {
    if id == get_builtin_type("int") {
        32 // TODO concrete int sizes
    } else if id == get_builtin_type("bool") {
        1
    } else {
        println!("TODO Named Structs Size");
        1 // todo!() // Named structs are not implemented yet
    }
}

/// Creates the Verilog variable declaration for tbis variable.
///
/// IE for `int[15] myVar` it creates `logic[31:0] myVar[14:0]`
fn typ_to_verilog_array(typ: &ConcreteType, var_name: &str) -> String {
    match typ {
        ConcreteType::Named(id) => {
            let sz = get_type_name_size(*id);
            if sz == 1 {
                format!("logic {var_name}")
            } else {
                format!("logic[{}:0] {var_name}", sz - 1)
            }
        }
        ConcreteType::Array(arr) => {
            let (content_typ, size) = arr.deref();
            let sz = size.unwrap_value().unwrap_integer();
            let mut result = typ_to_verilog_array(content_typ, var_name);
            use std::fmt::Write;
            write!(result, "[{}:0]", sz - 1).unwrap();
            result
        }
        ConcreteType::Value(_) | ConcreteType::Unknown | ConcreteType::Error => unreachable!(),
    }
}

struct CodeGenerationContext<'g, 'out, Stream: std::fmt::Write> {
    md: &'g Module,
    instance: &'g InstantiatedModule,
    program_text: &'out mut Stream,

    use_latency: bool,
}

fn wire_name_with_latency(wire: &RealWire, absolute_latency: i64, use_latency: bool) -> String {
    assert!(wire.absolute_latency <= absolute_latency);
    assert!(wire.needed_until >= absolute_latency);

    if use_latency && (wire.absolute_latency != absolute_latency) {
        format!("{}_D{}", wire.name, absolute_latency)
    } else {
        wire.name.to_string()
    }
}

fn wire_name_self_latency(wire: &RealWire, use_latency: bool) -> String {
    wire_name_with_latency(wire, wire.absolute_latency, use_latency)
}

impl<'g, 'out, Stream: std::fmt::Write> CodeGenerationContext<'g, 'out, Stream> {
    /// This is for making the resulting Verilog a little nicer to read
    fn can_inline(&self, wire: &RealWire) -> bool {
        match &wire.source {
            RealWireDataSource::Constant { value } => match value {
                Value::Bool(_) | Value::Integer(_) => true,
                _other => false,
            },
            _other => false,
        }
    }

    fn operation_to_string(&self, wire: &RealWire) -> String {
        match &wire.source {
            RealWireDataSource::Constant { value } => value.to_string(),
            _other => unreachable!(),
        }
    }

    fn wire_name(&self, wire_id: WireID, requested_latency: i64) -> String {
        let wire = &self.instance.wires[wire_id];
        if self.can_inline(wire) {
            self.operation_to_string(wire)
        } else {
            wire_name_with_latency(wire, requested_latency, self.use_latency)
        }
    }

    fn wire_ref_path_to_string(&self, path: &[RealWirePathElem], absolute_latency: i64) -> String {
        let mut result = String::new();
        for path_elem in path {
            result.push_str(&match path_elem {
                RealWirePathElem::ArrayAccess { span: _, idx_wire } => {
                    let idx_wire_name = self.wire_name(*idx_wire, absolute_latency);
                    format!("[{idx_wire_name}]")
                }
            });
        }
        result
    }

    fn add_latency_registers(&mut self, w: &RealWire) -> Result<(), std::fmt::Error> {
        if self.use_latency {
            // Can do 0 iterations, when w.needed_until == w.absolute_latency. Meaning it's only needed this cycle
            assert!(w.absolute_latency != CALCULATE_LATENCY_LATER);
            assert!(w.needed_until != CALCULATE_LATENCY_LATER);
            for i in w.absolute_latency..w.needed_until {
                let from = wire_name_with_latency(w, i, self.use_latency);
                let to = wire_name_with_latency(w, i + 1, self.use_latency);

                let var_decl = typ_to_verilog_array(&w.typ, &to);

                writeln!(
                    self.program_text,
                    "/*latency*/ {var_decl}; always_ff @(posedge clk) begin {to} <= {from}; end"
                )?;
            }
        }
        Ok(())
    }

    fn write_verilog_code(&mut self) -> Result<(), std::fmt::Error> {
        // First output the interface of the module
        writeln!(self.program_text, "module {}(", mangle(&self.instance.name))?;
        write!(self.program_text, "\tinput clk")?;
        for (_id, port) in self.instance.interface_ports.iter_valids() {
            let port_wire = &self.instance.wires[port.wire];
            let input_or_output = if port.is_input { "input" } else { "output" };
            let wire_doc = port_wire.source.get_sv_info_doc();
            let wire_name = wire_name_self_latency(port_wire, self.use_latency);
            let wire_decl = typ_to_verilog_array(&port_wire.typ, &wire_name);
            write!(
                self.program_text,
                ",\n\t{wire_doc}{input_or_output} {wire_decl}"
            )?;
        }
        writeln!(self.program_text, "")?;
        writeln!(self.program_text, ");\n")?;
        for (_id, port) in self.instance.interface_ports.iter_valids() {
            let port_wire = &self.instance.wires[port.wire];
            self.add_latency_registers(port_wire)?;
        }

        // Then output all declarations, and the wires we can already assign
        for (_id, w) in &self.instance.wires {
            // For better readability of output Verilog
            if self.can_inline(w) {
                continue;
            }

            if let Instruction::Declaration(wire_decl) =
                &self.md.instructions[w.original_instruction]
            {
                // Don't print named inputs and outputs, already did that in interface
                if let DeclarationPortInfo::RegularPort { .. } = wire_decl.is_port {
                    continue;
                }
            }
            let wire_or_reg = w.source.get_sv_info_doc();

            let wire_name = wire_name_self_latency(w, self.use_latency);
            let wire_decl = typ_to_verilog_array(&w.typ, &wire_name);
            write!(self.program_text, "{wire_or_reg}{wire_decl}")?;

            match &w.source {
                RealWireDataSource::Select { root, path } => {
                    let wire_name = self.wire_name(*root, w.absolute_latency);
                    let path = self.wire_ref_path_to_string(&path, w.absolute_latency);
                    writeln!(self.program_text, " = {wire_name}{path};")?;
                }
                RealWireDataSource::UnaryOp { op, right } => {
                    writeln!(
                        self.program_text,
                        " = {}{};",
                        op.op_text(),
                        self.wire_name(*right, w.absolute_latency)
                    )?;
                }
                RealWireDataSource::BinaryOp { op, left, right } => {
                    writeln!(
                        self.program_text,
                        " = {} {} {};",
                        self.wire_name(*left, w.absolute_latency),
                        op.op_text(),
                        self.wire_name(*right, w.absolute_latency)
                    )?;
                }
                RealWireDataSource::Constant { value } => {
                    writeln!(self.program_text, " = {};", value.to_string())?;
                }
                RealWireDataSource::ReadOnly => {
                    writeln!(self.program_text, ";")?;
                }
                RealWireDataSource::Multiplexer {
                    is_state,
                    sources: _,
                } => {
                    writeln!(self.program_text, ";")?;
                    if let Some(initial_value) = is_state {
                        if initial_value.is_valid() {
                            let initial_value_str = initial_value.to_string();
                            writeln!(
                                self.program_text,
                                "initial {wire_name} = {initial_value_str};"
                            )?;
                        }
                    }
                }
            }
            self.add_latency_registers(w)?;
        }

        // Output all submodules
        for (_id, sm) in &self.instance.submodules {
            let sm_inst: &InstantiatedModule = sm
                .instance
                .as_ref()
                .expect("Invalid submodules are impossible to remain by the time codegen happens");
            let sm_instance_name = mangle(&sm_inst.name);
            let sm_name = &sm.name;
            writeln!(self.program_text, "{sm_instance_name} {sm_name}(")?;
            write!(self.program_text, "\t.clk(clk)")?;
            for (port_id, iport) in sm_inst.interface_ports.iter_valids() {
                let port_name =
                    wire_name_self_latency(&sm_inst.wires[iport.wire], self.use_latency);
                let wire_name = if let Some(port_wire) = &sm.port_map[port_id] {
                    wire_name_self_latency(
                        &self.instance.wires[port_wire.maps_to_wire],
                        self.use_latency,
                    )
                } else {
                    // Ports that are defined on the submodule, but not used by impl
                    String::new()
                };
                write!(self.program_text, ",\n\t.{port_name}({wire_name})")?;
            }
            writeln!(self.program_text, "\n);")?;
        }

        // For multiplexers, output
        for (_id, w) in &self.instance.wires {
            match &w.source {
                RealWireDataSource::Multiplexer { is_state, sources } => {
                    let output_name = wire_name_self_latency(w, self.use_latency);
                    if is_state.is_some() {
                        writeln!(self.program_text, "always_ff @(posedge clk) begin")?;
                    } else {
                        writeln!(self.program_text, "always_comb begin")?;
                        writeln!(self.program_text, "\t{output_name} <= 1'bX; // Combinatorial wires are not defined when not valid")?;
                    }

                    for s in sources {
                        let path = self.wire_ref_path_to_string(&s.to_path, w.absolute_latency);
                        let from_name = self.wire_name(s.from.from, w.absolute_latency);
                        if let Some(cond) = s.from.condition {
                            let cond_name = self.wire_name(cond, w.absolute_latency);
                            writeln!(
                                self.program_text,
                                "\tif({cond_name}) begin {output_name}{path} <= {from_name}; end"
                            )?;
                        } else {
                            writeln!(self.program_text, "\t{output_name}{path} <= {from_name};")?;
                        }
                    }
                    writeln!(self.program_text, "end")?;
                }
                RealWireDataSource::ReadOnly => {}
                RealWireDataSource::Select { root: _, path: _ } => {}
                RealWireDataSource::UnaryOp { op: _, right: _ } => {}
                RealWireDataSource::BinaryOp {
                    op: _,
                    left: _,
                    right: _,
                } => {}
                RealWireDataSource::Constant { value: _ } => {}
            }
        }

        writeln!(self.program_text, "endmodule\n")?;

        Ok(())
    }
}

impl RealWireDataSource {
    fn get_sv_info_doc(&self) -> &str {
        match self {
            RealWireDataSource::Multiplexer {
                is_state: Some(_),
                sources: _,
            } => "/*state*/ ",
            RealWireDataSource::Multiplexer {
                is_state: None,
                sources: _,
            } => "/*mux_wire*/ ",
            _ => "",
        }
    }
}

pub fn gen_verilog_code(md: &Module, instance: &InstantiatedModule, use_latency: bool) -> String {
    let mut program_text = String::new();

    let mut ctx = CodeGenerationContext {
        md,
        instance,
        program_text: &mut program_text,
        use_latency,
    };
    ctx.write_verilog_code().unwrap();

    program_text
}

pub fn mangle(str: &str) -> String {
    let mut result = String::with_capacity(str.len());
    for c in str.chars() {
        if c.is_whitespace() || c == ':' {
            continue;
        }
        result.push(if c.is_alphanumeric() { c } else { '_' });
    }
    result
}
