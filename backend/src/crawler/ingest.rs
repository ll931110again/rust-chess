use crate::crawler::extract::{html_to_text, infer_topics, summarize};
use crate::quality;
use sqlx::SqlitePool;
use url::Url;

#[derive(Debug, Clone)]
pub struct FeedItem {
    pub title: String,
    pub url: String,
    pub summary: String,
    pub content_hint: String,
}

pub async fn upsert_page(pool: &SqlitePool, item: &FetchedArticle) -> anyhow::Result<bool> {
    let score = quality::score_content(&item.title, &item.content, &item.domain);
    if score < 65 {
        return Ok(false);
    }

    let topics = infer_topics(&item.title, &item.content);
    let summary = if item.summary.is_empty() {
        summarize(&item.content, 280)
    } else {
        item.summary.clone()
    };

    let existing: Option<(i64,)> =
        sqlx::query_as("SELECT id FROM pages WHERE url = ?")
            .bind(&item.url)
            .fetch_optional(pool)
            .await?;

    if existing.is_some() {
        sqlx::query(
            "UPDATE pages SET title = ?, content = ?, summary = ?, quality_score = ?, topics = ?, indexed_at = datetime('now')
             WHERE url = ?",
        )
        .bind(&item.title)
        .bind(&item.content)
        .bind(&summary)
        .bind(score)
        .bind(&topics)
        .bind(&item.url)
        .execute(pool)
        .await?;
        return Ok(true);
    }

    sqlx::query(
        "INSERT INTO pages (url, domain, title, content, summary, content_type, quality_score, topics)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&item.url)
    .bind(&item.domain)
    .bind(&item.title)
    .bind(&item.content)
    .bind(&summary)
    .bind(&item.content_type)
    .bind(score)
    .bind(&topics)
    .execute(pool)
    .await?;

    Ok(true)
}

#[derive(Debug, Clone)]
pub struct FetchedArticle {
    pub url: String,
    pub domain: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub content_type: String,
}

pub fn domain_from_url(raw: &str) -> String {
    Url::parse(raw)
        .ok()
        .and_then(|u| u.host_str().map(|h| h.trim_start_matches("www.").to_string()))
        .unwrap_or_else(|| "unknown".to_string())
}

pub fn merge_content(item: &FeedItem, page_html: Option<&str>) -> FetchedArticle {
    let content = if let Some(html) = page_html {
        let text = html_to_text(html);
        if text.len() > item.content_hint.len() + 100 {
            text
        } else if !item.content_hint.is_empty() {
            item.content_hint.clone()
        } else {
            text
        }
    } else if !item.content_hint.is_empty() {
        item.content_hint.clone()
    } else {
        item.summary.clone()
    };

    FetchedArticle {
        url: item.url.clone(),
        domain: domain_from_url(&item.url),
        title: item.title.clone(),
        summary: item.summary.clone(),
        content,
        content_type: "article".to_string(),
    }
}
