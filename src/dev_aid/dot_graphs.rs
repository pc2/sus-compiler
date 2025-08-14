use std::borrow::Cow;
use std::fs::{self, File};

use dot2::label::Text;
use dot2::{Edges, GraphWalk, Id, Labeller, Nodes, Style, render};

use crate::to_string::{FmtWrapper, join_string_iter_formatter};
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
    let output_path = dot_path.with_extension("svg");
    match std::process::Command::new("dot")
        .arg("-Tsvg")
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
pub enum NodeType {
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

pub type EdgeType = (NodeType, NodeType);

impl<'inst> Labeller<'inst> for ModuleTypingContext<'_> {
    type Node = NodeType;
    type Edge = EdgeType;
    type Subgraph = ();

    fn graph_id(&'inst self) -> dot2::Result<Id<'inst>> {
        Id::new(&self.mangled_name)
    }

    fn node_id(&'inst self, n: &NodeType) -> dot2::Result<Id<'inst>> {
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
    }

    fn node_label(&'inst self, n: &NodeType) -> dot2::Result<Text<'inst>> {
        Ok(Text::LabelStr(match *n {
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
        }))
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

    fn node_color<'a>(&'a self, n: &NodeType) -> Option<Text<'inst>> {
        match n {
            NodeType::Wire(w_id) => self.wires[*w_id]
                .is_port
                .map(|d| Text::LabelStr(Cow::Borrowed(d.node_color()))),
            NodeType::SubModule(_) => None,
        }
    }
}

impl<'inst> GraphWalk<'inst> for ModuleTypingContext<'_> {
    type Node = NodeType;
    type Edge = EdgeType;
    type Subgraph = ();

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

    use std::io::Write;
    write!(
        file,
        "{}",
        custom_render_latency_count_graph(
            lc_problem,
            wires,
            submodules,
            linker,
            solution,
            module_name
        )
    )
    .unwrap();
    try_convert_dot_to_image(&path);
}

struct NodeId {
    id: String,
    valid_parent: Option<SubModuleID>,
    print_separate: bool,
}

fn custom_render_latency_count_graph(
    lc_problem: &LatencyCountingProblem,
    wires: &FlatAlloc<RealWire, WireIDMarker>,
    submodules: &FlatAlloc<SubModule, SubModuleIDMarker>,
    linker: &Linker,
    solution: Option<&[i64]>,
    graph_name: &str,
) -> impl std::fmt::Display {
    FmtWrapper(move |f| {
        let digraph_name = graph_name;
        writeln!(f, "digraph \"{digraph_name}\" {{")?;
        writeln!(f, "    rankdir=LR;")?;
        writeln!(f, "    ranksep=1.5;")?;
        // writeln!(f, "    node [shape=ellipse];")?;

        // Generate all node ids and labels first
        let mut node_ids: Vec<NodeId> = (0..lc_problem.map_latency_node_to_wire.len())
            .map(|n| NodeId {
                id: format!("n{}", n),
                valid_parent: None,
                print_separate: true,
            })
            .collect();

        let write_wire = |f: &mut std::fmt::Formatter, wire_id: WireID, node_ids: &mut [NodeId]| {
            let wire = &wires[wire_id];
            let name = &wire.name;
            let idx = lc_problem.map_wire_to_latency_node[wire_id];
            let id = &node_ids[idx].id;
            let mut label = name.to_string();
            if let Some(sol) = solution {
                let sol = sol[idx];

                use std::fmt::Write;
                if sol != i64::MIN && sol != i64::MAX {
                    write!(label, "'{sol}")?;
                } else {
                    write!(label, "'?")?;
                }
            }
            if let Some(specified) = wire.specified_latency.get() {
                use std::fmt::Write as _;
                write!(label, " specified {specified}").unwrap();
            }
            write!(f, "    {id} [label=\"{label}\"")?;
            match wire.is_port {
                Some(Direction::Input) => {
                    node_ids[idx].id = format!("{id}:e");
                    // Makes the nice rightwards arrow-ey shape
                    writeln!(f, ",shape=cds,style=filled,fillcolor=darkolivegreen3];")
                }
                Some(Direction::Output) => {
                    node_ids[idx].id = format!("{id}:w");
                    writeln!(f, ",shape=cds,style=filled,fillcolor=skyblue];")
                }
                None => writeln!(f, ",style=filled,fillcolor=bisque];"),
            }
        };
        for (sm_id, sm) in submodules {
            let sm_md = &linker.modules[sm.refers_to.id];
            if let Some(inst) = sm.instance.get() {
                let inst_name = &inst.name;
                let sm_name = &sm.name;
                let mut inputs = Vec::new();
                let mut outputs = Vec::new();
                for (_, p) in inst.interface_ports.iter_valids() {
                    let p_wire = &inst.wires[p.wire];

                    match p.direction {
                        Direction::Input => inputs.push(p_wire),
                        Direction::Output => outputs.push(p_wire),
                    }
                }

                write!(
                    f,
                    "    {sm_id:?}[shape=record,style=filled,fillcolor=bisque,label=\"{inst_name} | {{"
                )?;
                if !inputs.is_empty() {
                    write!(f, " {{ ")?;
                    join_string_iter_formatter(" | ", f, &inputs, |p_wire, f| {
                        let name = &p_wire.name;
                        let abs_lat = &p_wire.absolute_latency;
                        write!(f, "<{name}> {name}'{abs_lat}")
                    })?;
                    write!(f, " }} |")?;
                }
                write!(f, " {sm_name} ")?;
                if !outputs.is_empty() {
                    write!(f, "| {{ ")?;
                    join_string_iter_formatter(" | ", f, &outputs, |p_wire, f| {
                        let name = &p_wire.name;
                        let abs_lat = &p_wire.absolute_latency;
                        write!(f, "<{name}> {name}'{abs_lat}")
                    })?;
                    write!(f, " }} ")?;
                }
                writeln!(f, "}}\"];")?;

                for (_, maps_to, port) in crate::alloc::zip_eq(&sm.port_map, &inst.interface_ports)
                {
                    let (Some(maps_to), Some(port)) = (maps_to, port) else {
                        continue;
                    };
                    let p_name = &inst.wires[port.wire].name;
                    let node =
                        &mut node_ids[lc_problem.map_wire_to_latency_node[maps_to.maps_to_wire]];
                    node.print_separate = false;
                    node.id = format!("{sm_id:?}:{p_name}");
                    node.valid_parent = Some(sm_id);
                }
            } else {
                let failed_sm_name = sm.refers_to.display(&linker.globals);
                writeln!(f, "subgraph cluster_{sm_id:?} {{")?;
                writeln!(f, "    label=\"{failed_sm_name}\";")?;
                writeln!(f, "    style=filled;")?;
                writeln!(f, "    color=lightgrey;")?;

                writeln!(f, "    {{ rank=same;")?;
                for (port_id, port) in sm.port_map.iter_valids() {
                    if sm_md.ports[port_id].direction == Direction::Input {
                        let idx = lc_problem.map_wire_to_latency_node[port.maps_to_wire];
                        node_ids[idx].print_separate = false;
                        write_wire(f, port.maps_to_wire, &mut node_ids)?;
                    }
                }
                writeln!(f, "    }}")?;
                writeln!(f, "    {{ rank=same;")?;
                for (port_id, port) in sm.port_map.iter_valids() {
                    if sm_md.ports[port_id].direction == Direction::Output {
                        let idx = lc_problem.map_wire_to_latency_node[port.maps_to_wire];
                        node_ids[idx].print_separate = false;
                        write_wire(f, port.maps_to_wire, &mut node_ids)?;
                    }
                }
                writeln!(f, "    }}")?;

                writeln!(f, "}}")?;
            }
        }

        for (wire_id, _) in wires {
            let idx = lc_problem.map_wire_to_latency_node[wire_id];
            if !node_ids[idx].print_separate {
                continue;
            }
            write_wire(f, wire_id, &mut node_ids)?;
        }

        // Edges (normal and poison)
        for (to, fan_from) in &lc_problem.edges {
            let from = fan_from.to_node;

            let NodeId {
                id: to_id,
                valid_parent: to_submod,
                ..
            } = &node_ids[*to];
            let NodeId {
                id: from_id,
                valid_parent: from_submod,
                ..
            } = &node_ids[from];

            // Skip edges withing entirely known modules
            if let (Some(to_submod), Some(from_submod)) = (to_submod, from_submod)
                && to_submod == from_submod
            {
                continue;
            }

            match fan_from.delta_latency {
                Some(0) => {
                    writeln!(f, "    {from_id} -> {to_id};")?;
                }
                Some(delta) => {
                    writeln!(f, "    {from_id} -> {to_id} [label={delta}];")?;
                }
                None => {
                    writeln!(f, "    {from_id} -> {to_id} [label=poison, color=red];")?;
                }
            }
        }

        // Inference edges (green)
        for (_, sm) in submodules {
            if sm.instance.get().is_some() {
                continue;
            }
            let sm_md = &linker.modules[sm.refers_to.id];
            for (_, infer_info, param) in crate::alloc::zip_eq(
                &sm_md.inference_info.parameter_inference_candidates,
                &sm_md.link_info.parameters,
            ) {
                if let TemplateKind::Value(v_info) = infer_info {
                    for c in &v_info.candidates {
                        if let InferenceTarget::PortLatency { from, to } = &c.target {
                            let (Some(from), Some(to)) = (&sm.port_map[*from], &sm.port_map[*to])
                            else {
                                continue;
                            };
                            let from_idx = lc_problem.map_wire_to_latency_node[from.maps_to_wire];
                            let to_idx = lc_problem.map_wire_to_latency_node[to.maps_to_wire];
                            let from_id = &node_ids[from_idx].id;
                            let to_id = &node_ids[to_idx].id;
                            let param = &param.name;
                            let mul = i64::try_from(&c.mul_by).unwrap();
                            let add = i64::try_from(&c.offset).unwrap();
                            let label = match (mul, add) {
                                (1, 0) => param.to_string(),
                                (-1, 0) => format!("-{param}"),
                                (1, add) if add < 0 => format!("-{param} - {}", -add),
                                (1, add) => format!("-{param} + {add}"),
                                (mul, add) if add < 0 => format!("{mul} * {param} - {}", -add),
                                (mul, add) => format!("{mul} * {param} + {add}"),
                            };
                            writeln!(
                                f,
                                "    {from_id} -> {to_id} [label=\"{label}\", color=green];"
                            )?;
                        }
                    }
                }
            }
        }

        writeln!(f, "}}")?;
        Ok(())
    })
}
