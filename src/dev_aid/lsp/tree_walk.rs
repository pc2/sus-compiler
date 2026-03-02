use crate::{
    dev_aid::ariadne_interface::{pretty_print_many_spans, pretty_print_span},
    linker::LinkerFiles,
    prelude::*,
    util::contains_duplicates,
};

use std::ops::Deref;

use crate::{
    flattening::*,
    linker::{FileData, GlobalObj, GlobalUUID, LinkInfo},
    typing::template::{Parameter, TemplateKind, TypeParameterKind},
};

/// See [LocationInfo]
#[derive(Clone, Copy, Debug)]
pub enum LocationKind<'linker> {
    WireRefRoot(&'linker WireReferenceRoot),
    GlobalReference(MultiGlobalRef<'linker>),
    LocalDecl(FlatID),
    LocalInterface(FlatID),
    LocalSubmodule(FlatID),
    Field {
        #[allow(unused)]
        name: &'linker str,
        #[allow(unused)]
        name_span: Span,
        refers_to: Option<&'linker PathElemRefersTo>,
        #[allow(unused)]
        in_wire_ref: &'linker WireReference,
    },
    UsedTemplateArg(
        GlobalUUID,
        &'linker WrittenTemplateArg,
        MultiGlobalRef<'linker>,
    ),
    TypeTemplateParam(GlobalUUID, TemplateID),
    Global(GlobalUUID),
}

/// Until we remove the ID template from [GlobalReference], this is the stop-gap
pub type MultiGlobalRef<'linker> = GlobalObj<
    &'linker GlobalReference<ModuleUUID>,
    &'linker GlobalReference<TypeUUID>,
    &'linker GlobalReference<ConstantUUID>,
>;
impl<'linker> MultiGlobalRef<'linker> {
    /// (global, name_span, whole_span, template_args)
    pub fn get_global(&self) -> (GlobalUUID, Span, Span, &'linker [WrittenTemplateArg]) {
        match self {
            GlobalObj::Module(gl) => (
                GlobalObj::Module(gl.id),
                gl.name_span,
                gl.get_total_span(),
                &gl.template_args,
            ),
            GlobalObj::Type(gl) => (
                GlobalObj::Type(gl.id),
                gl.name_span,
                gl.get_total_span(),
                &gl.template_args,
            ),
            GlobalObj::Constant(gl) => (
                GlobalObj::Constant(gl.id),
                gl.name_span,
                gl.get_total_span(),
                &gl.template_args,
            ),
        }
    }
}

/// Information about an object in the source code. Used for hovering, completions, syntax highlighting etc.
#[derive(Debug, Clone, Copy)]
pub struct LocationInfo<'linker> {
    pub span: Span,
    pub kind: LocationKind<'linker>,
    pub in_global: Option<GlobalUUID>,
}

/// A unique representation of what an object refers to.
#[derive(Clone, Copy, Debug)]
pub enum RefersTo<'linker> {
    LocalDecl(GlobalUUID, &'linker Declaration, FlatID),
    LocalSubModule(GlobalUUID, &'linker SubModuleInstance, FlatID),
    Global(GlobalUUID),
    Field(GlobalUUID, &'linker Field),
    Parameter(GlobalUUID, &'linker Parameter),
}

impl<'linker> PartialEq for RefersTo<'linker> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::LocalDecl(_, l1, _), Self::LocalDecl(_, r1, _)) => std::ptr::eq(*l1, *r1),
            (Self::LocalSubModule(_, l1, _), Self::LocalSubModule(_, r1, _)) => {
                std::ptr::eq(*l1, *r1)
            }
            (Self::Global(l0), Self::Global(r0)) => *l0 == *r0,
            (Self::Field(_, l1), Self::Field(_, r1)) => std::ptr::eq(*l1, *r1),
            (Self::Parameter(_, l1), Self::Parameter(_, r1)) => std::ptr::eq(*l1, *r1),
            _ => false,
        }
    }
}
impl<'linker> Eq for RefersTo<'linker> {}

impl<'linker> LocationInfo<'linker> {
    fn refer_to_decl(
        linker: &'linker Linker,
        in_global: GlobalUUID,
        decl_id: FlatID,
    ) -> RefersTo<'linker> {
        let decl = linker.globals[in_global].instructions[decl_id].unwrap_declaration();
        match decl.decl_kind {
            DeclarationKind::Port {
                parent_field,
                is_standalone_port: true,
                ..
            } => RefersTo::Field(
                in_global,
                &linker.globals[in_global.unwrap_module()].fields[parent_field],
            ),
            DeclarationKind::TemplateParameter(template_id) => RefersTo::Parameter(
                in_global,
                &linker.globals[in_global].parameters[template_id],
            ),
            _ => RefersTo::LocalDecl(in_global, decl, decl_id),
        }
    }
    fn refer_to_submodule(
        linker: &'linker Linker,
        in_global: GlobalUUID,
        submodule_decl_id: FlatID,
    ) -> RefersTo<'linker> {
        let link_info = &linker.globals[in_global];
        let submodule_decl = link_info.instructions[submodule_decl_id].unwrap_submodule();
        RefersTo::LocalSubModule(in_global, submodule_decl, submodule_decl_id)
    }
    fn refer_to_interface(
        linker: &'linker Linker,
        in_global: GlobalUUID,
        interface_decl_id: FlatID,
    ) -> RefersTo<'linker> {
        let md = &linker.modules[in_global.unwrap_module()];
        let interface_decl = md.link_info.instructions[interface_decl_id].unwrap_interface();
        RefersTo::Field(in_global, &md.fields[interface_decl.field_id])
    }
    pub fn refers_to(&self, linker: &'linker Linker) -> Option<RefersTo<'linker>> {
        match self.kind {
            LocationKind::WireRefRoot(wr_root) => {
                let in_global = self.in_global.unwrap();
                let link_info = &linker.globals[in_global];

                match wr_root {
                    WireReferenceRoot::LocalDecl(id) => {
                        Some(Self::refer_to_decl(linker, in_global, *id))
                    }
                    WireReferenceRoot::LocalSubmodule(id) => Some(RefersTo::LocalSubModule(
                        in_global,
                        link_info.instructions[*id].unwrap_submodule(),
                        *id,
                    )),
                    WireReferenceRoot::LocalInterface(id) => {
                        Some(Self::refer_to_interface(linker, in_global, *id))
                    }
                    WireReferenceRoot::NamedConstant(_) | WireReferenceRoot::NamedModule(_) => {
                        // these will be covered by more specific [LocationKind::GlobalReference]
                        None
                    }
                    WireReferenceRoot::Error => None,
                }
            }
            LocationKind::LocalDecl(decl_id) => {
                let in_global = self.in_global.unwrap();
                Some(Self::refer_to_decl(linker, in_global, decl_id))
            }
            LocationKind::LocalSubmodule(submodule_decl_id) => {
                let in_global = self.in_global.unwrap();
                Some(Self::refer_to_submodule(
                    linker,
                    in_global,
                    submodule_decl_id,
                ))
            }
            LocationKind::LocalInterface(interface_decl_id) => {
                let in_global = self.in_global.unwrap();
                Some(Self::refer_to_interface(
                    linker,
                    in_global,
                    interface_decl_id,
                ))
            }
            LocationKind::UsedTemplateArg(obj, param, _in_global_ref) => {
                if let Some(param_refers_to) = param.refers_to.get() {
                    let param_obj = &linker.globals[obj];
                    Some(RefersTo::Parameter(
                        obj,
                        &param_obj.parameters[*param_refers_to],
                    ))
                } else {
                    None
                }
            }
            LocationKind::TypeTemplateParam(obj, param) => {
                let param_obj = &linker.globals[obj];
                Some(RefersTo::Parameter(obj, &param_obj.parameters[param]))
            }
            LocationKind::Field { refers_to, .. } => match refers_to {
                Some(PathElemRefersTo::Field(md_id, Some(field_id))) => {
                    let md = &linker.modules[*md_id];
                    let field = &md.fields[*field_id];

                    Some(RefersTo::Field(GlobalObj::Module(*md_id), field))
                }
                Some(PathElemRefersTo::Field(_, None)) | None => None,
            },
            LocationKind::GlobalReference(_) => None, // For a proper Global this will have been covered by LocationKind::Global
            LocationKind::Global(global) => Some(RefersTo::Global(global)),
        }
    }
}

impl<'linker> RefersTo<'linker> {
    /// Used to optimize search for references. If not global, then no need to search through the whole program.
    pub fn is_strictly_local(&self) -> Option<GlobalUUID> {
        match self {
            RefersTo::LocalDecl(in_obj, ..) | RefersTo::LocalSubModule(in_obj, ..) => Some(*in_obj),
            RefersTo::Global(..) | RefersTo::Field(..) | RefersTo::Parameter(..) => None,
        }
    }
}

/// Walks the file, and provides all [LocationInfo]s.
pub fn visit_all<'linker>(
    linker: &'linker Linker,
    file: &'linker FileData,
    visitor: &mut dyn FnMut(LocationInfo<'linker>),
) {
    let mut walker = TreeWalker {
        linker,
        visitor,
        should_prune: &mut |_| false,
    };

    walker.walk_file(file);
}

/// Walks the file, and provides all [LocationInfo]s.
pub fn visit_all_in_module<'linker>(
    linker: &'linker Linker,
    obj_id: GlobalUUID,
    visitor: &mut dyn FnMut(LocationInfo<'linker>),
) {
    let mut walker = TreeWalker {
        linker,
        visitor,
        should_prune: &mut |_| false,
    };

    walker.walk_global(obj_id);
}

fn check_for_duplicate_spans(linker_files: &LinkerFiles, spans: &[Span]) {
    if contains_duplicates(spans) {
        pretty_print_many_spans(
            linker_files,
            spans.iter().map(|sp| (*sp, format!("{sp:?}"))),
        );
        panic!("Duplicate spans in references found!")
    }
}

fn gather_all_references_in_global(
    linker: &Linker,
    obj_id: GlobalUUID,
    refers_to: RefersTo,
) -> Vec<Span> {
    let mut ref_locations = Vec::new();
    visit_all_in_module(linker, obj_id, &mut |info| {
        if let Some(info_refers_to) = info.refers_to(linker) {
            if info_refers_to == refers_to {
                ref_locations.push(info.span);
            }
        }
    });
    check_for_duplicate_spans(&linker.files, &ref_locations);
    ref_locations
}

fn gather_references_in_file(
    linker: &Linker,
    file_data: &FileData,
    refers_to: RefersTo,
) -> Vec<Span> {
    let mut ref_locations = Vec::new();
    visit_all(linker, file_data, &mut |info| {
        if let Some(found_ref) = info.refers_to(linker)
            && refers_to == found_ref
        {
            ref_locations.push(info.span);
        }
    });
    ref_locations
}

pub fn gather_all_references_in_one_file(
    linker: &Linker,
    file: &FileData,
    pos: usize,
) -> Vec<Span> {
    let Some(hover_info) = get_selected_object(linker, file, pos) else {
        return Vec::new();
    };
    let Some(refers_to) = hover_info.refers_to(linker) else {
        return Vec::new();
    };
    if let Some(in_global) = refers_to.is_strictly_local() {
        gather_all_references_in_global(linker, in_global, refers_to)
    } else {
        gather_references_in_file(linker, file, refers_to)
    }
}

pub fn gather_all_references_across_all_files(
    linker: &Linker,
    file_id: FileUUID,
    pos: usize,
) -> Vec<(FileUUID, Vec<Span>)> {
    let Some(hover_info) = get_selected_object(linker, &linker.files[file_id], pos) else {
        return Vec::new();
    };
    let Some(refers_to) = hover_info.refers_to(linker) else {
        return Vec::new();
    };
    //eprintln!("Refers to {refers_to:?}");
    let mut ref_locations = Vec::new();

    if let Some(in_global) = refers_to.is_strictly_local() {
        let found_refs = gather_all_references_in_global(linker, in_global, refers_to);
        assert_all_refs_of_correct_length(hover_info.span, &found_refs, linker);
        if !found_refs.is_empty() {
            ref_locations.push((file_id, found_refs))
        }
    } else {
        for (other_file_id, other_file) in &linker.files {
            let found_refs = gather_references_in_file(linker, other_file, refers_to);
            assert_all_refs_of_correct_length(hover_info.span, &found_refs, linker);
            if !found_refs.is_empty() {
                ref_locations.push((other_file_id, found_refs))
            }
        }
    }
    for r in &ref_locations {
        check_for_duplicate_spans(&linker.files, &r.1);
    }

    ref_locations
}

fn assert_all_refs_of_correct_length(location: Span, refs: &[Span], linker: &Linker) {
    if refs.iter().any(|r| r.size() != location.size()) {
        let refs_vec: Vec<_> = refs.iter().map(|r| (*r, String::new())).collect();
        pretty_print_span(
            &linker.files,
            location,
            "Original location Span".to_string(),
        );
        pretty_print_many_spans(&linker.files, refs_vec.into_iter());
        panic!("One of the spans was not of the same size as the original span!")
    }
}

/// Walks the file, and finds the [LocationInfo] that is the most relevant
///
/// IE, the [LocationInfo] in the selection area that has the smallest span.
pub fn get_selected_object<'linker>(
    linker: &'linker Linker,
    file: &'linker FileData,
    position: usize,
) -> Option<LocationInfo<'linker>> {
    let mut best_object: Option<LocationInfo<'linker>> = None;

    let mut walker = TreeWalker {
        linker,
        visitor: &mut |info| {
            // Gotta do this condition in inverse, since we only want to set it if it's not already set, or the new span is more specific
            if let Some(best_obj) = best_object
                && best_obj.span.size() < best_obj.span.size()
            {
            } else {
                // Better spans are also spans that come later, even if they are the exact same span. Because more specific tree nodes are nested.
                best_object = Some(info);
            }
        },
        should_prune: &mut |span| !span.contains_pos(position),
    };

    walker.walk_file(file);

    best_object
}

struct TreeWalker<'linker, 'fns> {
    linker: &'linker Linker,
    visitor: &'fns mut dyn FnMut(LocationInfo<'linker>),
    should_prune: &'fns mut dyn Fn(Span) -> bool,
}

impl<'linker, 'fns> TreeWalker<'linker, 'fns> {
    fn visit(&mut self, info: LocationInfo<'linker>) {
        if !(self.should_prune)(info.span) {
            (self.visitor)(info);
        }
    }

    fn walk_global_reference(
        &mut self,
        parent: GlobalUUID,
        link_info: &'linker LinkInfo,
        global_ref: MultiGlobalRef<'linker>,
    ) {
        let (target_name_elem, name_span, total_span, template_args) = global_ref.get_global();
        let in_global = Some(parent);
        if (self.should_prune)(global_ref.get_global().2) {
            return;
        }
        self.visit(LocationInfo {
            span: total_span,
            kind: LocationKind::GlobalReference(global_ref),
            in_global,
        });
        self.visit(LocationInfo {
            span: name_span,
            kind: LocationKind::Global(target_name_elem),
            in_global,
        });
        for arg in template_args {
            self.visit(LocationInfo {
                span: arg.name_span,
                kind: LocationKind::UsedTemplateArg(target_name_elem, arg, global_ref),
                in_global,
            });
            match &arg.kind {
                Some(TemplateKind::Type(wr_typ)) => {
                    self.walk_type(parent, link_info, wr_typ);
                }
                Some(TemplateKind::Value(_val)) => {}
                None => {}
            }
        }
    }

    fn walk_wire_ref(
        &mut self,
        parent: GlobalUUID,
        link_info: &'linker LinkInfo,
        wire_ref: &'linker WireReference,
    ) {
        let in_global = Some(parent);
        self.visit(LocationInfo {
            span: wire_ref.root_span,
            kind: LocationKind::WireRefRoot(&wire_ref.root),
            in_global,
        });
        match &wire_ref.root {
            WireReferenceRoot::NamedConstant(cst) => {
                self.walk_global_reference(parent, link_info, GlobalObj::Constant(cst));
            }
            WireReferenceRoot::NamedModule(md) => {
                self.walk_global_reference(parent, link_info, GlobalObj::Module(md));
            }
            _ => {}
        }

        for p in &wire_ref.path {
            match p {
                WireReferencePathElement::FieldAccess {
                    name,
                    name_span,
                    refers_to,
                } => {
                    self.visit(LocationInfo {
                        span: *name_span,
                        kind: LocationKind::Field {
                            name,
                            name_span: *name_span,
                            refers_to: refers_to.get(),
                            in_wire_ref: wire_ref,
                        },
                        in_global,
                    });
                }
                WireReferencePathElement::ArrayAccess { .. }
                | WireReferencePathElement::ArraySlice { .. }
                | WireReferencePathElement::ArrayPartSelect { .. } => {}
            }
        }
    }

    fn walk_type(
        &mut self,
        parent: GlobalUUID,
        link_info: &'linker LinkInfo,
        typ_expr: &'linker WrittenType,
    ) {
        let typ_expr_span = typ_expr.get_span();
        if (self.should_prune)(typ_expr_span) {
            return;
        }
        match typ_expr {
            WrittenType::Error(_) => {}
            WrittenType::TemplateVariable(span, template_id) => {
                self.visit(LocationInfo {
                    span: *span,
                    kind: LocationKind::TypeTemplateParam(parent, *template_id),
                    in_global: Some(parent),
                });
            }
            WrittenType::Named(named_type) => {
                self.walk_global_reference(parent, link_info, GlobalObj::Type(named_type));
            }
            WrittenType::Array(_, arr_box) => {
                let (arr_content_typ, _size_id, _br_span) = arr_box.deref();

                self.walk_type(parent, link_info, arr_content_typ);
            }
        }
    }

    fn walk_name_and_template_arguments(
        &mut self,
        global: GlobalUUID,
        link_info: &'linker LinkInfo,
    ) {
        self.visit(LocationInfo {
            span: link_info.name_span,
            kind: LocationKind::Global(global),
            in_global: Some(global),
        });

        for (param_id, parameter) in &link_info.parameters {
            if let TemplateKind::Type(TypeParameterKind {}) = &parameter.kind {
                self.visit(LocationInfo {
                    span: parameter.name_span,
                    kind: LocationKind::TypeTemplateParam(global, param_id),
                    in_global: Some(global),
                });
            }
        }
    }

    fn walk_link_info(&mut self, parent: GlobalUUID) {
        let link_info = &self.linker.globals[parent];
        let in_global = Some(parent);
        if (self.should_prune)(link_info.span) {
            return;
        }
        for (id, inst) in &link_info.instructions {
            match inst {
                Instruction::SubModule(sm) => {
                    self.walk_global_reference(
                        parent,
                        link_info,
                        GlobalObj::Module(&sm.module_ref),
                    );
                    self.visit(LocationInfo {
                        span: sm.name_span,
                        kind: LocationKind::LocalSubmodule(id),
                        in_global,
                    });
                }
                Instruction::Declaration(decl) => {
                    self.walk_type(parent, link_info, &decl.typ_expr);
                    if decl.declaration_itself_is_not_written_to {
                        self.visit(LocationInfo {
                            span: decl.name_span,
                            kind: LocationKind::LocalDecl(id),
                            in_global,
                        });
                    }
                }
                Instruction::Interface(interface) => {
                    self.visit(LocationInfo {
                        span: interface.name_span,
                        kind: LocationKind::LocalInterface(id),
                        in_global,
                    });
                }
                Instruction::Expression(expr) => {
                    // Don't use [LinkInfo::iter_wire_refs()]
                    // such that walk_wire_ref is always ordered after the other subexpressions
                    if let ExpressionSource::WireRef(wire_ref) = &expr.source {
                        self.walk_wire_ref(parent, link_info, wire_ref);
                    }
                    if let ExpressionOutput::MultiWrite(writes) = &expr.output {
                        for wr in writes {
                            self.walk_wire_ref(parent, link_info, &wr.to);
                        }
                    }
                }
                Instruction::IfStatement(_) | Instruction::ForStatement(_) => {}
            };
        }

        // The global name for a constant should overrule other references, so we visit it last.
        self.walk_name_and_template_arguments(parent, link_info);
    }

    fn walk_global(&mut self, global: GlobalUUID) {
        self.walk_link_info(global);
    }

    fn walk_file(&mut self, file: &'linker FileData) {
        for global in &file.associated_values {
            self.walk_global(*global);
        }
    }
}
