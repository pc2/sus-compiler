use crate::alloc::UUIDAllocator;
use crate::prelude::*;

use std::cell::OnceCell;
use std::{cell::RefCell, ops::Deref};

use super::template::TemplateInputs;
use super::type_inference::{TypeVariableID, TypeVariableIDMarker};
use crate::flattening::{BinaryOperator, StructType, UnaryOperator};
use crate::linker::{get_builtin_type, Resolver};
use crate::to_string::map_to_type_names;

/// This contains only the information that can be easily type-checked.
///
/// Its most important components are the names and structure of types.
///
/// What isn't included are the parameters of types. So Array Sizes for example.
#[derive(Debug, Clone)]
pub enum AbstractType {
    Error,
    Unknown(TypeVariableID),
    Template(TemplateID),
    Named(TypeUUID),
    Array(Box<AbstractType>),
}

impl AbstractType {
    pub fn contains_error_or_unknown<const CHECK_ERROR: bool, const CHECK_UNKNOWN: bool>(
        &self,
    ) -> bool {
        match self {
            AbstractType::Error => CHECK_ERROR,
            AbstractType::Unknown(_) => CHECK_UNKNOWN,
            AbstractType::Template(_id) => false,
            AbstractType::Named(_id) => false,
            AbstractType::Array(arr_box) => arr_box
                .deref()
                .contains_error_or_unknown::<CHECK_ERROR, CHECK_UNKNOWN>(),
        }
    }
}

pub const BOOL_TYPE: AbstractType = AbstractType::Named(get_builtin_type("bool"));
pub const INT_TYPE: AbstractType = AbstractType::Named(get_builtin_type("int"));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainType {
    /// Generative conflicts with nothing
    Generative,
    /// This object is a real wire. It corresponds to a certain (clock) domain. It can only affect wires in the same clock domain.
    /// Initial value is Physical(DomainID::PLACEHOLDER)
    Physical(DomainID),
}
impl DomainType {
    pub fn unwrap_physical(&self) -> DomainID {
        let Self::Physical(w) = self else {
            unreachable!()
        };
        *w
    }
    pub fn is_generative(&self) -> bool {
        match self {
            DomainType::Generative => true,
            DomainType::Physical(_) => false,
        }
    }
    pub fn new_unset() -> DomainType {
        DomainType::Physical(DomainID::PLACEHOLDER)
    }
}

#[derive(Debug, Clone)]
pub struct FullType {
    pub typ: AbstractType,
    pub domain: DomainType,
}

#[derive(Debug, Clone)]
enum DomainTypeSubstitution {
    /// Known Domains correspond to the declared interfaces on the module.
    ///
    /// They are mutually exclusive. When two known domains come into contact, they produce an error
    KnownDomain { name: String },
    /// Unknown Domains are not mutually exclusive. When an unknown interface meets a known or unknown interface, it converts to an AliasFor
    UnknownDomain { converts_to: DomainID },
    /// Hindley-Milner Type Unification. When domains are merged, AliasFor objects are created from one of the domains to the other
    AliasFor(DomainID),
}

pub enum BestName {
    NamedDomain,
    SubModule(FlatID, DomainID),
    NamedWire(FlatID),
    UnnamedWire,
}
impl BestName {
    fn strength(&self) -> i8 {
        match self {
            BestName::NamedDomain => 3,
            BestName::SubModule(_, _) => 2,
            BestName::NamedWire(_) => 1,
            BestName::UnnamedWire => 0,
        }
    }
}

/// Unification of domains?
///
/// 'A U 'x -> 'x = 'A
///
/// 'x U 'y -> 'x = 'y
pub struct TypeUnifier<'linker, 'errs> {
    pub linker_types: Resolver<'linker, 'errs, TypeUUIDMarker, StructType>,
    template_type_names: FlatAlloc<String, TemplateIDMarker>,
    domain_substitutor: RefCell<FlatAlloc<DomainTypeSubstitution, DomainIDMarker>>,
    errors: &'errs ErrorCollector<'linker>,
    pub final_domains: FlatAlloc<BestName, DomainIDMarker>,
    pub type_substitutor: FlatAlloc<OnceCell<AbstractType>, TypeVariableIDMarker>
}

impl<'linker, 'errs> TypeUnifier<'linker, 'errs> {
    pub fn new(
        linker_types: Resolver<'linker, 'errs, TypeUUIDMarker, StructType>,
        template_inputs: &TemplateInputs,
        errors: &'errs ErrorCollector<'linker>,
        domain_names: &FlatAlloc<String, DomainIDMarker>,
        type_var_id_alloc: &UUIDAllocator<TypeVariableIDMarker>
    ) -> Self {
        let domains = domain_names
            .map(|(_id, name)| DomainTypeSubstitution::KnownDomain { name: name.clone() });
        let final_domains = domain_names.map(|(_id, _interface)| BestName::NamedDomain);
        Self {
            linker_types,
            template_type_names: map_to_type_names(template_inputs),
            errors,
            domain_substitutor: RefCell::new(domains),
            final_domains,
            type_substitutor: type_var_id_alloc.to_flat_alloc()
        }
    }

    // ===== Types =====

    fn type_compare(&self, expected: &AbstractType, found: &AbstractType) -> bool {
        match (expected, found) {
            (AbstractType::Named(exp), AbstractType::Named(fnd)) => exp == fnd,
            (AbstractType::Template(t), AbstractType::Template(t2)) => t == t2,
            (AbstractType::Array(exp), AbstractType::Array(fnd)) => {
                self.type_compare(&exp.deref(), &fnd.deref())
            }
            (AbstractType::Error, _) | (_, AbstractType::Error) => true, // Just assume correct, because the other side has an error
            (AbstractType::Unknown(_), _) | (_, AbstractType::Unknown(_)) => todo!("Type Unification"),
            _ => false,
        }
    }

    pub fn typecheck_abstr(
        &self,
        found: &AbstractType,
        span: Span,
        expected: &AbstractType,
        context: &str,
        declared_here: Option<SpanFile>,
    ) {
        if !self.type_compare(expected, found) {
            let expected_name = expected.to_string(&self.linker_types, &self.template_type_names);
            let found_name = found.to_string(&self.linker_types, &self.template_type_names);
            let err_ref = self.errors.error(span, format!("Typing Error: {context} expects a {expected_name} but was given a {found_name}"));
            if let Some(declared_here) = declared_here {
                err_ref.info(declared_here, "Declared here");
            }
            assert!(
                expected_name != found_name,
                "{expected_name} != {found_name}"
            );
        }
    }

    pub fn typecheck_unary_operator_abstr(
        &self,
        op: UnaryOperator,
        input_typ: &AbstractType,
        span: Span,
    ) -> AbstractType {
        if op == UnaryOperator::Not {
            self.typecheck_abstr(input_typ, span, &BOOL_TYPE, "! input", None);
            BOOL_TYPE
        } else if op == UnaryOperator::Negate {
            self.typecheck_abstr(input_typ, span, &INT_TYPE, "- input", None);
            INT_TYPE
        } else {
            let gather_type = match op {
                UnaryOperator::And => BOOL_TYPE,
                UnaryOperator::Or => BOOL_TYPE,
                UnaryOperator::Xor => BOOL_TYPE,
                UnaryOperator::Sum => INT_TYPE,
                UnaryOperator::Product => INT_TYPE,
                _ => unreachable!(),
            };
            let arr_content_typ = self.typecheck_is_array_abstr(input_typ, span);
            self.typecheck_abstr(
                &arr_content_typ,
                span,
                &gather_type,
                &format!("{op} input"),
                None,
            );

            gather_type
        }
    }

    pub fn typecheck_binary_operator_abstr(
        &self,
        op: BinaryOperator,
        left_typ: &AbstractType,
        right_typ: &AbstractType,
        left_span: Span,
        right_span: Span,
    ) -> AbstractType {
        let ((exp_left, exp_right), out) = match op {
            BinaryOperator::And => ((BOOL_TYPE, BOOL_TYPE), BOOL_TYPE),
            BinaryOperator::Or => ((BOOL_TYPE, BOOL_TYPE), BOOL_TYPE),
            BinaryOperator::Xor => ((BOOL_TYPE, BOOL_TYPE), BOOL_TYPE),
            BinaryOperator::Add => ((INT_TYPE, INT_TYPE), INT_TYPE),
            BinaryOperator::Subtract => ((INT_TYPE, INT_TYPE), INT_TYPE),
            BinaryOperator::Multiply => ((INT_TYPE, INT_TYPE), INT_TYPE),
            BinaryOperator::Divide => ((INT_TYPE, INT_TYPE), INT_TYPE),
            BinaryOperator::Modulo => ((INT_TYPE, INT_TYPE), INT_TYPE),
            BinaryOperator::Equals => ((INT_TYPE, INT_TYPE), BOOL_TYPE),
            BinaryOperator::NotEquals => ((INT_TYPE, INT_TYPE), BOOL_TYPE),
            BinaryOperator::GreaterEq => ((INT_TYPE, INT_TYPE), BOOL_TYPE),
            BinaryOperator::Greater => ((INT_TYPE, INT_TYPE), BOOL_TYPE),
            BinaryOperator::LesserEq => ((INT_TYPE, INT_TYPE), BOOL_TYPE),
            BinaryOperator::Lesser => ((INT_TYPE, INT_TYPE), BOOL_TYPE),
        };

        self.typecheck_abstr(
            left_typ,
            left_span,
            &exp_left,
            &format!("{op} left side"),
            None,
        );
        self.typecheck_abstr(
            right_typ,
            right_span,
            &exp_right,
            &format!("{op} right side"),
            None,
        );

        out
    }

    pub fn typecheck_is_array_abstr(
        &self,
        arr_type: &AbstractType,
        arr_span: Span,
    ) -> AbstractType {
        let AbstractType::Array(arr_element_type) = arr_type else {
            let arr_type_name = arr_type.to_string(&self.linker_types, &self.template_type_names);
            self.errors.error(arr_span, format!("Typing Error: Attempting to index into this, but it is not of array type, instead found a {arr_type_name}"));
            return AbstractType::Error;
        };
        arr_element_type.deref().clone()
    }

    // ===== Domains =====
    fn get_root_domain(&self, mut v: DomainID) -> DomainID {
        let doms_borrow = self.domain_substitutor.borrow();
        while let DomainTypeSubstitution::AliasFor(new_v) = &doms_borrow[v] {
            v = *new_v;
        }
        v
    }

    pub fn new_unknown_domain_id(&self) -> DomainID {
        self.domain_substitutor
            .borrow_mut()
            .alloc(DomainTypeSubstitution::UnknownDomain {
                converts_to: DomainID::PLACEHOLDER,
            })
    }

    pub fn new_unknown_domain(&self, is_generative: bool) -> DomainType {
        if is_generative {
            DomainType::Generative
        } else {
            DomainType::Physical(self.new_unknown_domain_id())
        }
    }

    /// Returns the names of the KnownDomains on error
    fn try_merge_physical<ErrFunc: FnOnce(&str, &str)>(
        &self,
        a: DomainID,
        b: DomainID,
        err_func: ErrFunc,
    ) -> bool {
        let root_a = self.get_root_domain(a);
        let root_b = self.get_root_domain(b);

        let mut domains_borrow = self.domain_substitutor.borrow_mut();
        let Some((dom_a, dom_b)) = domains_borrow.get2_mut(root_a, root_b) else {
            return true;
        }; // Same domain anyway

        match (dom_a, dom_b) {
            (DomainTypeSubstitution::AliasFor(_), _) | (_, DomainTypeSubstitution::AliasFor(_)) => {
                unreachable!()
            }
            (
                DomainTypeSubstitution::KnownDomain { name: name_a },
                DomainTypeSubstitution::KnownDomain { name: name_b },
            ) => {
                err_func(name_a, name_b);
                false
            }
            (
                DomainTypeSubstitution::KnownDomain { name: _ },
                dom_b @ DomainTypeSubstitution::UnknownDomain { converts_to: _ },
            ) => {
                *dom_b = DomainTypeSubstitution::AliasFor(root_a);
                true
            }
            (
                dom_a @ DomainTypeSubstitution::UnknownDomain { converts_to: _ },
                DomainTypeSubstitution::UnknownDomain { converts_to: _ },
            )
            | (
                dom_a @ DomainTypeSubstitution::UnknownDomain { converts_to: _ },
                DomainTypeSubstitution::KnownDomain { name: _ },
            ) => {
                *dom_a = DomainTypeSubstitution::AliasFor(root_b);
                true
            }
        }
    }

    /// Passes the names of the conflicting domain to the given error reporting function
    ///
    /// B_MUST_BE_SUBTYPE means that a value of type "b" must be able to be assigned to a variable of type "a"
    ///
    /// The error function is called with None as the first argument if it's generative. A bit hacky but eh
    pub fn combine_domains<const B_MUST_BE_SUBTYPE: bool, ErrFunc: FnOnce(Option<&str>, &str)>(
        &self,
        a: &DomainType,
        b: &DomainType,
        err_func: ErrFunc,
    ) -> DomainType {
        match (a, b) {
            (DomainType::Physical(wa), DomainType::Physical(wb)) => {
                self.try_merge_physical(*wa, *wb, |l, r| err_func(Some(l), r));
                DomainType::Physical(*wa)
            }
            (DomainType::Generative, DomainType::Physical(w)) => {
                if B_MUST_BE_SUBTYPE {
                    let root = self.get_root_domain(*w);
                    let domains_borrow = self.domain_substitutor.borrow();

                    let other_domain = match &domains_borrow[root] {
                        DomainTypeSubstitution::KnownDomain { name } => &name,
                        DomainTypeSubstitution::UnknownDomain { converts_to: _ } => {
                            "as-of-yet-unknown"
                        }
                        DomainTypeSubstitution::AliasFor(_) => unreachable!(),
                    };
                    err_func(None, other_domain);
                }
                DomainType::Physical(*w)
            }
            (DomainType::Physical(w), DomainType::Generative) => DomainType::Physical(*w),
            (DomainType::Generative, DomainType::Generative) => DomainType::Generative,
        }
    }

    // ===== Both =====

    /// Unify the given full type [found] with the expected type [expected]. 
    pub fn typecheck_and_generative<const MUST_BE_GENERATIVE: bool>(
        &self,
        found: &FullType,
        span: Span,
        expected: &AbstractType,
        context: &str,
        declared_here: Option<SpanFile>,
    ) {
        self.typecheck_abstr(&found.typ, span, &expected, context, declared_here);

        if MUST_BE_GENERATIVE && found.domain != DomainType::Generative {
            let err_ref = self
                .errors
                .error(span, format!("A generative value is required in {context}"));
            if let Some(span_file) = declared_here {
                err_ref.info(span_file, "Declared here");
            }
        }
    }

    pub fn typecheck_write_to(
        &self,
        found: &FullType,
        span: Span,
        expected: &FullType,
        context: &str,
        declared_here: Option<SpanFile>,
    ) {
        self.typecheck_abstr(&found.typ, span, &expected.typ, context, declared_here);

        self.combine_domains::<true, _>(&expected.domain, &found.domain, |expected_domain, found_domain| {
            let err_text = if let Some(expected_domain_is_physical) = expected_domain {
                format!("Cannot write to a wire in domain '{expected_domain_is_physical}' from a wire in domain '{found_domain}'")
            } else { // Expected is generative
                format!("Cannot write to a generative wire from a non-generative wire in domain '{found_domain}'")
            };
            let err_ref = self.errors.error(span, err_text);
            if let Some(declared_here) = declared_here {
                err_ref.info(declared_here, "Declared here");
            }
        });
    }
    pub fn typecheck_unary_operator(
        &self,
        op: UnaryOperator,
        input_typ: &FullType,
        span: Span,
    ) -> FullType {
        FullType {
            typ: self.typecheck_unary_operator_abstr(op, &input_typ.typ, span),
            domain: input_typ.domain,
        }
    }

    pub fn typecheck_binary_operator(
        &self,
        op: BinaryOperator,
        left_typ: &FullType,
        right_typ: &FullType,
        left_span: Span,
        right_span: Span,
    ) -> FullType {
        FullType {
            typ : self.typecheck_binary_operator_abstr(op, &left_typ.typ, &right_typ.typ, left_span, right_span),
            domain : self.combine_domains::<false, _>(&left_typ.domain, &right_typ.domain, |left_name, right_name| {
                let left_name = left_name.unwrap();
                self.errors.error(right_span, format!("Attempting to combine wires of different domains. The domain for this wire is '{right_name}' and the other is '{left_name}'"))
                    .info_same_file(left_span, format!("Other wire in domain '{left_name}'"));
            })
        }
    }

    pub fn typecheck_array_access(
        &self,
        arr_type: &FullType,
        arr_span: Span,
        idx_type: &FullType,
        idx_span: Span,
    ) -> FullType {
        self.typecheck_abstr(&idx_type.typ, idx_span, &INT_TYPE, "array index", None);
        FullType {
            typ : self.typecheck_is_array_abstr(&arr_type.typ, arr_span),
            domain : self.combine_domains::<false, _>(&arr_type.domain, &idx_type.domain, |arr_domain, idx_domain| {
                let arr_domain = arr_domain.unwrap();
                self.errors.error(idx_span, format!("Attempting to index into an array of a different domain. The domain for this index is '{idx_domain}' but the array is '{arr_domain}'"));
            })
        }
    }

    /// At this point, the only elements in self.domains that aren't [DomainTypeSubstitution::AliasFor] are unique, and each represent their domain
    pub fn finalize_domain(&mut self, w: DomainID, best_name: BestName) -> DomainID {
        let root_dom = self.get_root_domain(w);
        match &mut self.domain_substitutor.get_mut()[root_dom] {
            DomainTypeSubstitution::KnownDomain { name: _ } => root_dom,
            DomainTypeSubstitution::UnknownDomain { converts_to } => {
                if *converts_to == DomainID::PLACEHOLDER {
                    *converts_to = self.final_domains.alloc(best_name);
                } else {
                    let found_name = &mut self.final_domains[*converts_to];
                    if best_name.strength() > found_name.strength() {
                        *found_name = best_name;
                    }
                }
                *converts_to
            }
            DomainTypeSubstitution::AliasFor(_) => unreachable!(),
        }
    }

    pub fn finalize_type(&mut self, typ: &mut FullType, span: Span, best_name: BestName) {
        match &mut typ.domain {
            DomainType::Generative => {}
            DomainType::Physical(w) => {
                assert!(*w != DomainID::PLACEHOLDER);
                let root = self.finalize_domain(*w, best_name);
                *w = root;
            }
        }
        if typ.typ.contains_error_or_unknown::<true, true>() {
            self.errors.error(
                span,
                format!(
                    "Unresolved Type: {}",
                    typ.typ
                        .to_string(&self.linker_types, &self.template_type_names)
                ),
            );
        }
    }
}

impl<'linker, 'errs> std::fmt::Debug for TypeUnifier<'linker, 'errs> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TypeUnifier::domains\n")?;
        let domains_borrow = self.domain_substitutor.borrow();
        for (id, d) in domains_borrow.iter() {
            id.fmt(f)?;
            f.write_str(" -> ")?;
            d.fmt(f)?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}
