# Crawler Architecture

## Overview

Both platforms run a **background RSS crawler** that continuously ingests high-quality content. The crawler starts automatically with the API server.

## Performance Design

| Technique | Purpose |
|-----------|---------|
| RSS/Atom feeds | Fast, polite ingestion without full-site spidering |
| Concurrent page fetch (12 workers) | Full article text extraction in parallel |
| HTTP connection pooling | Reuse TCP connections per host |
| Semaphore rate limiting | Avoid overwhelming sources |
| SQLite FTS5 triggers | Incremental search index updates (Omnisearch) |
| Upsert by URL | Idempotent re-crawls update existing records |

## Omnisearch — News Crawler

**Sources (12 feeds):** Ars Technica, BBC Science, Nature, Quanta, MIT Tech Review, NYT Science, NASA, The Conversation, ScienceDaily, Reuters, Brookings, IEEE Spectrum.

**Quality gate:** Articles scoring ≥ 65 are stored; curated feed shows ≥ 70.

**Scoring:** Penalizes sensational keywords; boosts technical vocabulary and trusted domains.

## QualityDocs — Technical Docs Crawler

**Sources (15 feeds):** Google Research, OpenAI, Anthropic, Rust Blog, AWS Architecture, Cloudflare, GitHub Engineering, Stripe, Martin Fowler, Meta/Uber/Netflix engineering, arXiv CS, LLVM.

**Quality gate:** Documents scoring ≥ 75 are ingested (technical depth, word count, trusted sources).

## Configuration

| Env var | Default | Description |
|---------|---------|-------------|
| `CRAWLER_ENABLED` | `true` | Set `false` to disable background worker |
| `CRAWLER_INTERVAL_SECS` | `900` | Seconds between crawl runs (15 min) |

## API

```
GET  /api/crawler/status   — last run stats
POST /api/crawler/run      — trigger immediate crawl
```

## Manual trigger

```bash
curl -X POST https://omnisearch-api.onrender.com/api/crawler/run
curl -X POST https://qualitydocs-api.onrender.com/api/crawler/run
```
