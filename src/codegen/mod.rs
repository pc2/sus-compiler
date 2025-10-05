mod shared;
pub mod system_verilog;
pub mod vhdl;

pub use system_verilog::VerilogCodegenBackend;
pub use vhdl::VHDLCodegenBackend;

use crate::prelude::*;

use crate::{InstantiatedModule, Linker, Module};

use crate::config::VERSION_INFO;

use std::path::Path;
use std::{
    fs::{self, File},
    io::Write,
    ops::Deref,
    path::PathBuf,
    rc::Rc,
};

/// Implemented for SystemVerilog [self::system_verilog] or VHDL [self::vhdl]
pub trait CodeGenBackend {
    fn file_extension(&self) -> &str;
    fn output_dir_name(&self) -> &str;
    fn codegen(
        &self,
        md: &Module,
        instance: &InstantiatedModule,
        linker: &Linker,
        use_latency: bool,
    ) -> String;

    fn make_output_file_path(&self, name: &str) -> PathBuf {
        let mut path = PathBuf::with_capacity(
            name.len() + self.output_dir_name().len() + self.file_extension().len() + 2,
        );
        path.push(self.output_dir_name());
        fs::create_dir_all(&path).unwrap();
        path.push(name);
        path.set_extension(self.file_extension());
        path
    }
    fn make_output_file(&self, path: &Path) -> File {
        let mut file = File::create(path).unwrap();

        let generation_time =
            chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false);
        write!(file,
            "// THIS IS A GENERATED FILE (Generated at {generation_time})\n// This file was generated with SUS Compiler {VERSION_INFO}\n"
        )
        .unwrap();

        file
    }

    fn codegen_instance(
        &self,
        inst: &InstantiatedModule,
        md: &Module,
        linker: &Linker,
        out_file: &mut File,
    ) {
        if inst.errors.did_error {
            return; // Continue
        }
        let code = self.codegen(md, inst, linker, true); // hardcode use_latency = true for now. Maybe forever, we'll see
        write!(out_file, "{code}").unwrap();
    }

    fn codegen_to_file(&self, id: ModuleUUID, md: &Module, linker: &Linker) {
        let instantiatior_borrow = linker.instantiator.borrow();
        if instantiatior_borrow
            .iter_for_module(id)
            .any(|(_, inst)| !inst.errors.did_error)
        {
            let path = self.make_output_file_path(&md.link_info.name);
            let mut out_file = self.make_output_file(&path);
            for (_global_ref, inst) in instantiatior_borrow.iter_for_module(id) {
                self.codegen_instance(inst.as_ref(), md, linker, &mut out_file)
            }
        }
    }

    fn codegen_with_dependencies(&self, linker: &Linker, md_id: ModuleUUID, path: &Path) {
        info!("Codegen to {}", path.to_string_lossy());
        let mut out_file = self.make_output_file(path);
        let mut to_process_queue: Vec<Rc<InstantiatedModule>> = Vec::new();
        for (_template_args, inst) in linker.instantiator.borrow().iter_for_module(md_id) {
            if inst.errors.did_error {
                error!("Cannot codegen {}, due to errors!", inst.name);
            } else {
                to_process_queue.push(inst.clone());
            }
        }

        let mut to_process_queue: Vec<&InstantiatedModule> =
            to_process_queue.iter().map(|inst| inst.deref()).collect();

        let mut cur_idx = 0;

        while cur_idx < to_process_queue.len() {
            let cur_instance = to_process_queue[cur_idx];

            for (_, sub_mod) in &cur_instance.submodules {
                let new_inst = sub_mod.instance.get().unwrap().as_ref();

                // Skip duplicates
                // Yeah yeah I know O(n²) but this list shouldn't grow too big. Fix if needed
                if to_process_queue
                    .iter()
                    .any(|existing| std::ptr::eq(*existing, new_inst))
                {
                    continue;
                }

                to_process_queue.push(new_inst);
            }

            info!("Codegen instance {}", cur_instance.name);
            self.codegen_instance(
                cur_instance,
                &linker.modules[cur_instance.global_ref.id],
                linker,
                &mut out_file,
            );

            cur_idx += 1;
        }
    }
}
