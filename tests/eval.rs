use rustchess::board::Board;
use rustchess::eval::evaluate;
use rustchess::search::{SearchLimits, Searcher};

#[test]
fn material_advantage_is_reflected_in_score() {
    let white_to_move = Board::from_fen("4k3/8/8/8/8/8/8/4KQ2 w - - 0 1").expect("valid FEN");
    let black_to_move = Board::from_fen("4k3/8/8/8/8/8/8/4KQ2 b - - 0 1").expect("valid FEN");

    assert!(evaluate(&white_to_move) > 400);
    assert!(evaluate(&black_to_move) < -400);
}

#[test]
fn passed_pawn_scores_higher_than_blocked_pawn() {
    let passed = Board::from_fen("4k3/p7/8/4P3/8/8/8/4K3 w - - 0 1").expect("valid FEN");
    let blocked = Board::from_fen("4k3/8/4p3/4P3/8/8/8/4K3 w - - 0 1").expect("valid FEN");

    assert!(evaluate(&passed) > evaluate(&blocked));
}

#[test]
fn bishop_pair_gets_bonus() {
    let bishop_pair = Board::from_fen("4k3/8/8/8/8/8/8/2B1KB2 w - - 0 1").expect("valid FEN");
    let bishop_and_knight = Board::from_fen("4k3/8/8/8/8/8/8/2B1KN2 w - - 0 1").expect("valid FEN");

    assert!(evaluate(&bishop_pair) > evaluate(&bishop_and_knight));
}

#[test]
fn start_position_is_near_equal() {
    let board = Board::default();
    let score = evaluate(&board);
    assert!(
        score.abs() <= 20,
        "expected near-equal start position, got score={score}"
    );
}

#[test]
fn white_extra_queen_scores_positive() {
    let board = Board::from_fen("4k3/8/8/8/8/8/8/4KQ2 w - - 0 1").expect("valid FEN");
    assert!(evaluate(&board) > 800);
}

#[test]
fn black_extra_queen_scores_negative_for_white_to_move() {
    let board = Board::from_fen("4kq2/8/8/8/8/8/8/4K3 w - - 0 1").expect("valid FEN");
    assert!(evaluate(&board) < -800);
}

#[test]
fn checkmate_position_returns_no_legal_move_bestmove() {
    // Black to move and checkmated by Qf7 + Kg6.
    let board = Board::from_fen("7k/5Q2/6K1/8/8/8/8/8 b - - 0 1").expect("valid FEN");
    let mut searcher = Searcher::new();
    let result = searcher.iterative_deepening(
        &board,
        SearchLimits {
            depth: Some(2),
            ..SearchLimits::default()
        },
    );
    assert!(result.best_move.is_none());
}
