#!/usr/bin/env python3
"""Play against rustchess in terminal (human White vs engine Black)."""

from __future__ import annotations

import argparse
from pathlib import Path
import sys

import chess
import chess.engine


def parse_args() -> argparse.Namespace:
    repo_root = Path(__file__).resolve().parents[1]
    default_engine = repo_root / "target" / "release" / "rustchess"

    parser = argparse.ArgumentParser(
        description="Play a CLI chess game vs rustchess (engine as Black)."
    )
    parser.add_argument(
        "--engine",
        type=Path,
        default=default_engine,
        help=f"Path to UCI engine binary (default: {default_engine})",
    )
    parser.add_argument(
        "--depth",
        type=int,
        default=4,
        help="Search depth for engine moves (default: 4)",
    )
    return parser.parse_args()


def print_help() -> None:
    print("Commands:")
    print("  <uci-move>  e.g. e2e4, g1f3, e7e8q")
    print("  board       print board")
    print("  fen         print current FEN")
    print("  help        show this help")
    print("  quit        exit game")


def main() -> int:
    args = parse_args()
    engine_path = args.engine.expanduser().resolve()

    if not engine_path.exists():
        print(f"Engine not found: {engine_path}")
        print("Build first: cargo build --release")
        return 1

    board = chess.Board()
    print("rustchess CLI")
    print("You play White, engine plays Black.")
    print_help()

    try:
        with chess.engine.SimpleEngine.popen_uci(str(engine_path)) as engine:
            while not board.is_game_over():
                print("\n" + str(board) + "\n")
                user = input("White to move > ").strip().lower()

                if user in {"quit", "exit"}:
                    print("Exiting game.")
                    return 0
                if user == "help":
                    print_help()
                    continue
                if user == "board":
                    print(board)
                    continue
                if user == "fen":
                    print(board.fen())
                    continue
                if not user:
                    continue

                try:
                    user_move = chess.Move.from_uci(user)
                except ValueError:
                    print("Invalid move format. Use UCI like e2e4 or e7e8q.")
                    continue

                if user_move not in board.legal_moves:
                    print("Illegal move in current position.")
                    continue

                board.push(user_move)
                if board.is_game_over():
                    break

                result = engine.play(board, chess.engine.Limit(depth=max(1, args.depth)))
                board.push(result.move)
                print(f"Engine (Black): {result.move.uci()}")

    except FileNotFoundError:
        print(f"Failed to launch engine: {engine_path}")
        return 1
    except KeyboardInterrupt:
        print("\nInterrupted.")
        return 0

    print("\nFinal board:")
    print(board)
    print(f"Result: {board.result()} ({board.outcome()})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
