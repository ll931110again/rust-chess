use rustchess::board::{Board, Color, Move, Piece, PieceKind, coord_to_square, square_to_coord};

fn mv(from: &str, to: &str) -> Move {
    Move {
        from: coord_to_square(from).expect("valid from square"),
        to: coord_to_square(to).expect("valid to square"),
        promotion: None,
        is_en_passant: false,
        is_castling: false,
    }
}

#[test]
fn coord_square_roundtrip_for_corners() {
    for coord in ["a1", "h1", "a8", "h8"] {
        let sq = coord_to_square(coord).expect("coord should parse");
        assert_eq!(square_to_coord(sq), coord);
    }
}

#[test]
fn parse_startpos_fen_has_expected_state() {
    let board = Board::default();
    assert_eq!(board.side_to_move, Color::White);
    assert!(board.castling.white_king_side);
    assert!(board.castling.white_queen_side);
    assert!(board.castling.black_king_side);
    assert!(board.castling.black_queen_side);

    let e1 = coord_to_square("e1").unwrap_or(0);
    let d8 = coord_to_square("d8").unwrap_or(0);
    assert_eq!(
        board.piece_at(e1),
        Some(Piece {
            color: Color::White,
            kind: PieceKind::King
        })
    );
    assert_eq!(
        board.piece_at(d8),
        Some(Piece {
            color: Color::Black,
            kind: PieceKind::Queen
        })
    );
}

#[test]
fn pawn_double_push_sets_en_passant_square() {
    let board = Board::default();
    let next = board.apply_move(mv("e2", "e4")).expect("move should apply");

    assert_eq!(next.side_to_move, Color::Black);
    assert_eq!(next.en_passant, coord_to_square("e3"));
    assert_eq!(next.halfmove_clock, 0);
}

#[test]
fn en_passant_capture_removes_captured_pawn() {
    let board = Board::from_fen("8/8/8/3pP3/8/8/8/4K2k w - d6 0 1").expect("valid FEN");
    let mut ep = mv("e5", "d6");
    ep.is_en_passant = true;

    let next = board.apply_move(ep).expect("en-passant move should apply");
    let d5 = coord_to_square("d5").unwrap_or(0);
    let d6 = coord_to_square("d6").unwrap_or(0);

    assert_eq!(next.piece_at(d5), None);
    assert_eq!(
        next.piece_at(d6),
        Some(Piece {
            color: Color::White,
            kind: PieceKind::Pawn
        })
    );
}

#[test]
fn castling_moves_rook_and_clears_white_castling_rights() {
    let board = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").expect("valid FEN");
    let mut castle = mv("e1", "g1");
    castle.is_castling = true;

    let next = board.apply_move(castle).expect("castling should apply");
    let f1 = coord_to_square("f1").unwrap_or(0);
    let h1 = coord_to_square("h1").unwrap_or(0);

    assert_eq!(
        next.piece_at(f1),
        Some(Piece {
            color: Color::White,
            kind: PieceKind::Rook
        })
    );
    assert_eq!(next.piece_at(h1), None);
    assert!(!next.castling.white_king_side);
    assert!(!next.castling.white_queen_side);
    assert!(next.castling.black_king_side);
    assert!(next.castling.black_queen_side);
}
