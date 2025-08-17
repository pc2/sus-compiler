use std::borrow::Cow;
use std::ops::Deref;
use std::rc::Rc;

use ibig::IBig;
use sus_proc_macro::get_builtin_type;

use crate::alloc::zip_eq;
use crate::latency::AbsLat;
use crate::linker::{IsExtern, LinkInfo};
use crate::prelude::*;

use crate::flattening::{Direction, Module, PartSelectDirection, Port};
use crate::instantiation::{
    InstantiatedModule, MultiplexerSource, RealWire, RealWireDataSource, RealWirePathElem,
};
use crate::to_string::{join_string_iter, trim_known_prefix};
use crate::typing::concrete_type::{ConcreteGlobalReference, ConcreteTemplateArg, IntBounds};
use crate::typing::template::{TVec, TemplateKind};
use crate::{typing::concrete_type::ConcreteType, value::Value};

use super::shared::*;
use std::fmt::{Display, Write};

struct VariableAlloc {
    pub var_names: Vec<Rc<str>>,
    currently_used: usize,
    prefix: &'static str,
}
impl VariableAlloc {
    pub fn new(prefix: &'static str) -> Self {
        Self {
            var_names: Vec::new(),
            currently_used: 0,
            prefix,
        }
    }
    fn alloc(&mut self) -> Rc<str> {
        let claimed_id = self.currently_used;
        self.currently_used += 1;
        if claimed_id >= self.var_names.len() {
            assert_eq!(claimed_id, self.var_names.len(), "Skipping a var?");
            let prefix = self.prefix;
            self.var_names.push(format!("{prefix}{claimed_id}").into());
        }
        self.var_names[claimed_id].clone()
    }
    /// Does not empty the
    fn reuse(&mut self) {
        self.currently_used = 0;
    }
}

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
///
/// May return something with a leading space, to accomodate `logic`, `input`, etc.
fn typ_to_declaration(mut typ: &ConcreteType, var_name: &str) -> String {
    let mut array_string = String::new();

    loop {
        match typ {
            ConcreteType::Named(content_typ) => match content_typ.id {
                get_builtin_type!("int") => {
                    let bounds = content_typ.unwrap_int_bounds();
                    let bitwidth = bounds.bitwidth() - 1;
                    if bounds.from < &IBig::from(0) {
                        return format!(" signed[{bitwidth}:0] {var_name}{array_string}");
                    } else {
                        return format!("[{bitwidth}:0] {var_name}{array_string}");
                    }
                }
                get_builtin_type!("bool") => return format!(" {var_name}{array_string}"),
                get_builtin_type!("float") => return format!("[31:0] {var_name}{array_string}"),
                _ => todo!("Structs"),
            },
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
    for_vars: VariableAlloc,
    genvars: VariableAlloc,

    md: &'g Module,
    instance: &'g InstantiatedModule,
    linker: &'g Linker,

    use_latency: bool,

    needed_untils: FlatAlloc<i64, WireIDMarker>,
}

enum ForEachPathElement<'g> {
    Array { var: Rc<str>, arr_size: &'g IBig },
}

struct ForEachPath<'g, 'p> {
    path: &'p [ForEachPathElement<'g>],
}

impl<'g, 'p> Deref for ForEachPath<'g, 'p> {
    type Target = [ForEachPathElement<'g>];

    fn deref(&self) -> &[ForEachPathElement<'g>] {
        self.path
    }
}

impl<'g, 'p> Display for ForEachPath<'g, 'p> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for p in self.path {
            match p {
                ForEachPathElement::Array { var, arr_size: _ } => {
                    write!(f, "[{var}]")?;
                }
            }
        }
        Ok(())
    }
}

impl<'g, 'p> ForEachPath<'g, 'p> {}

impl ForEachPath<'_, '_> {
    fn to_bit_index_formula(&self) -> String {
        let mut path_iter = self.path.iter();
        let mut result = match path_iter.next().unwrap() {
            ForEachPathElement::Array { var, arr_size: _ } => var.to_string(),
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
                    let bounds = global_ref.unwrap_int_bounds();

                    let bitwidth = bounds.bitwidth();

                    let cst_str = match cst {
                        Value::Integer(ibig) => ibig.to_string(),
                        Value::Unset => "x".to_string(),
                        _ => unreachable!(),
                    };
                    if bounds.from < &IBig::from(0) {
                        result
                            .write_fmt(format_args!("{bitwidth}'sd{cst_str}"))
                            .unwrap()
                    } else {
                        result
                            .write_fmt(format_args!("{bitwidth}'d{cst_str}"))
                            .unwrap()
                    }
                }
                get_builtin_type!("float") => {
                    assert!(
                        matches!(cst, Value::Unset),
                        "TODO: Generative non-Unset floats"
                    );
                    result
                        .write_str("32'bxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                        .unwrap();
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

    fn wire_name(&self, wire: WireID, requested_latency: AbsLat) -> Cow<'g, str> {
        let wire = &self.instance.wires[wire];
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

    fn add_latency_registers(
        &mut self,
        wire_id: WireID,
        w: &RealWire,
    ) -> Result<(), std::fmt::Error> {
        if self.use_latency {
            // Can do 0 iterations, when w.needed_until == w.absolute_latency. Meaning it's only needed this cycle
            for i in w.absolute_latency.unwrap()..self.needed_untils[wire_id] {
                let from = wire_name_with_latency(w, AbsLat::new(i), self.use_latency);
                let to = wire_name_with_latency(w, AbsLat::new(i + 1), self.use_latency);

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
            added_text.trim_end().replace("\n", "\n// ")
        )
        .unwrap();
    }

    fn write_generative_decls_for(&mut self, f: impl FnOnce(&mut Self)) {
        let store_program_text_temporary = std::mem::take(&mut self.program_text);
        f(self);
        let added_text = std::mem::replace(&mut self.program_text, store_program_text_temporary);

        for var in &self.genvars.var_names {
            writeln!(self.program_text, "genvar {var};").unwrap()
        }
        self.program_text.write_str(&added_text).unwrap();
    }
    fn write_verilog_code(&mut self) {
        self.comment_out(|new_self| {
            let name = &self.instance.name;
            write!(new_self.program_text, "{name}").unwrap();
        });
        match self.md.link_info.is_extern {
            IsExtern::Normal => {
                self.write_module_signature();
                self.write_generative_decls_for(|new_self| {
                    new_self.write_wire_declarations();
                    new_self.write_submodules();
                    new_self.write_multiplexers();
                });
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
                self.write_generative_decls_for(|new_self| {
                    new_self.write_builtins();
                });
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
            let Some(direction) = port_wire.is_port else {
                continue;
            };
            let wire_doc = port_wire.source.wire_or_reg();
            let wire_name = wire_name_self_latency(port_wire, self.use_latency);
            let wire_decl = typ_to_declaration(&port_wire.typ, &wire_name);
            write!(self.program_text, ",\n\t{direction} {wire_doc}{wire_decl}").unwrap();
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

    /// Returns ("for(int _v3 = 0; .....)", "_v3")
    fn mk_for(&mut self, sz: &IBig, in_always: bool) -> (String, Rc<str>) {
        let var = if in_always {
            self.for_vars.alloc()
        } else {
            self.genvars.alloc()
        };
        let int_decl = if in_always { "int " } else { "" };
        let forloop = format!("for({int_decl}{var} = 0; {var} < {sz}; {var} = {var} + 1)");
        (forloop, var)
    }

    /// Add a generate / endgenerate block around what the function produces
    fn in_generate(&mut self, f: impl FnOnce(&mut Self) -> String) {
        self.genvars.reuse();
        let content = f(self);
        if self.genvars.currently_used != 0 {
            write!(self.program_text, "generate\n{content}endgenerate\n").unwrap();
        } else {
            write!(self.program_text, "{content}").unwrap();
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
    fn foreach_for_copy_unpacked(
        &mut self,
        typ: &'g ConcreteType,
        in_always: bool,
        mut operation: impl FnMut(ForEachPath<'g, '_>, u64) -> String,
    ) -> String {
        fn foreach_for_copy_unpacked_recurse<'g>(
            slf: &mut CodeGenerationContext<'g>,
            typ: &'g ConcreteType,
            in_always: bool,
            mut path: Vec<ForEachPathElement<'g>>,
            operation: &mut impl FnMut(ForEachPath<'g, '_>, u64) -> String,
        ) -> String {
            if let Some(fundamental_size) = typ.can_be_represented_as_packed_bits() {
                operation(ForEachPath { path: &path }, fundamental_size)
            } else {
                match typ {
                    ConcreteType::Named(_) => {
                        todo!("Structs");
                    }
                    ConcreteType::Array(arr_box) => {
                        let (new_typ, sz) = arr_box.deref();
                        let arr_size = sz.unwrap_integer();
                        let (for_stm, var) = slf.mk_for(arr_size, in_always);
                        path.push(ForEachPathElement::Array { var, arr_size });
                        let content_str = foreach_for_copy_unpacked_recurse(
                            slf, new_typ, in_always, path, operation,
                        );

                        format!("{for_stm} begin\n{content_str}end\n")
                    }
                }
            }
        }

        foreach_for_copy_unpacked_recurse(self, typ, in_always, Vec::new(), &mut operation)
    }

    /// Convert array accesses and slices
    ///
    /// `a = b[5][n+:3][3:8]` becomes
    ///
    /// ```Verilog
    /// generate
    /// for(_g0 = 0; _g0 < 3; _g0 = _g0 + 1) begin
    /// for(_g1 = 0; _g1 < 5; _g1 = _g1 + 1) begin
    /// a[_g0][_g1] = b[5][_g0][_g1 + 3]
    /// end
    /// end
    /// endgenerate
    /// ```
    fn foreach_for_real_path(
        &mut self,
        mut typ: &'g ConcreteType,
        path: &'g [RealWirePathElem],
        requested_latency: AbsLat,
        in_always: bool,
        operation: impl FnOnce(
            &mut CodeGenerationContext<'g>,
            String,
            String,
            &'g ConcreteType,
        ) -> String,
    ) -> String {
        let mut source_path = String::new();
        let mut target_path = String::new();
        let mut for_stack = String::new();
        let mut ends_stack = String::new();
        for p in path {
            match p {
                RealWirePathElem::Index { idx_wire, .. } => {
                    typ = &typ.unwrap_array().0;
                    write!(
                        source_path,
                        "[{}]",
                        self.wire_name(*idx_wire, requested_latency)
                    )
                    .unwrap();
                }
                RealWirePathElem::ConstIndex { idx, .. } => {
                    typ = &typ.unwrap_array().0;
                    write!(source_path, "[{idx}]").unwrap();
                }
                RealWirePathElem::PartSelect {
                    from_wire,
                    width,
                    direction,
                    ..
                } => {
                    typ = &typ.unwrap_array().0;

                    let (for_stm, var) = self.mk_for(width, in_always);

                    writeln!(for_stack, "{for_stm} begin").unwrap();
                    writeln!(ends_stack, "end").unwrap();

                    let wire_name = self.wire_name(*from_wire, requested_latency);
                    write!(target_path, "[{var}]").unwrap();

                    match direction {
                        PartSelectDirection::Up => {
                            write!(source_path, "[{wire_name} + {var}]").unwrap();
                        }
                        PartSelectDirection::Down => {
                            let sz_dec = width - 1;
                            write!(source_path, "[{wire_name} - ({sz_dec} - {var})]").unwrap();
                        }
                    }
                }
                RealWirePathElem::Slice { bounds, .. } => {
                    typ = &typ.unwrap_array().0;

                    let IntBounds { from, to } = bounds.unwrap_valid();

                    let (for_stm, var) = self.mk_for(&(to - from), in_always);

                    writeln!(for_stack, "{for_stm} begin").unwrap();
                    writeln!(ends_stack, "end").unwrap();

                    write!(target_path, "[{var}]").unwrap();
                    if from == &IBig::from(0) {
                        write!(source_path, "[{var}]").unwrap();
                    } else {
                        write!(source_path, "[{from} + {var}]").unwrap();
                    }
                }
            }
        }

        let content = operation(self, source_path, target_path, typ);
        format!("{for_stack}{content}{ends_stack}")
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
                    let root_name = self.wire_name(*root, w.absolute_latency);

                    // Custom [Self::in_generate], to generate logic[31:0] my_val = 5 + other_val
                    self.genvars.reuse();
                    let content = self.foreach_for_real_path(
                        &root_wire.typ,
                        path,
                        w.absolute_latency,
                        false,
                        |slf, source_path, target_path, result_typ| {
                            let source = format!("{root_name}{source_path}");
                            let target = format!("{wire_name}{target_path}");
                            slf.foreach_for_copy_unpacked(result_typ, false, |path, _| {
                                format!("assign {target}{path} = {source}{path};\n")
                            })
                        },
                    );

                    if self.genvars.currently_used != 0 {
                        write!(
                            self.program_text,
                            "{wire_or_reg}{wire_decl};\ngenerate\n{content}endgenerate\n"
                        )
                        .unwrap();
                    } else {
                        // We're basically trimming "<assert wire_name>[...] = ..." off the string, so we can stitch it to the declaration
                        let content = trim_known_prefix(&content, "assign ");
                        let content = trim_known_prefix(content, &wire_name);
                        write!(self.program_text, "{wire_or_reg}{wire_decl}{content}").unwrap();
                    }
                }
                RealWireDataSource::UnaryOp { op, right, .. } => {
                    writeln!(self.program_text, "{wire_or_reg}{wire_decl};").unwrap();

                    let op = op.op_text();
                    let right_name = self.wire_name(*right, w.absolute_latency);
                    self.in_generate(|slf| {
                        slf.foreach_for_copy_unpacked(&w.typ, false, |path, _| {
                            format!("assign {wire_name}{path} = {op}{right_name}{path};\n")
                        })
                    })
                }
                RealWireDataSource::BinaryOp {
                    op, left, right, ..
                } => {
                    writeln!(self.program_text, "{wire_or_reg}{wire_decl};").unwrap();

                    let op = op.op_text();
                    let left_name = self.wire_name(*left, w.absolute_latency);
                    let right_name = self.wire_name(*right, w.absolute_latency);
                    self.in_generate(|slf| {
                        slf.foreach_for_copy_unpacked(&w.typ, false, |path, _| {
                            format!(
                                "assign {wire_name}{path} = {left_name}{path} {op} {right_name}{path};\n"
                            )
                        })
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
                        let elem_wire = &self.instance.wires[*elem_id];
                        let element_wire_name = self.wire_name(*elem_id, w.absolute_latency);

                        self.in_generate(|slf| {
                            slf.foreach_for_copy_unpacked(&elem_wire.typ, false, |path, _| {
                                format!(
                                "assign {wire_name}[{arr_idx}]{path} = {element_wire_name}{path};\n"
                            )
                            })
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
            zip_eq(concrete_template_args, &link_info.parameters),
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
        s: &'g MultiplexerSource,
        target: &'g RealWire,
    ) {
        let from_name = self.wire_name(s.from, target.absolute_latency);
        self.program_text.write_char('\t').unwrap();
        let mut if_stack = String::new();
        for cond in s.condition.iter() {
            let cond_name = self.wire_name(cond.condition_wire, target.absolute_latency);
            let invert = if cond.inverse { "!" } else { "" };
            write!(if_stack, "if({invert}{cond_name}) ").unwrap();
        }
        self.for_vars.reuse();
        let content = self.foreach_for_real_path(
            &target.typ,
            &s.to_path,
            target.absolute_latency,
            true,
            |slf, source_path, target_path, copy_typ| {
                let to_path = format!("{if_stack}{output_name}{source_path}");
                let from_path = format!("{from_name}{target_path}");
                slf.foreach_for_copy_unpacked(copy_typ, true, |path, _| {
                    format!("{to_path}{path} {arrow_str} {from_path}{path};\n")
                })
            },
        );
        self.program_text.write_str(&content).unwrap();
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

    fn check_ports<const N: usize>(&self, ports: &[(Direction, &'static str)]) -> &[Port; N] {
        let actual_ports: &[Port; N] = self.md.ports.cast_to_array();

        for ((direction, name), port) in crate::util::zip_eq(ports, actual_ports) {
            assert_eq!(&port.name, *name);
            assert_eq!(port.direction, *direction);
        }

        actual_ports
    }

    /// TODO probably best to have some smarter system for this in the future.
    fn write_builtins(&mut self) {
        let args = &self.instance.global_ref.template_args;
        match self.md.link_info.name.as_str() {
            "LatencyOffset" => {
                let [_in_port, _out_port] =
                    self.check_ports(&[(Direction::Input, "in"), (Direction::Output, "out")]);

                self.program_text.write_str("\tassign out = in;\n").unwrap();
            }
            "CrossDomain" => {
                let [_in_port, _out_port] =
                    self.check_ports(&[(Direction::Input, "in"), (Direction::Output, "out")]);

                self.program_text.write_str("\tassign out = in;\n").unwrap();
            }
            "IntToBits" => {
                let [_num_bits] = args.cast_to_int_array();
                let [_value_port, _bits_port] =
                    self.check_ports(&[(Direction::Input, "value"), (Direction::Output, "bits")]);

                writeln!(self.program_text, "\tassign bits = value;").unwrap();
            }
            "BitsToInt" => {
                let [_num_bits] = args.cast_to_int_array();
                let [_bits_port, _value_port] =
                    self.check_ports(&[(Direction::Input, "bits"), (Direction::Output, "value")]);

                writeln!(self.program_text, "\tassign value = bits;").unwrap();
            }
            "UIntToBits" => {
                let [_num_bits] = args.cast_to_int_array();
                let [_value_port, _bits_port] =
                    self.check_ports(&[(Direction::Input, "value"), (Direction::Output, "bits")]);

                writeln!(self.program_text, "\tassign bits = value;").unwrap();
            }
            "BitsToUInt" => {
                let [_num_bits] = args.cast_to_int_array();
                let [_bits_port, _value_port] =
                    self.check_ports(&[(Direction::Input, "bits"), (Direction::Output, "value")]);

                writeln!(self.program_text, "\tassign value = bits;").unwrap();
            }
            "unsafe_int_cast" => {
                let [_from_i, _to_i, _from, _to] = args.cast_to_int_array();
                let [_bits_port, _value_port] =
                    self.check_ports(&[(Direction::Input, "in"), (Direction::Output, "out")]);

                writeln!(self.program_text, "\tassign out = in;").unwrap();
            }
            "transmute_to_bits" => {
                let [typ] = args.cast_to_array();
                let typ = typ.unwrap_type();

                let [_value_port, _bits_port] =
                    self.check_ports(&[(Direction::Input, "value"), (Direction::Output, "bits")]);

                self.in_generate(|slf| {
                    slf.foreach_for_copy_unpacked(typ, false, |path, num_bits| {
                        if path.is_empty() {
                            format!("assign bits = value{path};\n")
                        } else {
                            let path_formula = path.to_bit_index_formula();
                            format!("assign bits[({path_formula}) * {num_bits} +: {num_bits}] = value{path};\n")
                        }
                    })
                });
            }
            "transmute_from_bits" => {
                let [typ] = args.cast_to_array();
                let typ = typ.unwrap_type();

                let [_bits_port, _value_port] =
                    self.check_ports(&[(Direction::Input, "bits"), (Direction::Output, "value")]);

                self.in_generate(|slf| {
                    slf.foreach_for_copy_unpacked(typ, false, |path, num_bits| {
                        if path.is_empty() {
                            format!("assign value{path} = bits;\n")
                        } else {
                            let path_formula = path.to_bit_index_formula();
                            format!("assign value{path} = bits[({path_formula}) * {num_bits} +: {num_bits}];\n")
                        }
                    })
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
            _ => "wire", // Has to be "wire", because `logic[5:0] v = other_wire;` would *initialize* v to other_wire, so we need to use `wire`
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
        genvars: VariableAlloc::new("_g"),
        for_vars: VariableAlloc::new("_v"),
        use_latency,
        needed_untils: instance.compute_needed_untils(),
    };
    ctx.write_verilog_code();

    ctx.program_text
}
