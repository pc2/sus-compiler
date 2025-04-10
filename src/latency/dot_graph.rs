use std::{path::PathBuf, str::FromStr};

use dot::{render, Edges, GraphWalk, Id, LabelText, Labeller, Nodes};

use super::{
    latency_algorithm::{
        FanInOut, LatencyCountingPorts, LatencyInferenceCandidate, SpecifiedLatency,
    },
    list_of_lists::ListOfLists,
};

pub fn display_latency_count_graph(
    fanins: &ListOfLists<FanInOut>,
    ports: &LatencyCountingPorts,
    solution: &[i64],
    specified_latencies: &[SpecifiedLatency],
    inference_edges: &[LatencyInferenceCandidate],
    filename: &str,
) {
    // true for input
    let mut extra_node_info = vec![(None, None); fanins.len()];
    for port in ports.inputs() {
        extra_node_info[*port].0 = Some(true);
    }
    for port in ports.outputs() {
        extra_node_info[*port].0 = Some(false);
    }

    for spec in specified_latencies {
        extra_node_info[spec.node].1 = Some(spec.latency);
    }

    let mut file = std::fs::File::create(PathBuf::from_str(filename).unwrap()).unwrap();
    render(
        &Problem {
            fanins,
            extra_node_info,
            solution,
            inference_edges,
        },
        &mut file,
    )
    .unwrap();
}

#[derive(Clone, Copy)]
enum EdgeType {
    Normal(i64),
    Infer(usize),
    Poison,
}

type LatencyEdge = (usize, usize, EdgeType);

struct Problem<'a> {
    fanins: &'a ListOfLists<FanInOut>,
    extra_node_info: Vec<(Option<bool>, Option<i64>)>,
    solution: &'a [i64],
    inference_edges: &'a [LatencyInferenceCandidate],
}

impl<'a> Labeller<'a, usize, LatencyEdge> for Problem<'a> {
    fn graph_id(&'a self) -> Id<'a> {
        Id::new("lcGraph").unwrap()
    }

    fn node_id(&'a self, n: &usize) -> Id<'a> {
        Id::new(format!("n{n}")).unwrap()
    }

    fn node_label(&'a self, n: &usize) -> LabelText<'a> {
        let abs_lat = self.solution[*n];
        let mut result = if abs_lat == i64::MIN {
            String::new()
        } else {
            abs_lat.to_string()
        };
        if let Some(specified) = self.extra_node_info[*n].1 {
            use std::fmt::Write;
            write!(result, " specified {specified}").unwrap();
        }
        LabelText::LabelStr(result.into())
    }

    fn edge_label(&'a self, e: &LatencyEdge) -> LabelText<'a> {
        LabelText::LabelStr(match e.2 {
            EdgeType::Normal(delta) => delta.to_string().into(),
            EdgeType::Infer(var) => format!("Infer {var}").into(),
            EdgeType::Poison => "poison".into(),
        })
    }

    fn edge_color(&'a self, e: &LatencyEdge) -> Option<LabelText<'a>> {
        match e.2 {
            EdgeType::Normal(_) => None,
            EdgeType::Infer(_) => Some(LabelText::LabelStr("green".into())),
            EdgeType::Poison => Some(LabelText::LabelStr("red".into())),
        }
    }

    fn node_color(&'a self, node: &usize) -> Option<LabelText<'a>> {
        self.extra_node_info[*node]
            .0
            .map(|is_input| LabelText::LabelStr(if is_input { "red" } else { "blue" }.into()))
    }
}

impl<'a> GraphWalk<'a, usize, LatencyEdge> for Problem<'a> {
    fn nodes(&'a self) -> Nodes<'a, usize> {
        (0..self.fanins.len()).collect()
    }

    fn edges(&'a self) -> Edges<'a, LatencyEdge> {
        let mut result = Vec::with_capacity(self.fanins.len_nested());

        for (from, fanout) in self.fanins.iter().enumerate() {
            for edge in fanout {
                result.push((
                    from,
                    edge.to_node,
                    if let Some(delta) = edge.delta_latency {
                        EdgeType::Normal(delta)
                    } else {
                        EdgeType::Poison
                    },
                ));
            }
        }

        for infer_edge in self.inference_edges {
            result.push((
                infer_edge.from_node,
                infer_edge.to_node,
                EdgeType::Infer(infer_edge.target_to_infer.get_hidden_value()),
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
