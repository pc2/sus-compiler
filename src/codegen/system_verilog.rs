use std::borrow::Cow;
use std::ops::Deref;
use std::rc::Rc;

use ibig::{IBig, UBig};
use sus_proc_macro::get_builtin_type;

use crate::alloc::zip_eq;
use crate::latency::AbsLat;
use crate::linker::{IsExtern, LinkInfo};
use crate::prelude::*;

use crate::flattening::{BinaryOperator, Direction, Module, PartSelectDirection};
use crate::instantiation::{
    InstantiatedModule, InstantiatedPort, IsPort, MultiplexerSource, RealWire, RealWireDataSource,
    RealWirePathElem,
};
use crate::to_string::{FmtWrapper, display_join};
use crate::typing::concrete_type::{ConcreteGlobalReference, ConcreteTemplateArg, IntBounds};
use crate::typing::template::{TVec, TemplateKind};
use crate::{typing::concrete_type::ConcreteType, value::Value};

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
                get_builtin_type!("float") => {
                    return format!("[31:0] {var_name}{array_string}");
                }
                get_builtin_type!("double") => {
                    return format!("[63:0] {var_name}{array_string}");
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
                    return format!("[{sz}:0] {var_name}{array_string}");
                }
                write!(array_string, "[{sz}:0]").unwrap();
                typ = content_typ;
            }
        }
    }
}

fn should_not_codegen(wire: &RealWire) -> bool {
    wire.typ.sizeof() == ibig::ubig!(0)
}

fn get_zero_sized_type_inline_value(typ: &ConcreteType) -> Cow<'static, str> {
    assert_eq!(typ.sizeof(), ibig::ubig!(0));

    match typ {
        ConcreteType::Named(global_ref) => match global_ref.id {
            get_builtin_type!("int") => Cow::Borrowed("1'd0"),
            _ => unreachable!("Unknown zero-sized type {:?}", global_ref.id),
        },
        ConcreteType::Array(_) => unreachable!(
            "Since this is for inline values, and arrays cannot be used inline, they cannot appear in [get_zero_sized_type_inline_value]"
        ),
    }
}

pub fn wire_name_with_latency(wire: &RealWire, target_abs_lat: AbsLat) -> Cow<'_, str> {
    let wire_abs_lat = wire.absolute_latency.unwrap();
    let target_abs_lat = target_abs_lat.unwrap();
    assert!(wire_abs_lat <= target_abs_lat);
    if wire_abs_lat != target_abs_lat {
        if target_abs_lat < 0 {
            Cow::Owned(format!("_{}_N{}", wire.name, -target_abs_lat))
        } else {
            Cow::Owned(format!("_{}_D{}", wire.name, target_abs_lat))
        }
    } else {
        Cow::Borrowed(&wire.name)
    }
}

pub fn wire_name_self_latency(wire: &RealWire) -> Cow<'_, str> {
    wire_name_with_latency(wire, wire.absolute_latency)
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
                let bitslice_start_at = num_bits_to_slice - 1;
                format!("({left})[{bitslice_start_at}:0]; // == mod {mod_u}")
            }
        } else if left_int_range.from >= &IBig::from(0) {
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
                format!("(({left} % {mod_u}) + {mod_u}) % {mod_u}; // == mod {mod_u}")
            }
        } else {
            format!("(({left} % {mod_u}) + {mod_u}) % {mod_u}; // == mod {mod_u}")
        }
    } else {
        format!("(({left} % {right}) + {right}) % {right}; // == mod")
    }
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

impl<'t> ForEachPath<'_, '_> {
    fn walk_type(&self, mut t: &'t ConcreteType) -> &'t ConcreteType {
        for p in self.path {
            match p {
                ForEachPathElement::Array { .. } => {
                    t = &t.unwrap_array().0;
                }
            }
        }
        t
    }
}

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
                                    write!(f, "{bitwidth}'({v})")
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
                                Self::display_constant(content_typ, v).fmt(f)
                            });
                            write!(f, "'{{{content}}}")
                        }
                        Value::Unset => {
                            let content = display_join(", ", 0..size, |f, _| {
                                Self::display_constant(content_typ, &Value::Unset).fmt(f)
                            });
                            write!(f, "'{{{content}}}")
                        }
                        _ => unreachable!(),
                    }
                }
            }
        })
    }
    /// This is for making the resulting Verilog a little nicer to read
    fn can_inline(&self, wire: &RealWire) -> bool {
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

    fn wire_name(&self, wire: WireID, requested_latency: AbsLat) -> Cow<'g, str> {
        let wire = &self.instance.wires[wire];

        if should_not_codegen(wire) {
            return get_zero_sized_type_inline_value(&wire.typ);
        }
        match &wire.source {
            RealWireDataSource::Constant { value } if self.can_inline(wire) => {
                Cow::Owned(Self::display_constant(&wire.typ, value).to_string())
            }
            RealWireDataSource::Select { root, path } if path.is_empty() => {
                wire_name_with_latency(&self.instance.wires[*root], requested_latency)
            }
            _other => wire_name_with_latency(wire, requested_latency),
        }
    }

    fn add_latency_registers(
        &mut self,
        wire_id: WireID,
        w: &RealWire,
    ) -> Result<(), std::fmt::Error> {
        assert!(!should_not_codegen(w));

        // Can do 0 iterations, when w.needed_until == w.absolute_latency. Meaning it's only needed this cycle
        for i in w.absolute_latency.unwrap()..self.needed_untils[wire_id] {
            let from = wire_name_with_latency(w, AbsLat::new(i));
            let to = wire_name_with_latency(w, AbsLat::new(i + 1));

            let var_decl = typ_to_declaration(&w.typ, &to);

            let clk_name = self.md.get_clock_name();
            writeln!(
                self.program_text,
                "/*latency*/ logic{var_decl}; always_ff @(posedge {clk_name}) begin {to} <= {from}; end"
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
            if should_not_codegen(port_wire) {
                port_list.commented(format!("{direction} {}", port_wire.name));
            } else {
                let wire_doc = port_wire.source.wire_or_reg();
                let wire_name = wire_name_self_latency(port_wire);
                let wire_decl = typ_to_declaration(&port_wire.typ, &wire_name);
                port_list.line(format!("{direction} {wire_doc}{wire_decl}"));
            }
        }
        writeln!(self.program_text, "module {module_name}({port_list});\n").unwrap();

        // Add latency registers for the interface declarations
        // Should not appear in the program text for extern modules
        for (port_wire_id, port_wire) in &self.instance.wires {
            if should_not_codegen(port_wire) {
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

            if matches!(w.is_port, IsPort::Port(_, _)) {
                continue;
            }
            if should_not_codegen(w) {
                writeln!(self.program_text, "// (zero sized) {}", w.name).unwrap();
                continue;
            }
            let wire_or_reg = w.source.wire_or_reg();

            let wire_name = wire_name_self_latency(w);
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
                        let content = content.strip_prefix("assign ").unwrap();
                        let content = content.strip_prefix(wire_name.as_ref()).unwrap();
                        write!(self.program_text, "{wire_or_reg}{wire_decl}{content}").unwrap();
                    }
                }
                RealWireDataSource::UnaryOp { op, right, .. } => {
                    writeln!(self.program_text, "{wire_or_reg}{wire_decl};").unwrap();

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

                    let left_wire = &self.instance.wires[*left];
                    let right_wire = &self.instance.wires[*right];
                    let left_name = self.wire_name(*left, w.absolute_latency);
                    let right_name = self.wire_name(*right, w.absolute_latency);
                    self.in_generate(|slf| {
                        slf.foreach_for_copy_unpacked(&w.typ, false, |path, _| {
                            if *op == BinaryOperator::Modulo {
                                let left_int_range = path.walk_type(&left_wire.typ).unwrap_int_bounds();
                                let right_int_range = path.walk_type(&right_wire.typ).unwrap_int_bounds();

                                let content = codegen_optimized_modulo(left_int_range, right_int_range, &format!("{left_name}{path}"), &format!("{right_name}{path}"));
                                format!(
                                    "assign {wire_name}{path} = {content}\n"
                                )
                            } else {
                                format!(
                                    "assign {wire_name}{path} = {left_name}{path} {op} {right_name}{path};\n"
                                )
                            }
                        })
                    });
                }
                RealWireDataSource::Constant { value } => {
                    let const_str = Self::display_constant(&w.typ, value);
                    writeln!(self.program_text, "{wire_or_reg}{wire_decl} = {const_str};").unwrap();
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
                            let cst_str = Self::display_constant(&w.typ, initial_val);
                            write!(self.program_text, " = {cst_str}",).unwrap();
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
                if should_not_codegen(sm_port) {
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
            if should_not_codegen(w) {
                continue;
            }
            match &w.source {
                RealWireDataSource::Multiplexer { is_state, sources } => {
                    let output_name = wire_name_self_latency(w);
                    let arrow_str = if is_state.is_some() {
                        let clk_name = self.md.get_clock_name();
                        writeln!(self.program_text, "always_ff @(posedge {clk_name}) begin")
                            .unwrap();
                        "<="
                    } else {
                        writeln!(self.program_text, "always_comb begin\n\t// Combinatorial wires are not defined when not valid. This is just so that the synthesis tool doesn't generate latches").unwrap();
                        let unset_str = Self::display_constant(&w.typ, &Value::Unset);
                        writeln!(self.program_text, "\t{output_name} = {unset_str};").unwrap();
                        "="
                    };

                    for s in sources {
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

    /// Returns true if all ports are of size 0, or false if none are. Panics otherwise
    fn check_ports<const N: usize>(&self, ports: [(Direction, &'static str); N]) -> bool {
        let actual_ports: &[Option<InstantiatedPort>; N] =
            self.instance.interface_ports.cast_to_array();

        let mut zero_size_count = 0;
        for i in 0..N {
            let actual_port = actual_ports[i].as_ref().unwrap();
            let (direction, name) = ports[i];
            let port_wire = &self.instance.wires[actual_port.wire];
            assert_eq!(&port_wire.name, name);
            assert_eq!(actual_port.direction, direction);
            if should_not_codegen(port_wire) {
                zero_size_count += 1;
            }
        }

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
                if self.check_ports([(Input, "in"), (Output, "out")]) {
                    return;
                }

                self.program_text.write_str("\tassign out = in;\n").unwrap();
            }
            "CrossDomain" => {
                if self.check_ports([(Input, "in"), (Output, "out")]) {
                    return;
                }

                self.program_text.write_str("\tassign out = in;\n").unwrap();
            }
            "IntToBits" => {
                let [_num_bits] = args.cast_to_int_array();
                if self.check_ports([(Input, "value"), (Output, "bits")]) {
                    return;
                }

                writeln!(self.program_text, "\tassign bits = value;").unwrap();
            }
            "BitsToInt" => {
                let [_num_bits] = args.cast_to_int_array();
                if self.check_ports([(Input, "bits"), (Output, "value")]) {
                    return;
                }

                writeln!(self.program_text, "\tassign value = bits;").unwrap();
            }
            "UIntToBits" => {
                let [_num_bits] = args.cast_to_int_array();
                if self.check_ports([(Input, "value"), (Output, "bits")]) {
                    return;
                }

                writeln!(self.program_text, "\tassign bits = value;").unwrap();
            }
            "BitsToUInt" => {
                let [_num_bits] = args.cast_to_int_array();
                if self.check_ports([(Input, "bits"), (Output, "value")]) {
                    return;
                }

                writeln!(self.program_text, "\tassign value = bits;").unwrap();
            }
            "unsafe_int_cast" => {
                let [_from_i, _to_i, _from, _to] = args.cast_to_int_array();
                if self.check_ports([(Input, "in"), (Output, "out")]) {
                    return;
                }

                writeln!(self.program_text, "\tassign out = in;").unwrap();
            }
            "transmute_to_bits" => {
                let [typ] = args.cast_to_array();
                let typ = typ.unwrap_type();

                if self.check_ports([(Input, "value"), (Output, "bits")]) {
                    return;
                }

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

                if self.check_ports([(Input, "bits"), (Output, "value")]) {
                    return;
                }

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
            RealWireDataSource::Constant { .. } => "localparam",
            _ => "wire", // Has to be "wire", because `logic[5:0] v = other_wire;` would *initialize* v to other_wire, so we need to use `wire`
        }
    }
}

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
    ctx.write_verilog_code();

    ctx.program_text
}
