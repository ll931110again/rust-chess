use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Document {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub source_url: String,
    pub source_name: String,
    pub content_md: String,
    pub summary: String,
    pub tags: String,
    pub quality_score: i64,
    pub fetched_at: String,
}

#[derive(Debug, Deserialize)]
pub struct DocumentListQuery {
    pub q: Option<String>,
    pub tag: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ExplainRequest {
    pub section: String,
    pub instruction: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ExplainResponse {
    pub explanation: String,
    pub ai_powered: bool,
}

#[derive(Debug, Serialize)]
pub struct DocumentListResponse {
    pub documents: Vec<DocumentSummary>,
    pub total: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DocumentSummary {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub source_url: String,
    pub source_name: String,
    pub summary: String,
    pub tags: String,
    pub quality_score: i64,
    pub fetched_at: String,
}
