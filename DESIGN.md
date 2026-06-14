# QualityDocs — Design Document

## Problem

After leaving big tech, access to high-quality internal design documents disappears. The public web is flooded with low-signal content. We need a curated, technical reading experience comparable to OpenAI, Google, or Anthropic engineering blogs and design docs.

## Product Vision

QualityDocs is a **personal technical library** that aggregates publicly available design documents, architecture write-ups, and engineering deep-dives. It prioritizes signal over noise and stays unapologetically technical.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     qualitydocs.xyz                          │
│  Next.js frontend (Vercel/Render static)                     │
│  • Document reader with typography tuned for long-form       │
│  • Full-text search + tag filters                            │
│  • AI Explain panel (Phase 2)                                │
└──────────────────────────┬──────────────────────────────────┘
                           │ REST API
┌──────────────────────────▼──────────────────────────────────┐
│              Rust API (Render Web Service)                   │
│  axum + sqlx + SQLite                                        │
│  • Document CRUD & ingestion                                 │
│  • Crawler worker (scheduled) for public sources             │
│  • Optional OpenAI rewrite/explain endpoint                    │
└──────────────────────────┬──────────────────────────────────┘
                           │
                    ┌──────▼──────┐
                    │   SQLite    │
                    │  documents  │
                    │  sources    │
                    └─────────────┘
```

## Data Model

| Entity   | Fields |
|----------|--------|
| Document | id, title, slug, source_url, content_md, summary, tags[], quality_score, fetched_at |
| Source   | id, name, base_url, crawl_enabled |

## Seeded Sources (MVP)

- OpenAI research & engineering blog (public)
- Google Research publications index
- Anthropic research & news
- Meta engineering blog
- Rust RFCs and design docs

## UX Principles

1. **Reader-first** — generous line height, syntax highlighting, table of contents
2. **No clutter** — no ads, no related-slop sidebar
3. **Discoverability** — search + topic tags for under-explored areas
4. **Progressive enhancement** — AI explain is optional, never blocks reading

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | /api/documents | List with pagination, search, tag filter |
| GET | /api/documents/:slug | Single document |
| POST | /api/documents/:slug/explain | AI rewrite of a section (optional key) |
| GET | /api/health | Health check |

## Deployment

- **Backend**: Render Web Service (Rust binary)
- **Frontend**: Render Static Site or co-located
- **Domain**: qualitydocs.xyz via Vercel DNS

## Phase Roadmap

| Phase | Scope |
|-------|-------|
| 1 (MVP) | Seeded docs, search, beautiful reader |
| 2 | Live crawler + freshness |
| 3 | AI explain panel |
