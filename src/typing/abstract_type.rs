use std::sync::LazyLock;

use sus_proc_macro::get_builtin_type;

use crate::{
    prelude::*,
    typing::{
        template::{TVec, TemplateKind},
        unifyable_cell::{SyncWrapper, UniCell},
    },
};

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
}

pub const BOOL_INNER: AbstractInnerType = AbstractInnerType::Named(AbstractGlobalReference {
    id: get_builtin_type!("bool"),
    template_arg_types: TVec::new(),
});
pub const FLOAT_INNER: AbstractInnerType = AbstractInnerType::Named(AbstractGlobalReference {
    id: get_builtin_type!("float"),
    template_arg_types: TVec::new(),
});
pub const DOUBLE_INNER: AbstractInnerType = AbstractInnerType::Named(AbstractGlobalReference {
    id: get_builtin_type!("double"),
    template_arg_types: TVec::new(),
});
pub const STRING_INNER: AbstractInnerType = AbstractInnerType::Named(AbstractGlobalReference {
    id: get_builtin_type!("string"),
    template_arg_types: TVec::new(),
});
pub static INT_INNER: LazyLock<SyncWrapper<AbstractInnerType>> = LazyLock::new(|| {
    SyncWrapper::new(AbstractInnerType::Named(AbstractGlobalReference {
        id: get_builtin_type!("int"),
        template_arg_types: TVec::from_vec(vec![TemplateKind::Value(()), TemplateKind::Value(())]),
    }))
});

impl AbstractInnerType {
    #[allow(clippy::declare_interior_mutable_const)]
    pub const UNKNOWN: UniCell<AbstractInnerType> = UniCell::UNKNOWN;

    pub const fn scalar(self) -> AbstractRankedType {
        AbstractRankedType {
            inner: UniCell::from_known(self),
            rank: UniCell::from_known(PeanoType::Zero),
        }
    }
    pub fn with_rank(self, rank: impl Into<UniCell<PeanoType>>) -> AbstractRankedType {
        AbstractRankedType {
            inner: UniCell::from_known(self),
            rank: rank.into(),
        }
    }
    pub fn is_interface(&self) -> bool {
        match self {
            AbstractInnerType::Interface(_, _) | AbstractInnerType::LocalInterface(_) => true,
            AbstractInnerType::Template(_) | AbstractInnerType::Named(_) => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractRankedType {
    pub inner: UniCell<AbstractInnerType>,
    pub rank: UniCell<PeanoType>,
}

#[allow(clippy::declare_interior_mutable_const)]
pub const BOOL_SCALAR: AbstractRankedType = BOOL_INNER.scalar();
pub static BOOL_SCALAR_FOR_REF: SyncWrapper<AbstractRankedType> = SyncWrapper::new(BOOL_SCALAR);
#[allow(clippy::declare_interior_mutable_const)]
pub const FLOAT_SCALAR: AbstractRankedType = FLOAT_INNER.scalar();
#[allow(clippy::declare_interior_mutable_const)]
pub const DOUBLE_SCALAR: AbstractRankedType = DOUBLE_INNER.scalar();
#[allow(clippy::declare_interior_mutable_const)]
pub const STRING_SCALAR: AbstractRankedType = STRING_INNER.scalar();
pub static INT_SCALAR: LazyLock<SyncWrapper<AbstractRankedType>> =
    LazyLock::new(|| SyncWrapper::new(INT_INNER.clone().scalar()));

impl AbstractRankedType {
    #[allow(clippy::declare_interior_mutable_const)]
    pub const UNKNOWN: AbstractRankedType = AbstractRankedType {
        inner: AbstractInnerType::UNKNOWN,
        rank: PeanoType::UNKNOWN,
    };

    pub const fn scalar(inner: AbstractInnerType) -> Self {
        Self {
            inner: UniCell::from_known(inner),
            rank: UniCell::from_known(PeanoType::Zero),
        }
    }
    pub fn set_initial(&self, initial: AbstractRankedType) {
        self.inner.set_initial_cell(initial.inner);
        self.rank.set_initial_cell(initial.rank);
    }
    pub fn rank_up(self) -> Self {
        Self {
            inner: self.inner,
            rank: UniCell::from_known(PeanoType::Succ(Box::new(self.rank))),
        }
    }
    pub fn rank_up_multi(self, cnt: usize) -> Self {
        let mut cur = self;

        for _ in 0..cnt {
            cur = cur.rank_up()
        }

        cur
    }
    pub fn is_int_scalar(&self) -> bool {
        *self.rank.unwrap() == PeanoType::Zero
            && matches!(
                &self.inner.unwrap(),
                AbstractInnerType::Named(AbstractGlobalReference {
                    id: get_builtin_type!("int"),
                    ..
                })
            )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeanoType {
    Zero,
    Succ(Box<UniCell<PeanoType>>),
}

impl PeanoType {
    #[allow(clippy::declare_interior_mutable_const)]
    pub const UNKNOWN: UniCell<PeanoType> = UniCell::UNKNOWN;

    pub fn count(&self) -> usize {
        let mut cur = self;
        let mut sum = 0;

        while let PeanoType::Succ(succ) = cur {
            cur = succ.unwrap();
            sum += 1;
        }

        sum
    }
    pub fn from_natural(count: usize) -> Self {
        let mut result = PeanoType::Zero;

        for _ in 0..count {
            result = PeanoType::Succ(Box::new(UniCell::from(result)));
        }

        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractGlobalReference<ID> {
    pub id: ID,
    pub template_arg_types: TVec<TemplateKind<AbstractRankedType, ()>>,
}

impl AbstractRankedType {
    pub fn substitute_template_args(
        &self,
        args: &TVec<TemplateKind<AbstractRankedType, ()>>,
    ) -> Self {
        match self.inner.unwrap() {
            // We don't recursively substitute the type we get from the template arg, because those are in a different namespace
            AbstractInnerType::Template(id) => args[*id]
                .unwrap_type()
                .clone()
                .rank_up_multi(self.rank.count()),
            AbstractInnerType::Named(named_ref) => {
                AbstractInnerType::Named(named_ref.substitute_template_args(args))
                    .with_rank(self.rank.unwrap().clone())
            }
            AbstractInnerType::Interface(module_ref, interface_id) => AbstractInnerType::Interface(
                module_ref.substitute_template_args(args),
                *interface_id,
            )
            .with_rank(self.rank.unwrap().clone()),
            AbstractInnerType::LocalInterface(_) => self.clone(),
        }
    }
}

impl<ID: Copy> AbstractGlobalReference<ID> {
    pub fn substitute_template_args(
        &self,
        args: &TVec<TemplateKind<AbstractRankedType, ()>>,
    ) -> Self {
        let template_arg_types = self.template_arg_types.map(|(_, arg)| match arg {
            TemplateKind::Type(t) => TemplateKind::Type(t.substitute_template_args(args)),
            TemplateKind::Value(()) => TemplateKind::Value(()),
        });

        Self {
            id: self.id,
            template_arg_types,
        }
    }
}
