use crate::config::GenDocs;
use crate::dev_aid::port_diagram;
use crate::file_position::FileText;
use crate::flattening::{
    ClockVisibility, DeclarationKind, FieldDeclKind, InterfaceDeclaration, InterfaceKind, Module,
    NamedConstant, StructType,
};
use crate::latency::port_latency_inference::InferenceTarget;
use crate::linker::{FileData, GlobalObj, IsExtern, LinkInfo};
use crate::prelude::*;
use crate::typing::template::TemplateKind;
use pulldown_cmark::{Options, Parser, html::push_html as md_push_html};
use std::collections::HashMap;
use std::path::Path;

/// Maps module name → file stem for cross-page link resolution.
type ModuleIndex = HashMap<String, String>;

pub fn gen_docs(linker: &Linker, settings: &GenDocs) {
    let docs_dir = &settings.dir;
    if let Err(e) = std::fs::create_dir_all(docs_dir) {
        fatal_exit!("Could not create docs/ directory: {e}");
    }

    // First pass: build index of all module names → file stems, and collect all stems.
    let mut index: ModuleIndex = HashMap::new();
    let mut all_stems: Vec<String> = Vec::new();
    for (_, file_data) in &linker.files {
        let stem = file_stem_of(&file_data.file_identifier.name);
        let mut has_modules = false;
        for uuid in &file_data.associated_values {
            if let GlobalObj::Module(id) = uuid {
                index.insert(
                    linker.globals.modules[*id].link_info.name.clone(),
                    stem.clone(),
                );
                has_modules = true;
            }
        }
        if has_modules {
            all_stems.push(stem);
        }
    }
    all_stems.sort();
    all_stems.dedup();

    // Second pass: generate one HTML file per .sus file.
    for (_, file_data) in &linker.files {
        let stem = file_stem_of(&file_data.file_identifier.name);

        let html = generate_file_html(&stem, linker, file_data, &index, &all_stems, &settings.host);
        let out_path = docs_dir.join(format!("{stem}.html"));
        match std::fs::write(&out_path, &html) {
            Ok(()) => info!("Generated {}", out_path.display()),
            Err(e) => error!("Could not write {}: {e}", out_path.display()),
        }
    }
}

fn file_stem_of(path: &str) -> String {
    Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_owned()
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn fmt_latency(li: &LinkInfo, lat_spec: &Option<FlatID>, ft: &FileText) -> String {
    if let Some(lat_id) = lat_spec {
        let expr = li.instructions[*lat_id].unwrap_expression();
        format!("'{}", &ft[expr.span])
    } else {
        String::new()
    }
}

fn fmt_param(li: &LinkInfo, decl_id: FlatID, ft: &FileText) -> String {
    let decl = li.instructions[decl_id].unwrap_declaration();
    let typ = &ft[decl.typ_expr.get_span()];
    let lat = fmt_latency(li, &decl.latency_specifier, ft);
    format!("{typ} {}{lat}", decl.name)
}

fn build_interface_params(li: &LinkInfo, iface: &InterfaceDeclaration, ft: &FileText) -> String {
    if iface.inputs.is_empty() && iface.outputs.is_empty() {
        return String::new();
    }
    let mut params = String::from(" :");
    if !iface.inputs.is_empty() {
        params.push(' ');
        let parts: Vec<String> = iface
            .inputs
            .iter()
            .map(|&id| fmt_param(li, id, ft))
            .collect();
        params.push_str(&parts.join(", "));
    }
    if !iface.outputs.is_empty() {
        params.push_str(" -> ");
        let parts: Vec<String> = iface
            .outputs
            .iter()
            .map(|&id| fmt_param(li, id, ft))
            .collect();
        params.push_str(&parts.join(", "));
    }
    params
}

fn build_interface_block(md: &Module, ft: &FileText) -> String {
    let li = &md.link_info;
    let name = li.display_full_name_and_args::<true>(ft);
    let mut out = format!("module {name} {{\n");

    let multiple_domains = md.latency_domains.len() > 1;

    // Group fields by latency domain, preserving source order within each group.
    let mut domain_fields: HashMap<LatDomID, Vec<(usize, String)>> = HashMap::new();

    for (_, field) in &md.fields {
        let Some(decl_instr) = field.declaration_instruction else {
            continue;
        };
        let (lat_dom, line) = match decl_instr {
            FieldDeclKind::SinglePort(decl_id) => {
                let decl = li.instructions[decl_id].unwrap_declaration();
                let DeclarationKind::Port { latency_domain, .. } = decl.decl_kind else {
                    continue;
                };
                (
                    latency_domain,
                    format!("    {}\n", li.display_decl(None, decl, ft)),
                )
            }
            FieldDeclKind::Interface(iface_id) => {
                let iface = li.instructions[iface_id].unwrap_interface();
                if iface.is_local {
                    continue;
                }
                let kw = match iface.interface_kind {
                    InterfaceKind::RegularInterface => "interface",
                    InterfaceKind::Action(_) => "action",
                    InterfaceKind::Trigger(_) => "trigger",
                };
                let lat = fmt_latency(li, &iface.latency_specifier, ft);
                let params = build_interface_params(li, iface, ft);
                (
                    iface.latency_domain,
                    format!("    {kw} {}{lat}{params}\n", iface.name),
                )
            }
        };
        domain_fields
            .entry(lat_dom)
            .or_default()
            .push((field.name_span.start, line));
    }

    for fields in domain_fields.values_mut() {
        fields.sort_by_key(|(pos, _)| *pos);
    }

    // Emit in domain order: clock header when clock changes, domain header when multiple domains.
    let mut prev_clock: Option<ClockID> = None;
    for (dom_id, dom_info) in &md.latency_domains {
        if prev_clock != Some(dom_info.clock) {
            let clock = &md.clocks[dom_info.clock];
            match clock.visibility {
                ClockVisibility::Input => {
                    if clock.name_span.is_some() {
                        out.push_str(&format!("  clock {}\n", clock.name));
                    }
                }
                ClockVisibility::Output => {
                    out.push_str(&format!("  output clock {}\n", clock.name));
                }
                ClockVisibility::Local => {
                    unreachable!("Local clocks cannot be visible on the interface")
                }
            }
            prev_clock = Some(dom_info.clock);
        }
        if multiple_domains {
            out.push_str(&format!("   domain {}\n", dom_info.name));
        }
        if let Some(fields) = domain_fields.get(&dom_id) {
            for (_, line) in fields {
                out.push_str(line);
            }
        }
    }

    out.push('}');
    out
}

/// Resolve a `[Name]` or `[Module::action]` reference to an href.
/// Uses the first `::` segment as the module name for cross-file lookup.
fn resolve_ref_href(name: &str, current_stem: &str, index: &ModuleIndex) -> String {
    let module_part = name.split("::").next().unwrap_or(name);
    match index.get(module_part) {
        Some(stem) if stem == current_stem => format!("#{module_part}"),
        Some(stem) => format!("{stem}.html#{module_part}"),
        None => format!("#{module_part}"),
    }
}

/// Convert `[Name]` wiki-style references to proper Markdown links before parsing.
fn preprocess_refs(raw: &str, current_stem: &str, index: &ModuleIndex) -> String {
    let mut result = String::new();
    let mut rest = raw;
    while let Some(open) = rest.find('[') {
        result.push_str(&rest[..open]);
        rest = &rest[open + 1..];
        if let Some(close) = rest.find(']') {
            let name = &rest[..close];
            rest = &rest[close + 1..];
            if rest.starts_with('(') {
                // Already a proper Markdown link — keep as-is.
                result.push('[');
                result.push_str(name);
                result.push(']');
            } else {
                let href = resolve_ref_href(name, current_stem, index);
                result.push_str(&format!("[{name}]({href})"));
            }
        } else {
            result.push('[');
            result.push_str(rest);
            rest = "";
        }
    }
    result.push_str(rest);
    result
}

fn render_prose(raw: &str, current_stem: &str, index: &ModuleIndex) -> String {
    let md = preprocess_refs(raw, current_stem, index);
    let opts =
        Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES | Options::ENABLE_SMART_PUNCTUATION;
    let parser = Parser::new_ext(&md, opts);
    let mut html = String::new();
    md_push_html(&mut html, parser);
    html
}

fn has_port_latency_inference(md: &Module) -> bool {
    md.inference_info
        .parameter_inference_candidates
        .iter()
        .any(|(_, kind)| {
            if let TemplateKind::Value(v) = kind {
                v.candidates
                    .iter()
                    .any(|c| matches!(c.target, InferenceTarget::PortLatency { .. }))
            } else {
                false
            }
        })
}

fn render_module_section(
    md: &Module,
    ft: &FileText,
    current_stem: &str,
    index: &ModuleIndex,
) -> String {
    let li = &md.link_info;
    let name = &li.name;
    let raw_doc = li.documentation.to_string(ft);

    let module_name = li.display_full_name_and_args::<true>(ft);
    let heading = html_escape(&module_name.to_string());

    let mut s = format!("<section class=\"module\" id=\"{name}\">\n");
    s.push_str(&format!(
        "<div class=\"module-heading\"><h2>{heading}</h2></div>\n"
    ));

    let interface = build_interface_block(md, ft);
    s.push_str("<div class=\"interface-block\"><pre><code class=\"language-sus\">");
    s.push_str(&html_escape(&interface));
    s.push_str("</code></pre></div>\n");

    let show_poison = has_port_latency_inference(md);
    s.push_str(&port_diagram::render_port_diagram(md, show_poison));

    if !raw_doc.trim().is_empty() {
        s.push_str("<div class=\"doc-prose\">\n");
        s.push_str(&render_prose(&raw_doc, current_stem, index));
        s.push_str("</div>\n");
    }

    s.push_str("</section>\n");
    s
}

fn build_struct_block(typ: &StructType, ft: &FileText) -> String {
    let li = &typ.link_info;
    let name = li.display_full_name_and_args::<true>(ft);
    if matches!(li.is_extern, IsExtern::Builtin) {
        format!("__builtin__ struct {name}")
    } else {
        let mut out = format!("struct {name} {{\n",);
        let mut items: Vec<(usize, String)> = Vec::new();
        for (_, field) in &typ.fields {
            let decl = li.instructions[field.declaration_instruction].unwrap_declaration();
            items.push((
                field.name_span.start,
                format!("    {}\n", li.display_decl(None, decl, ft)),
            ));
        }
        items.sort_by_key(|(pos, _)| *pos);
        for (_, line) in items {
            out.push_str(&line);
        }
        out.push('}');
        out
    }
}

fn render_struct_section(
    typ: &StructType,
    ft: &FileText,
    current_stem: &str,
    index: &ModuleIndex,
) -> String {
    let li = &typ.link_info;
    let name = &li.name;
    let raw_doc = li.documentation.to_string(ft);
    let struct_name = li.display_full_name_and_args::<true>(ft);
    let heading = html_escape(&struct_name.to_string());
    let mut s = format!("<section class=\"module\" id=\"{name}\">\n");
    s.push_str(&format!(
        "<div class=\"module-heading\"><h2>{heading}</h2></div>\n"
    ));
    let block = build_struct_block(typ, ft);
    s.push_str("<div class=\"interface-block\"><pre><code class=\"language-sus\">");
    s.push_str(&html_escape(&block));
    s.push_str("</code></pre></div>\n");
    if !raw_doc.trim().is_empty() {
        s.push_str("<div class=\"doc-prose\">\n");
        s.push_str(&render_prose(&raw_doc, current_stem, index));
        s.push_str("</div>\n");
    }
    s.push_str("</section>\n");
    s
}

fn build_const_block(cst: &NamedConstant, ft: &FileText) -> String {
    let li = &cst.link_info;
    let decl = li.instructions[cst.output_decl].unwrap_declaration();
    let return_type = &ft[decl.typ_expr.get_span()];
    let const_name = li.display_full_name_and_args::<true>(ft);
    format!("const {return_type} {const_name}")
}

fn render_const_section(
    cst: &NamedConstant,
    ft: &FileText,
    current_stem: &str,
    index: &ModuleIndex,
) -> String {
    let li = &cst.link_info;
    let name = &li.name;
    let raw_doc = li.documentation.to_string(ft);
    let heading = html_escape(&li.display_full_name_and_args::<true>(ft).to_string());
    let mut s = format!("<section class=\"module\" id=\"{name}\">\n");
    s.push_str(&format!(
        "<div class=\"module-heading\"><h2>{heading}</h2></div>\n"
    ));
    let block = build_const_block(cst, ft);
    s.push_str("<div class=\"interface-block\"><pre><code class=\"language-sus\">");
    s.push_str(&html_escape(&block));
    s.push_str("</code></pre></div>\n");
    if !raw_doc.trim().is_empty() {
        s.push_str("<div class=\"doc-prose\">\n");
        s.push_str(&render_prose(&raw_doc, current_stem, index));
        s.push_str("</div>\n");
    }
    s.push_str("</section>\n");
    s
}

fn generate_file_html(
    file_stem: &str,
    linker: &Linker,
    file_data: &FileData,
    index: &ModuleIndex,
    all_stems: &[String],
    host: &str,
) -> String {
    let mut has_modules = false;
    let mut has_structs = false;
    let mut has_consts = false;

    for obj_id in &file_data.associated_values {
        match obj_id {
            GlobalObj::Module(_) => has_modules = true,
            GlobalObj::Type(_) => has_structs = true,
            GlobalObj::Constant(_) => has_consts = true,
        }
    }
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("  <meta charset=\"UTF-8\"/>\n  <meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"/>\n");
    html.push_str(&format!(
        "  <title>{file_stem} — SUS Documentation</title>\n"
    ));
    html.push_str(&format!(
        "  <link rel=\"stylesheet\" href=\"{host}/docs/highlight.css\"/>\n"
    ));
    html.push_str(&format!(
        "  <link rel=\"stylesheet\" href=\"{host}/susdoc.css\"/>\n"
    ));
    html.push_str("</head>\n<body>\n<main>\n<div class=\"doc-layout\">\n");

    html.push_str("<aside class=\"doc-sidebar\">\n");
    if all_stems.len() > 1 {
        html.push_str("<p class=\"sidebar-title\">Files</p>\n<ul>\n");
        for stem in all_stems {
            if stem == file_stem {
                html.push_str(&format!(
                    "<li><a class=\"language-item-ref sidebar-current\" href=\"{stem}.html\">{stem}</a></li>\n"
                ));
            } else {
                html.push_str(&format!(
                    "<li><a class=\"language-item-ref\" href=\"{stem}.html\">{stem}</a></li>\n"
                ));
            }
        }
        html.push_str("</ul>\n");
    }

    if has_structs {
        html.push_str("<a href=\"#Types\" class=\"sidebar-title\">Types</a>\n<ul>\n");
        for obj_id in &file_data.associated_values {
            let GlobalObj::Type(typ_id) = obj_id else {
                continue;
            };
            let name = &linker.globals.types[*typ_id].link_info.name;
            html.push_str(&format!(
                "<li><a class=\"language-item-ref\" href=\"#{name}\">{name}</a></li>\n"
            ));
        }
        html.push_str("</ul>\n");
    }

    if has_modules {
        html.push_str("<a href=\"#Modules\" class=\"sidebar-title\">Modules</a>\n<ul>\n");
        for obj_id in &file_data.associated_values {
            let GlobalObj::Module(md_id) = obj_id else {
                continue;
            };
            let name = &linker.globals.modules[*md_id].link_info.name;
            html.push_str(&format!(
                "<li><a class=\"language-item-ref\" href=\"#{name}\">{name}</a></li>\n"
            ));
        }
        html.push_str("</ul>\n");
    }

    if has_consts {
        html.push_str("<a href=\"#CompileTimeFunctions\" class=\"sidebar-title\">Compile-Time Functions</a>\n<ul>\n");
        for obj_id in &file_data.associated_values {
            let GlobalObj::Constant(const_id) = obj_id else {
                continue;
            };
            let name = &linker.globals.constants[*const_id].link_info.name;
            html.push_str(&format!(
                "<li><a class=\"language-item-ref\" href=\"#{name}\">{name}</a></li>\n"
            ));
        }
        html.push_str("</ul>\n");
    }

    html.push_str("</aside>\n");

    html.push_str("<div class=\"doc-main\">\n");
    html.push_str(&format!("<h1 class=\"page-title\">{file_stem}</h1>\n"));

    if has_structs {
        html.push_str("<h2 class=\"doc-section\" id=\"Types\">Types</h2>\n");
        for obj_id in &file_data.associated_values {
            let GlobalObj::Type(typ_id) = obj_id else {
                continue;
            };
            let typ = &linker.globals.types[*typ_id];
            html.push_str(&render_struct_section(
                typ,
                &file_data.file_text,
                file_stem,
                index,
            ));
        }
    }
    if has_modules {
        html.push_str("<h2 class=\"doc-section\" id=\"Modules\">Modules</h2>\n");
        for obj_id in &file_data.associated_values {
            let GlobalObj::Module(md_id) = obj_id else {
                continue;
            };
            let md = &linker.globals.modules[*md_id];
            html.push_str(&render_module_section(
                md,
                &file_data.file_text,
                file_stem,
                index,
            ));
        }
    }
    if has_consts {
        html.push_str(
            "<h2 class=\"doc-section\" id=\"CompileTimeFunctions\">Compile-Time Functions</h2>\n",
        );
        for obj_id in &file_data.associated_values {
            let GlobalObj::Constant(const_id) = obj_id else {
                continue;
            };
            let cst = &linker.globals.constants[*const_id];
            html.push_str(&render_const_section(
                cst,
                &file_data.file_text,
                file_stem,
                index,
            ));
        }
    }

    html.push_str("</div>\n");
    html.push_str("</div>\n</main>\n");
    html.push_str(&format!(
        "<script src=\"{host}/docs/highlight.js\"></script>\n"
    ));
    html.push_str(&format!("<script src=\"{host}/susdoc.js\"></script>\n"));
    html.push_str("</body>\n</html>\n");

    html
}
