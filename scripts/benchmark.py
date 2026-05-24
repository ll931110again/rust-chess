#!/usr/bin/env python3
"""Benchmark helpers for rustchess.

Modes:
- speed: run repeatable local engine speed benchmarks (perft + fixed-depth search)
- match: launch cutechess-cli matches and estimate Elo from W/D/L score
"""

from __future__ import annotations

import argparse
import math
import os
from pathlib import Path
import re
import shlex
import statistics
import subprocess
import sys
import time
from typing import Optional


PERFT_RE = re.compile(r"perft depth (\d+) nodes (\d+)")
SEARCH_DONE_RE = re.compile(
    r"info string search done depth (\d+) score_cp (-?\d+) nodes (\d+) time (\d+)"
)
SCORE_RE = re.compile(
    r"Score of .*?:\s*(\d+)\s*-\s*(\d+)\s*-\s*(\d+)\s*\[\s*([0-9.]+)\s*\]\s*(\d+)"
)


def parse_args() -> argparse.Namespace:
    repo_root = Path(__file__).resolve().parents[1]
    default_engine = repo_root / "target" / "release" / "rustchess"

    parser = argparse.ArgumentParser(description="Benchmark and test rustchess.")
    parser.add_argument(
        "--engine",
        type=Path,
        default=default_engine,
        help=f"Path to UCI engine binary (default: {default_engine})",
    )

    subparsers = parser.add_subparsers(dest="mode", required=True)

    speed = subparsers.add_parser("speed", help="Run local speed benchmarks.")
    speed.add_argument("--perft-depth", type=int, default=5, help="Perft depth (default: 5)")
    speed.add_argument(
        "--search-depth", type=int, default=6, help="Fixed search depth benchmark (default: 6)"
    )
    speed.add_argument("--repeats", type=int, default=3, help="Number of runs (default: 3)")

    match = subparsers.add_parser("match", help="Run cutechess matches and estimate Elo.")
    match.add_argument(
        "--opponent",
        required=True,
        help="Opponent engine command, e.g. stockfish or /path/to/engine",
    )
    match.add_argument("--tc", default="10+0.1", help="Time control (default: 10+0.1)")
    match.add_argument("--games", type=int, default=100, help="Total games (default: 100)")
    match.add_argument(
        "--openings-file",
        type=Path,
        default=None,
        help="Optional EPD openings file path",
    )
    match.add_argument(
        "--cutechess-cmd",
        default="cutechess-cli",
        help="cutechess-cli executable name/path",
    )

    mini = subparsers.add_parser(
        "mini-match",
        help="Run lightweight direct UCI matches via python-chess (no cutechess required).",
    )
    mini.add_argument(
        "--opponent",
        required=True,
        help="Opponent engine command, e.g. stockfish or '/path/to/engine --arg'",
    )
    mini.add_argument(
        "--opponent-elo",
        type=int,
        default=None,
        help="If supported, set opponent UCI_Elo (enables UCI_LimitStrength).",
    )
    mini.add_argument(
        "--use-all-cores",
        action="store_true",
        help="Try to set UCI Threads to all CPU cores where supported.",
    )
    mini.add_argument(
        "--our-option",
        action="append",
        default=[],
        help="UCI option for our engine as KEY=VALUE (repeatable).",
    )
    mini.add_argument(
        "--opponent-option",
        action="append",
        default=[],
        help="UCI option for opponent engine as KEY=VALUE (repeatable).",
    )
    mini.add_argument("--games", type=int, default=20, help="Total games (default: 20)")
    mini.add_argument(
        "--depth",
        type=int,
        default=None,
        help="Use fixed search depth for both engines (overrides movetime).",
    )
    mini.add_argument(
        "--movetime-ms",
        type=int,
        default=100,
        help="Per-move time for both engines in ms (default: 100)",
    )
    mini.add_argument(
        "--max-plies",
        type=int,
        default=300,
        help="Max plies before forced draw (default: 300)",
    )
    mini.add_argument(
        "--no-ply-cutoff",
        action="store_true",
        help="Do not force-draw by ply limit; play until natural game termination.",
    )
    mini.add_argument(
        "--full-game-log",
        action="store_true",
        default=True,
        help="Print full move list for every game (default: enabled).",
    )
    mini.add_argument(
        "--no-full-game-log",
        action="store_false",
        dest="full_game_log",
        help="Disable full move list printing for each game.",
    )

    return parser.parse_args()


def ensure_engine(path: Path) -> Path:
    resolved = path.expanduser().resolve()
    if not resolved.exists():
        print(f"Engine not found: {resolved}")
        print("Build first: cargo build --release")
        raise SystemExit(1)
    return resolved


def run_engine(engine_path: Path, commands: str, timeout_s: float = 60.0) -> tuple[str, str, float]:
    t0 = time.perf_counter()
    proc = subprocess.Popen(
        [str(engine_path)],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    out, err = proc.communicate(commands, timeout=timeout_s)
    dt = time.perf_counter() - t0
    if proc.returncode != 0:
        print("Engine exited with non-zero status")
        print(err.strip())
        raise SystemExit(1)
    return out, err, dt


def run_speed(engine_path: Path, perft_depth: int, search_depth: int, repeats: int) -> int:
    perft_times = []
    perft_nps = []
    perft_nodes_ref: Optional[int] = None

    print("== Speed benchmark ==")
    print(f"engine: {engine_path}")
    print(f"perft depth: {perft_depth}, search depth: {search_depth}, repeats: {repeats}")

    for i in range(1, repeats + 1):
        out, _, dt = run_engine(
            engine_path,
            f"uci\nisready\nposition startpos\ngo perft {perft_depth}\nquit\n",
            timeout_s=120.0,
        )
        match = PERFT_RE.search(out)
        if not match:
            print("Could not parse perft output.")
            print(out)
            return 1
        nodes = int(match.group(2))
        if perft_nodes_ref is None:
            perft_nodes_ref = nodes
        elif perft_nodes_ref != nodes:
            print(f"WARNING: perft nodes changed between runs ({perft_nodes_ref} vs {nodes})")

        nps = int(nodes / dt) if dt > 0 else 0
        perft_times.append(dt)
        perft_nps.append(nps)
        print(f"run {i}: perft nodes={nodes} time={dt:.3f}s nps={nps}")

    print(
        "perft summary: "
        f"avg_time={statistics.mean(perft_times):.3f}s "
        f"median_time={statistics.median(perft_times):.3f}s "
        f"avg_nps={int(statistics.mean(perft_nps))}"
    )

    for i in range(1, repeats + 1):
        out, _, dt = run_engine(
            engine_path,
            f"uci\nisready\nposition startpos\ngo depth {search_depth}\nquit\n",
            timeout_s=120.0,
        )
        match = SEARCH_DONE_RE.search(out)
        if not match:
            print("Could not parse fixed-depth search output.")
            print(out)
            return 1
        depth = int(match.group(1))
        score_cp = int(match.group(2))
        nodes = int(match.group(3))
        reported_ms = int(match.group(4))
        nps = int(nodes / dt) if dt > 0 else 0
        print(
            f"run {i}: search depth={depth} score_cp={score_cp} "
            f"nodes={nodes} wall={dt:.3f}s engine_time={reported_ms}ms nps={nps}"
        )

    return 0


def elo_from_score(wins: int, losses: int, draws: int) -> float:
    total = wins + losses + draws
    if total == 0:
        return 0.0
    score = (wins + 0.5 * draws) / total
    eps = 1e-9
    score = min(max(score, eps), 1.0 - eps)
    return -400.0 * math.log10((1.0 / score) - 1.0)


def parse_uci_options(raw_options: list[str]) -> dict[str, object]:
    options: dict[str, object] = {}
    for item in raw_options:
        if "=" not in item:
            continue
        key, value = item.split("=", 1)
        key = key.strip()
        value = value.strip()
        if not key:
            continue
        lower = value.lower()
        if lower in {"true", "false"}:
            options[key] = (lower == "true")
        else:
            try:
                options[key] = int(value)
            except ValueError:
                options[key] = value
    return options


def configure_supported_options(engine, options: dict[str, object], label: str) -> None:
    supported = {}
    for key, value in options.items():
        if key in engine.options:
            supported[key] = value
        else:
            print(f"note: {label} does not support UCI option '{key}', skipping")
    if supported:
        engine.configure(supported)
        rendered = ", ".join(f"{k}={v}" for k, v in supported.items())
        print(f"{label} configured: {rendered}")


def run_match(
    engine_path: Path,
    opponent_cmd: str,
    tc: str,
    games: int,
    openings_file: Optional[Path],
    cutechess_cmd: str,
) -> int:
    rounds = max(1, games // 2)
    cmd = [
        cutechess_cmd,
        "-engine",
        f"name=rustchess",
        f"cmd={engine_path}",
        "proto=uci",
        "-engine",
        "name=opponent",
        f"cmd={opponent_cmd}",
        "proto=uci",
        "-each",
        f"tc={tc}",
        "-games",
        str(games),
        "-rounds",
        str(rounds),
        "-repeat",
        "-recover",
    ]
    if openings_file is not None:
        openings = openings_file.expanduser().resolve()
        cmd.extend(
            [
                "-openings",
                f"file={openings}",
                "format=epd",
                "order=random",
            ]
        )

    print("== Match benchmark ==")
    print("command:")
    print(" ".join(map(str, cmd)))

    proc = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    score_line = None
    assert proc.stdout is not None
    for line in proc.stdout:
        print(line, end="")
        if "Score of" in line:
            score_line = line.strip()
    rc = proc.wait()
    if rc != 0:
        print(f"cutechess-cli failed with exit code {rc}")
        return rc
    if not score_line:
        print("Could not find final score line in cutechess output.")
        return 1

    m = SCORE_RE.search(score_line)
    if not m:
        print("Could not parse score line:")
        print(score_line)
        return 1

    wins = int(m.group(1))
    losses = int(m.group(2))
    draws = int(m.group(3))
    rate = float(m.group(4))
    total = int(m.group(5))
    elo = elo_from_score(wins, losses, draws)
    print()
    print(f"W-L-D: {wins}-{losses}-{draws} ({total} games, score={rate:.3f})")
    print(f"Elo estimate (rustchess - opponent): {elo:+.1f}")
    return 0


def run_mini_match(
    engine_path: Path,
    opponent_cmd: str,
    opponent_elo: Optional[int],
    use_all_cores: bool,
    our_options: dict[str, object],
    opponent_options: dict[str, object],
    games: int,
    depth: Optional[int],
    movetime_ms: int,
    max_plies: int,
    no_ply_cutoff: bool,
    full_game_log: bool,
) -> int:
    try:
        import chess
        import chess.engine
    except ImportError:
        print("python-chess is required for mini-match mode.")
        print("Install it in a virtualenv:")
        print("  python3 -m venv .venv")
        print("  source .venv/bin/activate")
        print("  pip install chess")
        return 1

    our_cmd = [str(engine_path)]
    opp_cmd = shlex.split(opponent_cmd)
    if not opp_cmd:
        print("Invalid --opponent command")
        return 1

    wins = 0
    losses = 0
    draws = 0

    print("== Mini match (direct UCI) ==")
    print(f"our engine: {' '.join(our_cmd)}")
    print(f"opponent: {' '.join(opp_cmd)}")
    if depth is not None:
        print(f"games: {games}, depth: {depth}, max_plies: {max_plies}")
    else:
        print(f"games: {games}, movetime_ms: {movetime_ms}, max_plies: {max_plies}")

    try:
        with chess.engine.SimpleEngine.popen_uci(our_cmd) as our_engine, chess.engine.SimpleEngine.popen_uci(opp_cmd) as opp_engine:
            if use_all_cores:
                cores = max(1, os.cpu_count() or 1)
                print(f"requested all CPU cores: {cores}")
                configure_supported_options(our_engine, {"Threads": cores}, "our engine")
                configure_supported_options(opp_engine, {"Threads": cores}, "opponent")

            configure_supported_options(our_engine, our_options, "our engine")
            configure_supported_options(opp_engine, opponent_options, "opponent")

            if opponent_elo is not None:
                try:
                    target_elo = int(opponent_elo)
                    clamped_elo = max(1320, min(3190, target_elo))
                    if clamped_elo != target_elo:
                        print(
                            f"note: requested opponent elo {target_elo} is outside Stockfish-supported "
                            f"range; using nearest supported value {clamped_elo}"
                        )
                    opp_engine.configure({"UCI_LimitStrength": True, "UCI_Elo": clamped_elo})
                    print(f"opponent limited to UCI_Elo={clamped_elo}")
                except Exception as exc:
                    print(f"warning: could not set opponent elo to {opponent_elo}: {exc}")

            if depth is not None:
                limit = chess.engine.Limit(depth=max(1, depth))
            else:
                limit = chess.engine.Limit(time=max(0.001, movetime_ms / 1000.0))
            match_start = time.perf_counter()
            for game_idx in range(games):
                board = chess.Board()
                our_is_white = (game_idx % 2 == 0)
                ply = 0
                game_moves = []

                while not board.is_game_over(claim_draw=True) and (no_ply_cutoff or ply < max_plies):
                    white_to_move = board.turn == chess.WHITE
                    our_turn = (white_to_move and our_is_white) or (
                        (not white_to_move) and (not our_is_white)
                    )
                    engine = our_engine if our_turn else opp_engine
                    result = engine.play(board, limit)
                    game_moves.append(result.move.uci())
                    board.push(result.move)
                    ply += 1

                outcome = board.outcome(claim_draw=True)
                if outcome is None:
                    draws += 1
                    game_result = "1/2-1/2 (max plies)"
                else:
                    if outcome.winner is None:
                        draws += 1
                    else:
                        our_won = (outcome.winner == chess.WHITE and our_is_white) or (
                            outcome.winner == chess.BLACK and not our_is_white
                        )
                        if our_won:
                            wins += 1
                        else:
                            losses += 1
                    game_result = outcome.result()

                elapsed_s = max(0.001, time.perf_counter() - match_start)
                done = game_idx + 1
                avg_s_per_game = elapsed_s / done
                eta_s = avg_s_per_game * (games - done)
                print(
                    f"game {done}/{games}: our_color={'W' if our_is_white else 'B'} "
                    f"result={game_result} plies={len(game_moves)} W-L-D={wins}-{losses}-{draws} "
                    f"elapsed={elapsed_s:.1f}s avg={avg_s_per_game:.1f}s/game eta={eta_s:.1f}s"
                )
                if full_game_log:
                    move_chunks = []
                    for idx in range(0, len(game_moves), 2):
                        move_no = idx // 2 + 1
                        white = game_moves[idx]
                        black = game_moves[idx + 1] if idx + 1 < len(game_moves) else ""
                        move_chunks.append(f"{move_no}. {white} {black}".rstrip())
                    print(f"full game {game_idx + 1}: {' '.join(move_chunks)}")
                if (game_idx + 1) % 10 == 0:
                    preview = " ".join(game_moves if full_game_log else game_moves[:40])
                    if not full_game_log and len(game_moves) > 40:
                        preview += " ..."
                    print(
                        f"sample game {game_idx + 1}: plies={len(game_moves)} "
                        f"moves={preview}"
                    )
    except FileNotFoundError as exc:
        print(f"Failed to launch engine: {exc}")
        return 1

    elo = elo_from_score(wins, losses, draws)
    total = wins + losses + draws
    score = (wins + 0.5 * draws) / total if total else 0.0
    print()
    print(f"Final W-L-D: {wins}-{losses}-{draws} ({total} games, score={score:.3f})")
    print(f"Elo estimate (rustchess - opponent): {elo:+.1f}")
    return 0


def main() -> int:
    args = parse_args()
    engine = ensure_engine(args.engine)

    if args.mode == "speed":
        return run_speed(
            engine_path=engine,
            perft_depth=max(1, args.perft_depth),
            search_depth=max(1, args.search_depth),
            repeats=max(1, args.repeats),
        )
    if args.mode == "match":
        return run_match(
            engine_path=engine,
            opponent_cmd=args.opponent,
            tc=args.tc,
            games=max(2, args.games),
            openings_file=args.openings_file,
            cutechess_cmd=args.cutechess_cmd,
        )
    if args.mode == "mini-match":
        our_options = parse_uci_options(args.our_option)
        opponent_options = parse_uci_options(args.opponent_option)
        return run_mini_match(
            engine_path=engine,
            opponent_cmd=args.opponent,
            opponent_elo=args.opponent_elo,
            use_all_cores=args.use_all_cores,
            our_options=our_options,
            opponent_options=opponent_options,
            games=max(2, args.games),
            depth=max(1, args.depth) if args.depth is not None else None,
            movetime_ms=max(1, args.movetime_ms),
            max_plies=max(20, args.max_plies),
            no_ply_cutoff=args.no_ply_cutoff,
            full_game_log=args.full_game_log,
        )

    print("Unknown mode")
    return 1


if __name__ == "__main__":
    sys.exit(main())
