# Omnisearch Platform — Design Document

Two products share one platform: **Omnisearch** (cross-site search) and **Curated News** (quality-filtered feed).

## Problem

Platform search engines optimize for engagement, not quality. YouTube, Reddit, and news sites surface clickbait. We want our own ranking criteria: informative, non-sensational, technically substantive.

## Product Vision

A unified platform where users can:
1. **Search** across indexed public web content with AI-assisted quality ranking
2. **Browse** a curated news feed scored for substance over sensation

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    omnisearch.xyz                            │
│  Next.js frontend                                            │
│  • /search — unified search UI                               │
│  • /news   — curated feed                                    │
└──────────────────────────┬──────────────────────────────────┘
                           │ REST API
┌──────────────────────────▼──────────────────────────────────┐
│              Rust API (Render Web Service)                   │
│  axum + sqlx + SQLite + tantivy (full-text index)            │
│  • Crawler ingests public pages                              │
│  • Quality scorer (heuristic + optional LLM)                 │
│  • Search: BM25 + quality boost                              │
│  • News synthesizer (multi-source merge, Phase 2)            │
└──────────────────────────┬──────────────────────────────────┘
                           │
              ┌────────────┴────────────┐
              │ SQLite + Tantivy index  │
              └─────────────────────────┘
```

## Omnisearch — Search Design

### Query modes
- **Global**: top-K across all indexed sites
- **Site-scoped**: top-K filtered by domain

### Ranking formula (MVP)
```
score = bm25(query, doc) * quality_weight * recency_decay
quality_weight = 0.3 + 0.7 * (quality_score / 100)
```

### Quality heuristics (no LLM required)
- Penalize sensational keywords (BREAKING, SHOCKING, you won't believe)
- Boost technical terms, citations, word count in sweet spot (800–4000)
- Boost domains on allowlist (arxiv, nature, ieee, github blog, etc.)

## Curated News — Feed Design

### Two prongs (per spec)
1. **Human-quality crawl** — score articles, surface score ≥ 70
2. **AI synthesis** (Phase 2) — merge sources into neutral rewrite

### Feed filters
- Hide politics sensationalism by default
- Surface under-reported topics (science, engineering, policy analysis)
- Sort by quality × recency

## Data Model

| Entity  | Fields |
|---------|--------|
| Page    | id, url, domain, title, content, content_type, quality_score, indexed_at |
| FeedItem| id, page_id, headline, summary, synthesized, published_at, topics[] |

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | /api/search?q=&site=&limit= | Unified search |
| GET | /api/news?limit=&offset= | Curated feed |
| GET | /api/sites | Indexed domains |
| GET | /api/health | Health check |

## Deployment

- **Backend**: Render Web Service
- **Frontend**: Render Static Site
- **Domain**: omnisearch.xyz via Vercel DNS

## Phase Roadmap

| Phase | Omnisearch | Curated News |
|-------|------------|--------------|
| 1 | BM25 search + seeded index | Scored feed from crawl |
| 2 | Live crawler | AI synthesis |
| 3 | Vector embeddings | Topic personalization |
