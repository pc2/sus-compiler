use crate::flattening::{Direction, FieldDeclKind, InterfaceKind, Module};
use crate::latency::port_latency_inference::InferenceTarget;
use crate::prelude::*;
use crate::typing::template::TemplateKind;
use std::collections::{HashMap, HashSet};

// ── Layout constants ────────────────────────────────────────────────────────
const BOX_W: i32 = 140;
const STICK: i32 = 28;
const ROW_H: i32 = 22;
const SEG_PAD_TOP: i32 = 16;
const SEG_PAD_BOT: i32 = 5;
const LABEL_H: i32 = 14; // domain-name label at bottom of segment
const CLOCK_H: i32 = 14; // "clock X" label at top of clock-boundary segment
const DOUBLE_RULE_HEIGHT: i32 = 4;
const TITLE_H: i32 = 24;
const MARGIN: i32 = 12;
const LW: i32 = 100; // space reserved for port-name labels on each side
const GROUP_GAP: i32 = 5; // extra vertical gap between action/trigger groups

fn svg_esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn port_y_list(ports: &[(PortID, usize)], start_y: i32) -> (Vec<(PortID, i32)>, i32) {
    let mut result = Vec::new();
    let mut y = start_y;
    let mut last_group = usize::MAX;
    for &(pid, group) in ports {
        if group != last_group && last_group != usize::MAX {
            y += GROUP_GAP;
        }
        result.push((pid, y));
        y += ROW_H;
        last_group = group;
    }
    (result, y)
}

pub fn render_port_diagram(md: &Module, show_poison: bool) -> String {
    let box_left: i32 = MARGIN + LW + STICK;
    let box_right: i32 = box_left + BOX_W;
    let box_cx: i32 = (box_left + box_right) / 2;
    let svg_w: i32 = box_right + STICK + LW + MARGIN;
    let box_top: i32 = MARGIN + TITLE_H;

    // ── 1. Build PortID → group index ──────────────────────────────────────
    // Ports belonging to the same action/trigger or standalone declaration
    // share a group index; a GROUP_GAP is added between groups.
    let mut port_group: HashMap<PortID, usize> = HashMap::new();
    let mut g = 0usize;
    for (_, field) in &md.fields {
        let Some(decl_kind) = field.declaration_instruction else {
            g += 1;
            continue;
        };
        match decl_kind {
            FieldDeclKind::SinglePort(decl_id) => {
                let (pid, _) = md.get_port_for_decl(decl_id);
                port_group.insert(pid, g);
            }
            FieldDeclKind::Interface(iface_id) => {
                let iface = md.link_info.instructions[iface_id].unwrap_interface();
                // The control (valid) port of an action/trigger
                if let InterfaceKind::Action(pid) | InterfaceKind::Trigger(pid) =
                    iface.interface_kind
                {
                    port_group.insert(pid, g);
                }
                for &id in iface.inputs.iter().chain(iface.outputs.iter()) {
                    let (pid, _) = md.get_port_for_decl(id);
                    port_group.insert(pid, g);
                }
            }
        }
        g += 1;
    }

    // ── 2. Bucket ports by domain ───────────────────────────────────────────
    let mut dom_in: HashMap<LatDomID, Vec<(usize, PortID, usize)>> = HashMap::new();
    let mut dom_out: HashMap<LatDomID, Vec<(usize, PortID, usize)>> = HashMap::new();
    for (port_id, port) in &md.ports {
        let grp = *port_group.get(&port_id).unwrap_or(&usize::MAX);
        let pos = port.name_span.start;
        match port.direction {
            Direction::Input => dom_in
                .entry(port.lat_dom)
                .or_default()
                .push((pos, port_id, grp)),
            Direction::Output => dom_out
                .entry(port.lat_dom)
                .or_default()
                .push((pos, port_id, grp)),
        }
    }
    for v in dom_in.values_mut() {
        v.sort_by_key(|&(pos, _, _)| pos);
    }
    for v in dom_out.values_mut() {
        v.sort_by_key(|&(pos, _, _)| pos);
    }

    // ── 3. Compute per-segment layout ───────────────────────────────────────
    struct Seg {
        dom_name: String,
        clock_name: Option<String>,
        seg_top: i32,
        seg_bot: i32,
        input_ys: Vec<(PortID, i32)>,
        output_ys: Vec<(PortID, i32)>,
    }

    let mut segs: Vec<Seg> = Vec::new();
    let mut prev_clock: Option<ClockID> = None;
    let mut y = box_top;

    for (dom_id, dom_info) in &md.latency_domains {
        let clock_id = dom_info.clock;
        let clock_info = &md.clocks[clock_id];
        let is_clock_boundary = prev_clock != Some(clock_id);
        let clock_name = is_clock_boundary.then(|| clock_info.name.clone());

        let inputs: Vec<(PortID, usize)> = dom_in
            .get(&dom_id)
            .map(|v| v.iter().map(|&(_, pid, g)| (pid, g)).collect())
            .unwrap_or_default();
        let outputs: Vec<(PortID, usize)> = dom_out
            .get(&dom_id)
            .map(|v| v.iter().map(|&(_, pid, g)| (pid, g)).collect())
            .unwrap_or_default();

        let seg_top = y;
        let top_extra = if clock_name.is_some() { CLOCK_H } else { 0 };
        let content_start = seg_top + SEG_PAD_TOP + top_extra;

        let (input_ys, in_end) = port_y_list(&inputs, content_start);
        let (output_ys, out_end) = port_y_list(&outputs, content_start);

        // in_end/out_end are ROW_H past the last port center; subtract to get the actual last row
        let last_row = (in_end.max(out_end) - ROW_H).max(content_start);
        let seg_bot = last_row + SEG_PAD_BOT + LABEL_H;
        y = seg_bot;
        if clock_name.is_some() {
            y += DOUBLE_RULE_HEIGHT; // Adjust for the double rule
        }

        segs.push(Seg {
            dom_name: dom_info.name.clone(),
            clock_name,
            seg_top,
            seg_bot,
            input_ys,
            output_ys,
        });
        prev_clock = Some(clock_id);
    }

    if segs.is_empty() {
        return String::new();
    }

    if segs.last().unwrap().clock_name.is_some() {
        y -= DOUBLE_RULE_HEIGHT; // Compensate for non-existing final double rule. 
    }

    let box_bot = y;
    let svg_h = box_bot + MARGIN;

    // Port → y-coordinate lookup (used for edge drawing)
    let mut port_y: HashMap<PortID, i32> = HashMap::new();
    for seg in &segs {
        for &(pid, py) in &seg.input_ys {
            port_y.insert(pid, py);
        }
        for &(pid, py) in &seg.output_ys {
            port_y.insert(pid, py);
        }
    }

    // ── 4. Collect edges ────────────────────────────────────────────────────
    struct Edge {
        from_y: i32,
        to_y: i32,
        color: &'static str,
        label: Option<String>,
    }
    let mut edges: Vec<Edge> = Vec::new();

    // Green: latency-inference edges from Module::inference_info
    for (_, infer_info, param) in crate::alloc::zip_eq(
        &md.inference_info.parameter_inference_candidates,
        &md.link_info.parameters,
    ) {
        let TemplateKind::Value(v_info) = infer_info else {
            continue;
        };
        for c in &v_info.candidates {
            let InferenceTarget::PortLatency {
                from_input,
                to_output,
            } = &c.target
            else {
                continue;
            };
            let (Some(&from_y), Some(&to_y)) = (port_y.get(from_input), port_y.get(to_output))
            else {
                continue;
            };
            let label = c.display_formula(&param.name).to_string();
            edges.push(Edge {
                from_y,
                to_y,
                color: "#22a85a",
                label: Some(label),
            });
        }
    }

    // Red: poison edges — ports with no explicit latency specifier and not covered
    // by any inference edge have unresolvable latency.
    if show_poison {
        let mut covered: HashSet<PortID> = HashSet::new();
        for e in &edges {
            // Recover port IDs from their y positions (reverse lookup)
            for (pid, &py) in &port_y {
                if py == e.from_y || py == e.to_y {
                    covered.insert(*pid);
                }
            }
        }
        for (dom_id, _) in &md.latency_domains {
            let mut bad_in: Vec<i32> = Vec::new();
            let mut bad_out: Vec<i32> = Vec::new();
            for (port_id, port) in &md.ports {
                if port.lat_dom != dom_id
                    || port.latency_specifier.is_some()
                    || covered.contains(&port_id)
                {
                    continue;
                }
                match port.direction {
                    Direction::Input => {
                        if let Some(&py) = port_y.get(&port_id) {
                            bad_in.push(py);
                        }
                    }
                    Direction::Output => {
                        if let Some(&py) = port_y.get(&port_id) {
                            bad_out.push(py);
                        }
                    }
                }
            }
            for &fy in &bad_in {
                for &ty in &bad_out {
                    edges.push(Edge {
                        from_y: fy,
                        to_y: ty,
                        color: "#e53e3e",
                        label: None,
                    });
                }
            }
        }
    }

    // ── 5. Render SVG ───────────────────────────────────────────────────────
    let mut s = String::new();
    let title = svg_esc(&md.link_info.name);

    s.push_str(&format!(
        "<figure class=\"port-diagram\"><svg xmlns=\"http://www.w3.org/2000/svg\" \
         width=\"{svg_w}\" height=\"{svg_h}\" \
         font-family=\"'Fira Code',monospace\" font-size=\"11\">\n"
    ));

    // Title
    s.push_str(&format!(
        "<text x=\"{box_cx}\" y=\"{}\" text-anchor=\"middle\" \
         font-size=\"13\" font-weight=\"bold\" fill=\"#1a1227\">{title}</text>\n",
        MARGIN + 16
    ));

    // Module box
    s.push_str(&format!(
        "<rect x=\"{box_left}\" y=\"{box_top}\" width=\"{BOX_W}\" height=\"{}\" \
         fill=\"#f7f6fb\" stroke=\"#c0b8d4\" stroke-width=\"1.5\" rx=\"2\"/>\n",
        box_bot - box_top
    ));

    // Separator rules between segments
    for i in 0..segs.len().saturating_sub(1) {
        let ry = segs[i].seg_bot;
        if segs[i + 1].clock_name.is_some() {
            // Double rule for clock boundary
            for offset in [0, 4] {
                s.push_str(&format!(
                    "<line x1=\"{box_left}\" y1=\"{}\" x2=\"{box_right}\" y2=\"{}\" \
                     stroke=\"#a098bb\" stroke-width=\"1.5\"/>\n",
                    ry + offset,
                    ry + offset
                ));
            }
        } else {
            s.push_str(&format!(
                "<line x1=\"{box_left}\" y1=\"{ry}\" x2=\"{box_right}\" y2=\"{ry}\" \
                 stroke=\"#c0b8d4\" stroke-width=\"1\"/>\n"
            ));
        }
    }

    // Per-segment annotations
    for seg in &segs {
        // Domain name: bottom-right inside segment
        s.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"end\" fill=\"#9b96aa\" font-size=\"10\">{}</text>\n",
            box_right - 4,
            seg.seg_bot - 5,
            svg_esc(&seg.dom_name)
        ));
        // Clock name: top-left of first segment in a new clock group
        if let Some(cname) = &seg.clock_name {
            s.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" fill=\"#5a5370\" font-size=\"10\">clock {}</text>\n",
                box_left + 4,
                seg.seg_top + 14,
                svg_esc(cname)
            ));
        }
    }

    // Latency edges (drawn before stubs so stubs render on top)
    for e in &edges {
        s.push_str(&format!(
            "<path d=\"M {box_left},{from_y} C {box_cx},{from_y} {box_cx},{to_y} {box_right},{to_y}\" \
             stroke=\"{c}\" fill=\"none\" stroke-width=\"1.2\" opacity=\"0.8\"/>\n",
            from_y = e.from_y,
            to_y = e.to_y,
            c = e.color,
        ));
        if let Some(ref lbl) = e.label {
            let mid_y = (e.from_y + e.to_y) / 2 - 3;
            s.push_str(&format!(
                "<text x=\"{box_cx}\" y=\"{mid_y}\" text-anchor=\"middle\" \
                 font-size=\"10\" fill=\"{c}\">{}</text>\n",
                svg_esc(lbl),
                c = e.color,
            ));
        }
    }

    // Port stubs and labels
    for seg in &segs {
        for &(pid, py) in &seg.input_ys {
            let nm = svg_esc(&md.ports[pid].name);
            s.push_str(&format!(
                "<line x1=\"{}\" y1=\"{py}\" x2=\"{box_left}\" y2=\"{py}\" \
                 stroke=\"#555\" stroke-width=\"1.5\"/>\n",
                box_left - STICK
            ));
            s.push_str(&format!(
                "<text x=\"{}\" y=\"{py}\" text-anchor=\"end\" \
                 dominant-baseline=\"middle\" fill=\"#1a1227\">{nm}</text>\n",
                box_left - STICK - 4
            ));
        }
        for &(pid, py) in &seg.output_ys {
            let nm = svg_esc(&md.ports[pid].name);
            s.push_str(&format!(
                "<line x1=\"{box_right}\" y1=\"{py}\" x2=\"{}\" y2=\"{py}\" \
                 stroke=\"#555\" stroke-width=\"1.5\"/>\n",
                box_right + STICK
            ));
            s.push_str(&format!(
                "<text x=\"{}\" y=\"{py}\" dominant-baseline=\"middle\" fill=\"#1a1227\">{nm}</text>\n",
                box_right + STICK + 4
            ));
        }
    }

    s.push_str("</svg></figure>\n");
    s
}
