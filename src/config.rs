use clap::{value_parser, Arg, Command, ValueEnum};
use std::sync::OnceLock;
use std::{
    collections::HashSet,
    env,
    ffi::{OsStr, OsString},
    path::PathBuf,
};

/// Describes at what point in the compilation process we should exit early.
///
/// This is mainly to aid in debugging, where incorrect results from flattening/typechecking may lead to errors,
/// which we still wish to see in say the LSP
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum EarlyExitUpTo {
    Initialize,
    Flatten,
    AbstractTypecheck,
    Lint,
    Instantiate,
    CodeGen,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum TargetLanguage {
    SystemVerilog,
    Vhdl,
}

/// All command-line flags are converted to this struct, of which the singleton instance can be acquired using [crate::config::config]
#[derive(Debug, PartialEq, Eq)]
pub struct ConfigStruct {
    pub use_lsp: bool,
    pub lsp_debug_mode: bool,
    pub lsp_port: u16,
    pub codegen: bool,
    pub debug_print_module_contents: bool,
    pub debug_print_latency_graph: bool,
    pub debug_whitelist: Option<HashSet<String>>,
    pub codegen_module_and_dependencies_one_file: Option<String>,
    pub early_exit: EarlyExitUpTo,
    pub use_color: bool,
    pub ci: bool,
    pub target_language: TargetLanguage,
    pub files: Vec<PathBuf>,

    pub dot_print_instance: bool,
    pub dot_print_latency: bool,
    pub dot_output_path: PathBuf,
}

fn command_builder() -> Command {
    Command::new("SUS Compiler")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("The compiler for the SUS Hardware Design Language. This compiler takes in .sus files, and produces equivalent SystemVerilog files")
        .arg(Arg::new("socket")
            .long("socket")
            .default_value("25000")
            .help("Set the LSP TCP socket port")
            .value_parser(|socket_int : &str| {
                match socket_int.parse::<u16>() {
                    Ok(port) => Ok(port),
                    Err(_) => Err("Must be a valid port 0-65535")
                }
            })
            .requires("lsp"))
        .arg(Arg::new("lsp")
            .long("lsp")
            .help("Enable LSP mode")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("lsp-debug")
            .long("lsp-debug")
            .hide(true)
            .help("Enable LSP debug mode")
            .requires("lsp")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("codegen")
            .long("codegen")
            .help("Enable code generation for all modules. This creates a file named [ModuleName].sv per module.")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("debug")
            .long("debug")
            .hide(true)
            .help("Print debug information about the module contents")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("debug-latency")
            .long("debug-latency")
            .hide(true)
            .help("Print latency graph for debugging")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("dot-module")
            .long("dot-module")
            .hide(false)
            .help("Select a module to write to the dot-output-path")
            .action(clap::ArgAction::Set))
        .arg(Arg::new("print-dot")
            .long("print-dot")
            .hide(false)
            .help("Enable Dot printing of Module Instances")
            .requires("dot-module")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("print-dot-latency")
            .long("print-dot-latency")
            .hide(false)
            .help("Enable Dot printing of Latency Counting Instances")
            .requires("dot-module")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("dot-output-path")
            .long("dot-output-path")
            .hide(false)
            .help("Path of output dot file for print-dot or print-dot-latency")
            .default_value("output.dot")
            .requires("dot-module")
            .value_parser(value_parser!(PathBuf))
            .action(clap::ArgAction::Set))
        .arg(Arg::new("debug-whitelist")
            .long("debug-whitelist")
            .hide(true)
            .help("Sets the modules that should be shown by --debug. When not provided all modules are whitelisted")
            .action(clap::ArgAction::Append))
        .arg(Arg::new("standalone")
            .long("standalone")
            .help("Generate standalone code with all dependencies in one file of the module specified."))
        .arg(Arg::new("upto")
            .long("upto")
            .help("Describes at what point in the compilation process we should exit early. This is mainly to aid in debugging, where incorrect results from flattening/typechecking may lead to errors, which we still wish to see in say the LSP")
            .value_parser(clap::builder::EnumValueParser::<EarlyExitUpTo>::new())
            .default_value("code-gen"))
        .arg(Arg::new("nocolor")
            .long("nocolor")
            .help("Disables color printing in the errors of the sus_compiler output")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("ci")
            .long("ci")
            .help("Makes the compiler output as environment agnostic as possible")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("target")
            .long("target")
            .help("Sets the target HDL")
            .value_parser(clap::builder::EnumValueParser::<TargetLanguage>::new())
            .default_value("system-verilog"))
        .arg(Arg::new("files")
            .action(clap::ArgAction::Append)
            .help(".sus Files")
            .value_parser(|file_path_str : &str| {
                let file_path = PathBuf::from(file_path_str);
                if !file_path.exists() {
                    Err("File does not exist")
                } else if !file_path.is_file() {
                    Err("Is a directory")
                } else if file_path.extension() != Some(OsStr::new("sus")) {
                    Err("Source files must end in .sus")
                } else {
                    Ok(file_path)
                }
            }))
}

fn parse_args<I, T>(itr: I) -> Result<ConfigStruct, clap::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let matches = command_builder().try_get_matches_from(itr)?;
    let codegen = matches.get_flag("codegen") || matches.get_many::<PathBuf>("files").is_none();
    let debug_whitelist = matches
        .get_many("debug-whitelist")
        .map(|s| s.cloned().collect());
    let use_color = !matches.get_flag("nocolor") && !matches.get_flag("lsp");
    let files: Vec<PathBuf> = match matches.get_many("files") {
        Some(files) => files.cloned().collect(),
        None => std::fs::read_dir(".")
            .unwrap()
            .map(|file| file.unwrap().path())
            .filter(|file_path| {
                file_path.is_file() && file_path.extension() == Some("sus".as_ref())
            })
            .collect(),
    };

    Ok(ConfigStruct {
        use_lsp: matches.get_flag("lsp"),
        lsp_debug_mode: matches.get_flag("lsp-debug"),
        lsp_port: *matches.get_one("socket").unwrap(),
        codegen,
        debug_print_module_contents: matches.get_flag("debug"),
        debug_print_latency_graph: matches.get_flag("debug-latency"),
        debug_whitelist,
        codegen_module_and_dependencies_one_file: matches.get_one("standalone").cloned(),
        early_exit: *matches.get_one("upto").unwrap(),
        use_color,
        ci: matches.get_flag("ci"),
        target_language: *matches.get_one("target").unwrap(),
        files,
        dot_print_instance: matches.get_flag("print-dot"),
        dot_print_latency: matches.get_flag("print-dot-latency"),
        dot_output_path: matches
            .get_one::<PathBuf>("dot-output-path")
            .unwrap()
            .clone(),
    })
}

static CONFIG: OnceLock<ConfigStruct> = OnceLock::new();

pub fn initialize_config_from_cli_args() {
    match parse_args(std::env::args_os()) {
        Ok(parsed_args) => CONFIG.set(parsed_args).unwrap(),
        Err(err) => err.exit(),
    }
}

/// Access the singleton [ConfigStruct] representing the CLI arguments passed to `sus_compiler`
pub fn config() -> &'static ConfigStruct {
    CONFIG.get().unwrap()
}

#[cfg(test)]
mod tests {
    use super::parse_args;

    #[test]
    fn test_socket_invalid_port() {
        let config = parse_args(["", "--lsp", "--socket", "1234567890"]);
        assert!(config.is_err());
        let err = config.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::ValueValidation);
    }

    #[test]
    fn test_socket_require_lsp() {
        let config = parse_args(["", "--socket", "1500"]);
        assert!(config.is_err());
        let err = config.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn test_lsp_debug_require_lsp() {
        let config = parse_args(["", "--lsp-debug"]);
        assert!(config.is_err());
        let err = config.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn test_lsp_no_color() {
        let config = parse_args(["", "--lsp"]).unwrap();
        assert!(!config.use_color)
    }

    #[test]
    fn test_automatic_codegen() {
        let config = parse_args([""]).unwrap();
        assert!(config.codegen)
    }
}
