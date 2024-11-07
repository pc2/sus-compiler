use std::borrow::Cow;
use std::ops::Deref;

use crate::linker::IsExtern;
use crate::prelude::*;

use crate::flattening::{DeclarationPortInfo, Instruction, Module, Port};
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

/// Creates the Verilog variable declaration for tbis variable.
///
/// IE for `int[15] myVar` it creates `[31:0] myVar[14:0]`
fn typ_to_declaration(mut typ: &ConcreteType, var_name: &str) -> String {
    let mut array_string = String::new();
    while let ConcreteType::Array(arr) = typ {
        let (content_typ, size) = arr.deref();
        let sz = size.unwrap_value().unwrap_integer();
        use std::fmt::Write;
        write!(array_string, "[{}:0]", sz - 1).unwrap();
        typ = content_typ;
    }
    match typ {
        ConcreteType::Named(id) => {
            let sz = get_type_name_size(*id);
            if sz == 1 {
                format!("{array_string} {var_name}")
            } else {
                format!("{array_string}[{}:0] {var_name}", sz - 1)
            }
        }
        ConcreteType::Array(_) => unreachable!("All arrays have been used up already"),
        ConcreteType::Value(_) | ConcreteType::Unknown(_) => unreachable!(),
    }
}

fn wire_name_with_latency(wire: &RealWire, absolute_latency: i64, use_latency: bool) -> Cow<str> {
    assert!(wire.absolute_latency <= absolute_latency);

    if use_latency && (wire.absolute_latency != absolute_latency) {
        if absolute_latency < 0 {
            Cow::Owned(format!("_{}_N{}", wire.name, -absolute_latency))
        } else {
            Cow::Owned(format!("_{}_D{}", wire.name, absolute_latency))
        }
    } else {
        Cow::Borrowed(&wire.name)
    }
}

fn wire_name_self_latency(wire: &RealWire, use_latency: bool) -> Cow<str> {
    wire_name_with_latency(wire, wire.absolute_latency, use_latency)
}

struct CodeGenerationContext<'g, 'out, Stream: std::fmt::Write> {
    md: &'g Module,
    instance: &'g InstantiatedModule,
    program_text: &'out mut Stream,

    use_latency: bool,

    needed_untils : FlatAlloc<i64, WireIDMarker>
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

    fn operation_to_string(&self, wire: &'g RealWire) -> Cow<'g, str> {
        assert!(self.can_inline(wire));
        match &wire.source {
            RealWireDataSource::Constant { value } => value.inline_constant_to_string(),
            _other => unreachable!(),
        }
    }

    fn wire_name(&self, wire_id: WireID, requested_latency: i64) -> Cow<'g, str> {
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

    fn add_latency_registers(&mut self, wire_id: WireID, w: &RealWire) -> Result<(), std::fmt::Error> {
        if self.use_latency {
            // Can do 0 iterations, when w.needed_until == w.absolute_latency. Meaning it's only needed this cycle
            assert!(w.absolute_latency != CALCULATE_LATENCY_LATER);
            assert!(self.needed_untils[wire_id] != CALCULATE_LATENCY_LATER);
            for i in w.absolute_latency..self.needed_untils[wire_id] {
                let from = wire_name_with_latency(w, i, self.use_latency);
                let to = wire_name_with_latency(w, i + 1, self.use_latency);

                let var_decl = typ_to_declaration(&w.typ, &to);

                writeln!(
                    self.program_text,
                    "/*latency*/ logic {var_decl}; always_ff @(posedge clk) begin {to} <= {from}; end"
                ).unwrap();
            }
        }
        Ok(())
    }

    fn write_verilog_code(&mut self) {
        match self.md.link_info.is_extern {
            IsExtern::Normal => {
                self.write_module_signature(false);
                self.write_wire_declarations();
                self.write_submodules();
                self.write_multiplexers();
                self.write_endmodule();
            }
            IsExtern::Extern => {
                // Do nothing, it's provided externally
                writeln!(self.program_text, "// Provided externally").unwrap();
                self.write_module_signature(true);
            }
            IsExtern::Builtin => {
                self.write_module_signature(false);
                self.write_builtins();
                self.write_endmodule();
            }
        }
    }

    fn write_module_signature(&mut self, commented_out : bool) {
        let comment_text = if commented_out { "// " } else { "" };
        // First output the interface of the module
        write!(self.program_text, "{comment_text}module {}(\n{comment_text}\tinput clk", mangle(&self.instance.name)).unwrap();
        for (_id, port) in self.instance.interface_ports.iter_valids() {
            let port_wire = &self.instance.wires[port.wire];
            let input_or_output = if port.is_input { "input" } else { "output" };
            let wire_doc = port_wire.source.wire_or_reg();
            let wire_name = wire_name_self_latency(port_wire, self.use_latency);
            let wire_decl = typ_to_declaration(&port_wire.typ, &wire_name);
            write!(
                self.program_text,
                ",\n{comment_text}\t{input_or_output} {wire_doc} {wire_decl}"
            ).unwrap();
        }
        write!(self.program_text, "\n{comment_text});\n\n").unwrap();

        // Add latency registers for the interface declarations
        // Should not appear in the program text for extern modules
        for (_id, port) in self.instance.interface_ports.iter_valids() {
            let port_wire = &self.instance.wires[port.wire];
            self.add_latency_registers(port.wire, port_wire).unwrap();
        }
    }

    /// Pass a `to` parameter to say to what the constant should be assigned.  
    fn write_constant(&mut self, to : &str, value : &Value) {
        match value {
            Value::Bool(_) | Value::Integer(_) | Value::Unset => {
                let v_str = value.inline_constant_to_string();
                write!(self.program_text, "{to} = {v_str};\n").unwrap();
            }
            Value::Array(arr) => {
                for (idx, v) in arr.iter().enumerate() {
                    let new_to = format!("{to}[{idx}]");
                    self.write_constant(&new_to, v);
                }
            }
            Value::Error => unreachable!("Error values should never have reached codegen!"),
        }
    }

    fn write_wire_declarations(&mut self) {
        for (wire_id, w) in &self.instance.wires {
            // For better readability of output Verilog
            if self.can_inline(w) {
                continue;
            }
        
            if let Instruction::Declaration(wire_decl) =
                &self.md.link_info.instructions[w.original_instruction]
            {
                // Don't print named inputs and outputs, already did that in interface
                if let DeclarationPortInfo::RegularPort { .. } = wire_decl.is_port {
                    continue;
                }
            }
            let wire_or_reg = w.source.wire_or_reg();
        
            let wire_name = wire_name_self_latency(w, self.use_latency);
            let wire_decl = typ_to_declaration(&w.typ, &wire_name);
            write!(self.program_text, "{wire_or_reg} {wire_decl}").unwrap();
        
            match &w.source {
                RealWireDataSource::Select { root, path } => {
                    let wire_name = self.wire_name(*root, w.absolute_latency);
                    let path = self.wire_ref_path_to_string(&path, w.absolute_latency);
                    writeln!(self.program_text, " = {wire_name}{path};").unwrap();
                }
                RealWireDataSource::UnaryOp { op, right } => {
                    writeln!(
                        self.program_text,
                        " = {}{};",
                        op.op_text(),
                        self.wire_name(*right, w.absolute_latency)
                    ).unwrap();
                }
                RealWireDataSource::BinaryOp { op, left, right } => {
                    writeln!(
                        self.program_text,
                        " = {} {} {};",
                        self.wire_name(*left, w.absolute_latency),
                        op.op_text(),
                        self.wire_name(*right, w.absolute_latency)
                    ).unwrap();
                }
                RealWireDataSource::Constant { value } => {
                    // Trivial constants (bools & ints) should have been inlined already
                    // So appearences of this are always arrays or other compound types
                    writeln!(self.program_text, ";").unwrap();
                    self.write_constant(&wire_name, value);
                }
                RealWireDataSource::ReadOnly => {
                    writeln!(self.program_text, ";").unwrap();
                }
                RealWireDataSource::Multiplexer {
                    is_state,
                    sources: _,
                } => {
                    writeln!(self.program_text, ";").unwrap();
                    if let Some(initial_value) = is_state {
                        let to = format!("initial {wire_name}");
                        self.write_constant(&to, initial_value);
                    }
                }
            }
            self.add_latency_registers(wire_id, w).unwrap();
        }
    }

    fn write_submodules(&mut self) {
        for (_id, sm) in &self.instance.submodules {
            let sm_inst: &InstantiatedModule = sm
                .instance
                .as_ref()
                .expect("Invalid submodules are impossible to remain by the time codegen happens");
            let sm_instance_name = mangle(&sm_inst.name);
            let sm_name = &sm.name;
            writeln!(self.program_text, "{sm_instance_name} {sm_name}(").unwrap();
            write!(self.program_text, "\t.clk(clk)").unwrap();
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
                    Cow::Borrowed("")
                };
                write!(self.program_text, ",\n\t.{port_name}({wire_name})").unwrap();
            }
            writeln!(self.program_text, "\n);").unwrap();
        }
    }

    fn write_multiplexers(&mut self) {
        for (_id, w) in &self.instance.wires {
            match &w.source {
                RealWireDataSource::Multiplexer { is_state, sources } => {
                    let output_name = wire_name_self_latency(w, self.use_latency);
                    let arrow_str = if is_state.is_some() {
                        writeln!(self.program_text, "always_ff @(posedge clk) begin").unwrap();
                        "<="
                    } else {
                        writeln!(self.program_text, "always_comb begin\n\t// Combinatorial wires are not defined when not valid. This is just so that the synthesys tool doesn't generate latches").unwrap();
                        let invalid_val = w.typ.get_initial_val();
                        let tabbed_name = format!("\t{output_name}");
                        self.write_constant(&tabbed_name, &invalid_val);
                        "="
                    };
    
                    for s in sources {
                        let path = self.wire_ref_path_to_string(&s.to_path, w.absolute_latency);
                        let from_name = self.wire_name(s.from.from, w.absolute_latency);
                        self.program_text.write_char('\t').unwrap();
                        for cond in s.from.condition.iter() {
                            let cond_name = self.wire_name(cond.condition_wire, w.absolute_latency);
                            let invert = if cond.inverse {"!"} else {""};
                            write!(self.program_text, "if({invert}{cond_name}) ").unwrap();
                        }
                        writeln!(self.program_text, "{output_name}{path} {arrow_str} {from_name};").unwrap();
                    }
                    writeln!(self.program_text, "end").unwrap();
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
    }
    
    /// TODO probably best to have some smarter system for this in the future. 
    fn write_builtins(&mut self) {
        match self.md.link_info.name.as_str() {
            "LatencyOffset" => {
                let _in_port = self.md.unwrap_port(PortID::from_hidden_value(0), true, "in");
                let _out_port = self.md.unwrap_port(PortID::from_hidden_value(1), false, "out");
                self.program_text.write_str("\tassign out = in;\n").unwrap();
            }
            "CrossDomain" => {
                let _in_port = self.md.unwrap_port(PortID::from_hidden_value(0), true, "in");
                let _out_port = self.md.unwrap_port(PortID::from_hidden_value(1), false, "out");
                self.program_text.write_str("\tassign out = in;\n").unwrap();
            }
            "IntToBits" => {
                let _value_port = self.md.unwrap_port(PortID::from_hidden_value(0), true, "value");
                let _bits_port = self.md.unwrap_port(PortID::from_hidden_value(1), false, "bits");
                for i in 0..32 {
                    write!(self.program_text, "\tassign bits[{i}] = value[{i}];\n").unwrap();
                }
            }
            "BitsToInt" => {
                let _bits_port = self.md.unwrap_port(PortID::from_hidden_value(0), true, "bits");
                let _value_port = self.md.unwrap_port(PortID::from_hidden_value(1), false, "value");
                for i in 0..32 {
                    write!(self.program_text, "\tassign value[{i}] = bits[{i}];\n").unwrap();
                }
            }
            other => panic!("Unknown Builtin: \"{other}\"! Do not mark modules as __builtin__ yourself!")
        }
    }

    fn write_endmodule(&mut self) {
        writeln!(self.program_text, "endmodule\n").unwrap();
    }
}

impl Value {
    fn inline_constant_to_string(&self) -> Cow<str> {
        match self {
            Value::Bool(b) => {
                Cow::Borrowed(if *b { "1'b1" } else { "1'b0" })
            }
            Value::Integer(v) => {
                Cow::Owned(format!("{v}"))
            }
            Value::Unset => {
                Cow::Borrowed("'x")
            }
            Value::Array(_) => unreachable!("Not an inline constant!"),
            Value::Error => unreachable!("Error values should never have reached codegen!"),
        }
    }
}

impl Module {
    fn unwrap_port(&self, port_id : PortID, is_input : bool, name : &str) -> &Port {
        let result = &self.ports[port_id];

        assert_eq!(result.name, name);
        assert_eq!(result.is_input, is_input);

        result
    }    
}

impl RealWireDataSource {
    fn wire_or_reg(&self) -> &str {
        match self {
            RealWireDataSource::Multiplexer {
                is_state: Some(_),
                sources: _,
            } => "/*state*/ logic",
            RealWireDataSource::Multiplexer {
                is_state: None,
                sources: _,
            } => "/*mux_wire*/ logic",
            _ => "wire",
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
        needed_untils: instance.compute_needed_untils()
    };
    ctx.write_verilog_code();

    program_text
}
