mod domain_check;
mod lints;
mod type_check;

use super::*;

pub use lints::perform_lints;
use typed_arena::Arena;

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
        unifyable_cell::{SubstituteRecurse, Unifier, UnifierTop, UnifyRecurse, UnifyResult},
    },
};

struct TypeCheckingContext<'l> {
    globals: GlobalResolver<'l, 'l>,
    errors: &'l ErrorCollector<'l>,
    link_info: &'l LinkInfo,
    instructions: &'l FlatAlloc<Instruction, FlatIDMarker>,
    domains: &'l FlatAlloc<ClockInfo, ClockIDMarker>,
    typ_alloc: &'l Arena<UniCell<AbstractInnerType>>,
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
        let result = self.unifier.unify_subtrees(found, expected);

        let expected = self.unifier.clone_known(expected);
        self.report_unify_error(found, expected, span, report, result);
    }
    pub fn set_type_report_error(
        &self,
        found: &'l AbstractRankedType,
        mut expected: AbstractRankedType,
        span: Span,
        report: impl UnifyErrorReport,
    ) {
        let result = self.unifier.set_subtrees(found, &mut expected);

        self.report_unify_error(found, expected, span, report, result);
    }
    pub fn report_unify_error(
        &self,
        found: &'l AbstractRankedType,
        expected: AbstractRankedType,
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
            unifier.fully_substitute_recurse(found);
            unifier.fully_substitute_recurse(&expected);

            let found = found.display(globals, link_info);
            let expected = expected.display(globals, link_info);

            let msg =
                format!("Typing Error: {context} expects '{expected}' but was given '{found}'");
            errors.error(span, msg).add_info_list(infos);

            assert!(expected.to_string() != found.to_string())
        });
    }
}

pub fn typecheck(pass: &mut LinkerPass, errors: &ErrorCollector) {
    let (working_on, globals) = pass.get_with_context();
    let link_info = working_on.get_link_info();
    let typ_alloc = Arena::new();
    let domains = if let GlobalObj::Module(md) = working_on {
        &md.clocks
    } else {
        &FlatAlloc::EMPTY_FLAT_ALLOC
    };
    let context = TypeCheckingContext {
        globals,
        errors,
        instructions: &link_info.instructions,
        link_info,
        domains,
        typ_alloc: &typ_alloc,
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
