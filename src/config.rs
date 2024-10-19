use std::{cell::UnsafeCell, env, ffi::OsStr, path::PathBuf};

use clap::{Arg, Command};

/// Describes at what point in the compilation process we should exit early. 
/// 
/// This is mainly to aid in debugging, where incorrect results from flattening/typechecking may lead to errors, 
/// which we still wish to see in say the LSP
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EarlyExitUpTo {
    Initialize,
    Flatten,
    AbstractTypecheck,
    Instantiate,
    CodeGen
}

pub struct ConfigStruct {
    pub use_lsp: bool,
    pub lsp_debug_mode: bool,
    pub lsp_port: u16,
    pub codegen: bool,
    pub debug_print_module_contents: bool,
    pub debug_print_latency_graph: bool,
    pub codegen_module_and_dependencies_one_file: Option<String>,
    pub early_exit: EarlyExitUpTo
}

pub fn config() -> &'static ConfigStruct {
    unsafe { &*CONFIG.cf.get() }
}

pub fn parse_args() -> Vec<PathBuf> {
    let matches = Command::new("SUS Compiler")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("The compiler for the SUS Hardware Design Language. This compiler takes in .sus files, and produces equivalent SystemVerilog files")
        .arg(Arg::new("socket")
            .long("socket")
            .takes_value(true)
            .default_value("25000")
            .help("Set the LSP TCP socket port")
            .validator(|socket_int : &str| {
                match u16::from_str_radix(socket_int, 10) {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Must be a valid port 0-65535")
                }
            })
            .requires("lsp"))
        .arg(Arg::new("lsp")
            .long("lsp")
            .help("Enable LSP mode"))
        .arg(Arg::new("lsp-debug")
            .long("lsp-debug")
            .hide(true)
            .help("Enable LSP debug mode")
            .requires("lsp"))
        .arg(Arg::new("codegen")
            .long("codegen")
            .help("Enable code generation for all modules. This creates a file named [ModuleName].sv per module."))
        .arg(Arg::new("debug")
            .long("debug")
            .hide(true)
            .help("Print debug information about the module contents"))
        .arg(Arg::new("debug-latency")
            .long("debug-latency")
            .hide(true)
            .help("Print latency graph for debugging"))
        .arg(Arg::new("standalone")
            .long("standalone")
            .takes_value(true)
            .help("Generate standalone code with all dependencies in one file of the module specified. "))
        .arg(Arg::new("upto")
            .long("upto")
            .help("Describes at what point in the compilation process we should exit early. This is mainly to aid in debugging, where incorrect results from flattening/typechecking may lead to errors, which we still wish to see in say the LSP")
            .takes_value(true)
            .possible_values(&["initialize", "flatten", "typecheck", "instantiate", "codegen"])
            .default_value("codegen"))
        .arg(Arg::new("files")
            .multiple_values(true)
            .help(".sus Files")
            .validator(|file_path_str : &str| {
                let file_path = PathBuf::from(file_path_str);
                if !file_path.exists() {
                    Err("File does not exist")
                } else if !file_path.is_file() {
                    Err("Is a directory")
                } else if file_path.extension() != Some(OsStr::new("sus")) {
                    Err("Source files must end in .sus")
                } else {
                    Ok(())
                }
            }))
        .get_matches();

    let config = unsafe { &mut *CONFIG.cf.get() };

    if let Some(socket) = matches.value_of("socket") {
        config.lsp_port = u16::from_str_radix(socket, 10).unwrap();
    }

    config.use_lsp = matches.is_present("lsp");
    config.lsp_debug_mode = matches.is_present("lsp-debug");
    config.codegen = matches.is_present("codegen");
    config.debug_print_module_contents = matches.is_present("debug");
    config.debug_print_latency_graph = matches.is_present("debug-latency");
    config.early_exit = match matches.value_of("upto").unwrap() {
        "initialize" => EarlyExitUpTo::Initialize,
        "flatten" => EarlyExitUpTo::Flatten,
        "typecheck" => EarlyExitUpTo::AbstractTypecheck,
        "instantiate" => EarlyExitUpTo::Instantiate,
        "codegen" => EarlyExitUpTo::CodeGen,
        _ => unreachable!()
    };


    if let Some(standalone) = matches.value_of("standalone") {
        config.codegen_module_and_dependencies_one_file = Some(standalone.to_string());
    }

    let mut file_paths: Vec<PathBuf> = Vec::new();
    if let Some(files) = matches.values_of("files") {
        for file in files {
            file_paths.push(PathBuf::from(file));
        }
    }

    // For debugging, if no files are provided
    if file_paths.is_empty() {
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
    cf: UnsafeCell<ConfigStruct>,
}

unsafe impl Sync for ConfigStructWrapper {}

static CONFIG: ConfigStructWrapper = ConfigStructWrapper {
    cf: UnsafeCell::new(ConfigStruct {
        use_lsp: false,
        lsp_port: 25000,
        lsp_debug_mode: false,
        debug_print_module_contents: false,
        codegen: false,
        debug_print_latency_graph: false,
        codegen_module_and_dependencies_one_file: None,
        early_exit: EarlyExitUpTo::CodeGen
    }),
};
