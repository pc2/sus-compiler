//! Some objects are used all over the compiler, like ErrorCollector, FlatAlloc and Linker. These are provided here to reduce imports
//!
//! This file also defines all IDs (Implementations of [UUIDMarker] & [UUID]) that are used in the rest of the program for use with [FlatAlloc].

use std::marker::PhantomData;

// public imports
pub use crate::alloc::FlatAlloc;
pub use crate::errors::ErrorCollector;
pub use crate::file_position::{BracketSpan, Span};
pub use crate::linker::Linker;

#[allow(unused_imports)]
pub use crate::let_unwrap;
#[allow(unused_imports)]
pub use crate::unwrap_variant;

#[allow(unused_imports)]
pub use crate::__debug_dbg;
#[allow(unused_imports)]
pub use crate::__debug_span;
#[allow(unused_imports)]
pub use log::{debug, error, info, trace, warn};
#[allow(unused_imports)]
pub use sus_proc_macro::__debug_breakpoint;
#[allow(unused_imports)]
pub use sus_proc_macro::__debug_breakpoint_if;

#[macro_export]
macro_rules! fatal_exit {
    ($($arg:tt)*) => {{
        use colored::*;

        let msg = format!("FATAL: {}", format!($($arg)*)).red();
        eprintln!("{msg}");
        std::process::exit(1);
    }};
}
pub use crate::fatal_exit;

// private imports, for the IDs

use crate::alloc::{UUID, UUIDMarker, UUIDRange};

// Global IDs

pub struct ModuleUUIDMarker;
impl UUIDMarker for ModuleUUIDMarker {
    const DISPLAY_NAME: &'static str = "module_";
}
pub type ModuleUUID = UUID<ModuleUUIDMarker>;

pub struct TypeUUIDMarker;
impl UUIDMarker for TypeUUIDMarker {
    const DISPLAY_NAME: &'static str = "type_";
}
pub type TypeUUID = UUID<TypeUUIDMarker>;

pub struct ConstantUUIDMarker;
impl UUIDMarker for ConstantUUIDMarker {
    const DISPLAY_NAME: &'static str = "constant_";
}
pub type ConstantUUID = UUID<ConstantUUIDMarker>;

pub struct FileUUIDMarker;
impl UUIDMarker for FileUUIDMarker {
    const DISPLAY_NAME: &'static str = "file_";
}
pub type FileUUID = UUID<FileUUIDMarker>;

// Flattened-local IDs

pub struct FlatIDMarker;
impl UUIDMarker for FlatIDMarker {
    const DISPLAY_NAME: &'static str = "obj_";
}
pub type FlatID = UUID<FlatIDMarker>;

pub type FlatIDRange = UUIDRange<FlatIDMarker>;

pub struct PortIDMarker;
impl UUIDMarker for PortIDMarker {
    const DISPLAY_NAME: &'static str = "port_";
}
pub type PortID = UUID<PortIDMarker>;

pub struct InterfaceIDMarker;
impl UUIDMarker for InterfaceIDMarker {
    const DISPLAY_NAME: &'static str = "interface_";
}
pub type InterfaceID = UUID<InterfaceIDMarker>;

impl InterfaceID {
    pub const MAIN_INTERFACE: InterfaceID = UUID(0, PhantomData);
}

pub struct ClockIDMarker;
impl UUIDMarker for ClockIDMarker {
    const DISPLAY_NAME: &'static str = "clock_";
}
pub type ClockID = UUID<ClockIDMarker>;

pub struct LatDomIDMarker;
impl UUIDMarker for LatDomIDMarker {
    const DISPLAY_NAME: &'static str = "latdom_";
}
pub type LatDomID = UUID<LatDomIDMarker>;

pub struct FieldIDMarker;
impl UUIDMarker for FieldIDMarker {
    const DISPLAY_NAME: &'static str = "field_";
}
pub type FieldID = UUID<FieldIDMarker>;

pub struct TemplateIDMarker;
impl UUIDMarker for TemplateIDMarker {
    const DISPLAY_NAME: &'static str = "template_arg_";
}
pub type TemplateID = UUID<TemplateIDMarker>;

// Instantiation-local IDs

pub struct WireIDMarker;
impl UUIDMarker for WireIDMarker {
    const DISPLAY_NAME: &'static str = "wire_";
}
pub type WireID = UUID<WireIDMarker>;

pub struct SubModuleIDMarker;
impl UUIDMarker for SubModuleIDMarker {
    const DISPLAY_NAME: &'static str = "submodule_";
}
pub type SubModuleID = UUID<SubModuleIDMarker>;
