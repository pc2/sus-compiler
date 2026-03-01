use crate::prelude::*;

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
    LocalDecl {
        decl_id: FlatID,
        decl: &'linker Declaration,
    },
    LocalInterface {
        interface_decl_id: FlatID,
        interface_decl: &'linker InterfaceDeclaration,
    },
    LocalSubmodule {
        submodule_decl_id: FlatID,
        submodule_decl: &'linker SubModuleInstance,
    },
    Field {
        name: &'linker str,
        name_span: Span,
        refers_to: Option<&'linker PathElemRefersTo>,
        in_wire_ref: &'linker WireReference,
    },
    Parameter(GlobalUUID, TemplateID, &'linker Parameter),
    Global(GlobalUUID),
}

/// Until we remove the ID template from [GlobalReference], this is the stop-gap
type MultiGlobalRef<'linker> = GlobalObj<
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
    pub in_global_ref: Option<MultiGlobalRef<'linker>>,
    pub instr_id: Option<FlatID>,
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
    fn refer_to_interface(
        linker: &'linker Linker,
        in_global: GlobalUUID,
        interface_decl: &'linker InterfaceDeclaration,
    ) -> RefersTo<'linker> {
        let md = &linker.modules[in_global.unwrap_module()];
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
                        let interface_decl = link_info.instructions[*id].unwrap_interface();
                        Some(Self::refer_to_interface(linker, in_global, interface_decl))
                    }
                    WireReferenceRoot::NamedConstant(_) | WireReferenceRoot::NamedModule(_) => {
                        // these will be covered by more specific [LocationKind::GlobalReference]
                        None
                    }
                    WireReferenceRoot::Error => None,
                }
            }
            LocationKind::LocalDecl { decl: _, decl_id } => {
                let in_global = self.in_global.unwrap();
                Some(Self::refer_to_decl(linker, in_global, decl_id))
            }
            LocationKind::LocalSubmodule {
                submodule_decl,
                submodule_decl_id,
            } => {
                let in_global = self.in_global.unwrap();
                Some(RefersTo::LocalSubModule(
                    in_global,
                    submodule_decl,
                    submodule_decl_id,
                ))
            }
            LocationKind::LocalInterface { interface_decl, .. } => {
                let in_global = self.in_global.unwrap();
                Some(Self::refer_to_interface(linker, in_global, interface_decl))
            }
            LocationKind::Parameter(obj, _, template_arg) => {
                Some(RefersTo::Parameter(obj, template_arg))
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
pub fn visit_all<'linker, Visitor: FnMut(LocationInfo<'linker>)>(
    linker: &'linker Linker,
    file: &'linker FileData,
    visitor: Visitor,
) {
    let mut walker = TreeWalker {
        linker,
        visitor,
        should_prune: |_| false,
    };

    walker.walk_file(file);
}

/// Walks the file, and provides all [LocationInfo]s.
pub fn visit_all_in_module<'linker, Visitor: FnMut(LocationInfo<'linker>)>(
    linker: &'linker Linker,
    obj_id: GlobalUUID,
    visitor: Visitor,
) {
    let mut walker = TreeWalker {
        linker,
        visitor,
        should_prune: |_| false,
    };

    walker.walk_global(obj_id);
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
        visitor: |info| {
            // Gotta do this condition in inverse, since we only want to set it if it's not already set, or the new span is more specific
            if let Some(best_obj) = best_object
                && best_obj.span.size() < best_obj.span.size()
            {
            } else {
                // Better spans are also spans that come later, even if they are the exact same span. Because more specific tree nodes are nested.
                best_object = Some(info);
            }
        },
        should_prune: |span| !span.contains_pos(position),
    };

    walker.walk_file(file);

    best_object
}

struct TreeWalker<'linker, Visitor: FnMut(LocationInfo<'linker>), Pruner: Fn(Span) -> bool> {
    linker: &'linker Linker,
    visitor: Visitor,
    should_prune: Pruner,
}

impl<'linker, Visitor: FnMut(LocationInfo<'linker>), Pruner: Fn(Span) -> bool>
    TreeWalker<'linker, Visitor, Pruner>
{
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
            in_global_ref: None,
            instr_id: None,
        });
        self.visit(LocationInfo {
            span: name_span,
            kind: LocationKind::Global(target_name_elem),
            in_global,
            in_global_ref: Some(global_ref),
            instr_id: None,
        });
        //global.name_span, LocationInfo::Global(target_name_elem));
        let target_link_info = &self.linker.globals[target_name_elem];
        for arg in template_args {
            if let Some(&refers_to) = arg.refers_to.get() {
                self.visit(LocationInfo {
                    span: arg.name_span,
                    kind: LocationKind::Parameter(
                        target_name_elem,
                        refers_to,
                        &target_link_info.parameters[refers_to],
                    ),
                    in_global,
                    in_global_ref: Some(global_ref),
                    instr_id: None,
                });
            }
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
            in_global_ref: None,
            instr_id: None,
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
                        in_global_ref: None,
                        instr_id: None,
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
                    kind: LocationKind::Parameter(
                        parent,
                        *template_id,
                        &link_info.parameters[*template_id],
                    ),
                    in_global: Some(parent),
                    in_global_ref: None,
                    instr_id: None,
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
            in_global_ref: None,
            instr_id: None,
        });

        for (param_id, parameter) in &link_info.parameters {
            if let TemplateKind::Type(TypeParameterKind {}) = &parameter.kind {
                self.visit(LocationInfo {
                    span: parameter.name_span,
                    kind: LocationKind::Parameter(global, param_id, parameter),
                    in_global: Some(global),
                    in_global_ref: None,
                    instr_id: None,
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
            let instr_id = Some(id);
            match inst {
                Instruction::SubModule(sm) => {
                    self.walk_global_reference(
                        parent,
                        link_info,
                        GlobalObj::Module(&sm.module_ref),
                    );
                    self.visit(LocationInfo {
                        span: sm.name_span,
                        kind: LocationKind::LocalSubmodule {
                            submodule_decl_id: id,
                            submodule_decl: sm,
                        },
                        in_global,
                        in_global_ref: None,
                        instr_id,
                    });
                }
                Instruction::Declaration(decl) => {
                    self.walk_type(parent, link_info, &decl.typ_expr);
                    if decl.declaration_itself_is_not_written_to {
                        self.visit(LocationInfo {
                            span: decl.name_span,
                            kind: LocationKind::LocalDecl { decl_id: id, decl },
                            in_global,
                            in_global_ref: None,
                            instr_id,
                        });
                    }
                }
                Instruction::Interface(interface) => {
                    self.visit(LocationInfo {
                        span: interface.name_span,
                        kind: LocationKind::LocalInterface {
                            interface_decl_id: id,
                            interface_decl: interface,
                        },
                        in_global,
                        in_global_ref: None,
                        instr_id,
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
