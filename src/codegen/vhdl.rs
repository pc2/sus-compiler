use super::mangle;
use crate::{
    linker::{get_builtin_type, IsExtern},
    typing::concrete_type::ConcreteType,
    FlatAlloc, InstantiatedModule, Module, TypeUUID, WireIDMarker,
};
use std::fmt::Write;
use std::ops::Deref;

#[derive(Debug)]
pub struct VHDLCodegenBackend;

impl super::CodeGenBackend for VHDLCodegenBackend {
    fn file_extension(&self) -> &str {
        "vhd"
    }
    fn output_dir_name(&self) -> &str {
        "vhdl_output"
    }
    fn comment(&self) -> &str {
        "--"
    }
    fn codegen(&self, md: &Module, instance: &InstantiatedModule) -> String {
        gen_vhdl_code(md, instance)
    }
}

struct CodeGenerationContext<'g, 'out, Stream: std::fmt::Write> {
    md: &'g Module,
    instance: &'g InstantiatedModule,
    program_text: &'out mut Stream,
    needed_untils: FlatAlloc<i64, WireIDMarker>,
}

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
                format!("{array_string} std_logic_vector({} downto 0)", sz - 1)
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
                self.write_module_signature(false);
                /*self.write_wire_declarations();
                self.write_submodules();
                self.write_multiplexers();
                self.write_endmodule();*/
            }
            IsExtern::Extern => {
                // Do nothing, it's provided externally
                writeln!(self.program_text, "-- Provided externally").unwrap();
                self.write_module_signature(true);
            }
            IsExtern::Builtin => {
                self.write_module_signature(false);
                //self.write_builtins();
                //self.write_endmodule();
            }
        }
    }

    fn write_module_signature(&mut self, commented_out: bool) {
        let comment_text = if commented_out { "-- " } else { "" };
        let instance_name = mangle(&self.instance.name);

        let mut it = self.instance.interface_ports.iter_valids().peekable();
        let end = if it.peek().is_some() { ";" } else { "" };
        write!(
            self.program_text,
            "{comment_text}entity {} is (\n{comment_text}    port (\n        clk : in std_logic{end}\n",
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
                "{comment_text}        {port_name} : {port_direction}{port_type}{end}\n"
            )
            .unwrap();
        }

        write!(
            self.program_text,
            "{comment_text}    );\n{comment_text}end entity {instance_name};\n"
        )
        .unwrap();
    }
}

fn gen_vhdl_code(md: &Module, instance: &InstantiatedModule) -> String {
    let mut program_text = String::new();

    let mut ctx = CodeGenerationContext {
        md,
        instance,
        program_text: &mut program_text,
        needed_untils: instance.compute_needed_untils(),
    };
    ctx.write_vhdl_code();

    program_text
}
