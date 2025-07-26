//! Shared utilities

use std::borrow::Cow;

use crate::{instantiation::RealWire, latency::AbsLat};

pub fn wire_name_with_latency(
    wire: &RealWire,
    target_abs_lat: AbsLat,
    use_latency: bool,
) -> Cow<'_, str> {
    let wire_abs_lat = wire.absolute_latency.unwrap();
    let target_abs_lat = target_abs_lat.unwrap();
    assert!(wire_abs_lat <= target_abs_lat);
    if use_latency && (wire_abs_lat != target_abs_lat) {
        if target_abs_lat < 0 {
            Cow::Owned(format!("_{}_N{}", wire.name, -target_abs_lat))
        } else {
            Cow::Owned(format!("_{}_D{}", wire.name, target_abs_lat))
        }
    } else {
        Cow::Borrowed(&wire.name)
    }
}

pub fn wire_name_self_latency(wire: &RealWire, use_latency: bool) -> Cow<'_, str> {
    wire_name_with_latency(wire, wire.absolute_latency, use_latency)
}
