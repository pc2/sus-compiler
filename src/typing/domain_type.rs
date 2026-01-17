use crate::prelude::*;

use crate::typing::unifyable_cell::UniCell;

/// These represent (clock) domains. While clock domains are included under this umbrella, domains can use the same clock.
/// The use case for non-clock-domains is to separate Latency Counting domains. So different pipelines where it doesn't
/// necessarily make sense that their values are related by a fixed number of clock cycles.
///
/// Domains are resolved pre-instantiation, because dynamic domain merging doesn't seem like a valuable use case.
///
/// As a convenience, we make [DomainType::Generative] a special case for a domain.
///
/// The fun thing is that we can now use this domain info for syntax highlighting, giving wires in different domains a different color.
#[derive(Debug, Clone)]
pub enum DomainType {
    /// Generative conflicts with nothing
    Generative,
    /// This object is a real wire. It corresponds to a certain (clock) domain. It can only affect wires in the same domain.
    Physical(UniCell<DomainID>),
}

impl DomainType {
    #[allow(clippy::declare_interior_mutable_const)]
    pub const UNKNOWN: UniCell<DomainType> = UniCell::UNKNOWN;

    #[track_caller]
    pub fn unwrap_physical(&self) -> DomainID {
        let_unwrap!(Self::Physical(w), self);
        *w.unwrap()
    }

    pub fn is_generative(&self) -> bool {
        matches!(self, DomainType::Generative)
    }
}

impl DomainID {
    #[allow(clippy::declare_interior_mutable_const)]
    pub const UNKNOWN: UniCell<DomainID> = UniCell::UNKNOWN;
}
