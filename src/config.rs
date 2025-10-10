use crate::prelude::*;

use clap::{Arg, Command, ValueEnum};
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum TargetLanguage {
    SystemVerilog,
    Vhdl,
}

#[derive(Debug)]
pub struct StandaloneCodegenSettings {
    pub top_module: String,
    pub file_path: Option<PathBuf>,
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
    pub codegen: bool,
    pub standalone: Option<StandaloneCodegenSettings>,
    pub use_color: bool,
    pub ci: bool,
    pub target_language: TargetLanguage,
    pub files: Vec<PathBuf>,
    pub sus_home: PathBuf,

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

    /// Temporary, just because flopoco forces weirdly-sized floats, have it provide a struct in the future instead
    pub float_size: usize,
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
            .action(clap::ArgAction::SetTrue)
            .requires("lsp")
            .requires("socket"))
        .arg(Arg::new("codegen")
            .long("codegen")
            .help("Enable code generation for all modules. This creates a file named [ModuleName].sv per module.")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("standalone")
            .long("standalone")
            .help("Generate standalone code with all dependencies in one file of the module specified."))
        .arg(Arg::new("standalone-file")
            .long("standalone-file")
            .requires("standalone")
            .help("Set the output file of --standalone code generation")
            .value_parser(|file_path_str : &str| {
                let file_path = PathBuf::from(file_path_str);
                Result::<PathBuf, &'static str>::Ok(file_path)
            }))
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
        .arg(Arg::new("sus-home")
            .long("sus-home")
            .hide(true)
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
        .arg(Arg::new("float-size")
            .long("float-size")
            .hide(true)
            .help("Set the size of floats - in bits. Makes no claims about the representation, that's up to the libraries")
            .value_parser(clap::value_parser!(usize))
            .default_value("32")
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

    let codegen = matches.get_flag("codegen")
        || matches.get_many::<PathBuf>("files").is_none()
        || matches.contains_id("standalone");
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

    let standalone =
        matches
            .get_one("standalone")
            .map(|top_module: &String| StandaloneCodegenSettings {
                top_module: top_module.to_string(),
                file_path: matches.get_one::<PathBuf>("standalone-file").cloned(),
            });

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

    let cfg = ConfigStruct {
        lsp_settings,
        codegen,
        debug_whitelist,
        enabled_debug_paths,
        kill_timeout: *matches.get_one::<Duration>("kill-timeout").unwrap(),
        standalone,
        early_exit: *matches.get_one("upto").unwrap(),
        use_color,
        ci,
        target_language: *matches.get_one("target").unwrap(),
        files,
        sus_home,
        no_redump: matches.get_flag("no-redump"),
        float_size: *matches.get_one("float-size").unwrap(),
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
