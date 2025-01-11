//! Shared utilities

use std::borrow::Cow;

use crate::{instantiation::RealWire, linker::get_builtin_type, TypeUUID};


pub fn mangle(str: &str) -> String {
    let mut result = String::with_capacity(str.len());
    for c in str.chars() {
        if c.is_whitespace() || c == ':' {
            continue;
        }
        result.push(if c.is_alphanumeric() { c } else { '_' });
    }
    result
}

pub fn get_type_name_size(id: TypeUUID) -> u64 {
    if id == get_builtin_type("int") {
        32 // TODO concrete int sizes
    } else if id == get_builtin_type("bool") {
        1
    } else {
        println!("TODO Named Structs Size");
        1 // todo!() // Named structs are not implemented yet
    }
}

pub fn wire_name_with_latency(wire: &RealWire, absolute_latency: i64, use_latency: bool) -> Cow<str> {
    assert!(wire.absolute_latency <= absolute_latency);
    if use_latency && (wire.absolute_latency != absolute_latency) {
        if absolute_latency < 0 {
            Cow::Owned(format!("_{}_N{}", wire.name, -absolute_latency))
        } else {
            Cow::Owned(format!("_{}_D{}", wire.name, absolute_latency))
        }
    } else {
        Cow::Borrowed(&wire.name)
    }
}

pub fn wire_name_self_latency(wire: &RealWire, use_latency: bool) -> Cow<str> {
    wire_name_with_latency(wire, wire.absolute_latency, use_latency)
}
