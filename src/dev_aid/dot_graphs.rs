use std::borrow::Cow;
use std::fs::{self, File};

use dot::{Edges, GraphWalk, Id, LabelText, Labeller, Nodes, Style, render};

use crate::{
    alloc::FlatAlloc,
    flattening::Direction,
    instantiation::{ForEachContainedWire, ModuleTypingContext, RealWire, SubModule},
    latency::{LatencyCountingProblem, port_latency_inference::InferenceTarget},
    linker::Linker,
    prelude::{SubModuleID, SubModuleIDMarker, WireID, WireIDMarker},
    typing::template::TemplateKind,
};

/// Ensures dot_output exists and returns a File in dot_output with a unique name based on `module_name`, `dot_type`, and `.dot` extension.
/// Returns the file handle and the full path to the file.
fn unique_file_name(
    module_name: &str,
    dot_type: &str,
) -> std::io::Result<(File, std::path::PathBuf)> {
    let dir = "dot_output";
    fs::create_dir_all(dir)?;
    let mut path = std::path::PathBuf::from(dir);
    let mut file_name = format!("{module_name}_{dot_type}.dot");
    path.push(&file_name);
    let mut count = 1;
    while path.exists() {
        file_name = format!("{module_name}_{dot_type}_{count}.dot");
        path.set_file_name(&file_name);
        count += 1;
    }
    let file = File::create(&path)?;
    Ok((file, path))
}

fn try_convert_dot_to_image(dot_path: &std::path::Path) {
    let output_path = dot_path.with_extension("png");
    match std::process::Command::new("dot")
        .arg("-Tpng")
        .arg(dot_path)
        .arg("-o")
        .arg(&output_path)
        .output()
    {
        Ok(output) => {
            if !output.status.success() {
                eprintln!(
                    "Failed to convert {:?} to image: {}",
                    dot_path,
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(e) => {
            eprintln!(
                "Could not run 'dot' to convert {:?} to image: {}",
                dot_path, e
            );
        }
    }
}

pub fn display_generated_hardware_structure(md_instance: &ModuleTypingContext<'_>) {
    let (mut file, path) = unique_file_name(&md_instance.name, "hw_structure").unwrap();
    render(md_instance, &mut file).unwrap();
    try_convert_dot_to_image(&path);
}

#[derive(Clone, Copy)]
enum NodeType {
    Wire(WireID),
    SubModule(SubModuleID),
}

impl Direction {
    fn node_color(&self) -> &'static str {
        match self {
            Direction::Input => "red",
            Direction::Output => "blue",
        }
    }
}

type EdgeType = (NodeType, NodeType);

impl<'inst> Labeller<'inst, NodeType, EdgeType> for ModuleTypingContext<'_> {
    fn graph_id(&'inst self) -> Id<'inst> {
        Id::new(&self.mangled_name).unwrap()
    }

    fn node_id(&'inst self, n: &NodeType) -> Id<'inst> {
        Id::new(match *n {
            NodeType::Wire(id) => {
                let wire = &self.wires[id];
                &wire.name
            }
            NodeType::SubModule(id) => {
                let sm = &self.submodules[id];
                &sm.name
            }
        })
        .unwrap()
    }

    fn node_label(&'inst self, n: &NodeType) -> LabelText<'inst> {
        LabelText::LabelStr(match *n {
            NodeType::Wire(id) => {
                let wire = &self.wires[id];
                let name = &wire.name;
                let abs_lat = wire.absolute_latency;
                Cow::Owned(format!("{name}'{abs_lat}"))
            }
            NodeType::SubModule(id) => {
                let sm = &self.submodules[id];
                Cow::Borrowed(&sm.name)
            }
        })
    }

    fn node_style(&'inst self, n: &NodeType) -> Style {
        match n {
            NodeType::Wire(w_id) => match self.wires[*w_id].is_port {
                Some(Direction::Input) => Style::Bold,
                Some(Direction::Output) => Style::Bold,
                None => Style::None,
            },
            NodeType::SubModule(_) => Style::Filled,
        }
    }

    fn node_color<'a>(&'a self, n: &NodeType) -> Option<LabelText<'a>> {
        match n {
            NodeType::Wire(w_id) => self.wires[*w_id]
                .is_port
                .map(|d| LabelText::LabelStr(Cow::Borrowed(d.node_color()))),
            NodeType::SubModule(_) => None,
        }
    }
}

impl<'inst> GraphWalk<'inst, NodeType, EdgeType> for ModuleTypingContext<'_> {
    fn nodes(&'inst self) -> Nodes<'inst, NodeType> {
        self.wires
            .iter()
            .map(|(w, _)| NodeType::Wire(w))
            .chain(self.submodules.iter().map(|(s, _)| NodeType::SubModule(s)))
            .collect()
    }

    fn edges(&'inst self) -> Edges<'inst, EdgeType> {
        let mut edges = Vec::new();

        for (id, w) in &self.wires {
            w.source
                .for_each_wire(&mut |v| edges.push((NodeType::Wire(v), NodeType::Wire(id))));
        }

        for (submod_id, s) in &self.submodules {
            let s_md = &self.linker.modules[s.refers_to.id];
            for (port_id, port) in s.port_map.iter_valids() {
                let w_id = port.maps_to_wire;
                match s_md.ports[port_id].direction {
                    Direction::Input => {
                        edges.push((NodeType::Wire(w_id), NodeType::SubModule(submod_id)));
                    }
                    Direction::Output => {
                        edges.push((NodeType::SubModule(submod_id), NodeType::Wire(w_id)));
                    }
                }
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
    submodules: &FlatAlloc<SubModule, SubModuleIDMarker>,
    linker: &Linker,
    solution: Option<&[i64]>,
    module_name: &str,
    dot_type: &str,
) {
    // true for input
    let mut extra_node_info = vec![(None, None); lc_problem.map_latency_node_to_wire.len()];
    for port in lc_problem.ports.inputs() {
        extra_node_info[*port].0 = Some(Direction::Input);
    }
    for port in lc_problem.ports.outputs() {
        extra_node_info[*port].0 = Some(Direction::Output);
    }

    for spec in &lc_problem.specified_latencies {
        extra_node_info[spec.node].1 = Some(spec.latency);
    }

    let (mut file, path) = unique_file_name(module_name, dot_type).unwrap();
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
    try_convert_dot_to_image(&path);
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
    submodules: &'a FlatAlloc<SubModule, SubModuleIDMarker>,
    linker: &'a Linker,
    solution: Option<&'a [i64]>,
    extra_node_info: Vec<(Option<Direction>, Option<i64>)>,
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
            .map(|direction| LabelText::LabelStr(direction.node_color().into()))
    }
}

impl<'a> GraphWalk<'a, usize, LatencyEdge<'a>> for Problem<'a> {
    fn nodes(&'a self) -> Nodes<'a, usize> {
        (0..self.lc_problem.map_latency_node_to_wire.len()).collect()
    }

    fn edges(&'a self) -> Edges<'a, LatencyEdge<'a>> {
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

        for (_, sm) in self.submodules {
            if sm.instance.get().is_some() {
                continue; // Don't use infer edges for submodules that have been correctly instantiated
            }
            let sm_md = &self.linker.modules[sm.refers_to.id];

            for (_, infer_info, param) in crate::alloc::zip_eq(
                &sm_md.inference_info.parameter_inference_candidates,
                &sm_md.link_info.parameters,
            ) {
                match infer_info {
                    TemplateKind::Type(_t_info) => {}
                    TemplateKind::Value(v_info) => {
                        for c in &v_info.candidates {
                            if let InferenceTarget::PortLatency { from, to } = &c.target {
                                let (Some(from), Some(to)) =
                                    (&sm.port_map[*from], &sm.port_map[*to])
                                else {
                                    continue;
                                };
                                let from =
                                    self.lc_problem.map_wire_to_latency_node[from.maps_to_wire];
                                let to = self.lc_problem.map_wire_to_latency_node[to.maps_to_wire];

                                let edge = LCEdgeType::Infer {
                                    in_submod: &sm.name,
                                    var: &param.name,
                                    offset: i64::try_from(&c.offset).unwrap(),
                                    multiplier: i64::try_from(&c.mul_by).unwrap(),
                                };
                                result.push((from, to, edge));
                            }
                        }
                    }
                }
            }
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
