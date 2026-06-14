use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

use crate::models::{
    NewsItem, NewsQuery, NewsResponse, SearchQuery, SearchResponse, SearchResult, SiteInfo,
};
use crate::quality;

pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok", "service": "omnisearch-api" }))
}

pub async fn search(
    State(pool): State<SqlitePool>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, StatusCode> {
    let query = params.q.unwrap_or_default();
    let limit = params.limit.unwrap_or(10).clamp(1, 50);
    let site_filter = params.site.filter(|s| !s.trim().is_empty());

    if query.trim().is_empty() {
        return Ok(Json(SearchResponse {
            results: vec![],
            query,
            site_filter,
        }));
    }

    let fts_query = query
        .split_whitespace()
        .map(|w| format!("\"{}\"", w.replace('"', "")))
        .collect::<Vec<_>>()
        .join(" OR ");

    let results = if let Some(site) = &site_filter {
        sqlx::query_as::<_, SearchResult>(
            "SELECT p.id, p.url, p.domain, p.title, p.summary, p.content_type, p.quality_score, p.topics, p.indexed_at,
                    (bm25(pages_fts) * ?) AS relevance
             FROM pages_fts
             JOIN pages p ON p.id = pages_fts.rowid
             WHERE pages_fts MATCH ? AND p.domain = ?
             ORDER BY relevance ASC
             LIMIT ?",
        )
        .bind(quality::quality_boost(80))
        .bind(&fts_query)
        .bind(site)
        .bind(limit)
        .fetch_all(&pool)
        .await
    } else {
        sqlx::query_as::<_, SearchResult>(
            "SELECT p.id, p.url, p.domain, p.title, p.summary, p.content_type, p.quality_score, p.topics, p.indexed_at,
                    (bm25(pages_fts) * (0.3 + 0.007 * p.quality_score)) AS relevance
             FROM pages_fts
             JOIN pages p ON p.id = pages_fts.rowid
             WHERE pages_fts MATCH ?
             ORDER BY relevance ASC
             LIMIT ?",
        )
        .bind(&fts_query)
        .bind(limit)
        .fetch_all(&pool)
        .await
    };

    let mut results = results.map_err(|e| {
        tracing::error!("Search error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    for r in &mut results {
        r.relevance = r.relevance.abs() * quality::quality_boost(r.quality_score);
    }
    results.sort_by(|a, b| a.relevance.partial_cmp(&b.relevance).unwrap());

    Ok(Json(SearchResponse {
        results,
        query,
        site_filter,
    }))
}

pub async fn news_feed(
    State(pool): State<SqlitePool>,
    Query(params): Query<NewsQuery>,
) -> Result<Json<NewsResponse>, StatusCode> {
    let limit = params.limit.unwrap_or(20).clamp(1, 50);
    let offset = params.offset.unwrap_or(0).max(0);

    let (total,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM pages WHERE quality_score >= 70")
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let items = sqlx::query_as::<_, NewsItem>(
        "SELECT id, url, domain, title, summary, quality_score, topics, indexed_at
         FROM pages
         WHERE quality_score >= 70
         ORDER BY quality_score DESC, indexed_at DESC
         LIMIT ? OFFSET ?",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(NewsResponse { items, total }))
}

pub async fn list_sites(State(pool): State<SqlitePool>) -> Result<Json<Vec<SiteInfo>>, StatusCode> {
    let sites = sqlx::query_as::<_, SiteInfo>(
        "SELECT domain, COUNT(*) as page_count FROM pages GROUP BY domain ORDER BY page_count DESC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(sites))
}
