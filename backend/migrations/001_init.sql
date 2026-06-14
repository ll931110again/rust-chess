CREATE TABLE IF NOT EXISTS documents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    source_url TEXT NOT NULL,
    source_name TEXT NOT NULL,
    content_md TEXT NOT NULL,
    summary TEXT NOT NULL,
    tags TEXT NOT NULL DEFAULT '[]',
    quality_score INTEGER NOT NULL DEFAULT 90,
    fetched_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_documents_slug ON documents(slug);
CREATE INDEX IF NOT EXISTS idx_documents_title ON documents(title);
