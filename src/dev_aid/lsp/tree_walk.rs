use std::ops::Deref;

use crate::flattening::*;
use crate::prelude::*;

use crate::linker::{FileData, GlobalUUID, LinkInfo};

use crate::typing::template::{
    GenerativeParameterKind, Parameter, TemplateKind, TypeParameterKind,
};

/// See [LocationInfo]
#[derive(Clone, Copy, Debug)]
pub enum InGlobal<'linker> {
    NamedLocal(&'linker Declaration),
    NamedSubmodule(&'linker SubModuleInstance),
    LocalInterface(&'linker InterfaceDeclaration),
    Temporary(SingleOutputExpression<'linker>),
}

/// Information about an object in the source code. Used for hovering, completions, syntax highlighting etc.
#[derive(Clone, Copy)]
pub enum LocationInfo<'linker> {
    InGlobal(GlobalUUID, &'linker LinkInfo, FlatID, InGlobal<'linker>),
    Parameter(
        GlobalUUID,
        &'linker LinkInfo,
        TemplateID,
        &'linker Parameter,
    ),
    Type(&'linker WrittenType, &'linker LinkInfo),
    Global(GlobalUUID),
    Interface(ModuleUUID, &'linker Module, InterfaceID, &'linker Interface),
}

impl<'linker> std::fmt::Debug for LocationInfo<'linker> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InGlobal(_arg0, global, arg2, arg3) => f
                .debug_tuple("InGlobal")
                .field(&global.name)
                .field(arg2)
                .field(arg3)
                .finish(),
            Self::Parameter(_arg0, global, arg2, arg3) => f
                .debug_tuple("Parameter")
                .field(&global.name)
                .field(arg2)
                .field(arg3)
                .finish(),
            Self::Type(arg0, arg1) => f.debug_tuple("Type").field(arg0).field(arg1).finish(),
            Self::Global(arg0) => f.debug_tuple("Global").field(arg0).finish(),
            Self::Interface(_arg0, md, arg2, arg3) => f
                .debug_tuple("Interface")
                .field(&md.link_info.name)
                .field(arg2)
                .field(arg3)
                .finish(),
        }
    }
}

/// Permits really efficient [RefersTo::refers_to_same_as] [LocationInfo] checking
#[derive(Clone, Copy, Debug)]
pub struct RefersTo {
    pub local: Option<(GlobalUUID, FlatID)>,
    pub global: Option<GlobalUUID>,
    pub interface: Option<(ModuleUUID, InterfaceID)>,
    pub parameter: Option<(GlobalUUID, TemplateID)>,
}

impl From<LocationInfo<'_>> for RefersTo {
    fn from(info: LocationInfo) -> Self {
        let mut result = RefersTo {
            local: None,
            global: None,
            interface: None,
            parameter: None,
        };
        match info {
            LocationInfo::InGlobal(obj_id, _, flat_id, InGlobal::NamedLocal(decl)) => {
                match decl.decl_kind {
                    DeclarationKind::RegularGenerative
                    | DeclarationKind::ConditionalBinding { .. }
                    | DeclarationKind::RegularWire { .. }
                    | DeclarationKind::StructField(..) => {}
                    DeclarationKind::Port {
                        parent_interface,
                        is_standalone_port,
                        ..
                    } => {
                        if is_standalone_port {
                            result.interface = Some((obj_id.unwrap_module(), parent_interface));
                        }
                    }
                    DeclarationKind::TemplateParameter(template_id) => {
                        result.parameter = Some((obj_id, template_id))
                    }
                }
                result.local = Some((obj_id, flat_id));
            }
            LocationInfo::InGlobal(obj_id, _, flat_id, InGlobal::NamedSubmodule(_)) => {
                result.local = Some((obj_id, flat_id));
            }
            LocationInfo::InGlobal(obj_id, _, flat_id, InGlobal::LocalInterface(interface)) => {
                result.interface = Some((obj_id.unwrap_module(), interface.interface_id));
                result.local = Some((obj_id, flat_id));
            }
            LocationInfo::InGlobal(_, _, _, InGlobal::Temporary(_)) => {}
            LocationInfo::Type(_, _) => {}
            LocationInfo::Parameter(obj, _link_info, template_id, template_arg) => {
                match &template_arg.kind {
                    TemplateKind::Type(TypeParameterKind {}) => {}
                    TemplateKind::Value(GenerativeParameterKind {
                        decl_span: _,
                        declaration_instruction,
                    }) => {
                        result.local = Some((obj, *declaration_instruction));
                    }
                }
                result.parameter = Some((obj, template_id))
            }
            LocationInfo::Global(name_elem) => {
                result.global = Some(name_elem);
            }
            LocationInfo::Interface(md_id, _md, i_id, interface) => {
                result.interface = Some((md_id, i_id));
                match interface.declaration_instruction.unwrap() {
                    InterfaceDeclKind::SinglePort(port_decl) => {
                        result.local = Some((GlobalUUID::Module(md_id), port_decl));
                    }
                    InterfaceDeclKind::Interface(decl_id) => {
                        result.local = Some((GlobalUUID::Module(md_id), decl_id));
                    }
                }
            }
        }
        result
    }
}

impl RefersTo {
    pub fn refers_to_same_as(&self, info: LocationInfo) -> bool {
        match info {
            LocationInfo::InGlobal(global_id, _, obj, _) => self.local == Some((global_id, obj)),
            LocationInfo::Parameter(parent, _, template_id, _) => {
                self.parameter == Some((parent, template_id))
            }
            LocationInfo::Type(_, _) => false,
            LocationInfo::Global(ne) => self.global == Some(ne),
            LocationInfo::Interface(md_id, _, i_id, _) => self.interface == Some((md_id, i_id)),
        }
    }
    pub fn is_global(&self) -> bool {
        self.global.is_some() | self.interface.is_some() | self.parameter.is_some()
    }
}

/// Walks the file, and provides all [LocationInfo]s.
pub fn visit_all<'linker, Visitor: FnMut(Span, LocationInfo<'linker>)>(
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
pub fn visit_all_in_module<'linker, Visitor: FnMut(Span, LocationInfo<'linker>)>(
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
) -> Option<(Span, LocationInfo<'linker>)> {
    let mut best_object: Option<(Span, LocationInfo<'linker>)> = None;

    let mut walker = TreeWalker {
        linker,
        visitor: |span, info| {
            // Gotta do this condition in inverse, since we only want to set it if it's not already set, or the new span is more specific
            if let Some((best_span, _)) = best_object
                && best_span.size() < span.size()
            {
            } else {
                // Better spans are also spans that come later, even if they are the exact same span. Because more specific tree nodes are nested.
                best_object = Some((span, info));
            }
        },
        should_prune: |span| !span.contains_pos(position),
    };

    walker.walk_file(file);

    best_object
}

struct TreeWalker<'linker, Visitor: FnMut(Span, LocationInfo<'linker>), Pruner: Fn(Span) -> bool> {
    linker: &'linker Linker,
    visitor: Visitor,
    should_prune: Pruner,
}

impl<'linker, Visitor: FnMut(Span, LocationInfo<'linker>), Pruner: Fn(Span) -> bool>
    TreeWalker<'linker, Visitor, Pruner>
{
    fn visit(&mut self, span: Span, info: LocationInfo<'linker>) {
        if !(self.should_prune)(span) {
            (self.visitor)(span, info);
        }
    }

    fn walk_global_reference<ID: Copy>(
        &mut self,
        parent: GlobalUUID,
        link_info: &'linker LinkInfo,
        global: &'linker GlobalReference<ID>,
    ) where
        GlobalUUID: From<ID>,
    {
        let target_name_elem = GlobalUUID::from(global.id);
        self.visit(global.name_span, LocationInfo::Global(target_name_elem));
        let target_link_info = &self.linker.globals[target_name_elem];
        for arg in &global.template_args {
            if let Some(&refers_to) = arg.refers_to.get() {
                self.visit(
                    arg.name_span,
                    LocationInfo::Parameter(
                        target_name_elem,
                        target_link_info,
                        refers_to,
                        &target_link_info.parameters[refers_to],
                    ),
                );
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
        obj_id: GlobalUUID,
        link_info: &'linker LinkInfo,
        wire_ref: &'linker WireReference,
    ) {
        match &wire_ref.root {
            WireReferenceRoot::LocalDecl(decl_id) => {
                // Deduplicate references to implicit wire refs from decl: `int myDecl = 123`, `myDecl` occurs in both `int myDecl`, and `myDecl = 123`
                let decl = link_info.instructions[*decl_id].unwrap_declaration();
                if decl.name_span != wire_ref.root_span {
                    self.visit(
                        wire_ref.root_span,
                        LocationInfo::InGlobal(
                            obj_id,
                            link_info,
                            *decl_id,
                            InGlobal::NamedLocal(decl),
                        ),
                    );
                }
            }
            WireReferenceRoot::LocalSubmodule(submod_decl) => {
                let submod = link_info.instructions[*submod_decl].unwrap_submodule();
                if submod.name_span != wire_ref.root_span {
                    self.visit(
                        wire_ref.root_span,
                        LocationInfo::InGlobal(
                            obj_id,
                            link_info,
                            *submod_decl,
                            InGlobal::NamedSubmodule(submod),
                        ),
                    );
                }
            }
            WireReferenceRoot::LocalInterface(interface_decl) => {
                let interface = link_info.instructions[*interface_decl].unwrap_interface();
                if interface.name_span != wire_ref.root_span {
                    self.visit(
                        wire_ref.root_span,
                        LocationInfo::InGlobal(
                            obj_id,
                            link_info,
                            *interface_decl,
                            InGlobal::LocalInterface(interface),
                        ),
                    );
                }
            }
            WireReferenceRoot::NamedConstant(cst) => {
                self.walk_global_reference(obj_id, link_info, cst);
            }
            WireReferenceRoot::NamedModule(md) => {
                self.walk_global_reference(obj_id, link_info, md);
            }
            WireReferenceRoot::Error => {}
        }

        for p in &wire_ref.path {
            match p {
                WireReferencePathElement::FieldAccess {
                    name: _,
                    name_span,
                    refers_to,
                } => {
                    let Some(refers_to) = refers_to.get() else {
                        continue;
                    };

                    let target = match refers_to {
                        PathElemRefersTo::Interface(_, None) => {
                            continue;
                        }
                        PathElemRefersTo::Interface(in_module, Some(interface)) => {
                            let submodule = &self.linker.modules[*in_module];
                            LocationInfo::Interface(
                                *in_module,
                                submodule,
                                *interface,
                                &submodule.interfaces[*interface],
                            )
                        }
                    };
                    self.visit(*name_span, target);
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
        if !(self.should_prune)(typ_expr_span) {
            (self.visitor)(typ_expr_span, LocationInfo::Type(typ_expr, link_info));
            match typ_expr {
                WrittenType::Error(_) => {}
                WrittenType::TemplateVariable(span, template_id) => {
                    self.visit(
                        *span,
                        LocationInfo::Parameter(
                            parent,
                            link_info,
                            *template_id,
                            &link_info.parameters[*template_id],
                        ),
                    );
                }
                WrittenType::Named(named_type) => {
                    self.walk_global_reference(parent, link_info, named_type);
                }
                WrittenType::Array(_, arr_box) => {
                    let (arr_content_typ, _size_id, _br_span) = arr_box.deref();

                    self.walk_type(parent, link_info, arr_content_typ);
                }
            }
        }
    }

    fn walk_name_and_template_arguments(
        &mut self,
        name_elem: GlobalUUID,
        link_info: &'linker LinkInfo,
    ) {
        self.visit(link_info.name_span, LocationInfo::Global(name_elem));

        for (template_id, template_arg) in &link_info.parameters {
            if let TemplateKind::Type(TypeParameterKind {}) = &template_arg.kind {
                self.visit(
                    template_arg.name_span,
                    LocationInfo::Parameter(name_elem, link_info, template_id, template_arg),
                );
            }
        }
    }

    fn walk_link_info(&mut self, obj_id: GlobalUUID) {
        let link_info = &self.linker.globals[obj_id];
        if !(self.should_prune)(link_info.span) {
            self.walk_name_and_template_arguments(obj_id, link_info);

            for (id, inst) in &link_info.instructions {
                match inst {
                    Instruction::SubModule(sm) => {
                        self.walk_global_reference(obj_id, link_info, &sm.module_ref);
                        self.visit(
                            sm.name_span,
                            LocationInfo::InGlobal(
                                obj_id,
                                link_info,
                                id,
                                InGlobal::NamedSubmodule(sm),
                            ),
                        );
                    }
                    Instruction::Declaration(decl) => {
                        self.walk_type(obj_id, link_info, &decl.typ_expr);
                        if decl.declaration_itself_is_not_written_to {
                            self.visit(
                                decl.name_span,
                                LocationInfo::InGlobal(
                                    obj_id,
                                    link_info,
                                    id,
                                    InGlobal::NamedLocal(decl),
                                ),
                            );
                        }
                    }
                    Instruction::Expression(expr) => {
                        if let Some(single_output_expr) = expr.as_single_output_expr() {
                            self.visit(
                                expr.span,
                                LocationInfo::InGlobal(
                                    obj_id,
                                    link_info,
                                    id,
                                    InGlobal::Temporary(single_output_expr),
                                ),
                            )
                        }
                        // Don't use [LinkInfo::iter_wire_refs()]
                        // such that walk_wire_ref is always ordered after the other subexpressions
                        if let ExpressionSource::WireRef(wire_ref) = &expr.source {
                            self.walk_wire_ref(obj_id, link_info, wire_ref);
                        }
                        if let ExpressionOutput::MultiWrite(writes) = &expr.output {
                            for wr in writes {
                                self.walk_wire_ref(obj_id, link_info, &wr.to);
                            }
                        }
                    }
                    Instruction::Interface(_) => {}
                    Instruction::IfStatement(_) | Instruction::ForStatement(_) => {}
                };
            }
        }
    }

    fn walk_global(&mut self, global: GlobalUUID) {
        self.walk_link_info(global);
        match global {
            GlobalUUID::Module(md_id) => {
                let md = &self.linker.modules[md_id];
                for (interface_id, interface) in &md.interfaces {
                    match interface.declaration_instruction {
                        Some(InterfaceDeclKind::Interface(_)) => {
                            self.visit(
                                interface.name_span,
                                LocationInfo::Interface(md_id, md, interface_id, interface),
                            );
                        }
                        Some(InterfaceDeclKind::SinglePort(_)) => {} // Ports have been covered by their respective declarations
                        None => {}
                    }
                }
            }
            GlobalUUID::Type(_typ_id) => {} // These cases are covered by walk_link_info
            GlobalUUID::Constant(_cst_id) => {} // These cases are covered by walk_link_info
        }
    }

    fn walk_file(&mut self, file: &'linker FileData) {
        for global in &file.associated_values {
            self.walk_global(*global);
        }
    }
}
