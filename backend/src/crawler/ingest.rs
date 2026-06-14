use crate::crawler::extract::{html_to_markdown, html_to_text, infer_doc_tags, summarize};
use crate::crawler::quality::{min_ingest_score, score_document};
use slug::slugify;
use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct FeedItem {
    pub title: String,
    pub url: String,
    pub summary: String,
    pub content_hint: String,
}

#[derive(Debug, Clone)]
pub struct FetchedDoc {
    pub title: String,
    pub source_url: String,
    pub source_name: String,
    pub summary: String,
    pub content_md: String,
}

pub fn merge_doc(source_name: &str, item: &FeedItem, page_html: Option<&str>) -> FetchedDoc {
    let content_md = if let Some(html) = page_html {
        let md = html_to_markdown(html);
        if md.len() > item.content_hint.len() + 100 {
            md
        } else if !item.content_hint.is_empty() {
            format!("# {}\n\n{}", item.title, item.content_hint)
        } else {
            md
        }
    } else if !item.content_hint.is_empty() {
        format!("# {}\n\n{}", item.title, item.content_hint)
    } else {
        format!("# {}\n\n{}", item.title, item.summary)
    };

    FetchedDoc {
        title: item.title.clone(),
        source_url: item.url.clone(),
        source_name: source_name.to_string(),
        summary: if item.summary.is_empty() {
            summarize(&html_to_text(&content_md), 280)
        } else {
            item.summary.clone()
        },
        content_md,
    }
}

pub async fn upsert_document(pool: &SqlitePool, doc: &FetchedDoc) -> anyhow::Result<bool> {
    let score = score_document(&doc.title, &doc.content_md, &doc.source_url);
    if score < min_ingest_score() {
        return Ok(false);
    }

    let tags = infer_doc_tags(&doc.title, &doc.content_md, &doc.source_name);
    let slug = unique_slug(pool, &doc.title).await?;

    let existing: Option<(i64,)> = sqlx::query_as("SELECT id FROM documents WHERE source_url = ?")
        .bind(&doc.source_url)
        .fetch_optional(pool)
        .await?;

    if let Some((id,)) = existing {
        sqlx::query(
            "UPDATE documents SET title = ?, slug = ?, content_md = ?, summary = ?, tags = ?, quality_score = ?, fetched_at = datetime('now') WHERE id = ?",
        )
        .bind(&doc.title)
        .bind(&slug)
        .bind(&doc.content_md)
        .bind(&doc.summary)
        .bind(&tags)
        .bind(score)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(true);
    }

    sqlx::query(
        "INSERT INTO documents (title, slug, source_url, source_name, content_md, summary, tags, quality_score)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&doc.title)
    .bind(&slug)
    .bind(&doc.source_url)
    .bind(&doc.source_name)
    .bind(&doc.content_md)
    .bind(&doc.summary)
    .bind(&tags)
    .bind(score)
    .execute(pool)
    .await?;

    Ok(true)
}

async fn unique_slug(pool: &SqlitePool, title: &str) -> anyhow::Result<String> {
    let base = slugify(title);
    let base = if base.is_empty() { "document".to_string() } else { base };

    let mut candidate = base.clone();
    for i in 0..20 {
        let exists: Option<(i64,)> = sqlx::query_as("SELECT id FROM documents WHERE slug = ?")
            .bind(&candidate)
            .fetch_optional(pool)
            .await?;
        if exists.is_none() {
            return Ok(candidate);
        }
        candidate = if i == 0 {
            format!("{}-2", base)
        } else {
            format!("{}-{}", base, i + 2)
        };
    }
    Ok(format!("{}-{}", base, chrono::Utc::now().timestamp()))
}
