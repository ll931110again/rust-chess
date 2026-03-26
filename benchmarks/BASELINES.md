# Benchmark Baselines

This file tracks repeatable performance snapshots for future development.

## How to Reproduce

Build release first:

```bash
cargo build --release
```

Speed benchmark command:

```bash
.venv/bin/python -u scripts/benchmark.py --engine target/release/rustchess speed --perft-depth 5 --search-depth <N> --repeats 5
```

## Latest Speed Baseline (depth 6/7/8)

Environment:
- Engine: `target/release/rustchess`
- Script: `scripts/benchmark.py speed`
- Perft depth: `5`
- Repeats: `3`

### Search depth 6
- Search nodes: `3,816,963`
- Wall time: `0.545s` to `0.560s` (avg ~`0.553s`)
- NPS: `6,816,082` to `6,997,278` (avg ~`6.90M`)
- Score: `score_cp=10` (stable across runs)

### Search depth 7
- Search nodes: `32,511,354`
- Wall time: `6.063s` to `6.170s` (avg ~`6.125s`)
- NPS: `5,269,110` to `5,362,610` (avg ~`5.31M`)
- Score: `score_cp=30` (stable across runs)

### Search depth 8
- Search nodes: `290,571,590`
- Wall time: `51.492s` to `51.772s` (avg ~`51.625s`)
- NPS: `5,612,496` to `5,643,004` (avg ~`5.63M`)
- Score: `score_cp=18` (stable across runs)

## Scaling Notes

- Depth `7 -> 8` is ~`8.9x` more nodes and ~`8.4x` more time.
- Search throughput is roughly stable (~`5.3M` to `5.6M NPS`) at deeper depths.

## Match Baseline Snapshot

- `benchmarks/stockfish_1800_depth6_100games_with_qs_null.log`
  - Final W-L-D: `54-27-19`
  - Elo estimate (rustchess - opponent): `+96.2`
  - Runtime: ~`80m`

## Update Template

When updating this file, include:
- commit/branch context
- exact command(s) used
- engine options (`Threads`, depth, movetime, etc.)
- machine notes if materially different (CPU/core count)
