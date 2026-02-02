use crate::{
    alloc::zip_eq,
    let_unwrap,
    prelude::ClockID,
    typing::{
        abstract_type::{
            AbstractGlobalReference, AbstractInnerType, AbstractRankedType, PeanoType,
        },
        domain_type::ClockDomain,
        template::TemplateKind,
        unifyable_cell::{
            Boo, ResolveError, SubTree, SubstituteRecurse, UniCell, Unifier, UnifierTop,
            UnifierTopInfo, UnifyRecurse, UnifyResult,
        },
    },
};

#[derive(Debug)]
pub struct AbstractUnifier<'s> {
    unifier_top_info: UnifierTopInfo<'s, Self>,
}

impl<'s> AbstractUnifier<'s> {
    pub fn new() -> Self {
        Self {
            unifier_top_info: UnifierTopInfo::new(),
        }
    }
}

impl<'s> Default for AbstractUnifier<'s> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'s> UnifierTop<'s> for AbstractUnifier<'s> {
    fn get_unifier_info(&self) -> &UnifierTopInfo<'s, Self> {
        &self.unifier_top_info
    }
}

impl<'s> SubstituteRecurse<'s, PeanoType> for AbstractUnifier<'s> {
    fn fully_substitute_recurse(&self, v: &PeanoType) -> bool {
        match v {
            PeanoType::Zero => true,
            PeanoType::Succ(succ) => self.fully_substitute(succ),
        }
    }

    fn resolve_recurse(&self, v: &'s PeanoType) -> Result<(), ResolveError<'s>> {
        match v {
            PeanoType::Zero => Ok(()),
            PeanoType::Succ(succ) => self.resolve_all(succ),
        }
    }
}
impl<'s> UnifyRecurse<'s, PeanoType> for AbstractUnifier<'s> {
    fn unify_subtrees(&self, a: &'s PeanoType, b: &'s PeanoType) -> UnifyResult {
        match (a, b) {
            (PeanoType::Zero, PeanoType::Zero) => UnifyResult::Success,
            (PeanoType::Succ(a_succ), PeanoType::Succ(b_succ)) => self.unify(a_succ, b_succ),
            _ => UnifyResult::Failure,
        }
    }

    fn set_subtrees(&self, a: &'s PeanoType, b: &mut PeanoType) -> UnifyResult {
        match (a, b) {
            (PeanoType::Zero, PeanoType::Zero) => UnifyResult::Success,
            (PeanoType::Succ(a_succ), PeanoType::Succ(b_succ)) => self.set(a_succ, b_succ),
            _ => UnifyResult::Failure,
        }
    }

    fn clone_known(&self, known: &'s PeanoType) -> PeanoType {
        match known {
            PeanoType::Zero => PeanoType::Zero,
            PeanoType::Succ(succ) => PeanoType::Succ(Box::new(self.clone_unify(succ))),
        }
    }
}
impl<'s> Unifier<'s, PeanoType> for AbstractUnifier<'s> {
    fn contains_subtree(&self, in_obj: &PeanoType, subtree: SubTree<PeanoType>) -> bool {
        match in_obj {
            PeanoType::Zero => false,
            PeanoType::Succ(succ) => self.contains_subtree_recurse(succ, subtree),
        }
    }
}
impl<'s, ID> SubstituteRecurse<'s, AbstractGlobalReference<ID>> for AbstractUnifier<'s> {
    fn fully_substitute_recurse(&self, v: &AbstractGlobalReference<ID>) -> bool {
        let mut total = true;
        // In any case, iterate all
        for (_, t) in &v.template_arg_types {
            total &= match t {
                TemplateKind::Type(t) => {
                    self.fully_substitute(&t.inner) & self.fully_substitute(&t.rank)
                }
                TemplateKind::Value(()) => true,
            };
        }
        total
    }

    fn resolve_recurse(&self, v: &'s AbstractGlobalReference<ID>) -> Result<(), ResolveError<'s>> {
        for (_, t) in &v.template_arg_types {
            match t {
                TemplateKind::Type(t) => {
                    self.resolve_all(&t.inner)?;
                    self.resolve_all(&t.rank)?;
                }
                TemplateKind::Value(()) => {}
            };
        }
        Ok(())
    }
}
impl<'s, ID: Eq + Copy> UnifyRecurse<'s, AbstractGlobalReference<ID>> for AbstractUnifier<'s> {
    fn unify_subtrees(
        &self,
        a: &'s AbstractGlobalReference<ID>,
        b: &'s AbstractGlobalReference<ID>,
    ) -> UnifyResult {
        if a.id != b.id {
            return UnifyResult::Failure;
        }
        let mut total = UnifyResult::Success;
        for (_, a, b) in zip_eq(a.template_arg_types.iter(), b.template_arg_types.iter()) {
            total &= match a.and_by_ref(b) {
                TemplateKind::Type((a, b)) => {
                    self.unify(&a.inner, &b.inner) & self.unify(&a.rank, &b.rank)
                }
                TemplateKind::Value(((), ())) => UnifyResult::Success,
            }
        }
        total
    }
    fn set_subtrees(
        &self,
        a: &'s AbstractGlobalReference<ID>,
        b: &mut AbstractGlobalReference<ID>,
    ) -> UnifyResult {
        if a.id != b.id {
            return UnifyResult::Failure;
        }
        let mut total = UnifyResult::Success;
        for (_, a, b) in zip_eq(a.template_arg_types.iter(), b.template_arg_types.iter_mut()) {
            total &= match a.as_ref().and(b.as_mut()) {
                TemplateKind::Type((a, b)) => {
                    self.set(&a.inner, &mut b.inner) & self.set(&a.rank, &mut b.rank)
                }
                TemplateKind::Value(((), ())) => UnifyResult::Success,
            }
        }
        total
    }
    fn clone_known(&self, known: &'s AbstractGlobalReference<ID>) -> AbstractGlobalReference<ID> {
        AbstractGlobalReference {
            id: known.id,
            template_arg_types: known.template_arg_types.map(|(_, arg)| match arg {
                TemplateKind::Type(t) => TemplateKind::Type(self.clone_known(t)),
                TemplateKind::Value(()) => TemplateKind::Value(()),
            }),
        }
    }
}

impl<'s> SubstituteRecurse<'s, AbstractRankedType> for AbstractUnifier<'s> {
    fn fully_substitute_recurse(&self, v: &AbstractRankedType) -> bool {
        self.fully_substitute(&v.inner) & self.fully_substitute(&v.rank)
    }

    fn resolve_recurse(&self, v: &'s AbstractRankedType) -> Result<(), ResolveError<'s>> {
        self.resolve_all(&v.inner)?;
        self.resolve_all(&v.rank)
    }
}
impl<'s> UnifyRecurse<'s, AbstractRankedType> for AbstractUnifier<'s> {
    fn unify_subtrees(&self, a: &'s AbstractRankedType, b: &'s AbstractRankedType) -> UnifyResult {
        self.unify(&a.inner, &b.inner) & self.unify(&a.rank, &b.rank)
    }

    fn set_subtrees(&self, a: &'s AbstractRankedType, b: &mut AbstractRankedType) -> UnifyResult {
        self.set(&a.inner, &mut b.inner) & self.set(&a.rank, &mut b.rank)
    }

    fn clone_known(&self, known: &'s AbstractRankedType) -> AbstractRankedType {
        AbstractRankedType {
            inner: self.clone_unify(&known.inner),
            rank: self.clone_unify(&known.rank),
        }
    }
}
impl<'s> SubstituteRecurse<'s, AbstractInnerType> for AbstractUnifier<'s> {
    fn fully_substitute_recurse(&self, known: &AbstractInnerType) -> bool {
        match known {
            AbstractInnerType::Template(_) => true,
            AbstractInnerType::Named(named) => self.fully_substitute_recurse(named),
            AbstractInnerType::Interface(named, _) => self.fully_substitute_recurse(named),
            AbstractInnerType::LocalInterface(_) => true,
        }
    }

    fn resolve_recurse(&self, v: &'s AbstractInnerType) -> Result<(), ResolveError<'s>> {
        match v {
            AbstractInnerType::Template(_) => Ok(()),
            AbstractInnerType::Named(named) => self.resolve_recurse(named),
            AbstractInnerType::Interface(named, _) => self.resolve_recurse(named),
            AbstractInnerType::LocalInterface(_) => Ok(()),
        }
    }
}
impl<'s> UnifyRecurse<'s, AbstractInnerType> for AbstractUnifier<'s> {
    fn unify_subtrees(&self, a: &'s AbstractInnerType, b: &'s AbstractInnerType) -> UnifyResult {
        match (a, b) {
            (AbstractInnerType::Template(a), AbstractInnerType::Template(b)) => {
                UnifyResult::from(a == b)
            }
            (AbstractInnerType::Named(a), AbstractInnerType::Named(b)) => self.unify_subtrees(a, b),
            (AbstractInnerType::Interface(a, a_uuid), AbstractInnerType::Interface(b, b_uuid)) => {
                self.unify_subtrees(a, b) & UnifyResult::from(a_uuid == b_uuid)
            }
            (AbstractInnerType::LocalInterface(a), AbstractInnerType::LocalInterface(b)) => {
                UnifyResult::from(a == b)
            }
            _ => UnifyResult::Failure,
        }
    }

    fn set_subtrees(&self, a: &'s AbstractInnerType, b: &mut AbstractInnerType) -> UnifyResult {
        match (a, b) {
            (AbstractInnerType::Template(a), AbstractInnerType::Template(b)) => {
                UnifyResult::from(a == b)
            }
            (AbstractInnerType::Named(a), AbstractInnerType::Named(b)) => self.set_subtrees(a, b),
            (AbstractInnerType::Interface(a, a_uuid), AbstractInnerType::Interface(b, b_uuid)) => {
                self.set_subtrees(a, b) & UnifyResult::from(a_uuid == b_uuid)
            }
            (AbstractInnerType::LocalInterface(a), AbstractInnerType::LocalInterface(b)) => {
                UnifyResult::from(a == b)
            }
            _ => UnifyResult::Failure,
        }
    }

    fn clone_known(&self, known: &'s AbstractInnerType) -> AbstractInnerType {
        match known {
            AbstractInnerType::Template(uuid) => AbstractInnerType::Template(*uuid),
            AbstractInnerType::Named(named) => AbstractInnerType::Named(self.clone_known(named)),
            AbstractInnerType::Interface(named, uuid) => {
                AbstractInnerType::Interface(self.clone_known(named), *uuid)
            }
            AbstractInnerType::LocalInterface(uuid) => AbstractInnerType::LocalInterface(*uuid),
        }
    }
}

fn contains_subtree_named<'s, ID>(
    unif: &AbstractUnifier<'s>,
    in_obj: &AbstractGlobalReference<ID>,
    subtree: SubTree<AbstractInnerType>,
) -> bool {
    in_obj.template_arg_types.iter().any(|(_, v)| {
        match v {
            TemplateKind::Type(t) => unif.contains_subtree_recurse(&t.inner, subtree), // Peanos can't contain AbstractInnerType
            TemplateKind::Value(()) => false,
        }
    })
}
impl<'s> Unifier<'s, AbstractInnerType> for AbstractUnifier<'s> {
    fn contains_subtree(
        &self,
        in_obj: &AbstractInnerType,
        subtree: SubTree<AbstractInnerType>,
    ) -> bool {
        match in_obj {
            AbstractInnerType::Template(_) => false,
            AbstractInnerType::Named(named) => contains_subtree_named(self, named, subtree),
            AbstractInnerType::Interface(named, _) => contains_subtree_named(self, named, subtree),
            AbstractInnerType::LocalInterface(_) => false,
        }
    }
}

impl<'s> SubstituteRecurse<'s, ClockDomain> for AbstractUnifier<'s> {
    fn fully_substitute_recurse(&self, _: &ClockDomain) -> bool {
        true
    }

    fn resolve_recurse(&self, _: &ClockDomain) -> Result<(), ResolveError<'s>> {
        Ok(())
    }
}
impl<'s> UnifyRecurse<'s, ClockDomain> for AbstractUnifier<'s> {
    fn unify_subtrees(&self, a: &'s ClockDomain, b: &'s ClockDomain) -> UnifyResult {
        match (a, b) {
            (ClockDomain::Generative, ClockDomain::Generative) => UnifyResult::Success,
            (ClockDomain::Physical(a), ClockDomain::Physical(b)) => self.unify(a, b),
            _ => UnifyResult::Failure,
        }
    }

    fn set_subtrees(&self, a: &'s ClockDomain, b: &mut ClockDomain) -> UnifyResult {
        match (a, b) {
            (ClockDomain::Generative, ClockDomain::Generative) => UnifyResult::Success,
            (ClockDomain::Physical(a), ClockDomain::Physical(b)) => self.set(a, b),
            _ => UnifyResult::Failure,
        }
    }

    fn clone_known(&self, known: &'s ClockDomain) -> ClockDomain {
        match known {
            ClockDomain::Generative => ClockDomain::Generative,
            ClockDomain::Physical(phys) => ClockDomain::Physical(self.clone_unify(phys)),
        }
    }
}

impl<'s> Unifier<'s, ClockDomain> for AbstractUnifier<'s> {
    fn contains_subtree(&self, _in_obj: &ClockDomain, _subtree: SubTree<ClockDomain>) -> bool {
        false
    }
}

impl<'s> SubstituteRecurse<'s, ClockID> for AbstractUnifier<'s> {
    fn fully_substitute_recurse(&self, _: &ClockID) -> bool {
        true
    }

    fn resolve_recurse(&self, _: &ClockID) -> Result<(), ResolveError<'s>> {
        Ok(())
    }
}
impl<'s> UnifyRecurse<'s, ClockID> for AbstractUnifier<'s> {
    fn unify_subtrees(&self, a: &'s ClockID, b: &'s ClockID) -> UnifyResult {
        UnifyResult::from(a == b)
    }

    fn set_subtrees(&self, a: &'s ClockID, b: &mut ClockID) -> UnifyResult {
        UnifyResult::from(a == b)
    }

    fn clone_known(&self, known: &'s ClockID) -> ClockID {
        *known
    }
}

impl<'s> Unifier<'s, ClockID> for AbstractUnifier<'s> {
    fn contains_subtree(&self, _in_obj: &ClockID, _subtree: SubTree<ClockID>) -> bool {
        false
    }
}

impl<'s> AbstractUnifier<'s> {
    /// Returns the type of the content of the array
    ///
    /// [None] indicates the input type was not an array.
    ///
    /// If rank_down was unsuccessful, this returns None
    pub fn rank_down(&self, mut arr_rank: UniCell<PeanoType>) -> Option<UniCell<PeanoType>> {
        match self.try_resolve(&mut arr_rank) {
            Ok(Some(Boo::Owned(PeanoType::Succ(succ)))) => Some(*succ),
            Ok(Some(Boo::Borrow(PeanoType::Succ(succ)))) => Some(self.clone_unify(succ)),
            Ok(Some(Boo::Owned(PeanoType::Zero))) | Ok(Some(Boo::Borrow(PeanoType::Zero))) => None,
            Ok(None) => Some(PeanoType::UNKNOWN),
            Err(unknown_cell) => {
                let set_v =
                    self.set_hard(unknown_cell, PeanoType::Succ(Box::new(PeanoType::UNKNOWN)));
                let_unwrap!(PeanoType::Succ(succ), set_v);
                Some(self.clone_unify(succ))
            }
        }
    }
}
