# QualityDocs

High-quality technical design document library. Curated engineering literature with search and AI explain.

## Stack

- **Backend**: Rust (axum, sqlx, SQLite)
- **Frontend**: Next.js 14, TypeScript, Tailwind CSS

## Local Development

```bash
# Backend
cd backend
cargo run

# Frontend (separate terminal)
cd frontend
NEXT_PUBLIC_API_URL=http://localhost:8080 npm run dev
```

## Deployment

See `render.yaml` for Render blueprint deployment.

Domain: **qualitydocs.xyz**
