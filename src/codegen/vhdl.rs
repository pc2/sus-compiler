
use crate::{
    flattening::{DeclarationKind, Instruction}, linker::IsExtern, typing::concrete_type::ConcreteType, FlatAlloc, InstantiatedModule, Linker, Module, WireIDMarker
};
use std::ops::Deref;
use std::fmt::Write;

use super::shared::*;

#[derive(Debug)]
pub struct VHDLCodegenBackend;

impl super::CodeGenBackend for VHDLCodegenBackend {
    fn file_extension(&self) -> &str {
        "vhd"
    }
    fn output_dir_name(&self) -> &str {
        "vhdl_output"
    }
    fn codegen(&self, md: &Module, instance: &InstantiatedModule, linker: &Linker, use_latency: bool) -> String {
        gen_vhdl_code(md, instance, use_latency)
    }
}

struct CodeGenerationContext<'g, 'out, Stream: std::fmt::Write> {
    md: &'g Module,
    instance: &'g InstantiatedModule,
    program_text: &'out mut Stream,
    use_latency: bool,
    needed_untils: FlatAlloc<i64, WireIDMarker>,
}

fn typ_to_declaration(mut typ: &ConcreteType) -> String {
    let mut array_string = String::new();
    while let ConcreteType::Array(arr) = typ {
        let (content_typ, size) = arr.deref();
        let sz = size.unwrap_value().unwrap_integer();
        write!(array_string, "array (0 to {}) of", sz - 1).unwrap();
        typ = content_typ;
    }
    match typ {
        ConcreteType::Named(id) => {
            let sz = get_type_name_size(*id);
            if sz == 1 {
                format!("{array_string} std_logic")
            } else {
                format!("{array_string} unsigned({} downto 0)", sz - 1)
            }
        }
        ConcreteType::Array(_) => unreachable!("All arrays have been used up already"),
        ConcreteType::Value(_) | ConcreteType::Unknown(_) => unreachable!(),
    }
}

impl<'g, 'out, Stream: std::fmt::Write> CodeGenerationContext<'g, 'out, Stream> {
    fn write_vhdl_code(&mut self) {
        match self.md.link_info.is_extern {
            IsExtern::Normal => {
                self.write_entity(false);
                self.write_architecture();
                /*self.write_wire_declarations();
                self.write_submodules();
                self.write_multiplexers();
                self.write_endmodule();*/
            }
            IsExtern::Extern => {
                // Do nothing, it's provided externally
                writeln!(self.program_text, "-- Provided externally").unwrap();
                self.write_entity(true);
            }
            IsExtern::Builtin => {
                self.write_entity(false);
                //self.write_builtins();
                //self.write_endmodule();
            }
        }
    }

    fn write_entity(&mut self, commented_out: bool) {
        let comment_text = if commented_out { "-- " } else { "" };
        let instance_name = &self.instance.name;

        let mut it = self.instance.interface_ports.iter_valids().peekable();
        let end = if it.peek().is_some() { ";" } else { "" };
        let clk_name = self.md.get_clock_name();
        write!(
            self.program_text,
            "{comment_text}entity {} is (\n{comment_text}    port (\n        {clk_name} : in std_logic{end}\n",
            instance_name
        )
        .unwrap();

        while let Some((_, port)) = it.next() {
            let port_wire = &self.instance.wires[port.wire];
            let port_name = &port_wire.name;
            let port_direction = if port.is_input { "in" } else { "out" };
            let port_type = typ_to_declaration(&port_wire.typ);
            let end = if it.peek().is_some() { ";" } else { "" };
            write!(
                self.program_text,
                "{comment_text}        {port_name} : {port_direction} {port_type}{end}\n"
            )
            .unwrap();
        }

        write!(
            self.program_text,
            "{comment_text}    );\n{comment_text}end entity {instance_name};\n"
        )
        .unwrap();
    }

    fn write_architecture(&mut self) {
        let instance_name = &self.instance.name;
        writeln!(
            &mut self.program_text,
            "architecture Behavioral of {instance_name} is"
        )
        .unwrap();
        self.write_signal_declarations();
        writeln!(&mut self.program_text, "begin").unwrap();
        writeln!(&mut self.program_text, "end Behavioral;").unwrap();
    }

    fn write_signal_declarations(&mut self) {
        let signals = self
            .instance
            .wires
            .iter()
            .filter(|(_, wire)| {
                if let Instruction::Declaration(wire_decl) =
                    &self.md.link_info.instructions[wire.original_instruction]
                {
                    if let DeclarationKind::RegularPort { .. } = wire_decl.decl_kind {
                        return false;
                    }
                }
                return true;
            })
            .map(|(_, wire)| {
                let signal_name = wire_name_self_latency(wire, self.use_latency);
                let signal_type = typ_to_declaration(&wire.typ);
                format!("    signal {signal_name} : {signal_type}")
            })
            .fold(String::new(), |mut a, b| {
                a.reserve(b.len() + 2);
                if !a.is_empty() {
                    a.push_str(";\n");
                }
                a.push_str(&b);
                a
            });
        self.program_text.write_str(&signals).unwrap();
        self.program_text.write_char('\n').unwrap();
    }
}

fn gen_vhdl_code(md: &Module, instance: &InstantiatedModule, use_latency: bool) -> String {
    todo!("VHDl codegen is unfinshed");
    let mut program_text = String::new();

    let mut ctx = CodeGenerationContext {
        md,
        instance,
        use_latency,
        program_text: &mut program_text,
        needed_untils: instance.compute_needed_untils(),
    };
    ctx.write_vhdl_code();

    program_text
}
