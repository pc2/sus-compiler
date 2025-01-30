//! Shared utilities

use std::borrow::Cow;

use crate::instantiation::RealWire;

pub fn wire_name_with_latency(
    wire: &RealWire,
    absolute_latency: i64,
    use_latency: bool,
) -> Cow<str> {
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
