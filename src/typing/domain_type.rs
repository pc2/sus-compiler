use crate::prelude::*;

use crate::typing::unifyable_cell::UniCell;

/// These represent clock domains.
///
/// Clock Domains are resolved pre-instantiation, because dynamic domain merging doesn't seem like a valuable use case.
///
/// As a convenience, we make [ClockDomain::Generative] a special case for compile-time values.
///
/// The fun thing is that we can now use this domain info for syntax highlighting, giving wires in different domains a different color.
#[derive(Debug, Clone)]
pub enum ClockDomain {
    /// Generative conflicts with nothing
    Generative,
    /// This object is a real wire. It corresponds to a certain clock domain. It can only connect to wires in the same domain.
    Physical(UniCell<ClockID>),
}

impl ClockDomain {
    #[allow(clippy::declare_interior_mutable_const)]
    pub const UNKNOWN: UniCell<ClockDomain> = UniCell::UNKNOWN;

    #[track_caller]
    pub fn unwrap_physical(&self) -> ClockID {
        let_unwrap!(Self::Physical(w), self);
        *w.unwrap()
    }

    pub fn is_generative(&self) -> bool {
        matches!(self, ClockDomain::Generative)
    }
}

impl ClockID {
    #[allow(clippy::declare_interior_mutable_const)]
    pub const UNKNOWN: UniCell<ClockID> = UniCell::UNKNOWN;
}
