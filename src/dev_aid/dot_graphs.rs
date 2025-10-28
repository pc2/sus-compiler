use crate::prelude::*;

use std::fmt::Display;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::alloc::UUID;
use crate::instantiation::IsPort;
use crate::to_string::{FmtWrapper, sanitize_filename};
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
fn unique_file_name(module_name: &str, dot_type: &str) -> std::io::Result<PathBuf> {
    let mut path = PathBuf::from("dot_output");
    fs::create_dir_all(&path)?;
    path.push(sanitize_filename(module_name, &format!("_{dot_type}.dot")));
    let mut count = 1;
    while path.exists() {
        path.set_file_name(sanitize_filename(
            module_name,
            &format!("_{dot_type}_{count}.dot"),
        ));
        count += 1;
    }
    Ok(path)
}

fn dot_command(dot_path: &Path, file_type: &str) {
    let output_path = dot_path.with_extension(file_type);
    match std::process::Command::new("dot")
        .arg(format!("-T{file_type}"))
        .arg(dot_path)
        .arg("-o")
        .arg(&output_path)
        .output()
    {
        Ok(output) => {
            if !output.status.success() {
                error!(
                    "Failed to convert {:?} to image: {}",
                    dot_path,
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(e) => {
            error!("Could not run 'dot' to convert {dot_path:?} to image: {e}");
        }
    }
}

fn try_convert_dot_to_image(dot_path: &Path) {
    if crate::debug::is_enabled("dot-svg") {
        dot_command(dot_path, "svg");
    }
    if crate::debug::is_enabled("dot-png") {
        dot_command(dot_path, "png");
    }
}

pub fn display_generated_hardware_structure(md_instance: &ModuleTypingContext<'_>) {
    write_dot_file(
        &md_instance.name,
        "hw_structure",
        custom_render_hardware_structure(md_instance),
    );
}

pub fn write_dot_file(module_name: &str, dot_type: &str, content: impl Display) {
    let path = match unique_file_name(module_name, dot_type) {
        Ok(o) => o,
        Err(e) => {
            error!("Could not create dot_output folder: {e}");
            return;
        }
    };
    let mut file = match File::create_new(&path) {
        Ok(o) => o,
        Err(e) => {
            error!("Could not create {}: {e}", path.to_string_lossy());
            return;
        }
    };
    if let Err(e) = write!(file, "{content}") {
        error!(
            "Could not write the hardware structure to {}: {e}",
            path.to_string_lossy()
        );
        return;
    }
    try_convert_dot_to_image(&path);
}

fn custom_render_hardware_structure<'a>(
    md_instance: &'a ModuleTypingContext<'a>,
) -> impl std::fmt::Display + 'a {
    FmtWrapper(move |f| {
        writeln!(f, "digraph \"{}\" {{", md_instance.mangled_name)?;
        writeln!(f, "    rankdir=LR;")?;
        writeln!(f, "    ranksep=1.5;")?;

        // Emit nodes for wires
        for (_, wire) in &md_instance.wires {
            let name = &wire.name;
            let abs_lat = wire.absolute_latency;
            let label = format!("{name}'{abs_lat}");
            let (style, color) = match wire.is_port {
                IsPort::Port(_, Direction::Input) => ("bold", "red"),
                IsPort::Port(_, Direction::Output) => ("bold", "blue"),
                _ => ("", "black"),
            };
            writeln!(
                f,
                "    \"{name}\" [label=\"{label}\" style={style} color={color}];"
            )?;
        }

        // Emit nodes for submodules
        for (_, sm) in &md_instance.submodules {
            writeln!(
                f,
                "    \"{}\" [label=\"{}\" style=filled];",
                sm.name, sm.name
            )?;
        }

        // Emit edges for wires
        for (id, w) in &md_instance.wires {
            w.source.for_each_wire(&mut |v| {
                let from = &md_instance.wires[v].name;
                let to = &md_instance.wires[id].name;
                writeln!(f, "    \"{from}\" -> \"{to}\";").ok();
            });
        }

        // Emit edges for submodules
        for (_, sm) in &md_instance.submodules {
            let s_md = &md_instance.linker.modules[sm.refers_to.id];
            let sm_name = &sm.name;
            for (port_id, port) in sm.port_map.iter_valids() {
                let w_id = port.maps_to_wire;
                let w_name = &md_instance.wires[w_id].name;
                match s_md.ports[port_id].direction {
                    Direction::Input => {
                        writeln!(f, "    \"{w_name}\" -> \"{sm_name}\";")?;
                    }
                    Direction::Output => {
                        writeln!(f, "    \"{sm_name}\" -> \"{w_name}\";")?;
                    }
                }
            }
        }

        writeln!(f, "}}")?;
        Ok(())
    })
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

    write_dot_file(
        module_name,
        dot_type,
        custom_render_latency_count_graph(
            lc_problem,
            wires,
            submodules,
            linker,
            solution,
            module_name,
        ),
    )
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
                id: format!("n{n}"),
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
                node_ids[idx].valid_parent = Some(UUID::PLACEHOLDER); // Use PLACEHOLDER to refer to the specified nodes of *this* submodule. That way their connection cycle gets omitted from the graph, for cleanlyness. 
            }
            write!(f, "    {id} [label=\"{label}\"")?;
            match wire.is_port {
                IsPort::Port(_, Direction::Input) => {
                    node_ids[idx].id = format!("{id}:e");
                    // Makes the nice rightwards arrow-ey shape
                    writeln!(f, ",shape=cds,style=filled,fillcolor=darkolivegreen3];")
                }
                IsPort::Port(_, Direction::Output) => {
                    node_ids[idx].id = format!("{id}:w");
                    writeln!(f, ",shape=cds,style=filled,fillcolor=skyblue];")
                }
                _ => writeln!(f, ",style=filled,fillcolor=bisque];"),
            }
        };
        for (sm_id, sm) in submodules {
            let sm_md = &linker.modules[sm.refers_to.id];
            if let Some(inst) = sm.instance.get() {
                let inst_name = &inst.name;
                let sm_name = &sm.name;
                let inputs_outputs_per_domain = sm_md.domains.map(|(domain_id, domain)| {
                    let mut inputs = Vec::new();
                    let mut outputs = Vec::new();
                    for (_, p) in inst.interface_ports.iter_valids() {
                        if p.domain != domain_id {
                            continue;
                        }
                        let p_wire = &inst.wires[p.wire];

                        match p.direction {
                            Direction::Input => inputs.push(p_wire),
                            Direction::Output => outputs.push(p_wire),
                        }
                    }
                    (inputs, outputs, &domain.name)
                });

                fn display_port_list<'l>(list: &'l [&'l RealWire]) -> impl Display + 'l {
                    FmtWrapper(move |f| {
                        write!(f, "<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\">")?;
                        for p_wire in list.iter() {
                            let name = &p_wire.name;
                            let abs_lat = &p_wire.absolute_latency;
                            write!(f, "<TR><TD PORT=\"{name}\">{name}'{abs_lat}</TD></TR>")?;
                        }
                        write!(f, "</TABLE>")?;
                        Ok(())
                    })
                }

                if let Some([(inputs, outputs, domain)]) =
                    &inputs_outputs_per_domain.try_cast_to_array()
                {
                    // Just a single domain, simplify print
                    let inputs = display_port_list(inputs);
                    let outputs = display_port_list(outputs);
                    write!(
                        f,
                        "    {sm_id:?}_{domain}[shape=plain,style=filled,fillcolor=bisque,label=<
                            <TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\" BGCOLOR=\"bisque\">
                                <TR><TD COLSPAN=\"3\"><B>{inst_name}</B></TD></TR>
                                <TR><TD CELLPADDING=\"0\" BORDER=\"0\">{inputs}</TD><TD>{sm_name}<BR/>{domain}</TD><TD CELLPADDING=\"0\" BORDER=\"0\">{outputs}</TD></TR>
                            </TABLE>
                        >];"
                    )?;
                } else {
                    writeln!(f, "subgraph cluster_{sm_id:?} {{")?;
                    writeln!(f, "    label=\"{inst_name}\";")?;
                    writeln!(f, "    style=filled;")?;
                    writeln!(f, "    color=lightgrey;")?;
                    for (_, (inputs, outputs, domain)) in &inputs_outputs_per_domain {
                        // Just a single domain, simplify print
                        let inputs = display_port_list(inputs);
                        let outputs = display_port_list(outputs);
                        write!(
                            f,
                            "    {sm_id:?}_{domain}[shape=plain,style=filled,fillcolor=bisque,label=<
                                <TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\" BGCOLOR=\"bisque\">
                                    <TR><TD CELLPADDING=\"0\" BORDER=\"0\">{inputs}</TD><TD>{sm_name}<BR/>{domain}</TD><TD CELLPADDING=\"0\" BORDER=\"0\">{outputs}</TD></TR>
                                </TABLE>
                            >];"
                        )?;
                    }
                    writeln!(f, "}}")?;
                }

                for (_, maps_to, port) in crate::alloc::zip_eq(&sm.port_map, &inst.interface_ports)
                {
                    let (Some(maps_to), Some(port)) = (maps_to, port) else {
                        continue;
                    };
                    let port_domain_name = &sm_md.domains[port.domain].name;
                    let p_name = &inst.wires[port.wire].name;
                    let node =
                        &mut node_ids[lc_problem.map_wire_to_latency_node[maps_to.maps_to_wire]];
                    node.print_separate = false;
                    node.id = format!("{sm_id:?}_{port_domain_name}:{p_name}");
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
                                (1, add) if add < 0 => format!("{param} - {}", -add),
                                (1, add) => format!("{param} + {add}"),
                                (-1, add) if add < 0 => format!("-{param} - {}", -add),
                                (-1, add) => format!("-{param} + {add}"),
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
