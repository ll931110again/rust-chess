use rustchess::board::{Board, Color, Move, coord_to_square};
use rustchess::movegen::{generate_legal_moves, in_check, perft};

fn has_move(moves: &[Move], uci: &str) -> bool {
    moves.iter().any(|m| m.to_uci() == uci)
}

#[test]
fn startpos_has_20_legal_moves() {
    let board = Board::default();
    let moves = generate_legal_moves(&board);
    assert_eq!(moves.len(), 20);
}

#[test]
fn perft_startpos_depths_match_known_values() {
    let board = Board::default();
    assert_eq!(perft(&board, 1), 20);
    assert_eq!(perft(&board, 2), 400);
    assert_eq!(perft(&board, 3), 8_902);
}

#[test]
fn castling_moves_are_generated_when_allowed() {
    let board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").expect("valid FEN");
    let moves = generate_legal_moves(&board);
    assert!(has_move(&moves, "e1g1"));
    assert!(has_move(&moves, "e1c1"));
}

#[test]
fn en_passant_move_is_generated_when_available() {
    let board = Board::from_fen("8/8/8/3pP3/8/8/8/4K2k w - d6 0 1").expect("valid FEN");
    let moves = generate_legal_moves(&board);
    assert!(has_move(&moves, "e5d6"));
}

#[test]
fn in_check_detects_simple_rook_check() {
    let board = Board::from_fen("4k3/8/8/8/8/8/4r3/4K3 w - - 0 1").expect("valid FEN");
    assert!(in_check(&board, Color::White));
    assert!(!in_check(&board, Color::Black));
}

#[test]
fn legal_moves_resolve_check() {
    let board = Board::from_fen("4k3/8/8/8/8/8/4r3/4K3 w - - 0 1").expect("valid FEN");
    let moves = generate_legal_moves(&board);
    assert!(!moves.is_empty());
    for mv in moves {
        let next = board.apply_move(mv).expect("legal move should apply");
        assert!(!in_check(&next, Color::White));
    }
}

#[test]
fn pinned_piece_cannot_move_illegally() {
    let board = Board::from_fen("4k3/8/8/8/8/4r3/4B3/4K3 w - - 0 1").expect("valid FEN");
    let moves = generate_legal_moves(&board);
    let e2 = coord_to_square("e2").unwrap_or(0);
    let bishop_moves: Vec<Move> = moves.into_iter().filter(|m| m.from == e2).collect();
    assert!(bishop_moves.is_empty());
}
