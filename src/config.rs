use crate::prelude::*;

use clap::{Arg, ArgGroup, Command, ValueEnum};
use log::info;
use std::collections::HashSet;
use std::sync::OnceLock;
use std::time::Duration;
use std::{env, ffi::OsStr, path::PathBuf};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetLanguage {
    SystemVerilog,
    Vhdl,
}

impl ValueEnum for TargetLanguage {
    fn value_variants<'a>() -> &'a [Self] {
        &[TargetLanguage::SystemVerilog, TargetLanguage::Vhdl]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            TargetLanguage::SystemVerilog => "sv".into(),
            TargetLanguage::Vhdl => "vhdl".into(),
        })
    }
}

#[derive(Debug)]
pub struct LSPSettings {
    pub connection_method: ConnectionMethod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionMethod {
    Stdio,
    Tcp { port: u16, should_listen: bool },
}

/// All command-line flags are converted to this struct, of which the singleton instance can be acquired using [crate::config::config]
#[derive(Debug)]
pub struct ConfigStruct {
    pub lsp_settings: Option<LSPSettings>,

    pub sus_home: PathBuf,
    pub codegen_file: Option<PathBuf>,
    pub codegen_separate_folder: Option<PathBuf>,
    /// When no top modules specified, then codegen all
    pub top_modules: Vec<String>,
    pub use_color: bool,
    pub ci: bool,
    pub target_language: TargetLanguage,
    pub files: Vec<PathBuf>,

    /// Enable debugging printouts and figures
    ///
    /// If an element in this list is a substring of a [crate::debug::SpanDebugger] message, then debugging is enabled.
    ///
    /// If the list is empty, debug everything
    ///
    /// See also [Self::enabled_debug_paths]
    pub debug_whitelist: Vec<String>,
    pub kill_timeout: std::time::Duration,
    pub enabled_debug_paths: HashSet<String>,
    pub early_exit: EarlyExitUpTo,
    pub no_redump: bool,
}

pub const VERSION_INFO: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("GIT_HASH"),
    ") built at ",
    env!("BUILD_DATE"),
    " ",
    env!("BUILD_FEATURES")
);

fn command_builder() -> Command {
    Command::new("SUS Compiler")
        .version(VERSION_INFO)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("The compiler for the SUS Hardware Design Language. This compiler takes in .sus files, and produces equivalent SystemVerilog files")
        .arg(Arg::new("lsp")
            .long("lsp")
            .help("Enable LSP mode")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("socket")
            .long("socket") // DO NOT RENAME: VSCode's LSP-server extension adds a hardcoded "--socket {port_id}" flag
            .help("Enables LSP over TCP, and sets the LSP TCP socket port")
            .value_parser(|socket_int : &str| {
                match socket_int.parse::<u16>() {
                    Ok(port) => Ok(port),
                    Err(_) => Err("Must be a valid port 0-65535")
                }
            })
            .requires("lsp"))
        .arg(Arg::new("stdio")
            .long("stdio") // DO NOT RENAME: VSCode's LSP-server extension adds a hardcoded "--stdio" flag for stdio LSP comms
            .help("Enables LSP over STDIO")
            .requires("lsp")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("lsp-listen")
            .long("lsp-listen")
            .help("Instead of the LSP Server connecting to an open socket provided by the parent process, this makes the LSP server open a socket and listen for incoming connections")
            .requires("lsp")
            .requires("socket")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("o")
            .short('o')
            .help("Activate code generation and writes the result to the provided output file")
            .conflicts_with("codegen-separate")
            .value_parser(|file_path_str : &str| {
                let file_path = PathBuf::from(file_path_str);
                Result::<PathBuf, &'static str>::Ok(file_path)
            }))
        .arg(Arg::new("codegen-separate")
            .long("codegen-separate")
            .help("Activate code generation and creates a systemverilog file per module in the chosen folder")
            .conflicts_with("o")
            .value_parser(|file_path_str : &str| {
                let file_path = PathBuf::from(file_path_str);
                Result::<PathBuf, &'static str>::Ok(file_path)
            }))
        .group(ArgGroup::new("codegen-enabled").args(["o", "codegen-separate"]))
        .arg(Arg::new("codegen-language")
            .long("codegen-language")
            .hide(true) // Hidden because we don't support VHDL
            .help("Sets the target HDL")
            .requires("codegen-enabled")
            .value_parser(clap::builder::EnumValueParser::<TargetLanguage>::new()))
        .arg(Arg::new("top")
            .long("top")
            .help("List of top module names to limit compilation/codegen to")
            .action(clap::ArgAction::Append))
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
            .hide(true)
            .help("Makes the compiler output as environment agnostic as possible")
            .action(clap::ArgAction::SetTrue))
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
        .arg(Arg::new("sus-home")
            .long("sus-home")
            .help("Override the SUS_HOME directory (for std/core.sus, crash_dumps, etc)")
            .value_parser(|dir: &str| {
                let path = PathBuf::from(dir);
                if !path.exists() {
                    Err("Directory does not exist")
                } else if !path.is_dir() {
                    Err("Path is not a directory")
                } else {
                    Ok(path)
                }
            })
        )
        // Debug stuff
        .arg(Arg::new("debug")
            .long("debug")
            .hide(true)
            .help("Enable specific debug paths for specific modules. Path names are found by searching for crate::debug::is_enabled in the source code. ")
            .action(clap::ArgAction::Append))
        .arg(Arg::new("debug-whitelist")
            .long("debug-whitelist")
            .hide(true)
            .help("Enable debug prints and figures for specific modules.\nDebugging checks if the current debug stage print has one of the debug-whitelist arguments as a substring. So passing 'FIFO' debugs all FIFO stuff, but passing 'Typechecking FIFO' only shows debug prints during typechecking. To show everything, pass --debug-whitelist-is-blacklist")
            .action(clap::ArgAction::Append))
        .arg(Arg::new("kill-timeout")
            .long("kill-timeout")
            .hide(true)
            .help("Sets how long (in seconds) an individual part of the compiler can take, before terminating. Set to 0 to disable")
            .action(clap::ArgAction::Set)
            .default_value("0.0")
            .value_parser(|duration : &str| -> Result<Duration, String> {
                Ok(Duration::from_secs_f64(duration.parse::<f64>().map_err(|e| e.to_string())?))
            }))
        .arg(Arg::new("no-redump")
            .long("no-redump")
            .hide(true)
            .help("Disable creation of new crash dump on panic")
            .action(clap::ArgAction::SetTrue))
}

pub fn parse_args() {
    assert!(CONFIG.get().is_none(), "parse_args() used twice!");

    let matches = match command_builder().try_get_matches_from(std::env::args_os()) {
        Ok(matches) => matches,
        Err(e) => e.exit(),
    };

    let debug_whitelist = matches
        .get_many("debug-whitelist")
        .unwrap_or_default()
        .cloned()
        .collect();

    let enabled_debug_paths = matches
        .get_many("debug")
        .unwrap_or_default()
        .cloned()
        .collect();
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

    let codegen_file: Option<PathBuf> = matches.get_one("o").cloned();
    let codegen_separate_folder: Option<PathBuf> = matches.get_one("codegen-separate").cloned();

    let top_modules = matches
        .get_many("top")
        .map(|t| t.cloned().collect())
        .unwrap_or(Vec::new());

    let sus_home_override = matches.get_one::<PathBuf>("sus-home").cloned();

    let lsp_settings = if matches.get_flag("lsp") {
        Some(LSPSettings {
            connection_method: if let Some(port) = matches.get_one("socket") {
                ConnectionMethod::Tcp {
                    port: *port,
                    should_listen: matches.get_flag("lsp-listen"),
                }
            } else if matches.get_flag("stdio") {
                ConnectionMethod::Stdio
            } else {
                fatal_exit!(
                    "When passing --lsp, must either pass --stdio for STDIO communication, or --socket {{port}} for TCP connection",
                );
            },
        })
    } else {
        None
    };

    // Compute sus_home based on priorities: --sus-home explicit override, then $SUS_HOME env var, then $INSTALL_SUS_HOME that was set while compiling
    let sus_home = if let Some(override_path) = sus_home_override {
        override_path
    } else if let Some(override_path) = std::env::var_os("SUS_HOME") {
        PathBuf::from(override_path) // Runtime env variable
    } else {
        PathBuf::from(env!("INSTALL_SUS_HOME")) // Compiletime env while building, this is baked into the compiler
    };

    let sus_home = match sus_home.canonicalize() {
        Ok(sus_home) => sus_home,
        Err(e) => {
            fatal_exit!(
                "Could not access SUS_HOME directory ({}): {e}",
                sus_home.to_string_lossy()
            );
        }
    };

    let ci = matches.get_flag("ci");

    if !ci {
        // Otherwise this might vary on build server, and spuriously change the output
        info!("SUS_HOME is {}", sus_home.to_string_lossy());
    }

    let target_language = matches
        .get_one("codegen-language")
        .copied()
        .unwrap_or_else(|| {
            if let Some(codegen_file) = &codegen_file
                && let Some(ext) = codegen_file.extension()
            {
                if ext == "sv" {
                    TargetLanguage::SystemVerilog
                } else if ext == "vhd" {
                    TargetLanguage::Vhdl
                } else {
                    TargetLanguage::SystemVerilog
                }
            } else {
                TargetLanguage::SystemVerilog
            }
        });

    if target_language == TargetLanguage::Vhdl {
        fatal_exit!(
            "VHDL as a target code generation language is not yet supported. Use SystemVerilog instead"
        );
    }

    let cfg = ConfigStruct {
        lsp_settings,
        sus_home,
        files,
        codegen_file,
        codegen_separate_folder,
        top_modules,
        target_language,
        use_color,
        ci,
        debug_whitelist,
        enabled_debug_paths,
        kill_timeout: *matches.get_one::<Duration>("kill-timeout").unwrap(),
        early_exit: *matches.get_one("upto").unwrap(),
        no_redump: matches.get_flag("no-redump"),
    };
    CONFIG.set(cfg).unwrap();
}

static CONFIG: OnceLock<ConfigStruct> = OnceLock::new();

/// Access the singleton [ConfigStruct] representing the CLI arguments passed to `sus_compiler`
pub fn config() -> &'static ConfigStruct {
    CONFIG.get().unwrap()
}

/// Access the singleton [ConfigStruct] representing the CLI arguments passed to `sus_compiler`
pub fn lsp_config() -> &'static LSPSettings {
    config().lsp_settings.as_ref().unwrap()
}
