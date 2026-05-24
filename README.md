# rust-chess

A minimal, working UCI chess engine in Rust.

This project currently focuses on core engine basics:
- board representation
- legal move generation and move tree exploration
- iterative deepening search with alpha-beta, quiescence, null-move pruning, and extensions
- UCI protocol loop so it can run in GUIs or the terminal

## Project Artifacts

- `Cargo.toml` - Rust crate metadata and dependencies.
- `src/main.rs` - program entry point; starts the UCI loop.
- `src/uci.rs` - UCI command handling (`uci`, `isready`, `position`, `go`, `quit`).
- `src/board.rs` - board state model, FEN parsing, move application.
- `src/movegen.rs` - pseudo-legal and legal move generation, check detection, perft.
- `src/opening_book.rs` - local opening-book client (`/move?fen=...`) with safe fallback to search.
- `src/search.rs` - iterative deepening negamax search, alpha-beta pruning, quiescence, null-move pruning, and extensions.
- `scripts/play_cli.py` - terminal helper to play White vs the engine (Black).
- `scripts/benchmark.py` - benchmark helper for speed tests and cutechess matches.
- `target/release/rustchess` - optimized engine binary (after release build).

## Search Features Matrix

Current search status (see `src/search.rs` for implementation details):

| Capability | Status | Notes |
|---|---|---|
| Iterative deepening | Implemented | Searches depth 1..N and keeps last complete result |
| Root parallel search | Implemented | Parallelizes root move search across configured threads |
| Negamax + alpha-beta | Implemented | Main full-width search |
| Quiescence search | Implemented | Tactical-only continuation at leaf (captures/promotions/en-passant) |
| Null-move pruning | Implemented | Reduced-depth null search with basic safeguards |
| Move ordering | Implemented | Capture/promotion priority (simple MVV-like ordering) |
| Check extension | Implemented | Extends a line when side to move is in check |
| Singular extension | Implemented (lightweight) | Shallow probe; extends move if clearly best |
| Passed pawn extension | Implemented | Extends pawn pushes reaching 7th rank (or 2nd for Black) |
| Time management | Implemented (basic) | Uses movetime or simple budget from remaining clock/increment |
| Opening book | Implemented | Local opening-book service lookup, toggle via `OwnBook` UCI option |
| Transposition table (TT) | Not implemented | No hash table, no TT cutoffs, no hash move ordering |
| Repetition / 50-move draw in search | Not implemented | Draw resolution mostly handled by outer game flow/driver |
| Aspiration windows | Not implemented | Uses full-window root iterations |
| Killer/history heuristics | Not implemented | No quiet-move history/killer ordering |
| LMR / futility pruning | Not implemented | No late-move or static-bound pruning |
| PV table output | Not implemented | UCI `pv` currently reports best root move only |

### Search Functionality (Current Behavior)

- Search loop uses iterative deepening with UCI-compatible `info depth ...` output.
- Root search is parallelized across legal root moves (`Threads` option).
- Core tree search is negamax with alpha-beta pruning.
- Depth-0 nodes switch to quiescence (captures/promotions/en-passant only) to reduce horizon noise.
- Null-move pruning is enabled in non-check, non-pawn-only positions.
- Extensions currently include:
  - check extension
  - singular extension (lightweight probe)
  - passed pawn push-to-7th/2nd extension
- Time handling supports `movetime` and basic clock-allocation from `wtime`/`btime` (+ increments).

### Search Functionality (Not Yet Implemented)

- Transposition table / hash move ordering
- Aspiration windows
- Killer/history heuristics
- Late-move reductions and futility pruning
- Full principal-variation line output (beyond current single best root move)

## Evaluation Functionality

Current evaluation status (see `src/eval.rs`):

### Evaluation (Implemented)

- Classical static evaluation (centipawn scale).
- Material values by piece type.
- Piece-square tables (PST) for all pieces.
- Pawn structure terms:
  - doubled pawn penalties
  - isolated pawn penalties
  - passed pawn bonuses (scaled by advancement)
  - small central pawn-mass bonus
- Bishop pair bonus.
- Tempo bonus for side to move.
- Side-to-move normalization (score is from current side perspective).

### Evaluation (Not Yet Implemented)

- Phase-aware blending (middlegame/endgame interpolation).
- Mobility terms by piece type.
- King safety model (pawn shield, attack pressure, open files near king).
- Rook/queen file terms (open/semi-open files).
- Outposts, trapped piece, and space heuristics.
- Pair-specific terms beyond bishop pair (e.g., knight pair interactions).
- Advanced pawn structure terms (backward pawns, candidate passers, connected passers).
- Specialized endgame knowledge (e.g., opposition, fortress recognition, tablebases).

## Build

```bash
cargo build --release
```

Binary path:

```text
target/release/rustchess
```

## Run (UCI mode)

```bash
./target/release/rustchess
```

Example handshake:

```text
uci
isready
ucinewgame
position startpos
go depth 4
quit
```

## Playing from Command Line

You can play by repeatedly sending `position ... moves ...` and `go ...`.

Example (you are White):

```text
position startpos moves e2e4
go depth 4
```

Engine answers:

```text
bestmove c7c5
```

Then continue by appending moves:

```text
position startpos moves e2e4 c7c5 g1f3
go depth 4
```

For easier play, use the helper script:

```bash
python3 -m pip install chess
python3 scripts/play_cli.py --depth 4
```

The script keeps game state for you and asks for UCI moves (like `e2e4`).

## Move Notation Explained (UCI Moves)

Moves use coordinate notation:

- `e2e4` - move piece from `e2` to `e4`
- `g1f3` - knight from `g1` to `f3`
- `e1g1` - king-side castling
- `e1c1` - queen-side castling
- `e7e8q` - promotion to queen (`q`)
- `e7e8n` - promotion to knight (`n`)

Promotion suffixes:
- `q` = queen
- `r` = rook
- `b` = bishop
- `n` = knight

Note: UCI moves are not SAN/PGN notation.
- UCI: `e2e4`
- SAN: `e4`

## Supported UCI Commands

- `uci`
- `isready`
- `ucinewgame`
- `position startpos [moves ...]`
- `position fen <fen-string> [moves ...]`
- `go depth <n>`
- `go movetime <ms>`
- `go wtime <ms> btime <ms> [winc <ms>] [binc <ms>] [movestogo <n>]`
- `go perft <depth>`
- `setoption name OwnBook value true|false`

Note: `OwnBook` queries a local service by default (`http://127.0.0.1:8765/move`).
If the service is unavailable or returns no move, the engine automatically falls back to normal search.

### Local Opening Book Setup

Download a strong Polyglot book:

```bash
bash scripts/download_opening_book.sh
```

Start local book service (requires `python-chess`):

```bash
python3 -m pip install chess
python3 scripts/opening_book_server.py --book books/komodo.bin --host 127.0.0.1 --port 8765
```

Optional override endpoint:

```bash
RUSTCHESS_BOOK_URL="http://127.0.0.1:8765/move" ./target/release/rustchess
```
- `stop`
- `quit`

## Quick Smoke Test

```bash
printf "uci\nisready\nposition startpos\ngo depth 3\nquit\n" | ./target/release/rustchess
```

You should see:
- `uciok`
- `readyok`
- one or more `info depth ...` lines
- `bestmove ...`

## Benchmarking

Build release first:

```bash
cargo build --release
```

Run local speed benchmark:

```bash
python3 scripts/benchmark.py speed --perft-depth 5 --search-depth 6 --repeats 3
```

Run match benchmark vs another UCI engine (Elo estimate from W/D/L):

```bash
python3 scripts/benchmark.py match --opponent stockfish --tc 10+0.1 --games 100
```

If `cutechess-cli` is unavailable, use the built-in lightweight fallback:

```bash
python3 scripts/benchmark.py mini-match --opponent stockfish --games 20 --movetime-ms 100
```

For ongoing engine tuning, keep benchmark snapshots in:

- `benchmarks/BASELINES.md` - recorded speed/Elo baselines and commands used.
