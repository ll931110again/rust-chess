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

    blocks_to_markdown(document.root_element(), &block_sel)
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

pub fn infer_topics(title: &str, content: &str) -> String {
    let text = format!("{} {}", title.to_lowercase(), content.to_lowercase());
    let candidates = [
        ("ai", &["ai", "machine learning", "llm", "neural", "transformer"]),
        ("science", &["research", "study", "scientists", "experiment", "physics"]),
        ("technology", &["software", "hardware", "chip", "computing", "internet"]),
        ("climate", &["climate", "carbon", "emissions", "renewable", "warming"]),
        ("health", &["health", "medical", "disease", "vaccine", "clinical"]),
        ("space", &["nasa", "space", "orbit", "telescope", "mars"]),
        ("policy", &["policy", "regulation", "government", "law", "congress"]),
        ("business", &["market", "economy", "company", "startup", "investment"]),
    ];

    let mut tags = Vec::new();
    for (tag, keywords) in candidates {
        if keywords.iter().any(|k| text.contains(k)) {
            tags.push(tag);
        }
    }
    if tags.is_empty() {
        tags.push("general");
    }
    serde_json::to_string(&tags).unwrap_or_else(|_| r#"["general"]"#.to_string())
}

fn collapse_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}
