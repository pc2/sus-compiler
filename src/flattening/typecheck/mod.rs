mod domain_check;
mod lints;
mod type_check;

use crate::{
    alloc::UUIDAllocator,
    linker::{GlobalResolver, GlobalUUID},
    typing::{
        template::Parameter,
        type_inference::{
            AbstractTypeSubstitutor, FailedUnification, TypeSubstitutor, TypeUnifier,
        },
    },
};

use super::*;

use std::{cell::OnceCell, ops::Deref};

pub use lints::perform_lints;
struct TypeCheckingContext<'l> {
    globals: &'l GlobalResolver<'l>,
    errors: &'l ErrorCollector<'l>,
    instructions: &'l FlatAlloc<Instruction, FlatIDMarker>,
    template_args: &'l TVec<Parameter>,
    type_checker: TypeUnifier<AbstractTypeSubstitutor>,
    domain_checker: TypeUnifier<TypeSubstitutor<DomainType>>,
}

struct FinalizationContext {
    substitution_failures: Vec<(AbstractRankedType, Span)>,
    type_checker: TypeUnifier<AbstractTypeSubstitutor>,
    domain_checker: TypeUnifier<TypeSubstitutor<DomainType>>,
}

pub fn typecheck_all(linker: &mut Linker, global_ids: &[GlobalUUID]) {
    for global_id in global_ids {
        let (type_checker, domain_checker) =
            linker.immutable_pass("Typechecking", *global_id, |working_on, errors, globals| {
                let mut context = TypeCheckingContext {
                    globals,
                    errors,
                    type_checker: TypeUnifier::from(AbstractTypeSubstitutor::default()),
                    domain_checker: TypeUnifier::default(),
                    instructions: &working_on.instructions,
                    template_args: &working_on.template_parameters,
                };

                for (_, instr) in context.instructions {
                    context.type_check_instr(instr);
                    context.domain_check_instr(instr);
                }

                (context.type_checker, context.domain_checker)
            });

        if let GlobalUUID::Module(md_id) = *global_id {
            let md = &mut linker.modules[md_id];
            // Set the remaining domain variables that aren't associated with a module port.
            // We just find domain IDs that haven't been
            let mut leftover_domain_alloc =
                UUIDAllocator::new_start_from(md.domains.get_next_alloc_id());
            for (_, d) in domain_checker.iter() {
                if d.get().is_none() {
                    assert!(d
                        .set(DomainType::Physical(leftover_domain_alloc.alloc()))
                        .is_ok());
                }
            }
        }

        let link_info = Linker::get_link_info_mut(
            &mut linker.modules,
            &mut linker.types,
            &mut linker.constants,
            *global_id,
        );
        // Grab another mutable copy of md so it doesn't force a borrow conflict
        let mut finalize_ctx = FinalizationContext {
            type_checker,
            domain_checker,
            substitution_failures: Vec::new(),
        };
        finalize_ctx.apply_types(&mut link_info.instructions);
        finalize_ctx.apply_domains(&mut link_info.instructions);

        linker.immutable_pass("Report Type Errors", *global_id, |link_info, errors, globals| {
            for FailedUnification {
                mut found,
                mut expected,
                span,
                context,
                infos,
            } in finalize_ctx.domain_checker.extract_errors()
            {
                let _ = found.fully_substitute(&finalize_ctx.domain_checker);
                let _ = expected.fully_substitute(&finalize_ctx.domain_checker);

                let expected_name = format!("{expected:?}");
                let found_name = format!("{found:?}");
                errors
                    .error(span, format!("Domain error: Attempting to combine domains {found_name} and {expected_name} in {context}"))
                    .add_info_list(infos);

                assert_ne!(found, expected);

                /*assert!(
                    expected_name != found_name,
                    "{expected_name} != {found_name}"
                );*/
            }
            // Print all errors
            for FailedUnification {
                mut found,
                mut expected,
                span,
                context,
                infos,
            } in finalize_ctx.type_checker.extract_errors()
            {
                // Not being able to fully substitute is not an issue. We just display partial types
                let _ = found.fully_substitute(&finalize_ctx.type_checker);
                let _ = expected.fully_substitute(&finalize_ctx.type_checker);

                let expected_name = expected
                    .display(globals.linker, &link_info.template_parameters)
                    .to_string();
                let found_name = found
                    .display(globals.linker, &link_info.template_parameters)
                    .to_string();
                errors
                    .error(span, format!("Typing Error: {context} expects a {expected_name} but was given a {found_name}"))
                    .add_info_list(infos);

                assert_ne!(found, expected);

                /*assert!(
                    expected_name != found_name,
                    "{expected_name} != {found_name}"
                );*/
            }

            // Skip printing not fully figured out types of there are type errors to reduce visual overhead. 
            if errors.did_error() {
                return;
            }
            for (typ, span) in finalize_ctx.substitution_failures {
                errors.error(
                    span,
                    format!(
                        "Could not fully figure out the type of this object. {}",
                        typ.display(globals.linker, &link_info.template_parameters)
                    ),
                );
            }
        });

        if let GlobalUUID::Module(md_id) = *global_id {
            let md = &mut linker.modules[md_id];
            // Also create the inference info now.
            md.latency_inference_info = PortLatencyInferenceInfo::make(
                &md.ports,
                &md.link_info.instructions,
                md.link_info.template_parameters.len(),
            );
            if crate::debug::is_enabled("print-flattened") {
                md.print_flattened_module(&linker.files[md.link_info.file]);
            }
        }
    }
}

/// Basically equivalent to [std::cell::OnceCell], but implements [std::ops::Deref] and automatically unwraps
/// This file defines a OnceCell variant for use with typechecking
///
/// Because in typechecking, we will always set it to uninitialized in Flatten, set it to an initial value (&self) in typechecking, and then finalize the type in (&mut self)
#[derive(Debug)]
pub struct TyCell<T>(OnceCell<T>);

impl<T: std::fmt::Debug> TyCell<T> {
    pub fn new() -> Self {
        Self::default()
    }
    #[track_caller]
    fn get_mut(&mut self) -> &mut T {
        self.0.get_mut().unwrap()
    }
    /// Private because only typechecking should be allowed to set TyCells
    #[track_caller]
    fn set(&self, v: T) {
        self.0.set(v).unwrap();
    }
}

impl<T> Default for TyCell<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Deref for TyCell<T> {
    type Target = T;

    #[track_caller]
    fn deref(&self) -> &Self::Target {
        self.0.get().expect("Deref on an unfinished TyCell!")
    }
}

/*
// This delegated IntoIterator impl causes infinite recursion due to a bug in rustc. https://github.com/rust-lang/rust/issues/106512
// Right now, just defer to .iter()
impl<'a, T> IntoIterator for &'a TyCell<T>
where
    &'a T: IntoIterator,
{
    type Item = <&'a T as IntoIterator>::Item; // NOTE diff
    type IntoIter = <&'a T as IntoIterator>::IntoIter; // NOTE diff
    fn into_iter(self) -> Self::IntoIter {
        self.0.get().unwrap().into_iter()
    }
}
*/
