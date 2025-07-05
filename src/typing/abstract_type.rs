use sus_proc_macro::get_builtin_type;

use crate::{prelude::*, typing::template::TVec};

use super::type_inference::{InnerTypeVariableID, PeanoVariableID};

/// This contains only the information that can be type-checked before template instantiation.
///
/// Its most important components are the names and structure of types.
///
/// What isn't included are the parameters of types. So Array Sizes for example.
///
/// This is such that useful information can still be given for modules that haven't been instantiated.
///
/// Not to be confused with [WrittenType], which is the in-source text representation.
///
/// Not to be confused with [crate::typing::concrete_type::ConcreteType], which is the
/// post-instantiation type.
///
/// [AbstractType]s don't actually get converted to [crate::typing::concrete_type::ConcreteType]s.
/// Instead [crate::typing::concrete_type::ConcreteType] gets created from [WrittenType] directly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbstractInnerType {
    Template(TemplateID),
    Named(TypeUUID),
    Interface(AbstractGlobalReference<ModuleUUID>, InterfaceID),
    LocalInterface(FlatID),
    /// Referencing [AbstractType::Unknown] is a strong code smell.
    /// It is likely you should use [TypeSubstitutor::unify_must_succeed] or [TypeSubstitutor::unify_report_error] instead
    ///
    /// It should only occur in creation `AbstractType::Unknown(self.type_substitutor.alloc())`
    Unknown(InnerTypeVariableID),
}

impl AbstractInnerType {
    pub const BOOL: AbstractInnerType = AbstractInnerType::Named(get_builtin_type!("bool"));
    pub const INT: AbstractInnerType = AbstractInnerType::Named(get_builtin_type!("int"));

    pub fn scalar(self) -> AbstractRankedType {
        AbstractRankedType {
            inner: self,
            rank: PeanoType::Zero,
        }
    }
    pub fn with_rank(self, rank: PeanoType) -> AbstractRankedType {
        AbstractRankedType { inner: self, rank }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractRankedType {
    pub inner: AbstractInnerType,
    pub rank: PeanoType,
}

impl AbstractRankedType {
    pub const BOOL: AbstractRankedType = AbstractRankedType {
        inner: AbstractInnerType::BOOL,
        rank: PeanoType::Zero,
    };
    pub const INT: AbstractRankedType = AbstractRankedType {
        inner: AbstractInnerType::INT,
        rank: PeanoType::Zero,
    };

    pub const fn scalar(inner: AbstractInnerType) -> Self {
        Self {
            inner,
            rank: PeanoType::Zero,
        }
    }
    pub fn rank_up(self) -> Self {
        Self {
            inner: self.inner,
            rank: PeanoType::Succ(Box::new(self.rank)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeanoType {
    Zero,
    Succ(Box<PeanoType>),
    Unknown(PeanoVariableID),
}

impl PeanoType {
    pub fn count(&self) -> Option<usize> {
        match self {
            PeanoType::Zero => Some(0),
            PeanoType::Succ(inner) => Some(inner.count()? + 1),
            PeanoType::Unknown(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractGlobalReference<ID> {
    pub id: ID,
    pub template_arg_types: TVec<AbstractRankedType>,
}
