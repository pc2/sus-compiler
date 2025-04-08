use std::borrow::Cow;

use dot::{render, Edges, GraphWalk, Id, LabelText, Labeller, Nodes};

use crate::{
    config::config,
    instantiation::{ForEachContainedWire, InstantiatedModule, RealWireDataSource},
    linker::Linker,
    prelude::{SubModuleID, WireID},
};

pub fn display_generated_hardware_structure(md_instance: &InstantiatedModule, linker: &Linker) {
    let mut file = std::fs::File::create(&config().dot_output_path).unwrap();
    render(&(md_instance, linker), &mut file).unwrap();
}

#[derive(Clone, Copy)]
enum NodeType {
    Wire(WireID),
    SubModule(SubModuleID),
}

type EdgeType = (NodeType, NodeType);

impl<'inst> Labeller<'inst, NodeType, EdgeType> for (&'inst InstantiatedModule, &'inst Linker) {
    fn graph_id(&'inst self) -> Id<'inst> {
        Id::new(&self.0.mangled_name).unwrap()
    }

    fn node_id(&'inst self, n: &NodeType) -> Id<'inst> {
        Id::new(match *n {
            NodeType::Wire(id) => {
                let wire = &self.0.wires[id];
                &wire.name
            }
            NodeType::SubModule(id) => {
                let sm = &self.0.submodules[id];
                &sm.name
            }
        })
        .unwrap()
    }

    fn node_label(&'inst self, n: &NodeType) -> LabelText<'inst> {
        LabelText::LabelStr(Cow::Owned(match *n {
            NodeType::Wire(id) => {
                let wire = &self.0.wires[id];
                let name: Cow<'_, str> = match &wire.source {
                    RealWireDataSource::ReadOnly | RealWireDataSource::Multiplexer { .. } => {
                        wire.name.as_str().into()
                    }
                    RealWireDataSource::UnaryOp { op, .. } => op.op_text().into(),
                    RealWireDataSource::BinaryOp { op, .. } => op.op_text().into(),
                    RealWireDataSource::Select { .. } => "xyz[][][]".into(),
                    RealWireDataSource::ConstructArray { .. } => "[...]".into(),
                    RealWireDataSource::Constant { value } => value.to_string().into(),
                };
                if wire.absolute_latency == i64::MIN {
                    format!("{id:?}: {name}'?")
                } else {
                    format!("{id:?}: {name}'{}", wire.absolute_latency)
                }
            }
            NodeType::SubModule(id) => {
                let sm = &self.0.submodules[id];
                format!("{id:?}: {}", &sm.name)
            }
        }))
    }
}

impl<'inst> GraphWalk<'inst, NodeType, EdgeType> for (&'inst InstantiatedModule, &'inst Linker) {
    fn nodes(&'inst self) -> Nodes<'inst, NodeType> {
        let (inst, _linker) = self;
        inst.wires
            .iter()
            .map(|(w, _)| NodeType::Wire(w))
            .chain(inst.submodules.iter().map(|(s, _)| NodeType::SubModule(s)))
            .collect()
    }

    fn edges(&'inst self) -> Edges<'inst, EdgeType> {
        let (inst, linker) = self;

        let mut edges = Vec::new();

        for (id, w) in &inst.wires {
            w.source
                .for_each_wire(&mut |v| edges.push((NodeType::Wire(v), NodeType::Wire(id))));
        }

        for (submod_id, s) in &inst.submodules {
            for (port_id, p) in s.port_map.iter_valids() {
                let sm = &linker.modules[s.refers_to.id];
                edges.push(if sm.ports[port_id].is_input {
                    (
                        NodeType::Wire(p.maps_to_wire),
                        NodeType::SubModule(submod_id),
                    )
                } else {
                    (
                        NodeType::SubModule(submod_id),
                        NodeType::Wire(p.maps_to_wire),
                    )
                });
            }
        }

        Cow::from(edges)
    }

    fn source(&'inst self, edge: &EdgeType) -> NodeType {
        edge.0
    }

    fn target(&'inst self, edge: &EdgeType) -> NodeType {
        edge.1
    }
}
