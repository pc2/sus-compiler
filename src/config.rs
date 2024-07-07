use std::{cell::UnsafeCell, env, ffi::OsStr, path::PathBuf};


pub struct ConfigStruct {
    pub use_lsp : bool,
    pub lsp_debug_mode : bool,
    pub lsp_port : u16,
    pub codegen : bool,
    pub debug_print_module_contents : bool,
    pub debug_print_latency_graph : bool,
    pub codegen_module_and_dependencies_one_file : Option<String>
}

pub fn config() -> &'static ConfigStruct {
    unsafe {
        &*CONFIG.cf.get()
    }
}

pub fn parse_args() -> Vec<PathBuf> {
    let mut args = env::args();

    let _executable_path = args.next();

    let config = unsafe{&mut *CONFIG.cf.get()};

    let mut file_paths : Vec<PathBuf> = Vec::new();
    
    while let Some(arg) = args.next() {
        if arg.starts_with("-") {
            if let Some((name, value)) = arg.split_once("=") {
                match name {
                    "--socket" => {
                        config.lsp_port = u16::from_str_radix(value, 10).unwrap();
                    }
                    other => panic!("Unknown option {other}"),
                }
            } else {
                match arg.as_str() {
                    "--lsp" => {
                        config.use_lsp = true;
                    }
                    "--lsp-debug" => {
                        config.lsp_debug_mode = true;
                    }
                    "--codegen" => {
                        config.codegen = true;
                    }
                    "--debug" => {
                        config.debug_print_module_contents = true;
                    }
                    "--debug-latency" => {
                        config.debug_print_latency_graph = true;
                    }
                    "--module-and-dependencies" => {
                        config.codegen_module_and_dependencies_one_file = Some(args.next().unwrap());
                    }
                    other => {
                        panic!("Unknown option {other}");
                    }
                }
            }
        } else {
            file_paths.push(PathBuf::from(arg));
        }
    }

    // For debugging
    if file_paths.len() == 0 {
        for file in std::fs::read_dir(".").unwrap() {
            let file_path = file.unwrap().path();
            if file_path.is_file() && file_path.extension() == Some(OsStr::new("sus")) {
                file_paths.push(file_path);
            }
        }
        config.codegen = true;
    }

    file_paths
}

struct ConfigStructWrapper {
    cf : UnsafeCell<ConfigStruct>
}

unsafe impl Sync for ConfigStructWrapper {}

static CONFIG : ConfigStructWrapper = ConfigStructWrapper{cf: UnsafeCell::new(ConfigStruct{
    use_lsp : false,
    lsp_port : 25000,
    lsp_debug_mode : false,
    debug_print_module_contents : false,
    codegen : false,
    debug_print_latency_graph : false,
    codegen_module_and_dependencies_one_file: None
})};
