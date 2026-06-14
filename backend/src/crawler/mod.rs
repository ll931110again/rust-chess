mod extract;
mod fetch;
mod ingest;
mod quality;
mod sources;

use fetch::{build_client, enrich_documents, fetch_feed_items, FetchConfig};
use ingest::upsert_document;
use sources::doc_sources;
use sqlx::SqlitePool;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CrawlerStatus {
    pub running: bool,
    pub last_run_at: Option<String>,
    pub last_ingested: u64,
    pub last_errors: u64,
    pub last_sources: u64,
}

static STATUS: std::sync::LazyLock<RwLock<CrawlerStatus>> =
    std::sync::LazyLock::new(|| RwLock::new(CrawlerStatus::default()));

pub async fn status() -> CrawlerStatus {
    STATUS.read().await.clone()
}

pub fn spawn(pool: SqlitePool) {
    let enabled = std::env::var("CRAWLER_ENABLED")
        .map(|v| v != "0" && v.to_lowercase() != "false")
        .unwrap_or(true);

    if !enabled {
        info!("Crawler disabled via CRAWLER_ENABLED");
        return;
    }

    let interval_secs: u64 = std::env::var("CRAWLER_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(900);

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(120)).await;
        loop {
            if let Err(e) = run_once(&pool).await {
                error!("Crawler run failed: {:#}", e);
            }
            tokio::time::sleep(Duration::from_secs(interval_secs)).await;
        }
    });

    info!("QualityDocs crawler scheduled every {}s", interval_secs);
}

pub async fn run_once(pool: &SqlitePool) -> anyhow::Result<()> {
    {
        let mut s = STATUS.write().await;
        s.running = true;
    }

    let result = sqlx::query("INSERT INTO crawl_runs (started_at) VALUES (datetime('now'))")
        .execute(pool)
        .await?;
    let run_id = result.last_insert_rowid();

    let client = build_client()?;
    let config = FetchConfig::default();
    let sources = doc_sources();
    let source_count = sources.len() as i64;

    let mut seen = 0u64;
    let mut ingested = 0u64;
    let mut errors = 0u64;

    for source in sources {
        match fetch_feed_items(&client, &source, config.max_items_per_feed).await {
            Ok(items) => {
                seen += items.len() as u64;
                let docs = enrich_documents(&client, source.name, items, &config).await;
                for doc in docs {
                    match upsert_document(pool, &doc).await {
                        Ok(true) => ingested += 1,
                        Ok(false) => {}
                        Err(e) => {
                            errors += 1;
                            warn!("Doc ingest failed {}: {:#}", doc.source_url, e);
                        }
                    }
                }
            }
            Err(e) => {
                errors += 1;
                warn!("Feed failed {}: {:#}", source.url, e);
            }
        }
    }

    sqlx::query(
        "UPDATE crawl_runs SET finished_at = datetime('now'), sources_checked = ?, articles_seen = ?, articles_ingested = ?, errors = ? WHERE id = ?",
    )
    .bind(source_count)
    .bind(seen as i64)
    .bind(ingested as i64)
    .bind(errors as i64)
    .bind(run_id)
    .execute(pool)
    .await?;

    {
        let mut s = STATUS.write().await;
        s.running = false;
        s.last_run_at = Some(chrono::Utc::now().to_rfc3339());
        s.last_ingested = ingested;
        s.last_errors = errors;
        s.last_sources = source_count as u64;
    }

    info!(
        "QualityDocs crawler: {} feeds, {} seen, {} ingested, {} errors",
        source_count,
        seen,
        ingested,
        errors
    );

    Ok(())
}
