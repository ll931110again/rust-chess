# Omnisearch — Implementation Notes

## Three Design Goals, Two Products

This platform implements **two** of the three experimentals design goals:

| Design Goal | Product | Route |
|-------------|---------|-------|
| Cross-site quality search | Omnisearch | `/search` |
| Curated non-sensational news | Curated News | `/news` |

The third goal (QualityDocs) is a separate project in `../qualitydocs/`.

## Quality Scoring

Heuristic scorer in `backend/src/quality.rs`:

- **Penalties**: sensational keywords (breaking, shocking, trump, etc.)
- **Boosts**: technical vocabulary, allowlisted domains (arxiv, nature, ietf, etc.)
- **Sweet spot**: 150–2500 word content length

Search ranking: `BM25 × quality_weight` where `quality_weight = 0.3 + 0.7 × (score/100)`.

## Indexing

SQLite FTS5 virtual table `pages_fts` synced via rebuild on seed. Crawler module stubbed for Phase 2 continuous indexing.

## Seeded Content

32 pages across arxiv, research blogs, standards bodies, and technical publications — intentionally diverse topics beyond mainstream news.

## Frontend Architecture

- `/search` — client-side search with global/site-scoped toggle
- `/news` — server-rendered feed (dynamic) from quality-filtered API

## Next Steps

- Vector embeddings for semantic search
- AI synthesis pipeline (multi-source neutral rewrites)
- Scheduled crawler worker on Render cron
