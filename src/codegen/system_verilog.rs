use std::ops::Deref;
use std::rc::Rc;

use ibig::{IBig, UBig};
use sus_proc_macro::get_builtin_type;

use crate::alloc::zip_eq;
use crate::codegen::patches::patch_empty_modules_should_have_content;
use crate::latency::AbsLat;
use crate::linker::{IsExtern, LinkInfo};
use crate::prelude::*;

use crate::flattening::{BinaryOperator, Direction, Module, PartSelectDirection, UnaryOperator};
use crate::instantiation::{
    InstantiatedModule, InstantiatedPort, IsPort, MultiplexerSource, RealWire, RealWireDataSource,
    RealWirePathElem,
};
use crate::to_string::{FmtWrapper, display_join};
use crate::typing::concrete_type::{ConcreteGlobalReference, ConcreteTemplateArg, IntBounds};
use crate::typing::template::{TVec, TemplateKind};
use crate::{typing::concrete_type::ConcreteType, value::Value};

use std::fmt::{Display, Write};

pub fn gen_verilog_code(instance: &InstantiatedModule, linker: &Linker) -> String {
    let mut ctx = CodeGenerationContext {
        md: &linker.modules[instance.global_ref.id],
        instance,
        linker,
        program_text: String::new(),
        genvars: VariableAlloc::new("_g"),
        for_vars: VariableAlloc::new("_v"),
        needed_untils: instance.compute_needed_untils(),
    };

    crate::debug::debug_context("codegen", instance.name.clone(), || {
        ctx.write_verilog_code();
    });

    ctx.program_text
}

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

impl ConcreteType {
    /// Zero-sized wires are not supported by SystemVerilog, therefore we don't generate them
    fn is_zero_sized(&self) -> bool {
        self.sizeof() == ibig::ubig!(0)
    }

    fn zero_sized_inline_value(&self) -> impl Display {
        assert_eq!(self.sizeof(), ibig::ubig!(0));

        FmtWrapper(move |f| {
            match self {
                ConcreteType::Named(global_ref) => match global_ref.id {
                    get_builtin_type!("int") => write!(f, "1'd0"),
                    _ => unreachable!("Unknown zero-sized type {:?}", global_ref.id),
                },
                ConcreteType::Array(_) => unreachable!(
                    "Since this is for inline values, and arrays cannot be used inline, they cannot appear in [zero_sized_inline_value]"
                ),
                /*// Turns out, totally possible: myFunc([0, 0, 0])
                ConcreteType::Array(arr_typ) => {
                    let (content, sz) = arr_typ.deref();

                    let content_str = display_zero_sized_type_inline_value(content);

                    let content_repeats = display_join(", ", 0..sz.unwrap_int(), |f, _| {
                        f.write_str(content_str.as_ref())
                    });
                    write!(f, "'{{{content_repeats}}}")
                }*/
            }
        })
    }

    fn walk_path(&self, path: &[PathElem]) -> &ConcreteType {
        let mut t = self;
        for p in path {
            t = match p {
                PathElem::Array { .. } => &t.unwrap_array().0,
            };
        }
        t
    }
}

fn should_not_codegen_assign(source: &MultiplexerSource) -> bool {
    source.to_path.iter().any(|e| match e {
        RealWirePathElem::Index { .. } | RealWirePathElem::ConstIndex { .. } => false,
        RealWirePathElem::PartSelect { width, .. } => width == &IBig::from(0),
        RealWirePathElem::Slice { bounds, .. } => bounds.unwrap_valid().is_empty(),
    })
}

pub fn display_wire_name_with_latency(wire: &RealWire, target_abs_lat: AbsLat) -> impl Display {
    FmtWrapper(move |f| {
        let wire_abs_lat = wire.absolute_latency.unwrap();
        let target_abs_lat = target_abs_lat.unwrap();
        assert!(wire_abs_lat <= target_abs_lat);
        if wire_abs_lat != target_abs_lat {
            if target_abs_lat < 0 {
                write!(f, "_{}_N{}", wire.name, -target_abs_lat)
            } else {
                write!(f, "_{}_D{}", wire.name, target_abs_lat)
            }
        } else {
            write!(f, "{}", &wire.name)
        }
    })
}

fn codegen_optimized_modulo(
    left_int_range: IntBounds<&IBig>,
    right_int_range: IntBounds<&IBig>,
    left: &str,
    right: &str,
) -> String {
    let modulo_to_minus_one = right_int_range.to - 1;
    if right_int_range.from == &modulo_to_minus_one {
        let mod_s = right_int_range.from;
        let mod_u = UBig::try_from(modulo_to_minus_one).unwrap();
        if mod_u.is_power_of_two() {
            // Unsigned/Signed mod power of two
            let num_bits_to_slice = mod_u.trailing_zeros().unwrap();
            if num_bits_to_slice == 0 {
                "0; // == mod 1".to_string()
            } else {
                format!("{left}; // == mod {mod_u} (target is {num_bits_to_slice} bits wide)")
            }
        } else if !left_int_range.is_signed() {
            // Unsigned mod
            if left_int_range.to <= mod_s {
                format!("{left}; // == mod {mod_u}")
            } else if *left_int_range.to == mod_s + 1 {
                // Optimize rollover by one
                format!("({left} == {mod_u}) ? 0 : {left}; // == mod {mod_u}")
            } else if *left_int_range.to <= mod_s * 2 {
                // Optimize to a conditional subtract
                format!("{left} - (({left} >= {mod_u}) ? {mod_u} : 0); // == mod {mod_u}")
            } else {
                format!("{left} % {mod_u}; // == mod {mod_u}")
            }
        } else if left_int_range.to <= mod_s {
            // Signed mod
            if left_int_range.from == &IBig::from(-1) {
                // Optimize rollover by one
                format!("({left} < 0) ? {} : {left}; // == mod {mod_u}", &mod_u - 1)
            } else if *left_int_range.from >= -mod_s {
                // Optimize to a conditional add
                format!("{left} + (({left} < 0) ? {mod_u} : 0); // == mod {mod_u}")
            } else {
                format!(
                    "$unsigned({left} % {mod_u}) + (({left} % {mod_u} < 0) ? {mod_u} : 0); // == mod {mod_u}"
                )
            }
        } else {
            format!(
                "$unsigned({left} % {mod_u}) + (({left} % {mod_u} < 0) ? {mod_u} : 0); // == mod {mod_u}"
            )
        }
    } else if left_int_range.is_signed() {
        format!(
            "$unsigned({left} % $signed({{1'b0, {right}}})) + ({left} % $signed({{1'b0, {right}}}) < 0 ? {right} : 0); // == mod"
        )
    } else {
        format!("{left} % {right}; // == mod")
    }
}

/// Used as &[ForEachPathElement]
enum PathElem {
    Array { idx: String },
}

impl PathElem {
    fn display_path(path: &[PathElem]) -> impl Display {
        FmtWrapper(move |f| {
            for p in path {
                match p {
                    PathElem::Array { idx } => {
                        write!(f, "[{idx}]")?;
                    }
                }
            }
            Ok(())
        })
    }
    fn make_bit_index_formula(mut typ: &ConcreteType, path: &[PathElem]) -> String {
        assert!(!path.is_empty());

        let mut path_iter = path.iter();
        let first_path_elem = path_iter.next().unwrap(); // Path can't be empty, it's called by ToBits & FromBits.
        let mut result = match first_path_elem {
            PathElem::Array { idx } => {
                typ = &typ.unwrap_array().0;
                idx.clone()
            }
        };

        for p in path_iter {
            match p {
                PathElem::Array { idx } => {
                    let (content, arr_size) = typ.unwrap_array();
                    typ = content;
                    result = format!("({arr_size} * {result}) + {idx}");
                }
            }
        }

        result
    }
}

struct CommaSeparatedList {
    /// (is_commented, line_text)
    lines: Vec<(bool, String)>,
    comment_text: &'static str,
}
impl CommaSeparatedList {
    fn new(comment_text: &'static str) -> Self {
        Self {
            lines: Vec::new(),
            comment_text,
        }
    }
    fn line(&mut self, line: String) {
        self.lines.push((false, line));
    }
    fn commented(&mut self, line: String) {
        self.lines.push((true, line));
    }
}
impl Display for CommaSeparatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.lines.is_empty() {
            return Ok(());
        }
        writeln!(f)?;
        let last_non_comment_line = self.lines.iter().rposition(|l| !l.0).unwrap_or(0);
        for (idx, (is_commented, line)) in self.lines.iter().enumerate() {
            if *is_commented {
                let c = self.comment_text;
                writeln!(f, "\t{c}{line}")?;
            } else {
                let comma = if idx < last_non_comment_line { "," } else { "" };
                writeln!(f, "\t{line}{comma}")?;
            }
        }
        Ok(())
    }
}

struct WireName<'g> {
    instance: &'g InstantiatedModule,
    wire: &'g RealWire,
    abs_lat: AbsLat,
    no_inlining: bool,
}
impl<'g> Display for WireName<'g> {
    /// Creates a string representation of the wire name, useable in expressions. myArr[{}]
    /// xyz, or zero-sized 1'b0, or inlining the constant 40'd545135135513...
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            instance,
            wire,
            abs_lat,
            no_inlining,
        } = self;
        if *no_inlining {
            display_wire_name_with_latency(wire, *abs_lat).fmt(f)
        } else if wire.typ.is_zero_sized() {
            wire.typ.zero_sized_inline_value().fmt(f)
        } else {
            match &wire.source {
                RealWireDataSource::Constant { value } if can_inline(wire) => {
                    display_constant(&wire.typ, value).fmt(f)
                }
                RealWireDataSource::Select { root, path } if path.is_empty() => {
                    // Inline empty selects for readability.
                    display_wire_name_with_latency(&instance.wires[*root], *abs_lat).fmt(f)
                }
                _other => display_wire_name_with_latency(wire, *abs_lat).fmt(f),
            }
        }
    }
}
impl<'g> WireName<'g> {
    /// Same as [Self::with_path], but with multiple
    fn with_paths<const N: usize>(&self, paths: [&[PathElem]; N]) -> impl Display {
        FmtWrapper(move |f| {
            let wire = self.wire;
            let mut final_value_typ = &wire.typ;
            for path in paths {
                final_value_typ = final_value_typ.walk_path(path);
            }
            if final_value_typ.is_zero_sized() {
                // No path added here, since we're just returning the zero-sized value for this type.
                final_value_typ.zero_sized_inline_value().fmt(f)
            } else {
                self.fmt(f)?;
                for path in paths {
                    let path = PathElem::display_path(path);
                    path.fmt(f)?;
                }
                Ok(())
            }
        })
    }
    /// Creates a string representation of the wire name
    /// xyz[a][b]...
    /// May override the result if the wire is zero-sized.
    fn with_path(&self, path: &[PathElem]) -> impl Display {
        self.with_paths([path])
    }

    /// Creates the Verilog variable declaration for tbis variable.
    ///
    /// IE for `int[15] myVar` it creates `[31:0] myVar[14:0]`
    ///
    /// May return something with a leading space, to accomodate `logic`, `input`, etc.
    fn make_declaration(&self) -> String {
        assert!(self.no_inlining); // This means the wire must be declared somewhere
        let mut typ = &self.wire.typ;
        let mut array_string = String::new();

        loop {
            match typ {
                ConcreteType::Named(content_typ) => match content_typ.id {
                    get_builtin_type!("int") => {
                        let bounds = content_typ.unwrap_int_bounds();
                        let bitwidth = bounds.bitwidth() - 1;
                        if bounds.from < &IBig::from(0) {
                            return format!(" signed[{bitwidth}:0] {self}{array_string}");
                        } else {
                            return format!("[{bitwidth}:0] {self}{array_string}");
                        }
                    }
                    get_builtin_type!("bool") => return format!(" {self}{array_string}"),
                    get_builtin_type!("float") => {
                        return format!("[31:0] {self}{array_string}");
                    }
                    get_builtin_type!("double") => {
                        return format!("[63:0] {self}{array_string}");
                    }
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
                        return format!("[{sz}:0] {self}{array_string}");
                    }
                    write!(array_string, "[{sz}:0]").unwrap();
                    typ = content_typ;
                }
            }
        }
    }
}

/// This is for making the resulting Verilog a little nicer to read
fn can_inline(wire: &RealWire) -> bool {
    match &wire.source {
        RealWireDataSource::Constant { .. } => {
            if let ConcreteType::Named(r) = &wire.typ {
                matches!(
                    r.id,
                    get_builtin_type!("int")
                        | get_builtin_type!("bool")
                        | get_builtin_type!("float")
                        | get_builtin_type!("double")
                )
            } else {
                false
            }
        }
        RealWireDataSource::Select { root: _, path } if path.is_empty() => true,
        _other => false,
    }
}

fn display_constant(typ: &ConcreteType, cst: &Value) -> impl Display {
    FmtWrapper(move |f| match typ {
        ConcreteType::Named(global_ref) => match global_ref.id {
            get_builtin_type!("bool") => {
                let b = match cst {
                    Value::Bool(true) => "1'b1",
                    Value::Bool(false) => "1'b0",
                    Value::Unset => "1'bx",
                    _ => unreachable!(),
                };
                f.write_str(b)
            }
            get_builtin_type!("int") => {
                let bounds = global_ref.unwrap_int_bounds();

                let bitwidth = bounds.bitwidth();

                match cst {
                    Value::Integer(v) => {
                        if bounds.from < &IBig::from(0) {
                            if v < &IBig::from(0) {
                                write!(f, "-{bitwidth}'sd{}", -v)
                            } else {
                                write!(f, "{bitwidth}'sd{v}")
                            }
                        } else {
                            write!(f, "{bitwidth}'d{v}")
                        }
                    }
                    Value::Unset => {
                        if bounds.from < &IBig::from(0) {
                            write!(f, "{bitwidth}'sdx")
                        } else {
                            write!(f, "{bitwidth}'dx")
                        }
                    }
                    _ => unreachable!(),
                }
            }
            get_builtin_type!("float") => match cst {
                Value::Float(fl32) => {
                    let as_bits = fl32.to_bits();
                    write!(f, "32'h{as_bits:08x} /* {cst} */")
                }
                Value::Unset => write!(f, "'x"),
                _ => unreachable!(),
            },
            get_builtin_type!("double") => match cst {
                Value::Double(fl64) => {
                    let as_bits = fl64.to_bits();
                    write!(f, "64'h{as_bits:16x} /* {cst} */")
                }
                Value::Unset => write!(f, "'x"),
                _ => unreachable!(),
            },
            _ => todo!("Structs"),
        },
        ConcreteType::Array(arr_box) => {
            let (content_typ, size) = arr_box.deref();

            let size: usize = size.unwrap_int();
            if let ConcreteType::Named(ConcreteGlobalReference {
                id: get_builtin_type!("bool"),
                ..
            }) = content_typ
            {
                write!(f, "{size}'b")?;
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
                            f.write_char(b)?;
                        }
                    }
                    Value::Unset => {
                        for _ in 0..size {
                            f.write_char('x')?;
                        }
                    }
                    _ => unreachable!(),
                }
                Ok(())
            } else {
                match cst {
                    Value::Array(values) => {
                        assert_eq!(values.len(), size);
                        let content = display_join(", ", values.iter(), |f, v| {
                            display_constant(content_typ, v).fmt(f)
                        });
                        write!(f, "'{{{content}}}")
                    }
                    Value::Unset => {
                        let content = display_join(", ", 0..size, |f, _| {
                            display_constant(content_typ, &Value::Unset).fmt(f)
                        });
                        write!(f, "'{{{content}}}")
                    }
                    _ => unreachable!(),
                }
            }
        }
    })
}

struct CodeGenerationContext<'g> {
    /// Generate code to this variable
    program_text: String,
    for_vars: VariableAlloc,
    genvars: VariableAlloc,

    md: &'g Module,
    instance: &'g InstantiatedModule,
    linker: &'g Linker,

    needed_untils: FlatAlloc<i64, WireIDMarker>,
}

impl<'g> CodeGenerationContext<'g> {
    fn wire_name(&self, wire: &'g RealWire, requested_latency: AbsLat) -> WireName<'g> {
        WireName {
            instance: self.instance,
            wire,
            abs_lat: requested_latency,
            no_inlining: false,
        }
    }
    fn wire_name_no_inling(&self, wire: &'g RealWire, requested_latency: AbsLat) -> WireName<'g> {
        WireName {
            instance: self.instance,
            wire,
            abs_lat: requested_latency,
            no_inlining: true,
        }
    }
    fn output_wire_name(&self, wire: &'g RealWire) -> WireName<'g> {
        WireName {
            instance: self.instance,
            wire,
            abs_lat: wire.absolute_latency,
            no_inlining: true,
        }
    }

    fn add_latency_registers(
        &mut self,
        wire_id: WireID,
        w: &RealWire,
    ) -> Result<(), std::fmt::Error> {
        assert!(!w.typ.is_zero_sized());

        // Can do 0 iterations, when w.needed_until == w.absolute_latency. Meaning it instantiates no registers
        for i in w.absolute_latency.unwrap()..self.needed_untils[wire_id] {
            let from = self.wire_name_no_inling(w, AbsLat::new(i));
            let to = self.wire_name_no_inling(w, AbsLat::new(i + 1));

            let to_decl = to.make_declaration();

            let clk_name = self.md.get_clock_name();
            writeln!(
                self.program_text,
                "/*latency*/ logic{to_decl}; always_ff @(posedge {clk_name}) begin {to} <= {from}; end"
            ).unwrap();
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

        // Patch XRT 2.16's over-zealous DRC for empty modules.
        patch_empty_modules_should_have_content(&mut self.program_text);
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
        let module_name = &self.instance.mangled_name;
        let mut port_list = CommaSeparatedList::new("// (zero sized) ");
        port_list.line(format!("input {clk_name}"));
        for (_id, port_wire) in &self.instance.wires {
            let IsPort::Port(_, direction) = port_wire.is_port else {
                continue;
            };
            port_wire.get_span(&self.md.link_info).debug();
            if port_wire.typ.is_zero_sized() {
                port_list.commented(format!("{direction} {}", port_wire.name));
            } else {
                let wire_or_reg = port_wire.source.wire_or_reg();
                let is_state = match direction {
                    Direction::Input => &None,
                    Direction::Output => {
                        let_unwrap!(
                            RealWireDataSource::Multiplexer { is_state, .. },
                            &port_wire.source
                        );
                        is_state
                    }
                };
                let output_name = self.output_wire_name(port_wire);
                let output_decl = output_name.make_declaration();
                let decl = Self::display_declaration(port_wire, wire_or_reg, output_decl, is_state);

                port_list.line(format!("{direction} {decl}"));
            }
        }
        writeln!(self.program_text, "module {module_name}({port_list});\n").unwrap();

        // Add latency registers for the interface declarations
        // Should not appear in the program text for extern modules
        for (port_wire_id, port_wire) in &self.instance.wires {
            port_wire.get_span(&self.md.link_info).debug();
            if port_wire.typ.is_zero_sized() {
                continue;
            }
            if matches!(port_wire.is_port, IsPort::Port(_, _)) {
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
        typ: &ConcreteType,
        in_always: bool,
        mut operation: impl FnMut(&[PathElem], u64) -> String,
    ) -> String {
        fn foreach_for_copy_unpacked_recurse<'g>(
            slf: &mut CodeGenerationContext<'g>,
            typ: &ConcreteType,
            in_always: bool,
            mut path: Vec<PathElem>,
            operation: &mut impl FnMut(&[PathElem], u64) -> String,
        ) -> String {
            if let Some(fundamental_size) = typ.can_be_represented_as_packed_bits() {
                operation(&path, fundamental_size)
            } else {
                match typ {
                    ConcreteType::Named(_) => {
                        todo!("Structs");
                    }
                    ConcreteType::Array(arr_box) => {
                        let (new_typ, sz) = arr_box.deref();
                        let arr_size = sz.unwrap_integer();
                        let (for_stm, idx) = slf.mk_for(arr_size, in_always);
                        path.push(PathElem::Array {
                            idx: idx.to_string(),
                        });
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
            &[PathElem],
            &[PathElem],
            &'g ConcreteType,
        ) -> String,
    ) -> String {
        let mut source_path = Vec::new();
        let mut target_path = Vec::new();
        let mut for_stack = String::new();
        let mut ends_stack = String::new();
        for p in path {
            match p {
                RealWirePathElem::Index { idx_wire, .. } => {
                    let (arr_content, _sz) = typ.unwrap_array();
                    typ = arr_content;
                    let idx_wire_name =
                        self.wire_name(&self.instance.wires[*idx_wire], requested_latency);
                    source_path.push(PathElem::Array {
                        idx: idx_wire_name.to_string(),
                    });
                }
                RealWirePathElem::ConstIndex { idx, .. } => {
                    let (arr_content, _sz) = typ.unwrap_array();
                    typ = arr_content;
                    source_path.push(PathElem::Array {
                        idx: idx.to_string(),
                    });
                }
                RealWirePathElem::PartSelect {
                    from_wire,
                    width,
                    direction,
                    ..
                } => {
                    let (arr_content, _sz) = typ.unwrap_array();
                    typ = arr_content;
                    let from_wire = &self.instance.wires[*from_wire];

                    let (for_stm, var) = self.mk_for(width, in_always);

                    writeln!(for_stack, "{for_stm} begin").unwrap();
                    writeln!(ends_stack, "end").unwrap();

                    let wire_name = self.wire_name(from_wire, requested_latency);
                    target_path.push(PathElem::Array {
                        idx: var.to_string(),
                    });

                    match direction {
                        PartSelectDirection::Up => {
                            source_path.push(PathElem::Array {
                                idx: format!("{wire_name} + {var}"),
                            });
                        }
                        PartSelectDirection::Down => {
                            let sz_dec = width - 1;
                            source_path.push(PathElem::Array {
                                idx: format!("{wire_name} - ({sz_dec} - {var})"),
                            });
                        }
                    }
                }
                RealWirePathElem::Slice { bounds, .. } => {
                    let (arr_content, _sz) = typ.unwrap_array();
                    typ = arr_content;

                    let IntBounds { from, to } = bounds.unwrap_valid();

                    let (for_stm, var) = self.mk_for(&(to - from), in_always);

                    writeln!(for_stack, "{for_stm} begin").unwrap();
                    writeln!(ends_stack, "end").unwrap();

                    target_path.push(PathElem::Array {
                        idx: var.to_string(),
                    });
                    if from == &IBig::from(0) {
                        source_path.push(PathElem::Array {
                            idx: var.to_string(),
                        });
                    } else {
                        source_path.push(PathElem::Array {
                            idx: format!("{from} + {var}"),
                        });
                    }
                }
            }
        }

        let content = operation(self, &source_path, &target_path, typ);
        format!("{for_stack}{content}{ends_stack}")
    }

    fn write_wire_declarations(&mut self) {
        for (wire_id, w) in &self.instance.wires {
            w.get_span(&self.md.link_info).debug();
            // For better readability of output Verilog
            if can_inline(w) {
                continue;
            }

            if matches!(w.is_port, IsPort::Port(_, _)) {
                continue;
            }
            if w.typ.is_zero_sized() {
                writeln!(self.program_text, "// (zero sized) {}", w.name).unwrap();
                continue;
            }
            let wire_or_reg = w.source.wire_or_reg();
            let output_name = self.output_wire_name(w);
            let output_decl = output_name.make_declaration();

            match &w.source {
                RealWireDataSource::Select { root, path } => {
                    let root = &self.instance.wires[*root];
                    let root_name = self.wire_name(root, w.absolute_latency);

                    // Custom [Self::in_generate], to generate logic[31:0] my_val = 5 + other_val
                    self.genvars.reuse();
                    self.for_vars.reuse();
                    let content = self.foreach_for_real_path(
                        &root.typ,
                        path,
                        w.absolute_latency,
                        false,
                        |slf, source_path, target_path, result_typ| {
                            slf.foreach_for_copy_unpacked(result_typ, false, |path, _| {
                                let source = root_name.with_paths([source_path, path]);
                                let target = output_name.with_paths([target_path, path]);
                                format!("assign {target} = {source};\n")
                            })
                        },
                    );

                    if self.genvars.currently_used != 0 {
                        write!(
                            self.program_text,
                            "{wire_or_reg}{output_decl};\ngenerate\n{content}endgenerate\n"
                        )
                        .unwrap();
                    } else {
                        // We're basically trimming "<assign wire_name>[...] = ..." off the string, so we can stitch it to the declaration
                        let content = content.strip_prefix("assign ").unwrap();
                        let out_name_string = output_name.to_string();
                        let content = content.strip_prefix(&out_name_string).unwrap();
                        write!(self.program_text, "{wire_or_reg}{output_decl}{content}").unwrap();
                    }
                }
                RealWireDataSource::UnaryOp { op, right, .. } => {
                    writeln!(self.program_text, "{wire_or_reg}{output_decl};").unwrap();

                    let right = &self.instance.wires[*right];
                    let right_name = self.wire_name(right, w.absolute_latency);

                    let for_var = match op {
                        UnaryOperator::Sum | UnaryOperator::Product => Some(self.for_vars.alloc()),
                        _ => None,
                    };
                    self.in_generate(|slf| {
                        slf.foreach_for_copy_unpacked(&w.typ, false, |path, _| {
                            let output = output_name.with_path(path);

                            let op_sv = match op {
                                UnaryOperator::And => "&",
                                UnaryOperator::Or => "|",
                                UnaryOperator::Xor => "^",
                                UnaryOperator::Not => "~",// SystemVerilog's '!' operator is like C's. !8b10101110 = 0
                                UnaryOperator::Sum => "+",
                                UnaryOperator::Product => "*",
                                UnaryOperator::Negate => "-",
                            };
                            match op {
                                UnaryOperator::And
                                | UnaryOperator::Or
                                | UnaryOperator::Xor
                                | UnaryOperator::Not
                                | UnaryOperator::Negate => {
                                    let right_with_path = right_name.with_path(path);
                                    format!("assign {output} = {op_sv}{right_with_path};\n")
                                }
                                UnaryOperator::Sum |
                                UnaryOperator::Product => {
                                    let start_at = match op {
                                        UnaryOperator::Sum => "0",
                                        UnaryOperator::Product => "1",
                                        _ => unreachable!()
                                    };
                                    let list_len = right.typ.walk_path(path).unwrap_array().1.unwrap_integer();
                                    let for_var = for_var.clone().unwrap();
                                    let for_var_path = [PathElem::Array { idx: for_var.to_string() }];
                                    let right_with_paths = right_name.with_paths([path, &for_var_path]);
                                    format!("always_comb begin\n\t{output} = {start_at};\n\tfor(int {for_var} = 0; {for_var} < {list_len}; {for_var} += 1) {output} {op_sv}= {right_with_paths};\nend\n")
                                }
                            }
                        })
                    })
                }
                RealWireDataSource::BinaryOp {
                    op, left, right, ..
                } => {
                    writeln!(self.program_text, "{wire_or_reg}{output_decl};").unwrap();

                    let left = &self.instance.wires[*left];
                    let right = &self.instance.wires[*right];
                    let left_name = self.wire_name(left, w.absolute_latency);
                    let right_name = self.wire_name(right, w.absolute_latency);
                    self.in_generate(|slf| {
                        slf.foreach_for_copy_unpacked(&w.typ, false, |path, _| {
                            let left_typ =  left.typ.walk_path(path);
                            let right_typ = right.typ.walk_path(path);

                            let output_with_path = output_name.with_path(path);
                            let left_with_path = left_name.with_path(path);
                            let right_with_path = right_name.with_path(path);

                            fn wrap_in_signed_if_needed(name_with_path: impl Display, require_signed: bool, bounds: IntBounds<&IBig>) -> impl Display {
                                FmtWrapper(move |f| {
                                    if require_signed && !bounds.is_signed() {
                                        write!(f, "$signed({{1'b0, {name_with_path}}})")
                                    } else {
                                        name_with_path.fmt(f)
                                    }
                                })
                            }

                            match *op {
                                BinaryOperator::ShiftLeft | BinaryOperator::ShiftRight => {
                                    let left_int_range = left_typ.unwrap_int_bounds();
                                    let right_int_range = right_typ.unwrap_int_bounds();

                                    let shift_op = match (*op, left_int_range.is_signed()) {
                                        (BinaryOperator::ShiftLeft, true) => "<<<",
                                        (BinaryOperator::ShiftLeft, false) => "<<",
                                        (BinaryOperator::ShiftRight, true) => ">>>",
                                        (BinaryOperator::ShiftRight, false) => ">>",
                                        _ => unreachable!()
                                    };

                                    assert!(!right_int_range.is_signed());

                                    format!("assign {output_with_path} = {left_with_path} {shift_op} {right_with_path};\n")
                                }
                                BinaryOperator::And |
                                BinaryOperator::Or |
                                BinaryOperator::Xor => {
                                    let op_sv = match op {
                                        BinaryOperator::And => "&",
                                        BinaryOperator::Or => "|",
                                        BinaryOperator::Xor => "^",
                                        _ => unreachable!()
                                    };
                                    format!("assign {output_with_path} = {left_with_path} {op_sv} {right_with_path};\n")
                                }
                                BinaryOperator::Add |
                                BinaryOperator::Subtract |
                                BinaryOperator::Multiply |
                                BinaryOperator::Divide |
                                BinaryOperator::Remainder |
                                BinaryOperator::Equals |
                                BinaryOperator::NotEquals |
                                BinaryOperator::Greater |
                                BinaryOperator::GreaterEq |
                                BinaryOperator::Lesser |
                                BinaryOperator::LesserEq => {
                                    let op_sv = match op {
                                        BinaryOperator::Add => "+",
                                        BinaryOperator::Subtract => "-",
                                        BinaryOperator::Multiply => "*",
                                        BinaryOperator::Divide => "/",
                                        BinaryOperator::Remainder => "%",
                                        BinaryOperator::Equals => "==",
                                        BinaryOperator::NotEquals => "!=",
                                        BinaryOperator::Greater => ">",
                                        BinaryOperator::GreaterEq => ">=",
                                        BinaryOperator::Lesser => "<",
                                        BinaryOperator::LesserEq => "<=",
                                        _ => unreachable!()
                                    };
                                    let left_int_range = left_typ.unwrap_int_bounds();
                                    let right_int_range = right_typ.unwrap_int_bounds();

                                    let op_is_signed = left_int_range.is_signed() | right_int_range.is_signed();
                                    let left_arg = wrap_in_signed_if_needed(left_with_path, op_is_signed, left_int_range);
                                    let right_arg = wrap_in_signed_if_needed(right_with_path, op_is_signed, right_int_range);

                                    format!("assign {output_with_path} = {left_arg} {op_sv} {right_arg};\n")
                                }
                                BinaryOperator::Modulo => {
                                    let left_int_range =
                                        left.typ.walk_path(path).unwrap_int_bounds();
                                    let right_int_range =
                                        right.typ.walk_path(path).unwrap_int_bounds();

                                    let content = codegen_optimized_modulo(
                                        left_int_range,
                                        right_int_range,
                                        &format!("{left_with_path}"),
                                        &format!("{right_with_path}"),
                                    );
                                    format!("assign {output_with_path} = {content}\n")
                                }
                            }
                        })
                    });
                }
                RealWireDataSource::Constant { value } => {
                    let const_str = display_constant(&w.typ, value);
                    writeln!(
                        self.program_text,
                        "{wire_or_reg}{output_decl} = {const_str};"
                    )
                    .unwrap();
                }
                RealWireDataSource::ReadOnly => {
                    writeln!(self.program_text, "{wire_or_reg}{output_decl};").unwrap();
                }
                RealWireDataSource::ConstructArray { array_wires } => {
                    writeln!(self.program_text, "{wire_or_reg}{output_decl};").unwrap();

                    for (arr_idx, elem) in array_wires.iter().enumerate() {
                        let elem = &self.instance.wires[*elem];
                        elem.get_span(&self.md.link_info).debug();
                        let element_wire_name = self.wire_name(elem, w.absolute_latency);

                        let idx_path = [PathElem::Array {
                            idx: arr_idx.to_string(),
                        }];
                        self.in_generate(|slf| {
                            slf.foreach_for_copy_unpacked(&elem.typ, false, |path, _| {
                                let elem_with_path = element_wire_name.with_path(path);
                                let output_with_path = output_name.with_paths([&idx_path, path]);
                                format!("assign {output_with_path} = {elem_with_path};\n")
                            })
                        });
                    }
                }
                RealWireDataSource::Multiplexer {
                    is_state,
                    sources: _,
                } => {
                    let decl_stm = Self::display_declaration(w, wire_or_reg, output_decl, is_state);
                    writeln!(self.program_text, "{decl_stm};").unwrap();
                }
            }
            self.add_latency_registers(wire_id, w).unwrap();
        }
    }

    fn display_declaration(
        w: &RealWire,
        wire_or_reg: &str,
        wire_decl: String,
        is_state: &Option<Value>,
    ) -> impl Display {
        FmtWrapper(move |f| {
            write!(f, "{wire_or_reg}{wire_decl}")?;
            match is_state {
                Some(initial_val) if !initial_val.is_unset() => {
                    let cst_str = display_constant(&w.typ, initial_val);
                    write!(f, " = {cst_str}")?;
                }
                _ => {}
            }
            Ok(())
        })
    }

    fn write_submodules(&mut self) {
        let parent_clk_name = self.md.get_clock_name();
        for (_id, sm) in &self.instance.submodules {
            let sm_md = &self.linker.modules[sm.refers_to.id];

            // Invalid submodules are impossible to remain by the time codegen happens
            let sm_inst: &InstantiatedModule = sm.instance.get().unwrap();
            if sm_md.link_info.is_extern == IsExtern::Extern {
                self.write_template_args(&sm_md.link_info, &sm_inst.global_ref.template_args);
            } else {
                self.program_text.write_str(&sm_inst.mangled_name).unwrap();
            };
            let sm_name = &sm.name;

            let mut port_list = CommaSeparatedList::new("// (zero sized port) ");
            let submod_clk = sm_md.get_clock_name();
            port_list.line(format!(".{submod_clk}({parent_clk_name})"));

            for (port_id, iport) in sm_inst.interface_ports.iter_valids() {
                let sm_port = &sm_inst.wires[iport.wire];
                let port_name = &sm_port.name;
                let wire_name = if let Some(port_wire) = &sm.port_map[port_id] {
                    &self.instance.wires[port_wire.maps_to_wire].name
                } else {
                    // Ports that are defined on the submodule, but not used by impl
                    ""
                };
                let line = format!(".{port_name}({wire_name})");
                if sm_port.typ.is_zero_sized() {
                    port_list.commented(line);
                } else {
                    port_list.line(line);
                }
            }
            writeln!(self.program_text, " {sm_name}({port_list});").unwrap();
        }
    }

    fn write_template_args(
        &mut self,
        link_info: &LinkInfo,
        concrete_template_args: &TVec<ConcreteTemplateArg>,
    ) {
        let args = display_join(
            ", ",
            zip_eq(concrete_template_args, &link_info.parameters),
            |f, (_, arg, arg_name)| {
                let arg_name = &arg_name.name;
                match arg {
                    TemplateKind::Type(_) => {
                        unreachable!(
                            "No extern module type arguments. Should have been caught by Lint"
                        );
                    }
                    TemplateKind::Value(value) => {
                        write!(f, ".{arg_name}({value})")
                    }
                }
            },
        );
        let extern_name = &link_info.name;
        write!(self.program_text, "{extern_name} #({args})").unwrap();
    }

    fn write_assign(
        &mut self,
        output_name: &WireName,
        arrow_str: &'static str,
        s: &'g MultiplexerSource,
        target: &'g RealWire,
    ) {
        let from = &self.instance.wires[s.from];
        from.get_span(&self.md.link_info).debug();
        let from_name = self.wire_name(from, target.absolute_latency);
        self.program_text.write_char('\t').unwrap();
        let mut if_stack = String::new();
        for cond in s.condition.iter() {
            let condition_wire = &self.instance.wires[cond.condition_wire];
            condition_wire.get_span(&self.md.link_info).debug();
            let cond_name = self.wire_name(condition_wire, target.absolute_latency);
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
                slf.foreach_for_copy_unpacked(copy_typ, true, |path, _| {
                    let output_with_path = output_name.with_paths([source_path, path]);
                    let from_with_path = from_name.with_paths([target_path, path]);
                    format!("{if_stack}{output_with_path} {arrow_str} {from_with_path};\n")
                })
            },
        );
        self.program_text.write_str(&content).unwrap();
    }

    fn write_multiplexers(&mut self) {
        for (_id, w) in &self.instance.wires {
            w.get_span(&self.md.link_info).debug();

            if w.typ.is_zero_sized() {
                continue;
            }
            w.get_span(&self.md.link_info).debug();
            match &w.source {
                RealWireDataSource::Multiplexer { is_state, sources } => {
                    let output_name = self.output_wire_name(w);
                    let arrow_str = if is_state.is_some() {
                        let clk_name = self.md.get_clock_name();
                        writeln!(
                            self.program_text,
                            "always_ff @(posedge {clk_name}) begin // state {output_name}"
                        )
                        .unwrap();
                        "<="
                    } else {
                        writeln!(self.program_text, "always_comb begin // combinatorial {output_name}\n\t// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches").unwrap();
                        let unset_str = display_constant(&w.typ, &Value::Unset);
                        writeln!(self.program_text, "\t{output_name} = {unset_str};").unwrap();
                        "="
                    };

                    for s in sources {
                        if should_not_codegen_assign(s) {
                            // Eliminate zero-size sub-slice assignments
                            continue;
                        }
                        self.write_assign(&output_name, arrow_str, s, w);
                    }

                    write!(
                        self.program_text,
                        "{}",
                        super::patches::patch_combinatorial_write_one_bit_dont_care(
                            is_state,
                            &output_name,
                            &w.typ
                        )
                    )
                    .unwrap();

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

    fn get_builtin_ports<const N: usize>(
        &self,
        ports: [(Direction, &'static str); N],
    ) -> [&RealWire; N] {
        let actual_ports: &[Option<InstantiatedPort>; N] =
            self.instance.interface_ports.cast_to_array();

        let mut zero_size_count = 0;
        std::array::from_fn(|i| {
            let actual_port = actual_ports[i].as_ref().unwrap();
            let (direction, name) = ports[i];
            let port_wire = &self.instance.wires[actual_port.wire];
            port_wire.get_span(&self.md.link_info).debug();
            assert_eq!(&port_wire.name, name);
            assert_eq!(actual_port.direction, direction);
            if port_wire.typ.is_zero_sized() {
                zero_size_count += 1;
            }
            port_wire
        })
    }

    /// Check the generated ports of builtin modules against what is expected by the builtin codegen.
    /// Returns true if all generated ports are of size 0. That means no implementation should be generated.
    /// Returns false all ports are non-zero. Otherwise this panics, requiring us to implement the more complex combinations of zero/nonzero sized ports.
    fn check_ports_basic<const N: usize>(&self, ports: [(Direction, &'static str); N]) -> bool {
        let port_wires = self.get_builtin_ports(ports);
        let zero_size_count = port_wires.iter().filter(|p| p.typ.is_zero_sized()).count();
        if zero_size_count == 0 {
            false
        } else if zero_size_count == N {
            true
        } else {
            panic!("Mishmash of zero and non-zero sized ports")
        }
    }

    /// TODO probably best to have some smarter system for this in the future.
    fn write_builtins(&mut self) {
        let args = &self.instance.global_ref.template_args;
        use Direction::{Input, Output};
        match self.md.link_info.name.as_str() {
            "LatencyOffset" => {
                if self.check_ports_basic([(Input, "din"), (Output, "dout")]) {
                    return;
                }

                self.program_text
                    .write_str("\tassign dout = din;\n")
                    .unwrap();
            }
            "CrossDomain" => {
                if self.check_ports_basic([(Input, "din"), (Output, "dout")]) {
                    return;
                }

                self.program_text
                    .write_str("\tassign dout = din;\n")
                    .unwrap();
            }
            "IntNarrow" => {
                let [_from_i, _to_i, _from, _to] = args.cast_to_int_array();
                let [din, dout] = self.get_builtin_ports([(Input, "din"), (Output, "dout")]);
                match (din.typ.is_zero_sized(), dout.typ.is_zero_sized()) {
                    (true, true) | (false, true) => {}
                    (true, false) => {
                        writeln!(self.program_text, "\tassign dout = 0;").unwrap();
                    }
                    (false, false) => {
                        writeln!(self.program_text, "\tassign dout = din;").unwrap();
                    }
                }
            }
            "IntToBits" => {
                let [_num_bits] = args.cast_to_int_array();
                if self.check_ports_basic([(Input, "value"), (Output, "bits")]) {
                    return;
                }

                writeln!(self.program_text, "\tassign bits = value;").unwrap();
            }
            "BitsToInt" => {
                let [_num_bits] = args.cast_to_int_array();
                if self.check_ports_basic([(Input, "bits"), (Output, "value")]) {
                    return;
                }

                writeln!(self.program_text, "\tassign value = bits;").unwrap();
            }
            "UIntToBits" => {
                let [_num_bits] = args.cast_to_int_array();
                if self.check_ports_basic([(Input, "value"), (Output, "bits")]) {
                    return;
                }

                writeln!(self.program_text, "\tassign bits = value;").unwrap();
            }
            "BitsToUInt" => {
                let [_num_bits] = args.cast_to_int_array();
                if self.check_ports_basic([(Input, "bits"), (Output, "value")]) {
                    return;
                }

                writeln!(self.program_text, "\tassign value = bits;").unwrap();
            }
            "ToBits" => {
                let [typ] = args.cast_to_array();
                let typ = typ.unwrap_type();

                if self.check_ports_basic([(Input, "value"), (Output, "bits")]) {
                    return;
                }

                self.in_generate(|slf| {
                    slf.foreach_for_copy_unpacked(typ, false, |path, num_bits| {
                        if path.is_empty() {
                            "assign bits = value;\n".to_string()
                        } else {
                            let path_formula = PathElem::make_bit_index_formula(typ, path);
                            let path = PathElem::display_path(path);
                            format!("assign bits[({path_formula}) * {num_bits} +: {num_bits}] = value{path};\n")
                        }
                    })
                });
            }
            "FromBits" => {
                let [typ] = args.cast_to_array();
                let typ = typ.unwrap_type();

                if self.check_ports_basic([(Input, "bits"), (Output, "value")]) {
                    return;
                }

                self.in_generate(|slf| {
                    slf.foreach_for_copy_unpacked(typ, false, |path, num_bits| {
                        if path.is_empty() {
                            "assign value = bits;\n".to_string()
                        } else {
                            let path_formula = PathElem::make_bit_index_formula(typ, path);
                            let path = PathElem::display_path(path);
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
            RealWireDataSource::Constant { .. } => "localparam",
            _ => "wire", // Has to be "wire", because `logic[5:0] v = other_wire;` would *initialize* v to other_wire, so we need to use `wire`
        }
    }
}
