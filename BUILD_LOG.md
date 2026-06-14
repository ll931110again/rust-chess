# Build Log — Omnisearch Platform

See also `../qualitydocs/BUILD_LOG.md` for shared deployment notes.

## Platform Split

One codebase, two user-facing products:

1. **Omnisearch** (`/search`) — global and site-scoped search with quality-weighted BM25
2. **Curated News** (`/news`) — feed of articles scoring ≥70 on heuristic quality scale

## Search Implementation Notes

SQLite FTS5 virtual table `pages_fts` joins to `pages` for retrieval. Query tokens are OR-joined for recall. Results re-sorted by `abs(bm25) × quality_boost(score)`.

Seeded **32 pages** across diverse domains intentionally including under-reported topics (quantum policy, urban heat, Fortran compilers, ocean carbon) per the curated news spec.

## Domain Choice

`omnisearch.xyz` unavailable at registrar. Selected **`omni-search.xyz`** ($0.99/yr) — easy to type, hyphenated variant of the product name.

## Live URLs

- Frontend: https://omnisearch-five.vercel.app
- API: https://omnisearch-api.onrender.com
- Git branch: `omnisearch-main` on `ll931110again/rust-chess`

## Try It

```bash
# Search
curl "https://omnisearch-api.onrender.com/api/search?q=database&limit=5"

# Curated feed
curl "https://omnisearch-api.onrender.com/api/news?limit=5"
```
