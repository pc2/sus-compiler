use crate::file_position::FileText;
use crate::flattening::{ClockVisibility, FieldDeclKind, InterfaceDeclaration, InterfaceKind, Module};
use crate::linker::{GlobalObj, LinkInfo};
use crate::prelude::*;
use std::path::Path;

pub fn gen_docs(linker: &Linker) {
    let docs_dir = Path::new("docs");
    if let Err(e) = std::fs::create_dir_all(docs_dir) {
        eprintln!("Could not create docs/ directory: {e}");
        return;
    }

    for (_, file_data) in &linker.files {
        let file_text = &file_data.file_text;
        let file_stem = Path::new(&file_data.file_identifier.name)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_owned();

        let modules: Vec<&Module> = file_data
            .associated_values
            .iter()
            .filter_map(|uuid| {
                if let GlobalObj::Module(id) = uuid {
                    Some(&linker.globals.modules[*id])
                } else {
                    None
                }
            })
            .collect();

        if modules.is_empty() {
            continue;
        }

        let html = generate_file_html(&file_stem, &modules, file_text);
        let out_path = docs_dir.join(format!("{file_stem}.html"));
        match std::fs::write(&out_path, &html) {
            Ok(()) => println!("Generated {}", out_path.display()),
            Err(e) => eprintln!("Could not write {}: {e}", out_path.display()),
        }
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
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
        let parts: Vec<String> = iface.inputs.iter().map(|&id| fmt_param(li, id, ft)).collect();
        params.push_str(&parts.join(", "));
    }
    if !iface.outputs.is_empty() {
        params.push_str(" -> ");
        let parts: Vec<String> = iface.outputs.iter().map(|&id| fmt_param(li, id, ft)).collect();
        params.push_str(&parts.join(", "));
    }
    params
}

fn build_interface_block(md: &Module, ft: &FileText) -> String {
    let li = &md.link_info;
    let mut out = String::new();

    if li.parameters.is_empty() {
        out.push_str(&format!("module {} {{\n", li.name));
    } else {
        out.push_str(&format!("module {} {{\n", li.display_full_name_and_args(ft)));
    }

    for (_, clock) in &md.clocks {
        match clock.visibility {
            ClockVisibility::Input | ClockVisibility::Output => {
                out.push_str(&format!("    clock {}\n", clock.name));
            }
            ClockVisibility::Local => {}
        }
    }

    for (_, field) in &md.fields {
        let Some(decl_instr) = field.declaration_instruction else {
            continue;
        };
        match decl_instr {
            FieldDeclKind::SinglePort(decl_id) => {
                let decl = li.instructions[decl_id].unwrap_declaration();
                out.push_str(&format!("    {}\n", li.display_decl(None, decl, ft)));
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
                out.push_str(&format!("    {kw} {}{lat}{params}\n", iface.name));
            }
        }
    }

    out.push('}');
    out
}

fn render_inline(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '`' => {
                let mut code = String::new();
                for ch in chars.by_ref() {
                    if ch == '`' {
                        break;
                    }
                    code.push(ch);
                }
                result.push_str("<code>");
                result.push_str(&html_escape(&code));
                result.push_str("</code>");
            }
            '[' => {
                let mut name = String::new();
                let mut closed = false;
                for ch in chars.by_ref() {
                    if ch == ']' {
                        closed = true;
                        break;
                    }
                    name.push(ch);
                }
                if closed && !name.is_empty() {
                    result.push_str(&format!("<a href=\"#{name}\">{name}</a>"));
                } else {
                    result.push('[');
                    result.push_str(&name);
                }
            }
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '&' => result.push_str("&amp;"),
            _ => result.push(c),
        }
    }
    result
}

fn render_prose(raw: &str) -> String {
    let mut html = String::new();
    let mut para = String::new();
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if !para.is_empty() {
                html.push_str("<p>");
                html.push_str(&render_inline(&para));
                html.push_str("</p>\n");
                para.clear();
            }
        } else {
            if !para.is_empty() {
                para.push(' ');
            }
            para.push_str(trimmed);
        }
    }
    if !para.is_empty() {
        html.push_str("<p>");
        html.push_str(&render_inline(&para));
        html.push_str("</p>\n");
    }
    html
}

fn split_example(raw: &str) -> (Option<String>, String) {
    if let Some(start) = raw.find("```sus") {
        let after_open = &raw[start + 6..];
        if let Some(end) = after_open.find("```") {
            let example = after_open[..end].trim().to_string();
            let remainder = format!("{}{}", &raw[..start], &after_open[end + 3..]);
            return (Some(example), remainder);
        }
    }
    (None, raw.to_string())
}

fn render_module_section(md: &Module, ft: &FileText) -> String {
    let li = &md.link_info;
    let name = &li.name;
    let raw_doc = li.documentation.to_string(ft);
    let (example, doc_text) = split_example(&raw_doc);

    let heading = if li.parameters.is_empty() {
        html_escape(name)
    } else {
        html_escape(&li.display_full_name_and_args(ft).to_string())
    };

    let mut s = format!("<section class=\"module\" id=\"{name}\">\n");
    s.push_str(&format!("<div class=\"module-heading\"><h2>{heading}</h2></div>\n"));

    if !doc_text.trim().is_empty() {
        s.push_str("<div class=\"doc-prose\">\n");
        s.push_str(&render_prose(&doc_text));
        s.push_str("</div>\n");
    }

    if let Some(code) = example {
        s.push_str("<div class=\"example-section\">\n<p class=\"block-label\">Example</p>\n");
        s.push_str("<div class=\"example-block\"><pre><code class=\"language-plaintext\">");
        s.push_str(&html_escape(&code));
        s.push_str("</code></pre></div>\n</div>\n");
    }

    let interface = build_interface_block(md, ft);
    s.push_str("<div class=\"interface-block\"><pre><code class=\"language-plaintext\">");
    s.push_str(&html_escape(&interface));
    s.push_str("</code></pre></div>\n</section>\n");

    s
}

const EMBEDDED_CSS: &str = r"
:root {
  --brand: #412173;
  --brand-light: #ede8f5;
  --bg: #ffffff;
  --bg-subtle: #f7f6fb;
  --bg-code: #f3f1f9;
  --text: #1a1227;
  --text-muted: #5a5370;
  --text-faint: #9b96aa;
  --border: #e0dcea;
  --link: #412173;
}
*,*::before,*::after{box-sizing:border-box}
body{background:var(--bg);color:var(--text);font-family:system-ui,sans-serif;margin:0}
.doc-layout{display:flex;max-width:1300px;margin:0 auto}
.doc-sidebar{width:210px;flex-shrink:0;padding:1.75rem 1rem 1.75rem 1.25rem;border-right:1px solid var(--border);position:sticky;top:0;align-self:flex-start;max-height:100vh;overflow-y:auto;background:var(--bg-subtle)}
.sidebar-title{font-size:.68rem;text-transform:uppercase;letter-spacing:.13em;color:var(--text-faint);margin:0 0 .6rem}
.doc-sidebar ul{list-style:none;padding:0;margin:0}
.doc-sidebar a{font-family:'Fira Code',monospace;font-size:.83rem;color:var(--text-muted);text-decoration:none;display:block;padding:3px 7px;border-radius:4px}
.doc-sidebar a:hover{color:var(--brand);background:var(--brand-light)}
.doc-main{flex:1;padding:1.75rem 2.5rem 3rem;min-width:0}
.page-title{font-family:'Fira Code',monospace;font-size:1.55rem;color:var(--text);margin:0 0 2rem}
.module{scroll-margin-top:1rem}
.module+.module{border-top:1px solid var(--border);padding-top:2.5rem;margin-top:2.5rem}
.module-heading h2{font-family:'Fira Code',monospace;font-size:1.2rem;color:var(--text);margin:0 0 .8rem}
.doc-prose{font-size:.95rem;color:var(--text-muted);line-height:1.75;max-width:680px;margin:0 0 1.1rem}
.doc-prose p{margin:0}.doc-prose p+p{margin-top:.6em}
.doc-prose code{font-family:'Fira Code',monospace;font-size:.875em;background:var(--bg-code);color:var(--brand);padding:1px 5px;border-radius:3px}
.doc-prose a{color:var(--link);text-decoration:none}
.interface-block{background:var(--bg-subtle);border-left:3px solid var(--brand);border-radius:0 6px 6px 0;overflow-x:auto;margin-bottom:1.1rem}
.interface-block pre{margin:0;padding:1rem 1.25rem;line-height:1.65}
.interface-block code{font-family:'Fira Code',monospace!important;font-size:.875rem!important;background:transparent!important;color:var(--text)!important}
.block-label{font-size:.68rem;text-transform:uppercase;letter-spacing:.13em;color:var(--text-faint);margin:0 0 .35rem}
.example-section{margin-bottom:1.1rem}
.example-block{background:#1e1e2e;border-radius:6px;overflow-x:auto}
.example-block pre{margin:0;padding:1rem 1.25rem;line-height:1.65}
.example-block code{font-family:'Fira Code',monospace!important;font-size:.875rem!important;background:transparent!important;color:#cdd6f4!important}
";

fn generate_file_html(file_stem: &str, modules: &[&Module], ft: &FileText) -> String {
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("  <meta charset=\"UTF-8\"/>\n  <meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"/>\n");
    html.push_str(&format!("  <title>{} — SUS Documentation</title>\n", html_escape(file_stem)));
    html.push_str("  <style>\n");
    html.push_str(EMBEDDED_CSS);
    html.push_str("  </style>\n</head>\n<body>\n<main>\n<div class=\"doc-layout\">\n");

    html.push_str("<aside class=\"doc-sidebar\">\n<p class=\"sidebar-title\">Modules</p>\n<ul>\n");
    for md in modules {
        let name = &md.link_info.name;
        html.push_str(&format!("<li><a href=\"#{name}\">{name}</a></li>\n"));
    }
    html.push_str("</ul>\n</aside>\n");

    html.push_str("<div class=\"doc-main\">\n");
    html.push_str(&format!("<h1 class=\"page-title\">{}</h1>\n", html_escape(file_stem)));
    for md in modules {
        html.push_str(&render_module_section(md, ft));
    }
    html.push_str("</div>\n</div>\n</main>\n</body>\n</html>\n");

    html
}
