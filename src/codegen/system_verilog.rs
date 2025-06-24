use std::borrow::Cow;
use std::ops::Deref;

use ibig::IBig;
use sus_proc_macro::get_builtin_type;

use crate::alloc::zip_eq;
use crate::latency::CALCULATE_LATENCY_LATER;
use crate::linker::{IsExtern, LinkInfo};
use crate::prelude::*;

use crate::flattening::{Module, Port};
use crate::instantiation::{
    InstantiatedModule, MultiplexerSource, RealWire, RealWireDataSource, RealWirePathElem,
};
use crate::to_string::join_string_iter;
use crate::typing::concrete_type::{
    get_int_bitwidth, ConcreteGlobalReference, ConcreteTemplateArg,
};
use crate::typing::template::{TVec, TemplateKind};
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

    loop {
        match typ {
            ConcreteType::Named(ConcreteGlobalReference {
                id: get_builtin_type!("int"),
                template_args,
            }) => {
                let [min, max] = template_args.cast_to_int_array();
                let bitwidth = get_int_bitwidth(min, max) - 1;
                if min < &IBig::from(0) {
                    return format!("signed [{bitwidth}:0] {var_name}{array_string}");
                } else {
                    return format!("[{bitwidth}:0] {var_name}{array_string}");
                }
            }
            ConcreteType::Named(ConcreteGlobalReference {
                id: get_builtin_type!("bool"),
                ..
            }) => return format!(" {var_name}{array_string}"),
            ConcreteType::Named(ConcreteGlobalReference { id: _, .. }) => {
                todo!("Structs")
            }
            ConcreteType::Array(arr) => {
                let (content_typ, size) = arr.deref();
                let sz = size.unwrap_integer() - 1;
                if let ConcreteType::Named(ConcreteGlobalReference {
                    id: get_builtin_type!("bool"),
                    ..
                }) = content_typ
                {
                    return format!("[{sz}:0] {var_name}{array_string}");
                }
                write!(array_string, "[{sz}:0]").unwrap();
                typ = content_typ;
            }
        }
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

enum ForEachPathElement {
    Array { var: String, arr_size: IBig },
}

impl ForEachPathElement {
    /// [_v0][_v1][...]
    fn to_string(path: &[Self]) -> String {
        let mut result = String::new();
        for p in path {
            match p {
                ForEachPathElement::Array { var, arr_size: _ } => {
                    write!(result, "[{var}]").unwrap();
                }
            }
        }
        result
    }
    fn to_bit_index_formula(path: &[Self]) -> String {
        let mut path_iter = path.iter();
        let mut result = match path_iter.next().unwrap() {
            ForEachPathElement::Array { var, arr_size: _ } => var.clone(),
        };
        for p in path_iter {
            match p {
                ForEachPathElement::Array { var, arr_size } => {
                    result = format!("({arr_size} * {result}) + {var}")
                }
            }
        }
        result
    }
}

impl<'g> CodeGenerationContext<'g> {
    fn constant_to_str(result: &mut String, typ: &ConcreteType, cst: &Value) {
        match typ {
            ConcreteType::Named(global_ref) => match global_ref.id {
                get_builtin_type!("bool") => {
                    let b = match cst {
                        Value::Bool(true) => "1'b1",
                        Value::Bool(false) => "1'b0",
                        Value::Unset => "1'bx",
                        _ => unreachable!(),
                    };
                    result.write_str(b).unwrap();
                }
                get_builtin_type!("int") => {
                    let [min, max] = global_ref.template_args.cast_to_int_array();

                    let bitwidth = get_int_bitwidth(min, max);

                    let cst_str = match cst {
                        Value::Integer(ibig) => ibig.to_string(),
                        Value::Unset => "x".to_string(),
                        _ => unreachable!(),
                    };
                    if min < &IBig::from(0) {
                        result
                            .write_fmt(format_args!("{bitwidth}'sd{cst_str}"))
                            .unwrap()
                    } else {
                        result
                            .write_fmt(format_args!("{bitwidth}'d{cst_str}"))
                            .unwrap()
                    }
                }
                _ => todo!("Structs"),
            },
            ConcreteType::Array(arr_box) => {
                let (content, size) = arr_box.deref();

                let size: usize = size.unwrap_int();
                if let ConcreteType::Named(ConcreteGlobalReference {
                    id: get_builtin_type!("bool"),
                    ..
                }) = content
                {
                    result.write_fmt(format_args!("{size}'b")).unwrap();
                    match cst {
                        Value::Array(values) => {
                            assert_eq!(values.len(), size);
                            for elem in values.iter().rev() {
                                let b = match elem {
                                    Value::Bool(true) => '1',
                                    Value::Bool(false) => '0',
                                    Value::Unset => 'x',
                                    _ => unreachable!(),
                                };
                                result.write_char(b).unwrap();
                            }
                        }
                        Value::Unset => {
                            for _ in 0..size {
                                result.write_char('x').unwrap();
                            }
                        }
                        _ => unreachable!(),
                    }
                } else {
                    result.write_str("{").unwrap();
                    match cst {
                        Value::Array(values) => {
                            assert_eq!(values.len(), size);
                            join_string_iter(result, ", ", values.iter(), |result, v| {
                                Self::constant_to_str(result, content, v);
                            })
                        }
                        Value::Unset => join_string_iter(result, ", ", 0..size, |result, _| {
                            Self::constant_to_str(result, content, &Value::Unset);
                        }),
                        _ => unreachable!(),
                    }
                    result.write_str("}").unwrap();
                }
            }
        }
    }
    /// This is for making the resulting Verilog a little nicer to read
    fn can_inline(&self, wire: &RealWire) -> bool {
        match &wire.source {
            RealWireDataSource::Constant { .. } => true,
            RealWireDataSource::Select { root: _, path } if path.is_empty() => true,
            _other => false,
        }
    }

    fn wire_name(&self, wire: &'g RealWire, requested_latency: i64) -> Cow<'g, str> {
        match &wire.source {
            RealWireDataSource::Constant { value } => {
                let mut result = String::new();
                Self::constant_to_str(&mut result, &wire.typ, value);
                Cow::Owned(result)
            }
            RealWireDataSource::Select { root, path } if path.is_empty() => wire_name_with_latency(
                &self.instance.wires[*root],
                requested_latency,
                self.use_latency,
            ),
            _other => wire_name_with_latency(wire, requested_latency, self.use_latency),
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
                    "/*latency*/ logic{var_decl}; always_ff @(posedge {clk_name}) begin {to} <= {from}; end"
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
                self.write_generative_declarations();
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
        for (_id, port_wire) in &self.instance.wires {
            let input_or_output = match port_wire.is_port {
                Some(true) => "input",
                Some(false) => "output",
                None => continue,
            };
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
        for (port_wire_id, port_wire) in &self.instance.wires {
            if port_wire.is_port.is_some() {
                self.add_latency_registers(port_wire_id, port_wire).unwrap();
            }
        }
    }

    /// Generates code to walk arrays (and in the future structs)
    ///
    /// `int[3][7] a`
    ///
    /// ```Verilog
    /// generate
    /// for(_g0 = 0; _g0 < 7; _g0 = _g0 + 1) begin
    /// for(_g1 = 0; _g1 < 3; _g1 = _g1 + 1) begin
    /// a[_g0][_g1] = ...
    /// end
    /// end
    /// endgenerate
    /// ```
    fn walk_typ_to_generate_foreach(
        &mut self,
        typ: &ConcreteType,
        in_always: bool,
        mut operation: impl FnMut(&[ForEachPathElement], u64) -> String,
    ) {
        fn walk_type_to_generate_foreach_recurse(
            typ: &ConcreteType,
            in_always: bool,
            mut path: Vec<ForEachPathElement>,
            var_idx: usize,
            operation: &mut impl FnMut(&[ForEachPathElement], u64) -> String,
        ) -> String {
            let for_should_declare_var = if in_always { "int " } else { "" };

            if let Some(fundamental_size) = typ.can_be_represented_as_packed_bits() {
                operation(&path, fundamental_size)
            } else {
                let ConcreteType::Array(arr_box) = typ else {
                    todo!("Structs");
                };

                let var = if in_always {
                    format!("_v{var_idx}")
                } else {
                    format!("_g{var_idx}")
                };
                let (new_typ, sz) = arr_box.deref();
                path.push(ForEachPathElement::Array {
                    var: var.clone(),
                    arr_size: sz.unwrap_integer().clone(),
                });
                let sz = sz.unwrap_integer();
                let content_str = walk_type_to_generate_foreach_recurse(
                    new_typ,
                    in_always,
                    path,
                    var_idx + 1,
                    operation,
                );

                format!(
                    "for({for_should_declare_var}{var} = 0; {var} < {sz}; {var} = {var} + 1) begin\n{content_str}end\n"
                )
            }
        }

        let content =
            walk_type_to_generate_foreach_recurse(typ, in_always, Vec::new(), 0, &mut operation);

        if in_always | typ.can_be_represented_as_packed_bits().is_some() {
            self.program_text.write_str(&content).unwrap();
        } else {
            write!(self.program_text, "generate\n{content}endgenerate\n").unwrap()
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

            if w.is_port.is_some() {
                continue;
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

                        self.walk_typ_to_generate_foreach(&w.typ, false, |path, _| {
                            let path = ForEachPathElement::to_string(path);
                            format!("assign {}{path} = {from_string}{path};\n", &w.name)
                        });
                    } else {
                        writeln!(
                            self.program_text,
                            "{wire_or_reg}{wire_decl} = {from_string};"
                        )
                        .unwrap();
                    }
                }
                RealWireDataSource::UnaryOp { op, rank, right } => {
                    let right_wire = &self.instance.wires[*right];

                    writeln!(self.program_text, "{wire_or_reg}{wire_decl};").unwrap();

                    let op = op.op_text();
                    let right_name = self.wire_name(right_wire, w.absolute_latency);
                    self.walk_typ_to_generate_foreach(&w.typ, false, |path, _| {
                        let path = ForEachPathElement::to_string(path);
                        format!("assign {wire_name}{path} = {op}{right_name}{path};\n")
                    });
                }
                RealWireDataSource::BinaryOp {
                    op,
                    rank,
                    left,
                    right,
                } => {
                    let left_wire = &self.instance.wires[*left];
                    let right_wire = &self.instance.wires[*right];

                    writeln!(self.program_text, "{wire_or_reg}{wire_decl};").unwrap();

                    let op = op.op_text();
                    let left_name = self.wire_name(left_wire, w.absolute_latency);
                    let right_name = self.wire_name(right_wire, w.absolute_latency);
                    self.walk_typ_to_generate_foreach(&w.typ, false, |path, _| {
                        let path = ForEachPathElement::to_string(path);
                        format!(
                            "assign {wire_name}{path} = {left_name}{path} {op} {right_name}{path};\n"
                        )
                    });
                }
                RealWireDataSource::Constant { .. } => {
                    unreachable!("All constants are inlined!");
                }
                RealWireDataSource::ReadOnly => {
                    writeln!(self.program_text, "{wire_or_reg}{wire_decl};").unwrap();
                }
                RealWireDataSource::ConstructArray { array_wires } => {
                    writeln!(self.program_text, "{wire_or_reg}{wire_decl};").unwrap();

                    for (arr_idx, elem_id) in array_wires.iter().enumerate() {
                        let element_wire = &self.instance.wires[*elem_id];
                        let element_wire_name = self.wire_name(element_wire, w.absolute_latency);

                        self.walk_typ_to_generate_foreach(&element_wire.typ, false, |path, _| {
                            let path = ForEachPathElement::to_string(path);
                            format!(
                                "assign {wire_name}[{arr_idx}]{path} = {element_wire_name}{path};\n"
                            )
                        });
                    }
                }
                RealWireDataSource::Multiplexer {
                    is_state,
                    sources: _,
                } => {
                    write!(self.program_text, "{wire_or_reg}{wire_decl}").unwrap();
                    match is_state {
                        Some(initial_val) if !initial_val.is_unset() => {
                            self.program_text.write_str(" = ").unwrap();
                            Self::constant_to_str(&mut self.program_text, &w.typ, initial_val);
                        }
                        _ => {}
                    }
                    self.program_text.write_str(";\n").unwrap();
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
        concrete_template_args: &TVec<ConcreteTemplateArg>,
    ) {
        self.program_text.write_str(&link_info.name).unwrap();
        self.program_text.write_str(" #(").unwrap();
        join_string_iter(
            &mut self.program_text,
            ", ",
            zip_eq(concrete_template_args, &link_info.template_parameters),
            |result, (_, arg, arg_name)| {
                let arg_name = &arg_name.name;
                match arg {
                    TemplateKind::Type(_) => {
                        unreachable!(
                            "No extern module type arguments. Should have been caught by Lint"
                        );
                    }
                    TemplateKind::Value(value) => {
                        result
                            .write_fmt(format_args!(".{arg_name}({value})"))
                            .unwrap();
                    }
                };
            },
        );
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
        self.walk_typ_to_generate_foreach(&from_wire.typ, true, |path, _| {
            let path = ForEachPathElement::to_string(path);
            format!("{to_path}{path} {arrow_str} {from_name}{path};\n")
        });
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
                        write!(self.program_text, "\t{output_name} = ").unwrap();
                        Self::constant_to_str(&mut self.program_text, &w.typ, &Value::Unset);
                        self.program_text.write_str(";\n").unwrap();
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
                let [num_bits] = self.instance.global_ref.template_args.cast_to_int_array();
                let _num_bits: usize = num_bits.try_into().unwrap();

                let _value_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(0), true, "value");
                let _bits_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(1), false, "bits");
                writeln!(self.program_text, "\tassign bits = value;").unwrap();
            }
            "BitsToInt" => {
                let [num_bits] = self.instance.global_ref.template_args.cast_to_int_array();
                let _num_bits: usize = num_bits.try_into().unwrap();

                let _bits_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(0), true, "bits");
                let _value_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(1), false, "value");
                writeln!(self.program_text, "\tassign value = bits;").unwrap();
            }
            "UIntToBits" => {
                let [num_bits] = self.instance.global_ref.template_args.cast_to_int_array();
                let _num_bits: usize = num_bits.try_into().unwrap();

                let _value_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(0), true, "value");
                let _bits_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(1), false, "bits");
                writeln!(self.program_text, "\tassign bits = value;").unwrap();
            }
            "BitsToUInt" => {
                let [num_bits] = self.instance.global_ref.template_args.cast_to_int_array();
                let _num_bits: usize = num_bits.try_into().unwrap();

                let _bits_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(0), true, "bits");
                let _value_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(1), false, "value");
                writeln!(self.program_text, "\tassign value = bits;").unwrap();
            }
            "transmute_to_bits" => {
                let [typ] = self.instance.global_ref.template_args.cast_to_array();
                let typ = typ.unwrap_type();

                let _value_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(0), true, "value");
                let _bits_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(1), false, "bits");

                self.walk_typ_to_generate_foreach(typ, false, |path, num_bits| {
                    let path_str = ForEachPathElement::to_string(path);
                    if path.is_empty() {
                        format!("assign bits = value{path_str};\n")
                    } else {
                        let path_formula = ForEachPathElement::to_bit_index_formula(path);
                        format!("assign bits[({path_formula}) * {num_bits} +: {num_bits}] = value{path_str};\n")
                    }
                });
            }
            "transmute_from_bits" => {
                let [typ] = self.instance.global_ref.template_args.cast_to_array();
                let typ = typ.unwrap_type();

                let _bits_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(0), true, "bits");
                let _value_port = self
                    .md
                    .unwrap_port(PortID::from_hidden_value(1), false, "value");

                self.walk_typ_to_generate_foreach(typ, false, |path, num_bits| {
                    let path_str = ForEachPathElement::to_string(path);
                    if path.is_empty() {
                        format!("assign value{path_str} = bits;\n")
                    } else {
                        let path_formula = ForEachPathElement::to_bit_index_formula(path);
                        format!("assign value{path_str} = bits[({path_formula}) * {num_bits} +: {num_bits}];\n")
                    }
                });
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
            _ => "logic",
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
