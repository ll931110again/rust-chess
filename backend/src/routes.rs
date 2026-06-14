use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

use crate::models::{
    Document, DocumentListQuery, DocumentListResponse, DocumentSummary, ExplainRequest,
    ExplainResponse,
};

pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok", "service": "qualitydocs-api" }))
}

pub async fn list_documents(
    State(pool): State<SqlitePool>,
    Query(params): Query<DocumentListQuery>,
) -> Result<Json<DocumentListResponse>, StatusCode> {
    let limit = params.limit.unwrap_or(20).clamp(1, 100);
    let offset = params.offset.unwrap_or(0).max(0);

    let mut conditions = vec!["1=1".to_string()];
    let mut binds: Vec<String> = vec![];

    if let Some(q) = &params.q {
        if !q.trim().is_empty() {
            conditions.push("(title LIKE ? OR summary LIKE ? OR content_md LIKE ?)".to_string());
            let pattern = format!("%{}%", q.trim());
            binds.push(pattern.clone());
            binds.push(pattern.clone());
            binds.push(pattern);
        }
    }

    if let Some(tag) = &params.tag {
        if !tag.trim().is_empty() {
            conditions.push("tags LIKE ?".to_string());
            binds.push(format!("%\"{}\"%", tag.trim()));
        }
    }

    let where_clause = conditions.join(" AND ");
    let count_sql = format!("SELECT COUNT(*) FROM documents WHERE {}", where_clause);
    let list_sql = format!(
        "SELECT id, title, slug, source_url, source_name, summary, tags, quality_score, fetched_at
         FROM documents WHERE {} ORDER BY quality_score DESC, fetched_at DESC LIMIT ? OFFSET ?",
        where_clause
    );

    let mut count_query = sqlx::query_as::<_, (i64,)>(&count_sql);
    for b in &binds {
        count_query = count_query.bind(b);
    }
    let (total,) = count_query.fetch_one(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut list_query = sqlx::query_as::<_, DocumentSummary>(&list_sql);
    for b in &binds {
        list_query = list_query.bind(b);
    }
    list_query = list_query.bind(limit).bind(offset);

    let documents = list_query
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(DocumentListResponse { documents, total }))
}

pub async fn get_document(
    State(pool): State<SqlitePool>,
    Path(slug): Path<String>,
) -> Result<Json<Document>, StatusCode> {
    let doc = sqlx::query_as::<_, Document>("SELECT * FROM documents WHERE slug = ?")
        .bind(&slug)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(doc))
}

pub async fn explain_section(
    State(pool): State<SqlitePool>,
    Path(slug): Path<String>,
    Json(body): Json<ExplainRequest>,
) -> Result<Json<ExplainResponse>, StatusCode> {
    let doc = sqlx::query_as::<_, Document>("SELECT * FROM documents WHERE slug = ?")
        .bind(&slug)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let instruction = body
        .instruction
        .unwrap_or_else(|| "Explain this section in clearer, more accessible language while preserving technical accuracy.".to_string());

    if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
        if !api_key.is_empty() {
            if let Ok(explanation) = call_openai(&api_key, &doc.title, &body.section, &instruction).await {
                return Ok(Json(ExplainResponse {
                    explanation,
                    ai_powered: true,
                }));
            }
        }
    }

    let explanation = format!(
        "**Simplified explanation of \"{}\"**\n\n{}\n\n---\n\nThe section discusses technical concepts from *{}*. Key terms may refer to established patterns in systems design or machine learning literature. For deeper context, consult the original source at {}.",
        doc.title,
        summarize_section(&body.section),
        doc.source_name,
        doc.source_url
    );

    Ok(Json(ExplainResponse {
        explanation,
        ai_powered: false,
    }))
}

fn summarize_section(text: &str) -> String {
    let sentences: Vec<&str> = text
        .split('.')
        .map(str::trim)
        .filter(|s| s.len() > 20)
        .take(4)
        .collect();

    if sentences.is_empty() {
        return "This section covers foundational concepts. Break it into smaller parts and look up unfamiliar terms.".to_string();
    }

    sentences.join(". ") + "."
}

async fn call_openai(
    api_key: &str,
    title: &str,
    section: &str,
    instruction: &str,
) -> anyhow::Result<String> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "gpt-4o-mini",
            "messages": [
                {"role": "system", "content": "You are a technical writer who explains complex engineering documents clearly."},
                {"role": "user", "content": format!("Document: {}\n\nInstruction: {}\n\nSection to explain:\n{}", title, instruction, section)}
            ],
            "max_tokens": 800
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(response["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Unable to generate explanation.")
        .to_string())
}

pub async fn crawler_status() -> Json<crate::crawler::CrawlerStatus> {
    Json(crate::crawler::status().await)
}

pub async fn crawler_run(
    State(pool): State<SqlitePool>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let pool = pool.clone();
    tokio::spawn(async move {
        if let Err(e) = crate::crawler::run_once(&pool).await {
            tracing::error!("Manual crawler run failed: {:#}", e);
        }
    });
    Ok(Json(serde_json::json!({ "status": "started" })))
}
