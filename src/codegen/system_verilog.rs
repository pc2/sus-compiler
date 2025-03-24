use std::borrow::Cow;
use std::ops::Deref;

use crate::latency::CALCULATE_LATENCY_LATER;
use crate::linker::{IsExtern, LinkInfo};
use crate::prelude::*;

use crate::flattening::{DeclarationKind, Instruction, Module, Port};
use crate::instantiation::{
    InstantiatedModule, MultiplexerSource, RealWire, RealWireDataSource, RealWirePathElem,
};
use crate::typing::template::TVec;
use crate::{typing::concrete_type::ConcreteType, value::Value};

use super::shared::*;
use std::fmt::Write;

#[derive(Debug)]
pub struct VerilogCodegenBackend;

impl super::CodeGenBackend for VerilogCodegenBackend {
    fn file_extension(&self) -> &str {
        "sv"
    }
    fn output_dir_name(&self) -> &str {
        "verilog_output"
    }
    fn codegen(
        &self,
        md: &Module,
        instance: &InstantiatedModule,
        linker: &Linker,
        use_latency: bool,
    ) -> String {
        gen_verilog_code(md, instance, linker, use_latency)
    }
}

/// Creates the Verilog variable declaration for tbis variable.
///
/// IE for `int[15] myVar` it creates `[31:0] myVar[14:0]`
fn typ_to_declaration(mut typ: &ConcreteType, var_name: &str) -> String {
    let mut array_string = String::new();
    while let ConcreteType::Array(arr) = typ {
        let (content_typ, size) = arr.deref();
        let sz = size.unwrap_value().unwrap_integer();
        write!(array_string, "[{}:0]", sz - 1).unwrap();
        typ = content_typ;
    }
    match typ {
        ConcreteType::Named(reference) => {
            let sz = ConcreteType::sizeof_named(reference);
            if sz == 1 {
                format!(" {var_name}{array_string}")
            } else {
                format!("[{}:0] {var_name}{array_string}", sz - 1)
            }
        }
        ConcreteType::Array(_) => unreachable!("All arrays have been used up already"),
        ConcreteType::Value(_) | ConcreteType::Unknown(_) => unreachable!(),
    }
}

struct CodeGenerationContext<'g> {
    /// Generate code to this variable
    program_text: String,

    md: &'g Module,
    instance: &'g InstantiatedModule,
    linker: &'g Linker,

    use_latency: bool,

    needed_untils: FlatAlloc<i64, WireIDMarker>,
}

impl<'g> CodeGenerationContext<'g> {
    /// This is for making the resulting Verilog a little nicer to read
    fn can_inline(&self, wire: &RealWire) -> bool {
        match &wire.source {
            RealWireDataSource::Constant {
                value: Value::Bool(_) | Value::Integer(_),
            } => true,
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

    fn wire_name(&self, wire: &'g RealWire, requested_latency: i64) -> Cow<'g, str> {
        if self.can_inline(wire) {
            self.operation_to_string(wire)
        } else {
            wire_name_with_latency(wire, requested_latency, self.use_latency)
        }
    }

    fn wire_ref_path_to_string(&self, path: &[RealWirePathElem], absolute_latency: i64) -> String {
        let mut result = String::new();
        for path_elem in path {
            match path_elem {
                RealWirePathElem::ArrayAccess { span: _, idx_wire } => {
                    let wire = &self.instance.wires[*idx_wire];
                    let idx_wire_name = self.wire_name(wire, absolute_latency);
                    write!(result, "[{idx_wire_name}]").unwrap();
                }
            }
        }
        result
    }

    fn add_latency_registers(
        &mut self,
        wire_id: WireID,
        w: &RealWire,
    ) -> Result<(), std::fmt::Error> {
        if self.use_latency {
            // Can do 0 iterations, when w.needed_until == w.absolute_latency. Meaning it's only needed this cycle
            assert!(w.absolute_latency != CALCULATE_LATENCY_LATER);
            assert!(self.needed_untils[wire_id] != CALCULATE_LATENCY_LATER);
            for i in w.absolute_latency..self.needed_untils[wire_id] {
                let from = wire_name_with_latency(w, i, self.use_latency);
                let to = wire_name_with_latency(w, i + 1, self.use_latency);

                let var_decl = typ_to_declaration(&w.typ, &to);

                let clk_name = self.md.get_clock_name();
                writeln!(
                    self.program_text,
                    "/*latency*/ logic {var_decl}; always_ff @(posedge {clk_name}) begin {to} <= {from}; end"
                ).unwrap();
            }
        }
        Ok(())
    }

    fn comment_out(&mut self, f: impl FnOnce(&mut Self)) {
        let store_program_text_temporary = std::mem::take(&mut self.program_text);
        f(self);
        let added_text = std::mem::replace(&mut self.program_text, store_program_text_temporary);

        writeln!(
            self.program_text,
            "// {}",
            added_text.replace("\n", "\n// ")
        )
        .unwrap();
    }

    fn write_verilog_code(&mut self) {
        self.comment_out(|new_self| {
            let name = &self.instance.name;
            write!(new_self.program_text, "{name}").unwrap();
        });
        match self.md.link_info.is_extern {
            IsExtern::Normal => {
                self.write_module_signature();
                self.write_generative_declarations();
                self.write_wire_declarations();
                self.write_submodules();
                self.write_multiplexers();
                self.write_endmodule();
            }
            IsExtern::Extern => {
                // Do nothing, it's provided externally
                writeln!(self.program_text, "// Provided externally").unwrap();
                self.comment_out(|new_self| {
                    new_self.write_module_signature();
                });
            }
            IsExtern::Builtin => {
                self.write_module_signature();
                self.write_builtins();
                self.write_endmodule();
            }
        }
    }

    fn write_module_signature(&mut self) {
        // First output the interface of the module
        let clk_name = self.md.get_clock_name();
        write!(
            self.program_text,
            "module {}(\n\tinput {clk_name}",
            &self.instance.mangled_name
        )
        .unwrap();
        for (_id, port) in self.instance.interface_ports.iter_valids() {
            let port_wire = &self.instance.wires[port.wire];
            let input_or_output = if port.is_input { "input" } else { "output" };
            let wire_doc = port_wire.source.wire_or_reg();
            let wire_name = wire_name_self_latency(port_wire, self.use_latency);
            let wire_decl = typ_to_declaration(&port_wire.typ, &wire_name);
            write!(
                self.program_text,
                ",\n\t{input_or_output} {wire_doc}{wire_decl}"
            )
            .unwrap();
        }
        write!(self.program_text, "\n);\n\n").unwrap();

        // Add latency registers for the interface declarations
        // Should not appear in the program text for extern modules
        for (_id, port) in self.instance.interface_ports.iter_valids() {
            let port_wire = &self.instance.wires[port.wire];
            self.add_latency_registers(port.wire, port_wire).unwrap();
        }
    }

    /// Pass a `to` parameter to say to what the constant should be assigned.  
    fn write_constant(&mut self, to: &str, value: &Value) {
        match value {
            Value::Bool(_) | Value::Integer(_) | Value::Unset => {
                let v_str = value.inline_constant_to_string();
                writeln!(self.program_text, "{to} = {v_str};").unwrap();
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

    fn write_assign_wires_to_wires(
        &mut self,
        to_wire_and_path: &str,
        arrow_str: &'static str,
        from_wire_and_path: &str,
        mut typ: &ConcreteType,
        // Generation rules are different outside and inside always blocks.
        in_always: bool,
    ) {
        let for_should_declare_var = if in_always { "int " } else { "" };
        let mut for_stack = String::new();
        let mut array_accesses_stack = String::new();
        let mut idx = 0;
        while let ConcreteType::Array(arr_box) = typ {
            let var_name = if in_always {
                format!("_v{idx}")
            } else {
                format!("_g{idx}")
            };
            idx += 1;
            let (new_typ, sz) = arr_box.deref();
            typ = new_typ;
            let sz = sz.unwrap_value().unwrap_integer();
            write!(
                for_stack,
                "for({for_should_declare_var}{var_name} = 0; {var_name} < {sz}; {var_name} = {var_name} + 1) "
            )
            .unwrap();
            write!(array_accesses_stack, "[{var_name}]").unwrap();
        }
        if idx == 0 {
            writeln!(
                self.program_text,
                "{to_wire_and_path} {arrow_str} {from_wire_and_path};"
            )
            .unwrap();
        } else if in_always {
            writeln!(
                self.program_text,
                "{for_stack}{to_wire_and_path}{array_accesses_stack} {arrow_str} {from_wire_and_path}{array_accesses_stack};"
            )
            .unwrap();
        } else {
            writeln!(
                self.program_text,
                "generate {for_stack}{to_wire_and_path}{array_accesses_stack} {arrow_str} {from_wire_and_path}{array_accesses_stack}; endgenerate"
            )
            .unwrap();
        }
    }

    fn write_generative_declarations(&mut self) {
        let mut deepest_array = 0;
        for (_, w) in &self.instance.wires {
            let mut this_array_depth = 0;
            let mut typ = &w.typ;

            while let ConcreteType::Array(a) = typ {
                this_array_depth += 1;
                typ = &a.0;
            }

            deepest_array = usize::max(deepest_array, this_array_depth);
        }

        for var in 0..deepest_array {
            writeln!(self.program_text, "genvar _g{var};").unwrap()
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
                if let DeclarationKind::RegularPort { .. } = wire_decl.decl_kind {
                    continue;
                }
            }
            let wire_or_reg = w.source.wire_or_reg();

            let wire_name = wire_name_self_latency(w, self.use_latency);
            let wire_decl = typ_to_declaration(&w.typ, &wire_name);

            match &w.source {
                RealWireDataSource::Select { root, path } => {
                    let root_wire = &self.instance.wires[*root];
                    let from_wire_name = self.wire_name(root_wire, w.absolute_latency);
                    let path = self.wire_ref_path_to_string(path, w.absolute_latency);
                    let from_string = format!("{from_wire_name}{path}");

                    if let ConcreteType::Array(_) = &w.typ {
                        writeln!(self.program_text, "{wire_or_reg}{wire_decl};").unwrap();
                        self.write_assign_wires_to_wires(
                            &format!("assign {}", w.name),
                            "=",
                            &from_string,
                            &w.typ,
                            false,
                        );
                    } else {
                        writeln!(
                            self.program_text,
                            "{wire_or_reg}{wire_decl} = {from_string};"
                        )
                        .unwrap();
                    }
                }
                RealWireDataSource::UnaryOp { op, right } => {
                    let right_wire = &self.instance.wires[*right];
                    writeln!(
                        self.program_text,
                        "{wire_or_reg}{wire_decl} = {}{};",
                        op.op_text(),
                        self.wire_name(right_wire, w.absolute_latency)
                    )
                    .unwrap();
                }
                RealWireDataSource::BinaryOp { op, left, right } => {
                    let left_wire = &self.instance.wires[*left];
                    let right_wire = &self.instance.wires[*right];
                    writeln!(
                        self.program_text,
                        "{wire_or_reg}{wire_decl} = {} {} {};",
                        self.wire_name(left_wire, w.absolute_latency),
                        op.op_text(),
                        self.wire_name(right_wire, w.absolute_latency)
                    )
                    .unwrap();
                }
                RealWireDataSource::Constant { value } => {
                    // Trivial constants (bools & ints) should have been inlined already
                    // So appearences of this are always arrays or other compound types
                    writeln!(self.program_text, "{wire_or_reg}{wire_decl};").unwrap();
                    self.write_constant(&wire_name, value);
                }
                RealWireDataSource::ReadOnly => {
                    writeln!(self.program_text, "{wire_or_reg}{wire_decl};").unwrap();
                }
                RealWireDataSource::ConstructArray { array_wires } => {
                    writeln!(self.program_text, "{wire_or_reg}{wire_decl};").unwrap();

                    for (arr_idx, elem_id) in array_wires.iter().enumerate() {
                        let element_wire = &self.instance.wires[*elem_id];
                        let element_wire_name =
                            wire_name_self_latency(element_wire, self.use_latency);

                        self.write_assign_wires_to_wires(
                            &format!("assign {}[{arr_idx}]", wire_name),
                            "=",
                            &element_wire_name,
                            &element_wire.typ,
                            false,
                        );
                    }
                }
                RealWireDataSource::Multiplexer {
                    is_state,
                    sources: _,
                } => {
                    writeln!(self.program_text, "{wire_or_reg}{wire_decl};").unwrap();
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
        let parent_clk_name = self.md.get_clock_name();
        for (_id, sm) in &self.instance.submodules {
            let sm_md = &self.linker.modules[sm.refers_to.id];
            let sm_inst: &InstantiatedModule = sm
                .instance
                .get()
                .expect("Invalid submodules are impossible to remain by the time codegen happens");
            if sm_md.link_info.is_extern == IsExtern::Extern {
                self.write_template_args(&sm_md.link_info, &sm_inst.global_ref.template_args);
            } else {
                self.program_text.write_str(&sm_inst.mangled_name).unwrap();
            };
            let sm_name = &sm.name;
            let submodule_clk_name = sm_md.get_clock_name();
            writeln!(self.program_text, " {sm_name}(").unwrap();
            write!(
                self.program_text,
                "\t.{submodule_clk_name}({parent_clk_name})"
            )
            .unwrap();
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

    fn write_template_args(
        &mut self,
        link_info: &LinkInfo,
        concrete_template_args: &TVec<ConcreteType>,
    ) {
        self.program_text.write_str(&link_info.name).unwrap();
        self.program_text.write_str(" #(").unwrap();
        let mut first = true;
        concrete_template_args.iter().for_each(|(arg_id, arg)| {
            let arg_name = &link_info.template_parameters[arg_id].name;
            let arg_value = match arg {
                ConcreteType::Named(..) | ConcreteType::Array(..) => {
                    unreachable!("No extern module type arguments. Should have been caught by Lint")
                }
                ConcreteType::Value(value) => value.inline_constant_to_string(),
                ConcreteType::Unknown(_) => unreachable!("All args are known at codegen"),
            };
            if first {
                self.program_text.write_char(',').unwrap();
            } else {
                first = false;
            }
            self.program_text.write_char('.').unwrap();
            self.program_text.write_str(arg_name).unwrap();
            self.program_text.write_char('(').unwrap();
            self.program_text.write_str(&arg_value).unwrap();
            self.program_text.write_char(')').unwrap();
        });
        self.program_text.write_char(')').unwrap();
    }

    fn write_assign(
        &mut self,
        output_name: &str,
        arrow_str: &'static str,
        s: &MultiplexerSource,
        w: &RealWire,
    ) {
        let path = self.wire_ref_path_to_string(&s.to_path, w.absolute_latency);
        let from_wire = &self.instance.wires[s.from];
        let from_name = self.wire_name(from_wire, w.absolute_latency);
        self.program_text.write_char('\t').unwrap();
        let mut if_stack = String::new();
        for cond in s.condition.iter() {
            let cond_wire = &self.instance.wires[cond.condition_wire];
            let cond_name = self.wire_name(cond_wire, w.absolute_latency);
            let invert = if cond.inverse { "!" } else { "" };
            write!(if_stack, "if({invert}{cond_name}) ").unwrap();
        }
        let to_path = format!("{if_stack}{output_name}{path}");
        self.write_assign_wires_to_wires(&to_path, arrow_str, &from_name, &from_wire.typ, true);
    }

    fn write_multiplexers(&mut self) {
        for (_id, w) in &self.instance.wires {
            match &w.source {
                RealWireDataSource::Multiplexer { is_state, sources } => {
                    let output_name = wire_name_self_latency(w, self.use_latency);
                    let arrow_str = if is_state.is_some() {
                        let clk_name = self.md.get_clock_name();
                        writeln!(self.program_text, "always_ff @(posedge {clk_name}) begin")
                            .unwrap();
                        "<="
                    } else {
                        writeln!(self.program_text, "always_comb begin\n\t// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches").unwrap();
                        let invalid_val = w.typ.get_initial_val();
                        let tabbed_name = format!("\t{output_name}");
                        self.write_constant(&tabbed_name, &invalid_val);
                        "="
                    };

                    for s in sources {
                        self.write_assign(&output_name, arrow_str, s, w);
                    }
                    writeln!(self.program_text, "end").unwrap();
                }
                RealWireDataSource::ReadOnly
                | RealWireDataSource::Select { .. }
                | RealWireDataSource::UnaryOp { .. }
                | RealWireDataSource::BinaryOp { .. }
                | RealWireDataSource::Constant { .. }
                | RealWireDataSource::ConstructArray { .. } => {}
            }
        }
    }

    /// TODO probably best to have some smarter system for this in the future.
    fn write_builtins(&mut self) {
        match self.md.link_info.name.as_str() {
            "LatencyOffset" => {
                let _in_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(0), true, "in");
                let _out_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(1), false, "out");
                self.program_text.write_str("\tassign out = in;\n").unwrap();
            }
            "CrossDomain" => {
                let _in_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(0), true, "in");
                let _out_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(1), false, "out");
                self.program_text.write_str("\tassign out = in;\n").unwrap();
            }
            "IntToBits" => {
                let [num_bits] = self.instance.global_ref.template_args.cast_to_array();
                let num_bits: usize = num_bits.unwrap_value().unwrap_int();

                let _value_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(0), true, "value");
                let _bits_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(1), false, "bits");
                for i in 0..num_bits {
                    writeln!(self.program_text, "\tassign bits[{i}] = value[{i}];").unwrap();
                }
            }
            "BitsToInt" => {
                let [num_bits] = self.instance.global_ref.template_args.cast_to_array();
                let num_bits: usize = num_bits.unwrap_value().unwrap_int();

                let _bits_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(0), true, "bits");
                let _value_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(1), false, "value");
                for i in 0..num_bits {
                    writeln!(self.program_text, "\tassign value[{i}] = bits[{i}];").unwrap();
                }
            }
            other => {
                panic!("Unknown Builtin: \"{other}\"! Do not mark modules as __builtin__ yourself!")
            }
        }
    }

    fn write_endmodule(&mut self) {
        writeln!(self.program_text, "endmodule\n").unwrap();
    }
}

impl Value {
    fn inline_constant_to_string(&self) -> Cow<str> {
        match self {
            Value::Bool(b) => Cow::Borrowed(if *b { "1'b1" } else { "1'b0" }),
            Value::Integer(v) => Cow::Owned(v.to_string()),
            Value::Unset => Cow::Borrowed("'x"),
            Value::Array(_) => unreachable!("Not an inline constant!"),
            Value::Error => unreachable!("Error values should never have reached codegen!"),
        }
    }
}

impl Module {
    fn unwrap_port(&self, port_id: PortID, is_input: bool, name: &str) -> &Port {
        let result = &self.ports[port_id];

        assert_eq!(result.name, name);
        assert_eq!(result.is_input, is_input);

        result
    }
}

impl RealWireDataSource {
    fn wire_or_reg(&self) -> &'static str {
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

fn gen_verilog_code(
    md: &Module,
    instance: &InstantiatedModule,
    linker: &Linker,
    use_latency: bool,
) -> String {
    let mut ctx = CodeGenerationContext {
        md,
        instance,
        linker,
        program_text: String::new(),
        use_latency,
        needed_untils: instance.compute_needed_untils(),
    };
    ctx.write_verilog_code();

    ctx.program_text
}
