
use std::ops::Deref;

use crate::{
    file_position::Span, flattening::{Declaration, FlatID, Instruction, Module, Port, PortID, SubModuleInstance, WireInstance, WireReference, WireReferenceRoot, WireSource}, linker::{FileData, Linker, ModuleUUID, NameElem}, typing::WrittenType
};

#[derive(Clone, Copy, Debug)]
pub enum LocationInfo<'linker> {
    NamedLocal(&'linker Module, FlatID, &'linker Declaration),
    NamedSubmodule(&'linker Module, FlatID, &'linker SubModuleInstance),
    Temporary(&'linker Module, FlatID, &'linker WireInstance),
    Type(&'linker WrittenType),
    Global(NameElem),
    Port(PortID, ModuleUUID, &'linker Port)
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

    fn walk_wire_ref(&mut self, md : &'linker Module, wire_ref : &'linker WireReference) {
        match &wire_ref.root {
            WireReferenceRoot::LocalDecl(decl_id, span) => {
                self.visit(*span, LocationInfo::NamedLocal(md, *decl_id, md.instructions[*decl_id].unwrap_wire_declaration()));
            }
            WireReferenceRoot::NamedConstant(cst, span) => {
                self.visit(*span, LocationInfo::Global(NameElem::Constant(*cst)))
            }
            WireReferenceRoot::SubModulePort(port) => {
                if let Some(submod_name_span) = port.submodule_name_span {
                    self.visit(submod_name_span, LocationInfo::NamedSubmodule(md, port.submodule_flat, md.instructions[port.submodule_flat].unwrap_submodule()));
                }
                if let Some(span) = port.port_name_span {
                    let module_uuid = md.instructions[port.submodule_flat].unwrap_submodule().module_uuid;
                    let submodule = &self.linker.modules[module_uuid];
                    self.visit(span, LocationInfo::Port(port.port, module_uuid, &submodule.module_ports.ports[port.port]))
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
    
    fn walk_module(&mut self, md : &'linker Module) {
        for (id, inst) in &md.instructions {
            match inst {
                Instruction::SubModule(sm) => {
                    self.visit(sm.module_name_span, LocationInfo::Global(NameElem::Module(sm.module_uuid)));
                    if let Some((_sm_name, sm_name_span)) = &sm.name {
                        self.visit(*sm_name_span, LocationInfo::NamedSubmodule(md, id, sm));
                    }
                }
                Instruction::Declaration(decl) => {
                    self.walk_type(&decl.typ_expr);
                    if decl.declaration_itself_is_not_written_to {
                        self.visit(decl.name_span, LocationInfo::NamedLocal(md, id, decl));
                    }
                }
                Instruction::Wire(wire) => {
                    if let WireSource::WireRef(wire_ref) = &wire.source {
                        self.walk_wire_ref(md, wire_ref);
                    } else {
                        self.visit(wire.span, LocationInfo::Temporary(md, id, wire));
                    };
                }
                Instruction::Write(write) => {
                    self.walk_wire_ref(md, &write.to);
                }
                Instruction::IfStatement(_) | Instruction::ForStatement(_) => {}
            };
        }
    }

    fn walk_file(&mut self, file : &'linker FileData) {
        for global in &file.associated_values {
            match *global {
                NameElem::Module(md_id) => {
                    let md = &self.linker.modules[md_id];
                    
                    if !(self.should_prune)(md.link_info.span) {
                        self.visit(md.link_info.name_span, LocationInfo::Global(NameElem::Module(md_id)));
                        self.walk_module(md);
                    }
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
