use std::sync::LazyLock;

use sus_proc_macro::get_builtin_type;

use crate::{
    prelude::*,
    typing::template::{TVec, TemplateKind},
};

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
    Named(AbstractGlobalReference<TypeUUID>),
    Interface(AbstractGlobalReference<ModuleUUID>, InterfaceID),
    LocalInterface(FlatID),
    /// Referencing [AbstractType::Unknown] is a strong code smell.
    /// It is likely you should use [TypeSubstitutor::unify_must_succeed] or [TypeSubstitutor::unify_report_error] instead
    ///
    /// It should only occur in creation `AbstractType::Unknown(self.type_substitutor.alloc())`
    Unknown(InnerTypeVariableID),
}

pub const BOOL_INNER: AbstractInnerType = AbstractInnerType::Named(AbstractGlobalReference {
    id: get_builtin_type!("bool"),
    template_arg_types: TVec::new(),
});
pub static INT_INNER: LazyLock<AbstractInnerType> = LazyLock::new(|| {
    AbstractInnerType::Named(AbstractGlobalReference {
        id: get_builtin_type!("int"),
        template_arg_types: TVec::from_vec(vec![TemplateKind::Value(()), TemplateKind::Value(())]),
    })
});

impl AbstractInnerType {
    pub fn scalar(self) -> AbstractRankedType {
        AbstractRankedType {
            inner: self,
            rank: PeanoType::Zero,
        }
    }
    pub fn with_rank(self, rank: PeanoType) -> AbstractRankedType {
        AbstractRankedType { inner: self, rank }
    }
    pub fn is_interface(&self) -> bool {
        match self {
            AbstractInnerType::Interface(_, _) | AbstractInnerType::LocalInterface(_) => true,
            AbstractInnerType::Template(_)
            | AbstractInnerType::Named(_)
            | AbstractInnerType::Unknown(_) => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractRankedType {
    pub inner: AbstractInnerType,
    pub rank: PeanoType,
}

pub const BOOL_SCALAR: AbstractRankedType = AbstractRankedType {
    inner: BOOL_INNER,
    rank: PeanoType::Zero,
};
pub static INT_SCALAR: LazyLock<AbstractRankedType> = LazyLock::new(|| AbstractRankedType {
    inner: INT_INNER.clone(),
    rank: PeanoType::Zero,
});

impl AbstractRankedType {
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
    pub fn count_unwrap(&self) -> usize {
        let Some(cnt) = self.count() else {
            panic!("Peano Number {self:?} still contains Unknown!");
        };
        cnt
    }
    pub fn from_natural(count: usize) -> Self {
        if count == 0 {
            PeanoType::Zero
        } else {
            PeanoType::Succ(Box::new(PeanoType::from_natural(count - 1)))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractGlobalReference<ID> {
    pub id: ID,
    pub template_arg_types: TVec<TemplateKind<AbstractRankedType, ()>>,
}
