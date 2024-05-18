
use std::ops::Deref;

use crate::{
    file_position::Span, flattening::{Declaration, FlatID, Instruction, Module, Port, PortID, SubModuleInstance, WireInstance, WireReference, WireReferenceRoot, WireSource}, linker::{FileData, Linker, ModuleUUID, NameElem}, typing::WrittenType
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
    Type(&'linker WrittenType),
    Global(NameElem),
    /// The contained module only refers to the module on which the port is defined
    /// No reference to the module in which the reference was found is provided
    Port(ModuleUUID, &'linker Module, PortID, &'linker Port)
}

/// Permits really efficient [RefersTo::refers_to_same_as] [LocationInfo] checking
#[derive(Clone, Copy, Debug)]
pub struct RefersTo {
    pub local : Option<(ModuleUUID, FlatID)>,
    pub global : Option<NameElem>,
    pub port : Option<(ModuleUUID, PortID)>
}

impl<'linker> From<LocationInfo<'linker>> for RefersTo {
    fn from(info : LocationInfo) -> Self {
        let mut result = RefersTo{
            local: None,
            global: None,
            port: None,
        };
        match info {
            LocationInfo::InModule(md_id, md, flat_id, flat_obj) => {
                match flat_obj {
                    InModule::NamedLocal(_) => {
                        for (port_id, port) in &md.ports {
                            if port.declaration_instruction == flat_id {
                                result.port = Some((md_id, port_id));
                            }
                        }
                        result.local = Some((md_id, flat_id));
                    },
                    InModule::NamedSubmodule(_) => {
                        result.local = Some((md_id, flat_id));
                    },
                    InModule::Temporary(_) => {}
                }
            }
            LocationInfo::Type(_) => {}
            LocationInfo::Global(name_elem) => {
                result.global = Some(name_elem);
            }
            LocationInfo::Port(md_id, _md, p_id, port) => {
                result.local = Some((md_id, port.declaration_instruction));
                result.port = Some((md_id, p_id))
            }
        }
        result
    }
}

impl RefersTo {
    pub fn refers_to_same_as(&self, info : LocationInfo) -> bool {
        match info {
            LocationInfo::InModule(md_id, _, obj, _) => self.local == Some((md_id, obj)),
            LocationInfo::Type(_) => false,
            LocationInfo::Global(ne) => self.global == Some(ne),
            LocationInfo::Port(md_id, _, p_id, _) => self.port == Some((md_id, p_id))
        }
    }
    pub fn is_global(&self) -> bool {
        self.global.is_some() | self.port.is_some()
    }
}

/// Walks the file, and provides all [LocationInfo]s. 
pub fn visit_all<'linker, Visitor : FnMut(Span, LocationInfo<'linker>)>(linker : &'linker Linker, file : &'linker FileData, visitor : Visitor) {
    let mut walker = TreeWalker {
        linker,
        visitor,
        should_prune: |_| false,
    };

    walker.walk_file(file);
}

/// Walks the file, and provides all [LocationInfo]s. 
pub fn visit_all_in_module<'linker, Visitor : FnMut(Span, LocationInfo<'linker>)>(linker : &'linker Linker, md_id : ModuleUUID, visitor : Visitor) {
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
pub fn get_selected<'linker>(linker : &'linker Linker, file : &'linker FileData, position : usize) -> Option<(Span, LocationInfo<'linker>)> {
    let mut best_object : Option<LocationInfo<'linker>> = None;
    let mut best_span : Span = Span::MAX_POSSIBLE_SPAN;
    
    let mut walker = TreeWalker {
        linker,
        visitor : |span, info| {
            if span.size() <= best_span.size() {
                //assert!(span.size() < self.best_span.size());
                // May not be the case. Do prioritize later ones, as they tend to be nested
                best_span = span;
                best_object = Some(info);
            }
        },
        should_prune: |span| !span.contains_pos(position),
    };

    walker.walk_file(file);

    best_object.map(|v| (best_span, v))
}


struct TreeWalker<'linker, Visitor : FnMut(Span, LocationInfo<'linker>), Pruner : Fn(Span) -> bool> {
    linker : &'linker Linker,
    visitor : Visitor,
    should_prune : Pruner
}

impl<'linker, Visitor : FnMut(Span, LocationInfo<'linker>), Pruner : Fn(Span) -> bool> TreeWalker<'linker, Visitor, Pruner>  {
    fn visit(&mut self, span : Span, info : LocationInfo<'linker>) {
        if !(self.should_prune)(span) {
            (self.visitor)(span, info);
        }
    }

    fn walk_wire_ref(&mut self, md_id : ModuleUUID, md : &'linker Module, wire_ref : &'linker WireReference) {
        match &wire_ref.root {
            WireReferenceRoot::LocalDecl(decl_id, span) => {
                self.visit(*span, LocationInfo::InModule(md_id, md, *decl_id, InModule::NamedLocal(md.instructions[*decl_id].unwrap_wire_declaration())));
            }
            WireReferenceRoot::NamedConstant(cst, span) => {
                self.visit(*span, LocationInfo::Global(NameElem::Constant(*cst)))
            }
            WireReferenceRoot::SubModulePort(port) => {
                if let Some(submod_name_span) = port.submodule_name_span {
                    self.visit(submod_name_span, LocationInfo::InModule(md_id, md, port.submodule_flat, InModule::NamedSubmodule(md.instructions[port.submodule_flat].unwrap_submodule())));
                }
                if let Some(span) = port.port_name_span {
                    let module_uuid = md.instructions[port.submodule_flat].unwrap_submodule().module_uuid;
                    let submodule = &self.linker.modules[module_uuid];
                    self.visit(span, LocationInfo::Port(module_uuid, submodule, port.port, &submodule.ports[port.port]))
                }
            }
        }
    }

    fn walk_type(&mut self, typ_expr : &'linker WrittenType) {
        let typ_expr_span = typ_expr.get_span();
        if !(self.should_prune)(typ_expr_span) {
            (self.visitor)(typ_expr_span, LocationInfo::Type(typ_expr));
            match typ_expr {
                WrittenType::Error(_) => {}
                WrittenType::Named(span, name_id) => {
                    self.visit(*span, LocationInfo::Global(NameElem::Type(*name_id)));
                }
                WrittenType::Array(_, arr_box) => {
                    let (arr_content_typ, _size_id, _br_span) = arr_box.deref();

                    self.walk_type(arr_content_typ)
                }
            }
        }
    }
    
    fn walk_module(&mut self, md_id : ModuleUUID) {
        let md = &self.linker.modules[md_id];
        if !(self.should_prune)(md.link_info.span) {
            self.visit(md.link_info.name_span, LocationInfo::Global(NameElem::Module(md_id)));

            for (id, inst) in &md.instructions {
                match inst {
                    Instruction::SubModule(sm) => {
                        self.visit(sm.module_name_span, LocationInfo::Global(NameElem::Module(sm.module_uuid)));
                        if let Some((_sm_name, sm_name_span)) = &sm.name {
                            self.visit(*sm_name_span, LocationInfo::InModule(md_id, md, id, InModule::NamedSubmodule(sm)));
                        }
                    }
                    Instruction::Declaration(decl) => {
                        self.walk_type(&decl.typ_expr);
                        if decl.declaration_itself_is_not_written_to {
                            self.visit(decl.name_span, LocationInfo::InModule(md_id, md, id, InModule::NamedLocal(decl)));
                        }
                    }
                    Instruction::Wire(wire) => {
                        if let WireSource::WireRef(wire_ref) = &wire.source {
                            self.walk_wire_ref(md_id, md, wire_ref);
                        } else {
                            self.visit(wire.span, LocationInfo::InModule(md_id, md, id, InModule::Temporary(wire)));
                        };
                    }
                    Instruction::Write(write) => {
                        self.walk_wire_ref(md_id, md, &write.to);
                    }
                    Instruction::IfStatement(_) | Instruction::ForStatement(_) => {}
                };
            }
        }
    }

    fn walk_file(&mut self, file : &'linker FileData) {
        for global in &file.associated_values {
            match *global {
                NameElem::Module(md_id) => {
                    self.walk_module(md_id);
                }
                NameElem::Type(_) => {
                    todo!()
                }
                NameElem::Constant(_) => {
                    todo!()
                }
            }
        }
    }
}
