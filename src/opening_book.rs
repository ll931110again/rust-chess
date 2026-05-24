use std::sync::LazyLock;
use std::time::Duration;

use reqwest::blocking::Client;
use serde::Deserialize;

use crate::board::{Board, Color, Move, Piece, PieceKind, square_to_coord};
use crate::movegen::generate_legal_moves;

const DEFAULT_LOCAL_BOOK_URL: &str = "http://127.0.0.1:8765/move";

static BOOK_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .timeout(Duration::from_millis(500))
        .build()
        .expect("opening book HTTP client")
});

#[derive(Debug, Deserialize)]
struct LocalBookResponse {
    uci: Option<String>,
}

/// Return a legal opening move from locally served opening book data.
///
/// The engine expects a local HTTP service (default: http://127.0.0.1:8765/move)
/// that accepts `?fen=<FEN>` and returns JSON `{ "uci": "e2e4" }` or `{ "uci": null }`.
/// If the local service is unavailable, this returns `None` and search is used instead.
pub fn pick_book_move(board: &Board) -> Option<Move> {
    let base_url = std::env::var("RUSTCHESS_BOOK_URL").unwrap_or_else(|_| DEFAULT_LOCAL_BOOK_URL.to_string());
    let fen = board_to_fen(board);
    let mut url = reqwest::Url::parse(&base_url).ok()?;
    {
        let mut q = url.query_pairs_mut();
        q.append_pair("fen", &fen);
    }

    let response = BOOK_CLIENT
        .get(url)
        .send()
        .ok()?
        .error_for_status()
        .ok()?
        .json::<LocalBookResponse>()
        .ok()?;

    let best_uci = response.uci?;
    let legal = generate_legal_moves(board);
    legal.into_iter().find(|m| m.to_uci() == best_uci)
}

fn board_to_fen(board: &Board) -> String {
    let mut board_part = String::new();
    for rank in (0..8).rev() {
        let mut empty = 0_u8;
        for file in 0..8 {
            let sq = (rank * 8 + file) as u8;
            if let Some(piece) = board.piece_at(sq) {
                if empty > 0 {
                    board_part.push(char::from(b'0' + empty));
                    empty = 0;
                }
                board_part.push(piece_char(piece));
            } else {
                empty += 1;
            }
        }
        if empty > 0 {
            board_part.push(char::from(b'0' + empty));
        }
        if rank > 0 {
            board_part.push('/');
        }
    }

    let side = match board.side_to_move {
        Color::White => "w",
        Color::Black => "b",
    };

    let mut castling = String::new();
    if board.castling.white_king_side {
        castling.push('K');
    }
    if board.castling.white_queen_side {
        castling.push('Q');
    }
    if board.castling.black_king_side {
        castling.push('k');
    }
    if board.castling.black_queen_side {
        castling.push('q');
    }
    if castling.is_empty() {
        castling.push('-');
    }

    let ep = board
        .en_passant
        .map(square_to_coord)
        .unwrap_or_else(|| "-".to_string());

    format!(
        "{} {} {} {} {} {}",
        board_part, side, castling, ep, board.halfmove_clock, board.fullmove_number
    )
}

fn piece_char(piece: Piece) -> char {
    let base = match piece.kind {
        PieceKind::Pawn => 'p',
        PieceKind::Knight => 'n',
        PieceKind::Bishop => 'b',
        PieceKind::Rook => 'r',
        PieceKind::Queen => 'q',
        PieceKind::King => 'k',
    };
    if piece.color == Color::White {
        base.to_ascii_uppercase()
    } else {
        base
    }
}

#[cfg(test)]
mod tests {
    use super::board_to_fen;
    use crate::board::Board;

    #[test]
    fn startpos_fen_serialization_is_valid() {
        let fen = board_to_fen(&Board::default());
        assert_eq!(
            fen,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }
}
