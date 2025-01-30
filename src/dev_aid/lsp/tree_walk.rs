use std::ops::Deref;

use crate::flattening::*;
use crate::prelude::*;

use crate::linker::{FileData, GlobalUUID, LinkInfo};

use crate::typing::template::{
    GenerativeParameterKind, GlobalReference, Parameter, ParameterKind, TemplateArgKind,
    TypeParameterKind,
};

/// See [LocationInfo]
#[derive(Clone, Copy, Debug)]
pub enum InGlobal<'linker> {
    NamedLocal(&'linker Declaration),
    NamedSubmodule(&'linker SubModuleInstance),
    Temporary(&'linker Expression),
}

/// Information about an object in the source code. Used for hovering, completions, syntax highlighting etc.
#[derive(Clone, Copy, Debug)]
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
    /// The contained module only refers to the module on which the port is defined
    /// No reference to the module in which the reference was found is provided
    Port(&'linker SubModuleInstance, &'linker Module, PortID),
    Interface(ModuleUUID, &'linker Module, InterfaceID, &'linker Interface),
}

/// Permits really efficient [RefersTo::refers_to_same_as] [LocationInfo] checking
#[derive(Clone, Copy, Debug)]
pub struct RefersTo {
    pub local: Option<(GlobalUUID, FlatID)>,
    pub global: Option<GlobalUUID>,
    pub port: Option<(ModuleUUID, PortID)>,
    pub interface: Option<(ModuleUUID, InterfaceID)>,
    pub parameter: Option<(GlobalUUID, TemplateID)>,
}

impl<'linker> From<LocationInfo<'linker>> for RefersTo {
    fn from(info: LocationInfo) -> Self {
        let mut result = RefersTo {
            local: None,
            global: None,
            port: None,
            interface: None,
            parameter: None,
        };
        match info {
            LocationInfo::InGlobal(obj_id, link_info, flat_id, flat_obj) => match flat_obj {
                InGlobal::NamedLocal(_) => {
                    let decl = link_info.instructions[flat_id].unwrap_declaration();
                    match decl.decl_kind {
                        DeclarationKind::NotPort => {}
                        DeclarationKind::StructField { field_id: _ } => {}
                        DeclarationKind::RegularPort {
                            is_input: _,
                            port_id,
                        } => {
                            result.port = Some((obj_id.unwrap_module(), port_id));
                        }
                        DeclarationKind::GenerativeInput(template_id) => {
                            result.parameter = Some((obj_id, template_id))
                        }
                    }
                    result.local = Some((obj_id, flat_id));
                }
                InGlobal::NamedSubmodule(_) => {
                    result.local = Some((obj_id, flat_id));
                }
                InGlobal::Temporary(_) => {}
            },
            LocationInfo::Type(_, _) => {}
            LocationInfo::Parameter(obj, _link_info, template_id, template_arg) => {
                match &template_arg.kind {
                    ParameterKind::Type(TypeParameterKind {}) => {}
                    ParameterKind::Generative(GenerativeParameterKind {
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
            LocationInfo::Port(sm, md, p_id) => {
                result.local = Some((
                    GlobalUUID::Module(sm.module_ref.id),
                    md.ports[p_id].declaration_instruction,
                ));
                result.port = Some((sm.module_ref.id, p_id))
            }
            LocationInfo::Interface(md_id, _md, i_id, _interface) => {
                result.interface = Some((md_id, i_id))
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
            LocationInfo::Port(sm, _, p_id) => self.port == Some((sm.module_ref.id, p_id)),
            LocationInfo::Interface(md_id, _, i_id, _) => self.interface == Some((md_id, i_id)),
        }
    }
    pub fn is_global(&self) -> bool {
        self.global.is_some()
            | self.port.is_some()
            | self.interface.is_some()
            | self.parameter.is_some()
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
    file: FileUUID,
    position: usize,
) -> Option<(Span, LocationInfo<'linker>)> {
    let file_data = &linker.files[file];

    let mut best_object: Option<LocationInfo<'linker>> = None;
    let mut best_span: Span = Span::MAX_POSSIBLE_SPAN;

    let mut walker = TreeWalker {
        linker,
        visitor: |span, info| {
            if span.size() <= best_span.size() {
                //assert!(span.size() < self.best_span.size());
                // May not be the case. Do prioritize later ones, as they tend to be nested
                best_span = span;
                best_object = Some(info);
            }
        },
        should_prune: |span| !span.contains_pos(position),
    };

    walker.walk_file(file_data);

    best_object.map(|v| (best_span, v))
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
        for (id, template_arg) in global.template_args.iter_valids() {
            let target_link_info = self.linker.get_link_info(target_name_elem);
            self.visit(
                template_arg.name_span,
                LocationInfo::Parameter(
                    target_name_elem,
                    target_link_info,
                    id,
                    &target_link_info.template_parameters[id],
                ),
            );
            match &template_arg.kind {
                TemplateArgKind::Type(typ_expr) => {
                    self.walk_type(parent, link_info, typ_expr);
                }
                TemplateArgKind::Value(_) => {} // Covered by FlatIDs
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
            WireReferenceRoot::LocalDecl(decl_id, span) => {
                self.visit(
                    *span,
                    LocationInfo::InGlobal(
                        obj_id,
                        link_info,
                        *decl_id,
                        InGlobal::NamedLocal(link_info.instructions[*decl_id].unwrap_declaration()),
                    ),
                );
            }
            WireReferenceRoot::NamedConstant(cst) => {
                self.walk_global_reference(obj_id, link_info, cst);
            }
            WireReferenceRoot::SubModulePort(port) => {
                if let Some(span) = port.port_name_span {
                    let sm_instruction =
                        link_info.instructions[port.submodule_decl].unwrap_submodule();
                    let submodule = &self.linker.modules[sm_instruction.module_ref.id];
                    self.visit(
                        span,
                        LocationInfo::Port(sm_instruction, submodule, port.port),
                    );

                    // port_name_span being enabled means submodule_name_span is for sure
                    // And if port_name_span is invalid, then submodule_name_span points to a duplicate!
                    // So in effect, port_name_span validity is a proxy for non-duplicate-ness of submodule_name_span
                    self.visit(
                        port.submodule_name_span.unwrap(),
                        LocationInfo::InGlobal(
                            obj_id,
                            link_info,
                            port.submodule_decl,
                            InGlobal::NamedSubmodule(
                                link_info.instructions[port.submodule_decl].unwrap_submodule(),
                            ),
                        ),
                    );
                }
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
                            &link_info.template_parameters[*template_id],
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

    fn walk_interface_reference(
        &mut self,
        obj_id: GlobalUUID,
        link_info: &'linker LinkInfo,
        iref: &ModuleInterfaceReference,
    ) {
        if let Some(submod_name_span) = iref.name_span {
            let submodule_instruction = iref.submodule_decl;
            let submodule = link_info.instructions[submodule_instruction].unwrap_submodule();
            self.visit(
                submod_name_span,
                LocationInfo::InGlobal(
                    obj_id,
                    link_info,
                    submodule_instruction,
                    InGlobal::NamedSubmodule(submodule),
                ),
            );
            if iref.interface_span != submod_name_span {
                let submod_md = &self.linker.modules[submodule.module_ref.id];
                let interface = &submod_md.interfaces[iref.submodule_interface];
                self.visit(
                    iref.interface_span,
                    LocationInfo::Interface(
                        submodule.module_ref.id,
                        submod_md,
                        iref.submodule_interface,
                        interface,
                    ),
                );
            }
        }
    }

    fn walk_name_and_template_arguments(
        &mut self,
        name_elem: GlobalUUID,
        link_info: &'linker LinkInfo,
    ) {
        self.visit(link_info.name_span, LocationInfo::Global(name_elem));

        for (template_id, template_arg) in &link_info.template_parameters {
            if let ParameterKind::Type(TypeParameterKind {}) = &template_arg.kind {
                self.visit(
                    template_arg.name_span,
                    LocationInfo::Parameter(name_elem, &link_info, template_id, template_arg),
                );
            }
        }
    }

    fn walk_link_info(&mut self, obj_id: GlobalUUID) {
        let link_info = self.linker.get_link_info(obj_id);
        if !(self.should_prune)(link_info.span) {
            self.walk_name_and_template_arguments(obj_id, &link_info);

            for (id, inst) in &link_info.instructions {
                match inst {
                    Instruction::SubModule(sm) => {
                        self.walk_global_reference(obj_id, &link_info, &sm.module_ref);
                        if let Some((_sm_name, sm_name_span)) = &sm.name {
                            self.visit(
                                *sm_name_span,
                                LocationInfo::InGlobal(
                                    obj_id,
                                    link_info,
                                    id,
                                    InGlobal::NamedSubmodule(sm),
                                ),
                            );
                        }
                    }
                    Instruction::Declaration(decl) => {
                        self.walk_type(obj_id, &link_info, &decl.typ_expr);
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
                    Instruction::Expression(wire) => {
                        if let ExpressionSource::WireRef(wire_ref) = &wire.source {
                            self.walk_wire_ref(obj_id, link_info, wire_ref);
                        } else {
                            self.visit(
                                wire.span,
                                LocationInfo::InGlobal(
                                    obj_id,
                                    link_info,
                                    id,
                                    InGlobal::Temporary(wire),
                                ),
                            );
                        };
                    }
                    Instruction::Write(write) => {
                        self.walk_wire_ref(obj_id, link_info, &write.to);
                    }
                    Instruction::FuncCall(fc) => {
                        self.walk_interface_reference(obj_id, link_info, &fc.interface_reference);
                    }
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
                    self.visit(
                        interface.name_span,
                        LocationInfo::Interface(md_id, md, interface_id, interface),
                    );
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
