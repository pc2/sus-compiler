use std::ops::{Deref, Index};

use crate::{errors::ErrorCollector, file_position::{Span, SpanFile}, flattening::{BinaryOperator, InterfaceID, Module, UnaryOperator}, linker::{get_builtin_type, Linkable, NamedType, Resolver, TypeUUID, TypeUUIDMarker}};

/// This contains only the information that can be easily type-checked. 
/// 
/// Its most important components are the names and structure of types. 
/// 
/// What isn't included are the parameters of types. So Array Sizes for example. 
#[derive(Debug, Clone)]
pub enum AbstractType {
    Error,
    Unknown,
    Named(TypeUUID),
    Array(Box<AbstractType>)
}

impl AbstractType {
    pub fn to_string<TypVec : Index<TypeUUID, Output = NamedType>>(&self, linker_types : &TypVec) -> String {
        match self {
            AbstractType::Error => {
                "{error}".to_owned()
            }
            AbstractType::Unknown => {
                "{unknown}".to_owned()
            }
            AbstractType::Named(id) => {
                linker_types[*id].get_full_name()
            }
            AbstractType::Array(sub) => sub.deref().to_string(linker_types) + "[]",
        }
    }
    pub fn contains_error_or_unknown<const CHECK_ERROR : bool, const CHECK_UNKNOWN : bool>(&self) -> bool {
        match self {
            AbstractType::Error => CHECK_ERROR,
            AbstractType::Unknown => CHECK_UNKNOWN,
            AbstractType::Named(_id) => false,
            AbstractType::Array(arr_box) => {
                arr_box.deref().contains_error_or_unknown::<CHECK_ERROR, CHECK_UNKNOWN>()
            }
        }
    }
}


pub const BOOL_TYPE : AbstractType = AbstractType::Named(get_builtin_type("bool"));
pub const INT_TYPE : AbstractType = AbstractType::Named(get_builtin_type("int"));

///     Error
///       |
///   Generative
///       | 
///     Wire*
///       |
///    Unknown
#[derive(Debug, Clone, Copy)]
pub enum DomainType {
    Generative,
    Unknown,
    Error,
    Wire(InterfaceID)
}

impl DomainType {
    pub fn contains_error_or_unknown<const CHECK_ERROR : bool, const CHECK_UNKNOWN : bool>(&self) -> bool {
        match self {
            DomainType::Generative | DomainType::Wire(_) => false,
            DomainType::Unknown => CHECK_UNKNOWN,
            DomainType::Error => CHECK_ERROR,
        }
    }

    fn combine_domains<ErrFunc : FnOnce(&DomainType, &DomainType)>(a : &DomainType, b : &DomainType, err_func : ErrFunc) -> DomainType {
        match (a, b) {
            (DomainType::Error, _) | (_, DomainType::Error) => DomainType::Error,
            (DomainType::Wire(wa), DomainType::Wire(wb)) => {
                if *wa == *wb {
                    DomainType::Wire(*wa)
                } else {
                    err_func(a, b);
                    DomainType::Error
                }
            }
            (DomainType::Generative, DomainType::Wire(w)) | (DomainType::Wire(w), DomainType::Generative) => {
                DomainType::Wire(*w)
            }
            (DomainType::Generative, DomainType::Generative) => {
                DomainType::Generative
            }
            (DomainType::Unknown, o) | (o, DomainType::Unknown) => {
                *o
            }
        }
    }

    /// Basically, if `this.is_subtype(other)`, then a value of type "this" can be assigned to a variable of type "other"
    fn is_subtype(&self, to_assign_to : &DomainType) -> bool {
        match (to_assign_to, self) {
            (_, DomainType::Error) | (DomainType::Error, _) => true, // Only report errors once
            (DomainType::Unknown, _) => true, // todo!("Assign to may not be to DomainType::Unknown, at least while I don't have type elision: {to_assign_to:?} <- {self:?}"),
            (_, DomainType::Unknown) => todo!("This unknown should have been resolved beforehand. {to_assign_to:?} <- {self:?}"),
            (DomainType::Wire(wa), DomainType::Wire(wb)) => *wa == *wb,
            (DomainType::Generative, DomainType::Generative) => true,
            (DomainType::Generative, DomainType::Wire(_)) => false,
            (DomainType::Wire(_), DomainType::Generative) => true, // Generative values can be converted to runtime wires
        }
    }
}

#[derive(Debug, Clone)]
pub struct FullType {
    pub typ : AbstractType,
    pub domain : DomainType
}

impl FullType {
    pub fn new_unknown_interface(typ : AbstractType, is_generative : bool) -> FullType {
        FullType {
            typ,
            domain: if is_generative {DomainType::Generative} else {DomainType::Wire(Module::MAIN_INTERFACE_ID)}, // TODO this is retrofitting single domains into the new abstract types. 
        }
    }
    pub fn new_unknown() -> FullType {
        FullType {
            typ : AbstractType::Unknown,
            domain: DomainType::Unknown,
        }
    }
    pub fn is_generative(&self) -> bool {
        match self.domain {
            DomainType::Generative => true,
            DomainType::Unknown | DomainType::Error => unreachable!("Unresolved InterfaceType"),
            DomainType::Wire(_) => false,
        }
    }
    pub fn to_string<TypVec : Index<TypeUUID, Output = NamedType>>(&self, linker_types : &TypVec) -> String {
        self.typ.to_string(linker_types)
    }

    pub fn contains_error_or_unknown<const CHECK_ERROR : bool, const CHECK_UNKNOWN : bool>(&self) -> bool {
        self.typ.contains_error_or_unknown::<CHECK_ERROR, CHECK_UNKNOWN>() || self.domain.contains_error_or_unknown::<CHECK_ERROR, CHECK_UNKNOWN>()
    }
}

/// TODO Type Unification - Hindley-Milner
pub struct TypeUnifier<'linker, 'errs : 'linker> {
    pub linker_types : Resolver<'linker, 'errs, TypeUUIDMarker, NamedType>,
    pub errors : &'errs ErrorCollector<'linker>,
}

impl<'linker, 'errs : 'linker> TypeUnifier<'linker, 'errs> {
    fn type_compare(&self, expected : &AbstractType, found : &AbstractType) -> bool {
        match (expected, found) {
            (AbstractType::Named(exp), AbstractType::Named(fnd)) => exp == fnd,
            (AbstractType::Array(exp), AbstractType::Array(fnd)) => {
                self.type_compare(&exp.deref(), &fnd.deref())
            }
            (AbstractType::Error, _) | (_, AbstractType::Error) => true, // Just assume correct, because the other side has an error
            (AbstractType::Unknown, _) | (_, AbstractType::Unknown) => todo!("Type Unification"),
            _ => false,
        }
    }

    pub fn typecheck_abstr(&self, found : &AbstractType, span : Span, expected : &AbstractType, context : &str, declared_here : Option<SpanFile>) {
        if !self.type_compare(expected, found) {
            let expected_name = expected.to_string(&self.linker_types);
            let found_name = found.to_string(&self.linker_types);
            let err_ref = self.errors.error(span, format!("Typing Error: {context} expects a {expected_name} but was given a {found_name}"));
            if let Some(declared_here) = declared_here {
                err_ref.info(declared_here, "Declared here");
            }
            assert!(expected_name != found_name, "{expected_name} != {found_name}");
        }
    }

    pub fn typecheck_unary_operator_abstr(&self, op : UnaryOperator, input_typ : &AbstractType, span : Span) -> AbstractType {
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
                _ => unreachable!()
            };
            let arr_content_typ = self.typecheck_is_array_abstr(input_typ, span);
            self.typecheck_abstr(&arr_content_typ, span, &gather_type, &format!("{op} input"), None);

            gather_type
        }
    }

    pub fn typecheck_binary_operator_abstr(&self, op : BinaryOperator, left_typ : &AbstractType, right_typ : &AbstractType, left_span : Span, right_span : Span) -> AbstractType {
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

        self.typecheck_abstr(left_typ, left_span, &exp_left, &format!("{op} left side"), None);
        self.typecheck_abstr(right_typ, right_span, &exp_right, &format!("{op} right side"), None);

        out
    }

    pub fn typecheck_is_array_abstr(&self, arr_type : &AbstractType, arr_span : Span) -> AbstractType {
        let AbstractType::Array(arr_element_type) = arr_type else {
            let arr_type_name = arr_type.to_string(&self.linker_types);
            self.errors.error(arr_span, format!("Typing Error: Attempting to index into this, but it is not of array type, instead found a {arr_type_name}"));
            return AbstractType::Error;
        };
        arr_element_type.deref().clone()
    }

    pub fn typecheck(&self, found : &FullType, span : Span, expected : &FullType, context : &str, declared_here : Option<SpanFile>) {
        self.typecheck_abstr(&found.typ, span, &expected.typ, context, declared_here);

        if !found.domain.is_subtype(&expected.domain) {
            let err_ref = self.errors.error(span, format!("Cannot use {} where {} is expected", found.to_string(&self.linker_types), expected.to_string(&self.linker_types)));
            if let Some(declared_here) = declared_here {
                err_ref.info(declared_here, "Declared here");
            }
        }
    }
    pub fn typecheck_unary_operator(&self, op : UnaryOperator, input_typ : &FullType, span : Span) -> FullType {
        FullType {
            typ : self.typecheck_unary_operator_abstr(op, &input_typ.typ, span),
            domain : input_typ.domain
        }
    }

    pub fn typecheck_binary_operator(&self, op : BinaryOperator, left_typ : &FullType, right_typ : &FullType, left_span : Span, right_span : Span) -> FullType {
        FullType {
            typ : self.typecheck_binary_operator_abstr(op, &left_typ.typ, &right_typ.typ, left_span, right_span),
            domain : DomainType::combine_domains(&left_typ.domain, &right_typ.domain, |_, _| {
                self.errors.error(right_span, "Attempting to combine wires of different domains")
                    .info_same_file(left_span, "Other wire");
                todo!("Name Domains")
            })
        }
    }

    pub fn typecheck_array_access(&self, arr_type : &FullType, arr_span : Span, idx_type : &FullType, idx_span : Span) -> FullType {
        self.typecheck_abstr(&idx_type.typ, idx_span, &INT_TYPE, "array index", None);
        FullType {
            typ : self.typecheck_is_array_abstr(&arr_type.typ, arr_span),
            domain : DomainType::combine_domains(&arr_type.domain, &idx_type.domain, |l, r| {
                self.errors.error(idx_span, "Attempting to index into an array of a different domain");
                todo!("Name Domains")
            })
        }
    }
}
