# Crawler Architecture

See `../omnisearch/CRAWLER.md` for shared design. QualityDocs-specific notes:

## Focus

Ingests **technical and design documentation** from engineering blogs and research feeds — not general news.

## Quality scoring (`src/crawler/quality.rs`)

- Penalizes listicles, sponsored content, sensational phrasing
- Boosts architecture, implementation, research terminology
- Requires ~400+ words for full score boost; rejects < 200 words
- Trusted domains: Google Research, OpenAI, Anthropic, AWS, Stripe, arXiv, etc.

## Output

Each crawled article becomes a `documents` row with:
- Full markdown content (extracted from article HTML)
- Auto-generated tags (architecture, ai, databases, etc.)
- Unique slug (deduplicated)

## Sources

15 RSS/Atom feeds from major engineering publications. Failed feeds are logged and skipped; crawl continues.
