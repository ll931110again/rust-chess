use scraper::{Html, Selector};

pub fn html_to_text(html: &str) -> String {
    let document = Html::parse_document(html);
    for selector in ["article", "main", "body"] {
        if let Ok(sel) = Selector::parse(selector) {
            if let Some(el) = document.select(&sel).next() {
                let text: String = el.text().collect::<Vec<_>>().join(" ");
                let flat = collapse_whitespace(&text);
                if flat.len() > 200 {
                    return flat;
                }
            }
        }
    }
    let text: String = document.root_element().text().collect::<Vec<_>>().join(" ");
    collapse_whitespace(&text)
}

pub fn html_to_markdown(html: &str) -> String {
    let document = Html::parse_document(html);
    let block_sel = Selector::parse("h1, h2, h3, h4, p, li, pre, code, blockquote").unwrap();

    for selector in ["article", "main", "body"] {
        if let Ok(root_sel) = Selector::parse(selector) {
            if let Some(root) = document.select(&root_sel).next() {
                let md = blocks_to_markdown(root, &block_sel);
                if md.len() > 300 {
                    return md;
                }
            }
        }
    }

    let md = blocks_to_markdown(document.root_element(), &block_sel);
    if md.len() < 300 {
        format!("# Extracted content\n\n{}", html_to_text(html))
    } else {
        md
    }
}

fn blocks_to_markdown(root: scraper::ElementRef<'_>, block_sel: &Selector) -> String {
    let mut out = String::new();
    for el in root.select(block_sel) {
        let tag = el.value().name();
        let text = collapse_whitespace(&el.text().collect::<Vec<_>>().join(" "));
        if text.len() < 8 {
            continue;
        }
        match tag {
            "h1" => out.push_str(&format!("# {}\n\n", text)),
            "h2" => out.push_str(&format!("## {}\n\n", text)),
            "h3" | "h4" => out.push_str(&format!("### {}\n\n", text)),
            "pre" | "code" => out.push_str(&format!("```\n{}\n```\n\n", text)),
            "blockquote" => out.push_str(&format!("> {}\n\n", text)),
            "li" => out.push_str(&format!("- {}\n", text)),
            _ => out.push_str(&format!("{}\n\n", text)),
        }
    }
    out.trim().to_string()
}

pub fn summarize(text: &str, max_len: usize) -> String {
    let flat = collapse_whitespace(text);
    if flat.len() <= max_len {
        flat
    } else {
        format!("{}…", &flat[..max_len.saturating_sub(1)])
    }
}

pub fn infer_doc_tags(title: &str, content: &str, source: &str) -> String {
    let text = format!("{} {} {}", title.to_lowercase(), content.to_lowercase(), source.to_lowercase());
    let candidates: &[(&str, &[&str])] = &[
        ("architecture", &["architecture", "design", "distributed", "system"]),
        ("ai", &["machine learning", "llm", "model", "transformer", "inference"]),
        ("databases", &["database", "sql", "storage", "query", "index"]),
        ("infrastructure", &["kubernetes", "cloud", "deploy", "scaling", "infra"]),
        ("security", &["security", "auth", "encryption", "vulnerability"]),
        ("performance", &["latency", "throughput", "optimization", "benchmark"]),
        ("rust", &["rust", "cargo", "ownership"]),
        ("research", &["paper", "research", "study", "arxiv"]),
    ];

    let mut tags: Vec<&str> = Vec::new();
    for (tag, keywords) in candidates {
        if keywords.iter().any(|k| text.contains(k)) {
            tags.push(tag);
        }
    }
    if tags.is_empty() {
        tags.push("engineering");
    }
    serde_json::to_string(&tags).unwrap_or_else(|_| r#"["engineering"]"#.to_string())
}

fn collapse_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}
