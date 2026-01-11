mod domain_check;
mod lints;
mod type_check;

use super::*;

use bumpalo::Bump;
pub use lints::perform_lints;

use crate::{
    alloc::UUIDAllocator,
    errors::ErrorInfo,
    linker::{
        GlobalObj,
        passes::{GlobalResolver, LinkerPass},
    },
    typing::{
        abstract_type::AbstractInnerType,
        abstract_unifier::AbstractUnifier,
        unifyable_cell::{Unifier, UnifierTop, UnifyResult},
    },
};

struct TypeCheckingContext<'l> {
    globals: GlobalResolver<'l, 'l>,
    errors: &'l ErrorCollector<'l>,
    link_info: &'l LinkInfo,
    instructions: &'l FlatAlloc<Instruction, FlatIDMarker>,
    domains: &'l FlatAlloc<DomainInfo, DomainIDMarker>,
    extra_allocator: &'l Bump,
    unifier: AbstractUnifier<'l>,
}

/// To be passed to [TypeCheckingContext::unify_type_report_error]
pub trait UnifyErrorReport {
    fn report(self) -> (String, Vec<ErrorInfo>);
}
impl UnifyErrorReport for &str {
    fn report(self) -> (String, Vec<ErrorInfo>) {
        (self.to_string(), Vec::new())
    }
}
impl<F: FnOnce() -> (String, Vec<ErrorInfo>)> UnifyErrorReport for F {
    fn report(self) -> (String, Vec<ErrorInfo>) {
        self()
    }
}

impl<'l> TypeCheckingContext<'l> {
    pub fn unify_type_report_error(
        &self,
        found: &'l AbstractRankedType,
        expected: &'l AbstractRankedType,
        span: Span,
        report: impl UnifyErrorReport,
    ) {
        self.unify_type_parts_report_error(
            &found.inner,
            &found.rank,
            &expected.inner,
            &expected.rank,
            span,
            report,
        );
    }
    pub fn unify_type_parts_report_error(
        &self,
        found_inner: &'l UniCell<AbstractInnerType>,
        found_rank: &'l UniCell<PeanoType>,
        expected_inner: &'l UniCell<AbstractInnerType>,
        expected_rank: &'l UniCell<PeanoType>,
        span: Span,
        report: impl UnifyErrorReport,
    ) {
        let inner_result = self.unifier.unify(found_inner, expected_inner);
        let rank_result = self.unifier.unify(found_rank, expected_rank);

        let expected_inner = self.unifier.clone_unify(expected_inner);
        let expected_rank = self.unifier.clone_unify(expected_rank);
        self.report_unify_error(
            found_inner,
            found_rank,
            expected_inner,
            expected_rank,
            span,
            report,
            inner_result & rank_result,
        );
    }
    pub fn set_type_report_error(
        &self,
        found: &'l AbstractRankedType,
        expected: AbstractRankedType,
        span: Span,
        report: impl UnifyErrorReport,
    ) {
        self.set_type_parts_report_error(
            &found.inner,
            &found.rank,
            expected.inner,
            expected.rank,
            span,
            report,
        );
    }
    pub fn set_type_parts_report_error(
        &self,
        found_inner: &'l UniCell<AbstractInnerType>,
        found_rank: &'l UniCell<PeanoType>,
        mut expected_inner: UniCell<AbstractInnerType>,
        mut expected_rank: UniCell<PeanoType>,
        span: Span,
        report: impl UnifyErrorReport,
    ) {
        let inner_result = self.unifier.set(found_inner, &mut expected_inner);
        let rank_result = self.unifier.set(found_rank, &mut expected_rank);

        self.report_unify_error(
            found_inner,
            found_rank,
            expected_inner,
            expected_rank,
            span,
            report,
            inner_result & rank_result,
        );
    }
    pub fn report_unify_error(
        &self,
        found_inner: &'l UniCell<AbstractInnerType>,
        found_rank: &'l UniCell<PeanoType>,
        expected_inner: UniCell<AbstractInnerType>,
        expected_rank: UniCell<PeanoType>,
        span: Span,
        report: impl UnifyErrorReport,
        unify_result: UnifyResult,
    ) {
        if unify_result == UnifyResult::Success {
            return;
        }
        let globals = self.globals.globals;
        let errors = self.errors;
        let link_info = self.link_info;
        let (mut context, infos) = report.report();
        if unify_result == UnifyResult::FailureInfiniteTypes {
            context.push_str(": Creating Infinite Types is Forbidden!");
        }
        self.unifier.delayed_error(move |unifier| {
            unifier.fully_substitute(found_inner);
            unifier.fully_substitute(found_rank);
            unifier.fully_substitute(&expected_inner);
            unifier.fully_substitute(&expected_rank);

            let found_inner = found_inner.display(globals, link_info);
            let expected_inner = expected_inner.display(globals, link_info);

            errors
                .error(
                    span,
                    format!(
                        "Typing Error: {context} expects '{expected_inner}{expected_rank}' but was given '{found_inner}{found_rank}'"
                    ),
                )
                .add_info_list(infos);
                
            assert!(format!("{expected_inner}{expected_rank}") != format!("{found_inner}{found_rank}"))
        });
    }
}

pub fn typecheck(pass: &mut LinkerPass, errors: &ErrorCollector) {
    let (working_on, globals) = pass.get_with_context();
    let link_info = working_on.get_link_info();
    let extra_allocator = Bump::new();
    let domains = if let GlobalObj::Module(md) = working_on {
        &md.domains
    } else {
        &FlatAlloc::EMPTY_FLAT_ALLOC
    };
    let context = TypeCheckingContext {
        globals,
        errors,
        instructions: &link_info.instructions,
        link_info,
        domains,
        extra_allocator: &extra_allocator,
        unifier: AbstractUnifier::new(),
    };

    context.init_all_declarations();

    for (_, instr) in context.instructions {
        context.type_check_instr(instr);
        context.domain_check_instr(instr);
    }

    // This order is important, such that unknown domains get IDd here,
    // but the errors for incomplete types are reported *after* reporting the unification errors,
    // so we can choose not to report incomplete type errors.
    context.finalize_domains();
    context.unifier.report_delayed_errors();
    context.finalize_types();
    std::mem::drop(context);

    if let GlobalObj::Module(md) = pass.get_mut() {
        // Also create the inference info now.
        md.inference_info = PortLatencyInferenceInfo::make(
            &md.ports,
            &md.link_info.instructions,
            &md.link_info.parameters,
        );
    }
}
