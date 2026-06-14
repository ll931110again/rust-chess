CREATE TABLE IF NOT EXISTS crawl_runs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    started_at TEXT NOT NULL DEFAULT (datetime('now')),
    finished_at TEXT,
    sources_checked INTEGER NOT NULL DEFAULT 0,
    articles_seen INTEGER NOT NULL DEFAULT 0,
    articles_ingested INTEGER NOT NULL DEFAULT 0,
    errors INTEGER NOT NULL DEFAULT 0
);
