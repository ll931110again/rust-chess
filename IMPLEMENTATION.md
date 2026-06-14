# QualityDocs — Implementation Notes

## Design Goal Mapping

| Goal | Implementation |
|------|----------------|
| Host high-quality public design docs | 10 seeded documents from major research/engineering sources |
| Technical, academic tone | Full markdown content with code blocks, tables, formal structure |
| AI explain for confusing sections | `POST /api/documents/:slug/explain` with OpenAI fallback to heuristic summary |

## Key Decisions

1. **SQLite over Postgres** — Sufficient for curated library; zero-config on Render free tier with persistent disk
2. **Seed-on-first-run** — Idempotent seeding avoids migration complexity for demo content
3. **Reader typography** — Source Serif 4 for body, Inter for UI; dark theme reduces eye strain for long reads
4. **Explain panel** — Text selection → side panel; works without API key using template explanation

## File Structure

```
qualitydocs/
├── DESIGN.md          # Architecture & roadmap
├── IMPLEMENTATION.md  # This file
├── backend/           # Rust API
├── frontend/          # Next.js app
└── render.yaml        # Render blueprint
```

## API Contract

Document list supports `?q=` full-text search and `?tag=` filter. Explain endpoint accepts `{ section, instruction? }`.

## Next Steps

- Live crawler for freshness (Phase 2)
- Vector search for semantic discovery
- User bookmarks and reading progress
