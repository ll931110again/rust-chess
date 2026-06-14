# Omnisearch Platform

Cross-site quality search and curated news feed. Two products, one platform.

## Products

1. **Omnisearch** — BM25 search with quality-weighted ranking across indexed sites
2. **Curated News** — Feed filtered for substance over sensationalism

## Stack

- **Backend**: Rust (axum, sqlx, SQLite FTS5)
- **Frontend**: Next.js 14, TypeScript, Tailwind CSS

## Local Development

```bash
# Backend
cd backend
cargo run

# Frontend
cd frontend
NEXT_PUBLIC_API_URL=http://localhost:8080 npm run dev
```

Domain: **omnisearch.xyz**
