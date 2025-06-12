use crate::{alloc::UUID, prelude::*, typing::type_inference::DomainVariableID};

/// These represent (clock) domains. While clock domains are included under this umbrella, domains can use the same clock.
/// The use case for non-clock-domains is to separate Latency Counting domains. So different pipelines where it doesn't
/// necessarily make sense that their values are related by a fixed number of clock cycles.
///
/// Domains are resolved pre-instantiation, because dynamic domain merging doesn't seem like a valuable use case.
///
/// As a convenience, we make [DomainType::Generative] a special case for a domain.
///
/// The fun thing is that we can now use this domain info for syntax highlighting, giving wires in different domains a different color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainType {
    /// Generative conflicts with nothing
    Generative,
    /// This object is a real wire. It corresponds to a certain (clock) domain. It can only affect wires in the same domain.
    Physical(DomainID),

    /// These are unified by Hindley-Milner unification
    ///
    /// They always point to non-generative domains.
    ///
    /// Referencing [DomainType::Unknown] is a strong code smell.
    /// It is likely you should use [TypeSubstitutor::unify_must_succeed] or [TypeSubstitutor::unify_report_error] instead
    ///
    /// It should only occur in creation `DomainType::Unknown(self.domain_substitutor.alloc())`
    Unknown(DomainVariableID),
}

impl DomainType {
    pub const PLACEHOLDER: DomainType = DomainType::Unknown(UUID::PLACEHOLDER);

    pub fn unwrap_physical(&self) -> DomainID {
        let Self::Physical(w) = self else {
            unreachable!()
        };
        *w
    }
}
