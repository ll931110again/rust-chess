use rustchess::board::Board;
use rustchess::movegen::generate_legal_moves;
use rustchess::opening_book::pick_book_move;

#[test]
fn startpos_book_move_is_legal_when_available() {
    let board = Board::default();
    if let Some(book_move) = pick_book_move(&board) {
        let legal = generate_legal_moves(&board);
        assert!(legal.contains(&book_move), "book move must be legal");
    }
}

#[test]
fn book_lookup_handles_uncommon_position_without_panicking() {
    let mut board = Board::default();
    // Push an uncommon sequence; online explorer may or may not still have data.
    for uci in ["a2a4", "h7h5", "a4a5", "h5h4"] {
        let legal = generate_legal_moves(&board);
        let Some(mv) = legal.into_iter().find(|m| m.to_uci() == uci) else {
            panic!("test setup move {uci} must be legal");
        };
        board = board.apply_move(mv).expect("apply setup move");
    }
    if let Some(book_move) = pick_book_move(&board) {
        let legal = generate_legal_moves(&board);
        assert!(legal.contains(&book_move), "book move must be legal");
    }
}
