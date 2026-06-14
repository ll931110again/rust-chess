use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Page {
    pub id: i64,
    pub url: String,
    pub domain: String,
    pub title: String,
    pub content: String,
    pub summary: String,
    pub content_type: String,
    pub quality_score: i64,
    pub topics: String,
    pub indexed_at: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SearchResult {
    pub id: i64,
    pub url: String,
    pub domain: String,
    pub title: String,
    pub summary: String,
    pub content_type: String,
    pub quality_score: i64,
    pub topics: String,
    pub indexed_at: String,
    pub relevance: f64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct NewsItem {
    pub id: i64,
    pub url: String,
    pub domain: String,
    pub title: String,
    pub summary: String,
    pub quality_score: i64,
    pub topics: String,
    pub indexed_at: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub site: Option<String>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct NewsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub query: String,
    pub site_filter: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NewsResponse {
    pub items: Vec<NewsItem>,
    pub total: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SiteInfo {
    pub domain: String,
    pub page_count: i64,
}
