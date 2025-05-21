use std::{borrow::Cow, path::PathBuf};

use dot::{render, Edges, GraphWalk, Id, LabelText, Labeller, Nodes, Style};

use crate::{
    alloc::FlatAlloc,
    instantiation::{
        ConcreteSubModule, ForEachContainedWire, InstantiatedModule, RealWire, RealWireDataSource,
    },
    latency::LatencyCountingProblem,
    linker::Linker,
    prelude::{SubModuleID, SubModuleIDMarker, WireID, WireIDMarker},
};

pub fn display_generated_hardware_structure(md_instance: &InstantiatedModule, linker: &Linker) {
    let mut file = std::fs::File::create("hardware_structure.dot").unwrap();
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

    fn node_style(&'inst self, n: &NodeType) -> Style {
        match n {
            NodeType::Wire(_) => Style::None,
            NodeType::SubModule(_) => Style::Filled,
        }
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

pub fn display_latency_count_graph(
    lc_problem: &LatencyCountingProblem,
    wires: &FlatAlloc<RealWire, WireIDMarker>,
    submodules: &FlatAlloc<ConcreteSubModule, SubModuleIDMarker>,
    linker: &Linker,
    solution: Option<&[i64]>,
    filename: &str,
) {
    // true for input
    let mut extra_node_info = vec![(None, None); lc_problem.map_latency_node_to_wire.len()];
    for port in lc_problem.ports.inputs() {
        extra_node_info[*port].0 = Some(true);
    }
    for port in lc_problem.ports.outputs() {
        extra_node_info[*port].0 = Some(false);
    }

    for spec in &lc_problem.specified_latencies {
        extra_node_info[spec.node].1 = Some(spec.latency);
    }

    use std::str::FromStr;

    let mut file = std::fs::File::create(PathBuf::from_str(filename).unwrap()).unwrap();
    render(
        &Problem {
            lc_problem,
            wires,
            submodules,
            linker,
            solution,
            extra_node_info,
        },
        &mut file,
    )
    .unwrap();
}

#[derive(Clone, Copy)]
enum LCEdgeType<'a> {
    Normal(i64),
    Infer {
        in_submod: &'a str,
        var: &'a str,
        offset: i64,
        multiplier: i64,
    },
    Poison,
}

type LatencyEdge<'a> = (usize, usize, LCEdgeType<'a>);

struct Problem<'a> {
    lc_problem: &'a LatencyCountingProblem,
    wires: &'a FlatAlloc<RealWire, WireIDMarker>,
    submodules: &'a FlatAlloc<ConcreteSubModule, SubModuleIDMarker>,
    linker: &'a Linker,
    solution: Option<&'a [i64]>,
    extra_node_info: Vec<(Option<bool>, Option<i64>)>,
}

impl<'a> Labeller<'a, usize, LatencyEdge<'a>> for Problem<'a> {
    fn graph_id(&'a self) -> Id<'a> {
        Id::new("lcGraph").unwrap()
    }

    fn node_id(&'a self, n: &usize) -> Id<'a> {
        Id::new(format!("n{n}")).unwrap()
    }

    fn node_label(&'a self, n: &usize) -> LabelText<'a> {
        let name = &self.wires[self.lc_problem.map_latency_node_to_wire[*n]].name;
        let mut result = format!("[{name}] ");

        if let Some(sol) = self.solution {
            result.push_str(&sol[*n].to_string())
        }
        if let Some(specified) = self.extra_node_info[*n].1 {
            use std::fmt::Write;
            write!(result, " specified {specified}").unwrap();
        }
        LabelText::LabelStr(result.into())
    }

    fn edge_label(&'a self, e: &LatencyEdge) -> LabelText<'a> {
        LabelText::LabelStr(match e.2 {
            LCEdgeType::Normal(delta) => delta.to_string().into(),
            LCEdgeType::Infer {
                var,
                in_submod,
                offset,
                multiplier,
            } => format!("Infer <= {multiplier} * {in_submod}.{var} + {offset}").into(),
            LCEdgeType::Poison => "poison".into(),
        })
    }

    fn edge_color(&'a self, e: &LatencyEdge) -> Option<LabelText<'a>> {
        match e.2 {
            LCEdgeType::Normal(_) => None,
            LCEdgeType::Infer { .. } => Some(LabelText::LabelStr("green".into())),
            LCEdgeType::Poison => Some(LabelText::LabelStr("red".into())),
        }
    }

    fn node_color(&'a self, node: &usize) -> Option<LabelText<'a>> {
        self.extra_node_info[*node]
            .0
            .map(|is_input| LabelText::LabelStr(if is_input { "red" } else { "blue" }.into()))
    }
}

impl<'a> GraphWalk<'a, usize, LatencyEdge<'a>> for Problem<'a> {
    fn nodes(&'a self) -> Nodes<'a, usize> {
        (0..self.lc_problem.map_latency_node_to_wire.len()).collect()
    }

    fn edges(&'a self) -> Edges<'a, LatencyEdge> {
        let mut result = Vec::with_capacity(self.lc_problem.edges.len());

        for (to, fan_from) in &self.lc_problem.edges {
            result.push((
                fan_from.to_node,
                *to,
                if let Some(delta) = fan_from.delta_latency {
                    LCEdgeType::Normal(delta)
                } else {
                    LCEdgeType::Poison
                },
            ));
        }

        for infer_edge in &self.lc_problem.inference_edges {
            let var = &self.lc_problem.inference_variables[infer_edge.target_to_infer];
            let submod = &self.submodules[var.back_reference.0];
            let submod_md = &self.linker.modules[submod.refers_to.id];
            let var_param = submod_md.link_info.template_parameters[var.back_reference.1]
                .kind
                .unwrap_value();
            let var_decl = submod_md.link_info.instructions[var_param.declaration_instruction]
                .unwrap_declaration();

            result.push((
                infer_edge.from_node,
                infer_edge.to_node,
                LCEdgeType::Infer {
                    in_submod: &submod.name,
                    var: &var_decl.name,
                    offset: infer_edge.offset,
                    multiplier: infer_edge.multiply_var_by,
                },
            ));
        }

        result.into()
    }

    fn source(&'a self, edge: &LatencyEdge) -> usize {
        edge.0
    }

    fn target(&'a self, edge: &LatencyEdge) -> usize {
        edge.1
    }
}
