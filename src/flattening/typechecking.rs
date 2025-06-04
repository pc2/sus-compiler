use std::ops::Index;

use crate::alloc::{zip_eq, ArenaAllocator};
use crate::errors::{ErrorInfo, ErrorInfoObject, FileKnowingErrorInfoObject};
use crate::prelude::*;
use crate::typing::abstract_type::{AbstractInnerType, AbstractRankedType};
use crate::typing::type_inference::{AbstractTypeSubstitutor, FailedUnification, Substitutor};

use crate::debug::SpanDebugger;
use crate::linker::{FileData, GlobalResolver, GlobalUUID, AFTER_TYPECHECK_CP};

use crate::typing::written_type::WrittenType;
use crate::typing::{
    abstract_type::{FullTypeUnifier, BOOL_TYPE, INT_TYPE},
    template::TemplateKind,
};

use super::*;

pub fn typecheck_all_modules(linker: &mut Linker) {
    let module_uuids: Vec<ModuleUUID> = linker.modules.iter().map(|(id, _md)| id).collect();
    for module_uuid in module_uuids {
        let working_on_mut = &mut linker.modules[module_uuid];
        let errs_globals = working_on_mut.link_info.take_errors_globals();
        let type_alloc = *working_on_mut.link_info.type_variable_alloc.take().unwrap();

        let working_on: &Module = &linker.modules[module_uuid];
        let globals = GlobalResolver::new(linker, &working_on.link_info, errs_globals);

        println!("Typechecking {}", &working_on.link_info.name);
        let _panic_guard = SpanDebugger::new(
            "Typechecking",
            &working_on.link_info.name,
            &linker.files[working_on.link_info.file],
        );

        let mut context = TypeCheckingContext {
            globals: &globals,
            errors: &globals.errors,
            type_checker: FullTypeUnifier::new(
                &working_on.link_info.template_parameters,
                type_alloc,
            ),
            working_on: &working_on.link_info,
        };

        context.typecheck();

        let type_checker = context.type_checker;
        let (errs, globals) = globals.decommission();
        let errors = ErrorCollector::from_storage(errs, working_on.link_info.file, &linker.files);

        // Grab another mutable copy of md so it doesn't force a borrow conflict
        let working_on_mut = &mut linker.modules[module_uuid];
        let finalize_ctx = FinalizationContext {
            linker_types: &linker.types,
            errors: &errors,
            type_checker,
        };
        finalize_ctx.apply_types(working_on_mut);

        working_on_mut
            .link_info
            .reabsorb_errors_globals((errors.into_storage(), globals), AFTER_TYPECHECK_CP);

        // Also create the inference info now.
        working_on_mut.latency_inference_info = PortLatencyInferenceInfo::make(
            &working_on_mut.ports,
            &working_on_mut.link_info.instructions,
            working_on_mut.link_info.template_parameters.len(),
        );

        if crate::debug::is_enabled("print-flattened") {
            working_on_mut.print_flattened_module(&linker.files[working_on_mut.link_info.file]);
        }
    }
}

/*struct TypeCheckInitializationContext<'l, 'instrs> {
    linker: &'l Linker,
    errors: ErrorCollector<'l>,
    type_substitutor: TypeUnifier<AbstractTypeSubstitutor>,
    domain_substitutor: TypeUnifier<TypeSubstitutor<DomainType>>,
    instructions: IndexExcept<'instrs, FlatID, Instruction, FlatAlloc<Instruction, FlatIDMarker>>,
}

type InstructionList<'i> =
    IndexExcept<'i, FlatID, Instruction, FlatAlloc<Instruction, FlatIDMarker>>;

fn initialize_typecheck<'l>(
    linker: &'l Linker,
    mut errors: ErrorCollector<'l>,
    instructions: &mut FlatAlloc<Instruction, FlatIDMarker>,
) -> (ErrorCollector<'l>, TypeUnifier<AbstractTypeSubstitutor>) {
    let mut type_substitutor = TypeUnifier::default();
    let mut domain_substitutor = TypeUnifier::default();

    for (instr, instructions) in instructions.iter_mut_convenient() {
        let mut ctx = TypeCheckInitializationContext {
            linker,
            errors,
            type_substitutor,
            domain_substitutor,
            instructions,
        };
        ctx.initialize_instr(instr);
        errors = ctx.errors;
        type_substitutor = ctx.type_substitutor;
        domain_substitutor = ctx.domain_substitutor;
    }

    (errors, type_substitutor)
}

impl<'l, 'instrs> TypeCheckInitializationContext<'l, 'instrs> {
    fn initialize_instr(&mut self, instr: &mut Instruction) {
        match instr {
            Instruction::Declaration(declaration) => {
                declaration.typ_expr.for_each_generative_input(&mut |id| {
                    self.must_be_generative(id, "Type Parameter")
                });

                self.written_type_must_be_generative(&declaration.typ_expr);
                let (inner, rank) =
                    self.written_to_abstract_type_allow_modules(&declaration.typ_expr);

                declaration.meaning = match inner {
                    SubModuleOrWire::SubModule(submodule_info) => {
                        let md = &self.linker.modules[submodule_info.id];
                        SubModuleOrWire::SubModule(SubModuleDeclaration {
                            local_interface_domains: md
                                .domains
                                .map(|_| self.domain_substitutor.alloc_unknown()),
                            submodule_info,
                        })
                    }
                    SubModuleOrWire::Wire(inner) => {
                        let domain = match declaration.identifier_type {
                            IdentifierType::Local | IdentifierType::State => {
                                if let DeclarationKind::RegularPort { domain, .. } =
                                    declaration.decl_kind
                                {
                                    DomainType::Physical(domain)
                                } else {
                                    self.domain_substitutor.alloc_unknown()
                                }
                            }
                            IdentifierType::Generative => DomainType::Generative,
                        };
                        SubModuleOrWire::Wire(FullType {
                            typ: AbstractRankedType { inner, rank },
                            domain,
                        })
                    }
                }
            }
            Instruction::Expression(expression) => {
                let mut total_domain = DomainType::Generative;

                let mut total_domain = match &mut expression.source {
                    ExpressionSource::FuncCall(FuncCall {
                        interface_reference: wire_ref,
                        ..
                    })
                    | ExpressionSource::WireRef(wire_ref) => {
                        self.initialize_wire_ref(wire_ref);
                    }
                    _ => DomainType::Generative,
                };

                expression.source.for_each_input_wire(&mut |id| {
                    self.instructions[id].unwrap_subexpression().domain
                });

                total_domain.combine_with(other);
            }
        }
    }

    fn initialize_wire_ref(&mut self, wire_ref: &mut WireReference) {}

    pub fn written_to_abstract_type(&mut self, wr_typ: &WrittenType) -> AbstractRankedType {
        match wr_typ {
            WrittenType::Error(_span) => self.type_substitutor.alloc_unknown(),
            WrittenType::TemplateVariable(_span, argument_id) => {
                AbstractInnerType::Template(*argument_id).scalar()
            }
            WrittenType::Named(global_reference) => {
                AbstractInnerType::Named(global_reference.id).scalar()
            }
            WrittenType::Array(_span, array_content_and_size) => {
                let (arr_content_type, _size_flat, _array_bracket_span) =
                    array_content_and_size.deref();

                let content = self.written_to_abstract_type(arr_content_type);

                content.rank_up()
            }
        }
    }
    fn written_template_args_to_abstract_type(
        &mut self,
        link_info: &LinkInfo,
        args: &[WrittenTemplateArg],
    ) -> TVec<AbstractRankedType> {
        link_info.template_parameters.map(|(template_id, param)| {
            let found_arg = args
                .iter()
                .find_map(|arg| (arg.refers_to == Some(template_id)).then_some(arg));
            match found_arg {
                Some(WrittenTemplateArg {
                    kind: Some(TemplateKind::Type(wr_typ)),
                    ..
                }) => self.written_to_abstract_type(wr_typ),
                Some(WrittenTemplateArg {
                    kind: Some(TemplateKind::Value(sub_expr)),
                    ..
                }) => self.instructions[*sub_expr]
                    .unwrap_subexpression()
                    .typ
                    .clone(),
                _ => self.type_substitutor.alloc_unknown(),
            }
        })
    }
}*/

#[derive(Clone, Copy)]
pub struct RemoteSubModule<'l> {
    pub submodule: &'l SubModuleInstance,
    pub md: &'l Module,
}
impl<'l> RemoteSubModule<'l> {
    pub fn make(
        submodule_instr: FlatID,
        instructions: &'l impl Index<FlatID, Output = Instruction>,
        modules: &'l impl Index<ModuleUUID, Output = Module>,
    ) -> Self {
        let submodule = instructions[submodule_instr].unwrap_submodule();
        Self {
            submodule,
            md: &modules[submodule.module_ref.id],
        }
    }

    pub fn get_port(self, port_id: PortID) -> RemotePort<'l> {
        RemotePort {
            parent: self,
            port: &self.md.ports[port_id],
            remote_decl: self.md.get_port_decl(port_id),
            file: self.md.link_info.file,
        }
    }
    pub fn get_interface_reference(self, interface_id: InterfaceID) -> RemoteInterface<'l> {
        let interface = &self.md.interfaces[interface_id];
        RemoteInterface {
            parent: self,
            interface,
        }
    }
}
#[derive(Clone, Copy)]
pub struct RemoteInterface<'l> {
    parent: RemoteSubModule<'l>,
    interface: &'l Interface,
}
impl<'l> RemoteInterface<'l> {
    pub fn get_port(self, port_id: PortID) -> RemotePort<'l> {
        self.parent.get_port(port_id)
    }
    pub fn get_local_domain(self) -> DomainType {
        self.parent.submodule.local_interface_domains.get().unwrap()[self.interface.domain]
    }
}
/// For interfaces of this module
impl FileKnowingErrorInfoObject for RemoteInterface<'_> {
    fn make_global_info(&self, _files: &ArenaAllocator<FileData, FileUUIDMarker>) -> ErrorInfo {
        ErrorInfo {
            position: self.interface.name_span,
            file: self.parent.md.link_info.file,
            info: format!("Interface '{}' defined here", &self.interface.name),
        }
    }
}

#[derive(Clone, Copy)]
pub struct RemotePort<'l> {
    parent: RemoteSubModule<'l>,
    port: &'l Port,
    remote_decl: &'l Declaration,
    file: FileUUID,
}
impl<'l> RemotePort<'l> {
    pub fn get_local_type(&self, type_checker: &mut AbstractTypeSubstitutor) -> AbstractRankedType {
        type_checker.written_to_abstract_type_substitute_templates(
            &self.remote_decl.typ_expr,
            &self.parent.submodule.module_ref.template_arg_types,
        )
    }
    pub fn get_local_domain(&self) -> DomainType {
        self.parent.submodule.local_interface_domains.get().unwrap()[self.port.domain]
    }
    pub fn make_info(&self) -> ErrorInfo {
        self.remote_decl.make_info(self.file).unwrap()
    }
    pub fn is_input(&self) -> bool {
        self.remote_decl.decl_kind.is_io_port().unwrap()
    }
}
impl FileKnowingErrorInfoObject for RemotePort<'_> {
    fn make_global_info(&self, _files: &ArenaAllocator<FileData, FileUUIDMarker>) -> ErrorInfo {
        self.make_info()
    }
}

struct TypeCheckingContext<'l, 'errs> {
    globals: &'l GlobalResolver<'l>,
    errors: &'errs ErrorCollector<'l>,
    working_on: &'l LinkInfo,
    type_checker: FullTypeUnifier,
}

impl<'l> TypeCheckingContext<'l, '_> {
    fn typecheck_wire_reference(&mut self, wire_ref: &WireReference) {
        match &wire_ref.root {
            WireReferenceRoot::LocalDecl(_uuid) => {
                // When the decl was flattened, we set wire_ref.root_typ to decl.typ
            }
            WireReferenceRoot::NamedConstant(cst) => {
                self.typecheck_template_global(cst);

                let linker_cst = &self.globals[cst.id];
                let decl =
                    linker_cst.link_info.instructions[linker_cst.output_decl].unwrap_declaration();
                let typ = self
                    .type_checker
                    .abstract_type_substitutor
                    .written_to_abstract_type_substitute_templates(
                        &decl.typ_expr,
                        &cst.template_arg_types,
                    );
                self.type_checker
                    .abstract_type_substitutor
                    .unify_must_succeed(&wire_ref.root_typ, &typ);
            }
            WireReferenceRoot::SubModulePort(port) => {
                let submod_port = RemoteSubModule::make(
                    port.submodule_decl,
                    &self.working_on.instructions,
                    self.globals,
                )
                .get_port(port.port);
                if submod_port.remote_decl.domain.get() == DomainType::Generative {
                    self.errors
                        .error(
                            wire_ref.root_span,
                            "Invalid Submodule port: It is marked as generative!",
                        )
                        .info_obj(&submod_port);
                }
                let submod_port_typ =
                    submod_port.get_local_type(&mut self.type_checker.abstract_type_substitutor);
                self.type_checker
                    .abstract_type_substitutor
                    .unify_must_succeed(&wire_ref.root_typ, &submod_port_typ);
            }
            WireReferenceRoot::Error => {}
        }

        let mut current_type_in_progress = &wire_ref.root_typ;
        for p in &wire_ref.path {
            match p {
                WireReferencePathElement::ArrayAccess {
                    idx,
                    bracket_span,
                    output_typ,
                } => {
                    let idx_expr = self.working_on.instructions[*idx].unwrap_subexpression();

                    let arr_span = bracket_span.outer_span();
                    self.type_checker
                        .abstract_type_substitutor
                        .unify_report_error(
                            idx_expr.typ,
                            &INT_TYPE.scalar(),
                            idx_expr.span,
                            "array index",
                        );

                    self.type_checker
                        .abstract_type_substitutor
                        .unify_report_error(
                            current_type_in_progress,
                            &output_typ.clone().rank_up(),
                            arr_span,
                            "array access",
                        );

                    current_type_in_progress = output_typ;
                }
            }
        }
    }

    fn typecheck_template_global<ID: Copy + Into<GlobalUUID>>(
        &mut self,
        global_ref: &GlobalReference<ID>,
    ) {
        let global_obj: GlobalUUID = global_ref.id.into();
        let target_link_info = self.globals.get_link_info(global_obj);

        for (_parameter_id, arg_typ, parameter) in zip_eq(
            &global_ref.template_arg_types,
            &target_link_info.template_parameters,
        ) {
            match &parameter.kind {
                TemplateKind::Type(_) => {} // Do nothing, nothing to unify with. Maybe in the future traits?
                TemplateKind::Value(parameter) => {
                    let decl = target_link_info.instructions[parameter.declaration_instruction]
                        .unwrap_declaration();

                    let param_required_typ = self
                        .type_checker
                        .abstract_type_substitutor
                        .written_to_abstract_type_substitute_templates(
                            &decl.typ_expr,
                            &global_ref.template_arg_types, // Yes that's right. We already must substitute the templates for type variables here
                        );

                    self.type_checker
                        .abstract_type_substitutor
                        .unify_must_succeed(arg_typ, &param_required_typ);
                }
            }
        }
    }

    /// Critically, this is different from [TypeUnifier::unify_with_written_type].
    /// That one unifies a given typ with the written type, without checking the written type.
    ///
    /// This function checks the written type itself.
    fn typecheck_written_type(&mut self, wr_typ: &WrittenType) {
        match wr_typ {
            WrittenType::Error(_) => {}
            WrittenType::TemplateVariable(_, _) => {}
            WrittenType::Named(global_ref) => {
                self.typecheck_template_global(global_ref);
            }
            WrittenType::Array(_, arr_box) => {
                let (content_typ, arr_idx, _bracket_span) = arr_box.deref();

                self.typecheck_written_type(content_typ);

                let idx_expr = self.working_on.instructions[*arr_idx].unwrap_subexpression();
                self.type_checker.unify_write_to_abstract(
                    idx_expr.typ,
                    &INT_TYPE.scalar(),
                    idx_expr.span,
                    "array size",
                );
            }
        }
    }

    fn typecheck_visit_latency_specifier(&mut self, lat_spec: Option<FlatID>) {
        if let Some(latency_spec) = lat_spec {
            let latency_specifier_expr =
                self.working_on.instructions[latency_spec].unwrap_subexpression();
            self.type_checker.unify_write_to_abstract(
                latency_specifier_expr.typ,
                &INT_TYPE.scalar(),
                latency_specifier_expr.span,
                "latency specifier",
            );
        }
    }

    fn report_errors_for_bad_function_call(
        &self,
        func_call: &FuncCall,
        interface: &RemoteInterface<'l>,
        whole_func_span: Span,
        mut to_spans_iter: impl ExactSizeIterator<Item = Span>,
    ) {
        let arg_count = func_call.arguments.len();
        let expected_arg_count = interface.interface.func_call_inputs.len();

        if arg_count != expected_arg_count {
            if arg_count > expected_arg_count {
                // Too many args, complain about excess args at the end
                let excess_args_span = Span::new_overarching(
                    self.working_on.instructions[func_call.arguments[expected_arg_count]]
                        .unwrap_expression()
                        .span,
                    self.working_on.instructions[*func_call.arguments.last().unwrap()]
                        .unwrap_expression()
                        .span,
                );

                self.errors
                    .error(excess_args_span, format!("Excess argument. Function takes {expected_arg_count} args, but {arg_count} were passed."))
                    .info_obj(interface);
            } else {
                // Too few args, mention missing argument names
                self.errors
                    .error(func_call.arguments_span.close_bracket(), format!("Too few arguments. Function takes {expected_arg_count} args, but {arg_count} were passed."))
                    .info_obj(interface);
            }
        }

        let num_func_outputs = interface.interface.func_call_outputs.len();
        let num_targets = to_spans_iter.size_hint().0;
        if num_targets != num_func_outputs {
            if num_targets > num_func_outputs {
                let start_span: Span = to_spans_iter.nth(num_func_outputs).unwrap();
                let mut end_span = start_span;
                if let Some(end) = to_spans_iter.last() {
                    end_span = end;
                }

                let excess_results_span = Span::new_overarching(start_span, end_span);
                self.errors
                    .error(excess_results_span, format!("Excess output targets. Function returns {num_func_outputs} results, but {num_targets} targets were given."))
                    .info_obj(interface);
            } else {
                self.errors
                    .error(whole_func_span, format!("Too few output targets. Function returns {num_func_outputs} results, but {num_targets} targets were given."))
                    .info_obj(interface);
            }
        }
    }

    fn typecheck_func_call(&mut self, func_call: &FuncCall) -> RemoteInterface<'l> {
        let interface = RemoteSubModule::make(
            func_call.interface_reference.submodule_decl,
            &self.working_on.instructions,
            self.globals,
        )
        .get_interface_reference(func_call.interface_reference.submodule_interface);

        for (port, arg) in
            std::iter::zip(interface.interface.func_call_inputs, &func_call.arguments)
        {
            let port_decl = interface.get_port(port);
            let port_type =
                port_decl.get_local_type(&mut self.type_checker.abstract_type_substitutor);

            // Typecheck the value with target type
            let from = self.working_on.instructions[*arg].unwrap_subexpression();

            self.type_checker
                .unify_write_to_abstract(from.typ, &port_type, from.span, || {
                    ("function argument".to_string(), vec![port_decl.make_info()])
                });
        }

        interface
    }

    fn typecheck_single_output_expr(&mut self, expr: SingleOutputExpression) {
        match expr.source {
            ExpressionSource::WireRef(wire_ref) => {
                self.typecheck_wire_reference(wire_ref);

                self.type_checker
                    .abstract_type_substitutor
                    .unify_report_error(
                        expr.typ,
                        wire_ref.get_output_typ(),
                        expr.span,
                        "reading from wire reference",
                    );
            }
            ExpressionSource::UnaryOp { op, rank, right } => {
                let right_expr = self.working_on.instructions[*right].unwrap_subexpression();
                self.type_checker.typecheck_unary_operator_abstr(
                    *op,
                    rank,
                    right_expr.typ,
                    right_expr.span,
                    expr.typ,
                );
            }
            ExpressionSource::BinaryOp {
                op,
                rank,
                left,
                right,
            } => {
                let left_expr = self.working_on.instructions[*left].unwrap_subexpression();
                let right_expr = self.working_on.instructions[*right].unwrap_subexpression();
                {
                    self.type_checker.typecheck_binary_operator_abstr(
                        *op,
                        rank,
                        left_expr.typ,
                        right_expr.typ,
                        left_expr.span,
                        right_expr.span,
                        expr.typ,
                    );
                }
            }
            ExpressionSource::FuncCall(func_call) => {
                let interface = self.typecheck_func_call(func_call);

                self.report_errors_for_bad_function_call(
                    func_call,
                    &interface,
                    expr.span,
                    std::iter::once(expr.span),
                );

                if let Some(first_output) = interface.interface.func_call_outputs.first() {
                    let port_decl = interface.get_port(first_output);
                    let port_type =
                        port_decl.get_local_type(&mut self.type_checker.abstract_type_substitutor);

                    self.type_checker.unify_write_to_abstract(
                        expr.typ,
                        &port_type,
                        expr.span,
                        "function call as expression",
                    );
                }
            }
            ExpressionSource::Constant(value) => {
                let var_typ = AbstractRankedType {
                    inner: AbstractInnerType::Named(value.get_type_id()),
                    rank: PeanoType::Zero,
                };
                self.type_checker
                    .unify_write_to_abstract(expr.typ, &var_typ, expr.span, "Constant");
            }
            ExpressionSource::ArrayConstruct(arr) => {
                for elem_id in arr {
                    let elem_expr = self.working_on.instructions[*elem_id].unwrap_subexpression();

                    self.type_checker
                        .abstract_type_substitutor
                        .unify_report_error(
                            expr.typ,
                            &elem_expr.typ.clone().rank_up(),
                            elem_expr.span,
                            "array access",
                        );
                }
            }
        };
    }
    fn typecheck_multi_output_expr(&mut self, expr: &Expression, multi_write: &[WriteTo]) {
        for wr in multi_write {
            self.typecheck_wire_reference(&wr.to);
        }
        match &expr.source {
            ExpressionSource::FuncCall(func_call) => {
                let interface = self.typecheck_func_call(func_call);

                self.report_errors_for_bad_function_call(
                    func_call,
                    &interface,
                    expr.span,
                    multi_write.iter().map(|v| v.to_span),
                );

                for (port, to) in std::iter::zip(interface.interface.func_call_outputs, multi_write)
                {
                    let port_decl = interface.get_port(port);
                    let port_type =
                        port_decl.get_local_type(&mut self.type_checker.abstract_type_substitutor);

                    self.type_checker.unify_write_to_abstract(
                        to.to.get_output_typ(),
                        &port_type,
                        to.to_span,
                        || ("function output".to_string(), vec![port_decl.make_info()]),
                    );
                }
            }
            ExpressionSource::WireRef(..)
            | ExpressionSource::UnaryOp { .. }
            | ExpressionSource::BinaryOp { .. }
            | ExpressionSource::ArrayConstruct(..)
            | ExpressionSource::Constant(..) => {
                if let Some(first_write) = multi_write.first() {
                    self.typecheck_single_output_expr(SingleOutputExpression {
                        typ: first_write.to.get_output_typ(),
                        domain: expr.domain.get(),
                        span: expr.span,
                        source: &expr.source,
                    });
                } else {
                    let sentinel = self.type_checker.abstract_type_substitutor.alloc_unknown();

                    self.typecheck_single_output_expr(SingleOutputExpression {
                        typ: &sentinel,
                        domain: expr.domain.get(),
                        span: expr.span,
                        source: &expr.source,
                    });
                }

                // Don't output errors for 0 outputs. See no errors on zero outputs (#79)
                if multi_write.len() > 1 {
                    self.errors.error(
                        expr.span,
                        format!(
                            "Non-function assignments must output exactly 1 output instead of {}",
                            multi_write.len()
                        ),
                    );
                }
            }
        }
    }

    fn typecheck_visit_instruction(&mut self, instr_id: FlatID) {
        match &self.working_on.instructions[instr_id] {
            Instruction::SubModule(sm) => {
                self.typecheck_template_global(&sm.module_ref);
            }
            Instruction::Declaration(decl) => {
                self.typecheck_visit_latency_specifier(decl.latency_specifier);

                self.typecheck_written_type(&decl.typ_expr);
            }
            Instruction::IfStatement(stm) => {
                let condition_expr =
                    &self.working_on.instructions[stm.condition].unwrap_subexpression();
                self.type_checker.unify_write_to_abstract(
                    condition_expr.typ,
                    &BOOL_TYPE.scalar(),
                    condition_expr.span,
                    "if statement condition",
                );
            }
            Instruction::ForStatement(stm) => {
                let loop_var = self.working_on.instructions[stm.loop_var_decl].unwrap_declaration();
                let start = self.working_on.instructions[stm.start].unwrap_subexpression();
                let end = self.working_on.instructions[stm.end].unwrap_subexpression();

                self.type_checker.unify_write_to_abstract(
                    start.typ,
                    &loop_var.typ,
                    start.span,
                    "for loop start",
                );
                self.type_checker.unify_write_to_abstract(
                    end.typ,
                    &loop_var.typ,
                    end.span,
                    "for loop end",
                );
            }
            Instruction::Expression(expr) => match &expr.output {
                ExpressionOutput::SubExpression(typ) => {
                    self.typecheck_single_output_expr(SingleOutputExpression {
                        typ,
                        domain: expr.domain.get(),
                        span: expr.span,
                        source: &expr.source,
                    });
                }
                ExpressionOutput::MultiWrite(write_tos) => {
                    self.typecheck_multi_output_expr(expr, write_tos);
                }
            },
        }
    }

    /// Should be followed up by a [apply_types] call to actually apply all the checked types.
    fn typecheck(&mut self) {
        for elem_id in self.working_on.instructions.id_range() {
            self.typecheck_visit_instruction(elem_id);
        }
    }
}

// ====== Free functions for actually applying the result of type checking ======

struct FinalizationContext<'l, 'errs> {
    linker_types: &'l ArenaAllocator<StructType, TypeUUIDMarker>,
    errors: &'errs ErrorCollector<'l>,
    type_checker: FullTypeUnifier,
}

impl FinalizationContext<'_, '_> {
    pub fn apply_types(mut self, working_on: &mut Module) {
        // Post type application. Solidify types and flag any remaining AbstractType::Unknown
        for (_id, inst) in working_on.link_info.instructions.iter_mut() {
            match inst {
                Instruction::Expression(expr) => {
                    match &mut expr.output {
                        ExpressionOutput::SubExpression(expr_typ) => {
                            self.finalize_abstract_type(expr_typ, expr.span);
                        }
                        ExpressionOutput::MultiWrite(write_tos) => {
                            for wr in write_tos {
                                self.finalize_wire_ref(&mut wr.to);
                            }
                        }
                    }
                    match &mut expr.source {
                        ExpressionSource::WireRef(wr) => {
                            self.finalize_wire_ref(wr);
                        }
                        ExpressionSource::UnaryOp { rank, .. }
                        | ExpressionSource::BinaryOp { rank, .. } => {
                            let _ = rank.fully_substitute(
                                &self.type_checker.abstract_type_substitutor.rank_substitutor,
                            ); // No need to report incomplete peano error, as one of the ports would have reported it
                        }
                        _ => {}
                    }
                }
                Instruction::Declaration(decl) => {
                    self.finalize_abstract_type(&mut decl.typ, decl.name_span)
                }
                // TODO Submodule domains may not be crossed either?
                Instruction::SubModule(sm) => {
                    self.finalize_global_ref(&mut sm.module_ref);
                }
                _other => {}
            }
        }

        // Print all errors
        for FailedUnification {
            mut found,
            mut expected,
            span,
            context,
            infos,
        } in self.type_checker.abstract_type_substitutor.extract_errors()
        {
            // Not being able to fully substitute is not an issue. We just display partial types
            let _ = found.fully_substitute(&self.type_checker.abstract_type_substitutor);
            let _ = expected.fully_substitute(&self.type_checker.abstract_type_substitutor);

            let expected_name = expected
                .display(self.linker_types, &self.type_checker.template_type_names)
                .to_string();
            let found_name = found
                .display(self.linker_types, &self.type_checker.template_type_names)
                .to_string();
            self.errors
            .error(span, format!("Typing Error: {context} expects a {expected_name} but was given a {found_name}"))
            .add_info_list(infos);

            assert_ne!(found, expected);

            /*assert!(
                expected_name != found_name,
                "{expected_name} != {found_name}"
            );*/
        }
    }

    pub fn finalize_abstract_type(&self, typ: &mut AbstractRankedType, span: Span) {
        if !typ.fully_substitute(&self.type_checker.abstract_type_substitutor) {
            self.errors.error(
                span,
                format!(
                    "Could not fully figure out the type of this object. {}",
                    typ.display(self.linker_types, &self.type_checker.template_type_names)
                ),
            );

            if crate::debug::is_enabled("TEST") {
                println!("COULD_NOT_FULLY_FIGURE_OUT")
            }
        }
    }

    pub fn finalize_global_ref<ID>(&self, global_ref: &mut GlobalReference<ID>) {
        let global_ref_span = global_ref.get_total_span();
        for (_template_id, arg) in &mut global_ref.template_arg_types {
            self.finalize_abstract_type(arg, global_ref_span);
        }
    }

    pub fn finalize_wire_ref(&self, wire_ref: &mut WireReference) {
        if let WireReferenceRoot::NamedConstant(cst) = &mut wire_ref.root {
            self.finalize_global_ref(cst);
        }
        self.finalize_abstract_type(&mut wire_ref.root_typ, wire_ref.root_span);
        for path_elem in &mut wire_ref.path {
            match path_elem {
                WireReferencePathElement::ArrayAccess {
                    output_typ,
                    bracket_span,
                    ..
                } => {
                    self.finalize_abstract_type(output_typ, bracket_span.outer_span());
                }
            }
        }
    }
}
