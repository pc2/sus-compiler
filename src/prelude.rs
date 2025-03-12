//! Some objects are used all over the compiler, like ErrorCollector, FlatAlloc and Linker. These are provided here to reduce imports
//!
//! This file also defines all IDs (Implementations of [UUIDMarker] & [UUID]) that are used in the rest of the program for use with [FlatAlloc].

// public imports
pub use crate::alloc::FlatAlloc;
pub use crate::errors::ErrorCollector;
pub use crate::file_position::{BracketSpan, Span, SpanFile};
pub use crate::linker::Linker;

// private imports, for the IDs

use crate::alloc::{UUIDMarker, UUIDRange, UUID};

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

pub struct PeanoUUIDMarker;
impl UUIDMarker for PeanoUUIDMarker {
    const DISPLAY_NAME: &'static str = "rank_";
}
pub type PeanoUUID = UUID<PeanoUUIDMarker>;

pub struct RankedTypeUUIDMarker;
impl UUIDMarker for RankedTypeUUIDMarker {
    const DISPLAY_NAME: &'static str = "ranked_type_";
}
pub type RankedTypeUUID = UUID<RankedTypeUUIDMarker>;

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

pub type PortIDRange = UUIDRange<PortIDMarker>;

pub struct InterfaceIDMarker;
impl UUIDMarker for InterfaceIDMarker {
    const DISPLAY_NAME: &'static str = "interface_";
}
pub type InterfaceID = UUID<InterfaceIDMarker>;

pub struct DomainIDMarker;
impl UUIDMarker for DomainIDMarker {
    const DISPLAY_NAME: &'static str = "domain_";
}
/// Interfaces are also indexed using DomainIDs. But in general, these refer to (clock/latency counting) domains
pub type DomainID = UUID<DomainIDMarker>;

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
