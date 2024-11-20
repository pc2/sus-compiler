use super::mangle;
use crate::{linker::IsExtern, FlatAlloc, InstantiatedModule, Module, WireIDMarker};

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
        write!(
            self.program_text,
            "{comment_text}entity {} is (\n{comment_text}    port (\n",
            instance_name
        )
        .unwrap();

        for (_, port) in self.instance.interface_ports.iter_valids() {
            let port_wire = &self.instance.wires[port.wire];
            let name = &port_wire.name;
            let direction = if port.is_input { "in" } else { "out" };
            dbg!(format!("{comment_text}       {name} : {direction}"));
        }

        write!(
            self.program_text,
            "{comment_text}    ;\n{comment_text}end entity {}\n",
            instance_name
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
