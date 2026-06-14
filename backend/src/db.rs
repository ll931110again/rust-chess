use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;

pub async fn init_pool(database_url: &str) -> anyhow::Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    run_migrations(&pool).await?;
    Ok(pool)
}

async fn run_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    for migration in [include_str!("../migrations/001_init.sql"), include_str!("../migrations/002_crawler.sql")] {
        for statement in migration.split(';').map(str::trim).filter(|s| !s.is_empty()) {
            sqlx::query(statement).execute(pool).await?;
        }
    }

    sqlx::query(
        "CREATE VIRTUAL TABLE IF NOT EXISTS pages_fts USING fts5(title, content, content='pages', content_rowid='id')",
    )
    .execute(pool)
    .await?;

    let triggers = [
        "CREATE TRIGGER IF NOT EXISTS pages_ai AFTER INSERT ON pages BEGIN INSERT INTO pages_fts(rowid, title, content) VALUES (new.id, new.title, new.content); END",
        "CREATE TRIGGER IF NOT EXISTS pages_ad AFTER DELETE ON pages BEGIN INSERT INTO pages_fts(pages_fts, rowid, title, content) VALUES('delete', old.id, old.title, old.content); END",
        "CREATE TRIGGER IF NOT EXISTS pages_au AFTER UPDATE ON pages BEGIN INSERT INTO pages_fts(pages_fts, rowid, title, content) VALUES('delete', old.id, old.title, old.content); INSERT INTO pages_fts(rowid, title, content) VALUES (new.id, new.title, new.content); END",
    ];
    for t in triggers {
        sqlx::query(t).execute(pool).await?;
    }

    Ok(())
}

pub async fn rebuild_fts(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO pages_fts(pages_fts) VALUES('rebuild')")
        .execute(pool)
        .await?;
    Ok(())
}
