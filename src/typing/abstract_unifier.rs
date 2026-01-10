use crate::{
    alloc::zip_eq,
    prelude::DomainID,
    typing::{
        abstract_type::{
            AbstractGlobalReference, AbstractInnerType, AbstractRankedType, PeanoType,
        },
        domain_type::DomainType,
        template::TemplateKind,
        unifyable_cell::{
            ResolveError, SubTree, SubstituteRecurse, Substitutor, UniCell, Unifier, UnifierTop,
            UnifierTopInfo, UnifyRecurse, UnifyResult,
        },
    },
};

#[derive(Debug)]
pub struct AbstractUnifier<'s> {
    inner_substitutor: Substitutor<'s, AbstractInnerType, Self>,
    rank_substitutor: Substitutor<'s, PeanoType, Self>,
    domain_substitutor: Substitutor<'s, DomainType, Self>,
    physical_domain_substitutor: Substitutor<'s, DomainID, Self>,
    unifier_top_info: UnifierTopInfo<'s, Self>,
}

impl<'s> AbstractUnifier<'s> {
    pub fn new() -> Self {
        Self {
            inner_substitutor: Substitutor::new(),
            rank_substitutor: Substitutor::new(),
            domain_substitutor: Substitutor::new(),
            physical_domain_substitutor: Substitutor::new(),
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
    fn execute_ready_constraints(&self) {
        self.inner_substitutor.execute_ready_constraints(self);
        self.rank_substitutor.execute_ready_constraints(self);
        self.domain_substitutor.execute_ready_constraints(self);
    }

    fn get_unifier_info(&self) -> &UnifierTopInfo<'s, Self> {
        &self.unifier_top_info
    }
}

impl<'unif, 's: 'unif> SubstituteRecurse<'unif, 's, PeanoType> for AbstractUnifier<'s> {
    fn fully_substitute_recurse(&'unif self, v: &PeanoType) -> bool {
        match v {
            PeanoType::Zero => true,
            PeanoType::Succ(succ) => self.fully_substitute(succ),
        }
    }

    fn resolve_recurse(&'unif self, v: &'s PeanoType) -> Result<(), ResolveError<'unif, 's, Self>> {
        match v {
            PeanoType::Zero => Ok(()),
            PeanoType::Succ(succ) => self.resolve_all(succ),
        }
    }
}
impl<'unif, 's: 'unif> UnifyRecurse<'unif, 's, PeanoType> for AbstractUnifier<'s> {
    fn unify_subtrees(&'unif self, a: &'s PeanoType, b: &'s PeanoType) -> UnifyResult {
        match (a, b) {
            (PeanoType::Zero, PeanoType::Zero) => UnifyResult::Success,
            (PeanoType::Succ(a_succ), PeanoType::Succ(b_succ)) => self.unify(a_succ, b_succ),
            _ => UnifyResult::Failure,
        }
    }

    fn set_subtrees(&'unif self, a: &'s PeanoType, b: &mut PeanoType) -> UnifyResult {
        match (a, b) {
            (PeanoType::Zero, PeanoType::Zero) => UnifyResult::Success,
            (PeanoType::Succ(a_succ), PeanoType::Succ(b_succ)) => self.set(a_succ, b_succ),
            _ => UnifyResult::Failure,
        }
    }

    fn clone_known(&'unif self, known: &'s PeanoType) -> PeanoType {
        match known {
            PeanoType::Zero => PeanoType::Zero,
            PeanoType::Succ(succ) => PeanoType::Succ(Box::new(self.clone_unify(succ))),
        }
    }
}
impl<'unif, 's: 'unif> Unifier<'unif, 's, PeanoType> for AbstractUnifier<'s> {
    fn get_substitutor(&'unif self) -> &'unif Substitutor<'s, PeanoType, Self> {
        &self.rank_substitutor
    }
    fn contains_subtree(&'unif self, in_obj: &PeanoType, subtree: SubTree<PeanoType>) -> bool {
        match in_obj {
            PeanoType::Zero => false,
            PeanoType::Succ(succ) => self.contains_subtree_recurse(succ, subtree),
        }
    }
}
impl<'unif, 's: 'unif, ID> SubstituteRecurse<'unif, 's, AbstractGlobalReference<ID>>
    for AbstractUnifier<'s>
{
    fn fully_substitute_recurse(&'unif self, v: &AbstractGlobalReference<ID>) -> bool {
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

    fn resolve_recurse(
        &'unif self,
        v: &'s AbstractGlobalReference<ID>,
    ) -> Result<(), ResolveError<'unif, 's, Self>> {
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
impl<'unif, 's: 'unif, ID: Eq + Copy> UnifyRecurse<'unif, 's, AbstractGlobalReference<ID>>
    for AbstractUnifier<'s>
{
    fn unify_subtrees(
        &'unif self,
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
        &'unif self,
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
    fn clone_known(
        &'unif self,
        in_obj: &'s AbstractGlobalReference<ID>,
    ) -> AbstractGlobalReference<ID> {
        AbstractGlobalReference {
            id: in_obj.id,
            template_arg_types: in_obj.template_arg_types.map(|(_, arg)| match arg {
                TemplateKind::Type(t) => TemplateKind::Type(self.clone_known(t)),
                TemplateKind::Value(()) => TemplateKind::Value(()),
            }),
        }
    }
}

impl<'unif, 's: 'unif> SubstituteRecurse<'unif, 's, AbstractRankedType> for AbstractUnifier<'s> {
    fn fully_substitute_recurse(&'unif self, v: &AbstractRankedType) -> bool {
        self.fully_substitute(&v.inner) & self.fully_substitute(&v.rank)
    }

    fn resolve_recurse(
        &'unif self,
        v: &'s AbstractRankedType,
    ) -> Result<(), ResolveError<'unif, 's, Self>> {
        self.resolve_all(&v.inner)?;
        self.resolve_all(&v.rank)
    }
}
impl<'unif, 's: 'unif> UnifyRecurse<'unif, 's, AbstractRankedType> for AbstractUnifier<'s> {
    fn unify_subtrees(
        &'unif self,
        a: &'s AbstractRankedType,
        b: &'s AbstractRankedType,
    ) -> UnifyResult {
        self.unify(&a.inner, &b.inner) & self.unify(&a.rank, &b.rank)
    }

    fn set_subtrees(
        &'unif self,
        a: &'s AbstractRankedType,
        b: &mut AbstractRankedType,
    ) -> UnifyResult {
        self.set(&a.inner, &mut b.inner) & self.set(&a.rank, &mut b.rank)
    }

    fn clone_known(&'unif self, known: &'s AbstractRankedType) -> AbstractRankedType {
        AbstractRankedType {
            inner: self.clone_unify(&known.inner),
            rank: self.clone_unify(&known.rank),
        }
    }
}
impl<'unif, 's: 'unif> SubstituteRecurse<'unif, 's, AbstractInnerType> for AbstractUnifier<'s> {
    fn fully_substitute_recurse(&'unif self, known: &AbstractInnerType) -> bool {
        match known {
            AbstractInnerType::Template(_) => true,
            AbstractInnerType::Named(named) => self.fully_substitute_recurse(named),
            AbstractInnerType::Interface(named, _) => self.fully_substitute_recurse(named),
            AbstractInnerType::LocalInterface(_) => true,
        }
    }

    fn resolve_recurse(
        &'unif self,
        v: &'s AbstractInnerType,
    ) -> Result<(), ResolveError<'unif, 's, Self>> {
        match v {
            AbstractInnerType::Template(_) => Ok(()),
            AbstractInnerType::Named(named) => self.resolve_recurse(named),
            AbstractInnerType::Interface(named, _) => self.resolve_recurse(named),
            AbstractInnerType::LocalInterface(_) => Ok(()),
        }
    }
}
impl<'unif, 's: 'unif> UnifyRecurse<'unif, 's, AbstractInnerType> for AbstractUnifier<'s> {
    fn unify_subtrees(
        &'unif self,
        a: &'s AbstractInnerType,
        b: &'s AbstractInnerType,
    ) -> UnifyResult {
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

    fn set_subtrees(
        &'unif self,
        a: &'s AbstractInnerType,
        b: &mut AbstractInnerType,
    ) -> UnifyResult {
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

    fn clone_known(&'unif self, known: &'s AbstractInnerType) -> AbstractInnerType {
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

fn contains_subtree_named<'unif, 's: 'unif, ID>(
    unif: &'unif AbstractUnifier<'s>,
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
impl<'unif, 's: 'unif> Unifier<'unif, 's, AbstractInnerType> for AbstractUnifier<'s> {
    fn get_substitutor(&'unif self) -> &'unif Substitutor<'s, AbstractInnerType, Self> {
        &self.inner_substitutor
    }
    fn contains_subtree(
        &'unif self,
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

impl<'unif, 's: 'unif> SubstituteRecurse<'unif, 's, DomainType> for AbstractUnifier<'s> {
    fn fully_substitute_recurse(&'unif self, _: &DomainType) -> bool {
        true
    }

    fn resolve_recurse(
        &'unif self,
        _: &'s DomainType,
    ) -> Result<(), ResolveError<'unif, 's, Self>> {
        Ok(())
    }
}
impl<'unif, 's: 'unif> UnifyRecurse<'unif, 's, DomainType> for AbstractUnifier<'s> {
    fn unify_subtrees(&'unif self, a: &'s DomainType, b: &'s DomainType) -> UnifyResult {
        match (a, b) {
            (DomainType::Generative, DomainType::Generative) => UnifyResult::Success,
            (DomainType::Physical(a), DomainType::Physical(b)) => self.unify(a, b),
            _ => UnifyResult::Failure,
        }
    }

    fn set_subtrees(&'unif self, a: &'s DomainType, b: &mut DomainType) -> UnifyResult {
        match (a, b) {
            (DomainType::Generative, DomainType::Generative) => UnifyResult::Success,
            (DomainType::Physical(a), DomainType::Physical(b)) => self.set(a, b),
            _ => UnifyResult::Failure,
        }
    }

    fn clone_known(&'unif self, known: &'s DomainType) -> DomainType {
        match known {
            DomainType::Generative => DomainType::Generative,
            DomainType::Physical(phys) => DomainType::Physical(self.clone_unify(phys)),
        }
    }
}

impl<'unif, 's: 'unif> Unifier<'unif, 's, DomainType> for AbstractUnifier<'s> {
    fn get_substitutor(&'unif self) -> &'unif Substitutor<'s, DomainType, Self> {
        &self.domain_substitutor
    }

    fn contains_subtree(&'unif self, _in_obj: &DomainType, _subtree: SubTree<DomainType>) -> bool {
        false
    }
}

impl<'unif, 's: 'unif> SubstituteRecurse<'unif, 's, DomainID> for AbstractUnifier<'s> {
    fn fully_substitute_recurse(&'unif self, _: &DomainID) -> bool {
        true
    }

    fn resolve_recurse(&'unif self, _: &'s DomainID) -> Result<(), ResolveError<'unif, 's, Self>> {
        Ok(())
    }
}
impl<'unif, 's: 'unif> UnifyRecurse<'unif, 's, DomainID> for AbstractUnifier<'s> {
    fn unify_subtrees(&'unif self, a: &'s DomainID, b: &'s DomainID) -> UnifyResult {
        UnifyResult::from(a == b)
    }

    fn set_subtrees(&'unif self, a: &'s DomainID, b: &mut DomainID) -> UnifyResult {
        UnifyResult::from(a == b)
    }

    fn clone_known(&'unif self, known: &'s DomainID) -> DomainID {
        *known
    }
}

impl<'unif, 's: 'unif> Unifier<'unif, 's, DomainID> for AbstractUnifier<'s> {
    fn get_substitutor(&'unif self) -> &'unif Substitutor<'s, DomainID, Self> {
        &self.physical_domain_substitutor
    }

    fn contains_subtree(&'unif self, _in_obj: &DomainID, _subtree: SubTree<DomainID>) -> bool {
        false
    }
}

impl<'s> AbstractUnifier<'s> {
    /// Returns the type of the content of the array
    ///
    /// [None] indicates the input type was not an array.
    pub fn rank_down(&self, arr_rank: &'s UniCell<PeanoType>) -> Option<&'s UniCell<PeanoType>> {
        // We'll check if unification failed in resolve below
        let _ = self.set(
            arr_rank,
            &mut UniCell::new(PeanoType::Succ(Box::new(PeanoType::UNKNOWN))),
        );

        // At this point we can *always* resolve.
        // Either set succeeded, in which case we should resolve to PeanoType::Succ
        // Or it failed, in which case we should resolve to PeanoType::Zero
        match self.resolve(arr_rank).unwrap() {
            PeanoType::Zero => None,
            PeanoType::Succ(down_rank) => Some(down_rank),
        }
    }
}
