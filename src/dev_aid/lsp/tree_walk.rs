use std::ops::Deref;

use crate::flattening::*;
use crate::linker::NamedType;
use crate::prelude::*;

use crate::linker::{FileData, LinkInfo, NameElem};

use crate::typing::template::{
    GenerativeTemplateInputKind, GlobalReference, TemplateArgKind, TemplateInput,
    TemplateInputKind, TypeTemplateInputKind,
};

#[derive(Clone, Copy, Debug)]
pub enum InModule<'linker> {
    NamedLocal(&'linker Declaration),
    NamedSubmodule(&'linker SubModuleInstance),
    Temporary(&'linker WireInstance),
}

#[derive(Clone, Copy, Debug)]
pub enum LocationInfo<'linker> {
    InModule(ModuleUUID, &'linker Module, FlatID, InModule<'linker>),
    TemplateInput(
        NameElem,
        &'linker LinkInfo,
        TemplateID,
        &'linker TemplateInput,
    ),
    Type(&'linker WrittenType, &'linker LinkInfo),
    Global(NameElem),
    /// The contained module only refers to the module on which the port is defined
    /// No reference to the module in which the reference was found is provided
    Port(&'linker SubModuleInstance, &'linker Module, PortID),
    Interface(ModuleUUID, &'linker Module, InterfaceID, &'linker Interface),
}

/// Permits really efficient [RefersTo::refers_to_same_as] [LocationInfo] checking
#[derive(Clone, Copy, Debug)]
pub struct RefersTo {
    pub local: Option<(ModuleUUID, FlatID)>,
    pub global: Option<NameElem>,
    pub port: Option<(ModuleUUID, PortID)>,
    pub interface: Option<(ModuleUUID, InterfaceID)>,
    pub template_input: Option<(NameElem, TemplateID)>,
}

impl<'linker> From<LocationInfo<'linker>> for RefersTo {
    fn from(info: LocationInfo) -> Self {
        let mut result = RefersTo {
            local: None,
            global: None,
            port: None,
            interface: None,
            template_input: None,
        };
        match info {
            LocationInfo::InModule(md_id, md, flat_id, flat_obj) => match flat_obj {
                InModule::NamedLocal(_) => {
                    let decl = md.instructions[flat_id].unwrap_wire_declaration();
                    match decl.is_port {
                        DeclarationPortInfo::NotPort => {}
                        DeclarationPortInfo::StructField { field_id:_ } => {}
                        DeclarationPortInfo::RegularPort {
                            is_input: _,
                            port_id,
                        } => {
                            result.port = Some((md_id, port_id));
                        }
                        DeclarationPortInfo::GenerativeInput(template_id) => {
                            result.template_input = Some((NameElem::Module(md_id), template_id))
                        }
                    }
                    result.local = Some((md_id, flat_id));
                }
                InModule::NamedSubmodule(_) => {
                    result.local = Some((md_id, flat_id));
                }
                InModule::Temporary(_) => {}
            },
            LocationInfo::Type(_, _) => {}
            LocationInfo::TemplateInput(obj, _link_info, template_id, template_arg) => {
                match &template_arg.kind {
                    TemplateInputKind::Type(TypeTemplateInputKind {}) => {}
                    TemplateInputKind::Generative(GenerativeTemplateInputKind {
                        decl_span: _,
                        declaration_instruction,
                    }) => {
                        let NameElem::Module(md_id) = obj else {
                            unreachable!()
                        }; // TODO, local names in types?
                        result.local = Some((md_id, *declaration_instruction));
                    }
                }
                result.template_input = Some((obj, template_id))
            }
            LocationInfo::Global(name_elem) => {
                result.global = Some(name_elem);
            }
            LocationInfo::Port(sm, md, p_id) => {
                result.local = Some((sm.module_ref.id, md.ports[p_id].declaration_instruction));
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
            LocationInfo::InModule(md_id, _, obj, _) => self.local == Some((md_id, obj)),
            LocationInfo::TemplateInput(parent, _, template_id, _) => {
                self.template_input == Some((parent, template_id))
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
            | self.template_input.is_some()
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
    md_id: ModuleUUID,
    visitor: Visitor,
) {
    let mut walker = TreeWalker {
        linker,
        visitor,
        should_prune: |_| false,
    };

    walker.walk_module(md_id);
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
        parent: NameElem,
        link_info: &'linker LinkInfo,
        global: &'linker GlobalReference<ID>,
    ) where
        NameElem: From<ID>,
    {
        let target_name_elem = NameElem::from(global.id);
        self.visit(global.span, LocationInfo::Global(target_name_elem));
        for (id, template_arg) in global.template_args.iter_valids() {
            if let Some(name_span) = template_arg.name_specification {
                let target_link_info = self.linker.get_link_info(target_name_elem).unwrap();
                self.visit(
                    name_span,
                    LocationInfo::TemplateInput(
                        target_name_elem,
                        target_link_info,
                        id,
                        &target_link_info.template_arguments[id],
                    ),
                );
            }
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
        md_id: ModuleUUID,
        md: &'linker Module,
        wire_ref: &'linker WireReference,
    ) {
        match &wire_ref.root {
            WireReferenceRoot::LocalDecl(decl_id, span) => {
                self.visit(
                    *span,
                    LocationInfo::InModule(
                        md_id,
                        md,
                        *decl_id,
                        InModule::NamedLocal(md.instructions[*decl_id].unwrap_wire_declaration()),
                    ),
                );
            }
            WireReferenceRoot::NamedConstant(cst, span) => {
                self.visit(*span, LocationInfo::Global(NameElem::Constant(*cst)))
            }
            WireReferenceRoot::SubModulePort(port) => {
                if let Some(span) = port.port_name_span {
                    let sm_instruction = md.instructions[port.submodule_decl].unwrap_submodule();
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
                        LocationInfo::InModule(
                            md_id,
                            md,
                            port.submodule_decl,
                            InModule::NamedSubmodule(
                                md.instructions[port.submodule_decl].unwrap_submodule(),
                            ),
                        ),
                    );
                }
            }
        }
    }

    fn walk_type(
        &mut self,
        parent: NameElem,
        link_info: &'linker LinkInfo,
        typ_expr: &'linker WrittenType,
    ) {
        let typ_expr_span = typ_expr.get_span();
        if !(self.should_prune)(typ_expr_span) {
            (self.visitor)(typ_expr_span, LocationInfo::Type(typ_expr, link_info));
            match typ_expr {
                WrittenType::Error(_) => {}
                WrittenType::Template(span, template_id) => {
                    self.visit(
                        *span,
                        LocationInfo::TemplateInput(
                            parent,
                            link_info,
                            *template_id,
                            &link_info.template_arguments[*template_id],
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
        md_id: ModuleUUID,
        md: &'linker Module,
        iref: &ModuleInterfaceReference,
    ) {
        if let Some(submod_name_span) = iref.name_span {
            let submodule_instruction = iref.submodule_decl;
            let submodule = md.instructions[submodule_instruction].unwrap_submodule();
            self.visit(
                submod_name_span,
                LocationInfo::InModule(
                    md_id,
                    md,
                    submodule_instruction,
                    InModule::NamedSubmodule(submodule),
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

    fn walk_name_and_template_arguments(&mut self, name_elem : NameElem, link_info: &'linker LinkInfo) {
        self.visit(
            link_info.name_span,
            LocationInfo::Global(name_elem),
        );

        for (template_id, template_arg) in &link_info.template_arguments {
            if let TemplateInputKind::Type(TypeTemplateInputKind {}) =
                &template_arg.kind
            {
                self.visit(
                    template_arg.name_span,
                    LocationInfo::TemplateInput(
                        name_elem,
                        &link_info,
                        template_id,
                        template_arg,
                    ),
                );
            }
        }
    }

    fn walk_module(&mut self, md_id: ModuleUUID) {
        let md = &self.linker.modules[md_id];
        if !(self.should_prune)(md.link_info.span) {
            self.walk_name_and_template_arguments(NameElem::Module(md_id), &md.link_info);

            for (interface_id, interface) in &md.interfaces {
                self.visit(
                    interface.name_span,
                    LocationInfo::Interface(md_id, md, interface_id, interface),
                );
            }

            for (id, inst) in &md.instructions {
                match inst {
                    Instruction::SubModule(sm) => {
                        self.walk_global_reference(
                            NameElem::Module(md_id),
                            &md.link_info,
                            &sm.module_ref,
                        );
                        if let Some((_sm_name, sm_name_span)) = &sm.name {
                            self.visit(
                                *sm_name_span,
                                LocationInfo::InModule(md_id, md, id, InModule::NamedSubmodule(sm)),
                            );
                        }
                    }
                    Instruction::Declaration(decl) => {
                        self.walk_type(NameElem::Module(md_id), &md.link_info, &decl.typ_expr);
                        if decl.declaration_itself_is_not_written_to {
                            self.visit(
                                decl.name_span,
                                LocationInfo::InModule(md_id, md, id, InModule::NamedLocal(decl)),
                            );
                        }
                    }
                    Instruction::Wire(wire) => {
                        if let WireSource::WireRef(wire_ref) = &wire.source {
                            self.walk_wire_ref(md_id, md, wire_ref);
                        } else {
                            self.visit(
                                wire.span,
                                LocationInfo::InModule(md_id, md, id, InModule::Temporary(wire)),
                            );
                        };
                    }
                    Instruction::Write(write) => {
                        self.walk_wire_ref(md_id, md, &write.to);
                    }
                    Instruction::FuncCall(fc) => {
                        self.walk_interface_reference(md_id, md, &fc.interface_reference);
                    }
                    Instruction::IfStatement(_) | Instruction::ForStatement(_) => {}
                };
            }
        }
    }

    fn walk_struct(&mut self, typ_id: TypeUUID) {
        let NamedType::Struct(typ) = &self.linker.types[typ_id] else {unreachable!("TODO remove Builtin Type")};
        if !(self.should_prune)(typ.link_info.span) {
            self.walk_name_and_template_arguments(NameElem::Type(typ_id), &typ.link_info);

            println!("TODO struct instructions")
        }
    }

    fn walk_file(&mut self, file: &'linker FileData) {
        for global in &file.associated_values {
            match *global {
                NameElem::Module(md_id) => {
                    self.walk_module(md_id);
                }
                NameElem::Type(typ_id) => {
                    self.walk_struct(typ_id);
                }
                NameElem::Constant(_) => {
                    todo!()
                }
            }
        }
    }
}
