#!/usr/bin/env python3
"""
Serve a local Polyglot opening book over HTTP.

Endpoint:
  GET /move?fen=<FEN>

Response JSON:
  {"uci": "e2e4", "weight": 123}
  {"uci": null}
"""

from __future__ import annotations

import argparse
import json
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import parse_qs, urlparse

import chess
import chess.polyglot


def choose_book_move(reader: chess.polyglot.MemoryMappedReader, fen: str) -> dict[str, object]:
    try:
        board = chess.Board(fen)
    except ValueError:
        return {"error": "invalid fen"}

    entries = list(reader.find_all(board))
    if not entries:
        return {"uci": None}

    # Deterministic strongest move by weight, then UCI tie-break.
    entries.sort(key=lambda e: (e.weight, e.move.uci()), reverse=True)
    best = entries[0]
    return {"uci": best.move.uci(), "weight": int(best.weight)}


def build_handler(reader: chess.polyglot.MemoryMappedReader):
    class Handler(BaseHTTPRequestHandler):
        def do_GET(self) -> None:
            parsed = urlparse(self.path)
            if parsed.path != "/move":
                self.send_json(404, {"error": "not found"})
                return

            query = parse_qs(parsed.query)
            fen_values = query.get("fen", [])
            if not fen_values:
                self.send_json(400, {"error": "missing fen"})
                return

            result = choose_book_move(reader, fen_values[0])
            if "error" in result:
                self.send_json(400, result)
                return
            self.send_json(200, result)

        def log_message(self, format: str, *args) -> None:  # noqa: A003
            return

        def send_json(self, status: int, payload: dict[str, object]) -> None:
            body = json.dumps(payload).encode("utf-8")
            self.send_response(status)
            self.send_header("Content-Type", "application/json; charset=utf-8")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

    return Handler


def main() -> int:
    parser = argparse.ArgumentParser(description="Serve local Polyglot opening book")
    parser.add_argument(
        "--book",
        default="books/komodo.bin",
        help="Path to polyglot .bin opening book (default: books/komodo.bin)",
    )
    parser.add_argument("--host", default="127.0.0.1", help="Host bind (default: 127.0.0.1)")
    parser.add_argument("--port", type=int, default=8765, help="Port (default: 8765)")
    args = parser.parse_args()

    book_path = Path(args.book)
    if not book_path.exists():
        print(f"book file not found: {book_path}")
        return 1

    with chess.polyglot.open_reader(str(book_path)) as reader:
        handler = build_handler(reader)
        server = ThreadingHTTPServer((args.host, args.port), handler)
        print(f"opening book server listening on http://{args.host}:{args.port}")
        print(f"book: {book_path}")
        try:
            server.serve_forever()
        except KeyboardInterrupt:
            pass
        finally:
            server.server_close()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
