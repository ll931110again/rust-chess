# rustchess

A minimal, working UCI chess engine in Rust.

This project currently focuses on core engine basics:
- board representation
- legal move generation and move tree exploration
- iterative deepening search with alpha-beta pruning
- UCI protocol loop so it can run in GUIs or the terminal

## Project Artifacts

- `Cargo.toml` - Rust crate metadata and dependencies.
- `src/main.rs` - program entry point; starts the UCI loop.
- `src/uci.rs` - UCI command handling (`uci`, `isready`, `position`, `go`, `quit`).
- `src/board.rs` - board state model, FEN parsing, move application.
- `src/movegen.rs` - pseudo-legal and legal move generation, check detection, perft.
- `src/search.rs` - iterative deepening negamax alpha-beta search and evaluation.
- `scripts/play_cli.py` - terminal helper to play White vs the engine (Black).
- `target/release/rustchess` - optimized engine binary (after release build).

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
