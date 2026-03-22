use crate::board::{Board, Color, Move, Piece, PieceKind};

const KNIGHT_DELTAS: [(i8, i8); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
];

const KING_DELTAS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const BISHOP_DIRS: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
const ROOK_DIRS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn generate_legal_moves(board: &Board) -> Vec<Move> {
    let pseudo = generate_pseudo_legal_moves(board);
    let mut legal = Vec::with_capacity(pseudo.len());
    for mv in pseudo {
        if let Some(next) = board.apply_move(mv) {
            if !in_check(&next, board.side_to_move) {
                legal.push(mv);
            }
        }
    }
    legal
}

pub fn in_check(board: &Board, color: Color) -> bool {
    let Some(king_sq) = board.king_square(color) else {
        return false;
    };
    is_square_attacked(board, king_sq, color.opposite())
}

pub fn is_square_attacked(board: &Board, sq: u8, by_color: Color) -> bool {
    let (file, rank) = to_coord(sq);

    let pawn_dirs: [(i8, i8); 2] = match by_color {
        Color::White => [(-1, -1), (1, -1)],
        Color::Black => [(-1, 1), (1, 1)],
    };
    for (df, dr) in pawn_dirs {
        if let Some(from) = from_coord(file + df, rank + dr) {
            if matches!(board.piece_at(from), Some(Piece { color, kind: PieceKind::Pawn }) if color == by_color)
            {
                return true;
            }
        }
    }

    for (df, dr) in KNIGHT_DELTAS {
        if let Some(from) = from_coord(file + df, rank + dr) {
            if matches!(board.piece_at(from), Some(Piece { color, kind: PieceKind::Knight }) if color == by_color)
            {
                return true;
            }
        }
    }

    for (df, dr) in KING_DELTAS {
        if let Some(from) = from_coord(file + df, rank + dr) {
            if matches!(board.piece_at(from), Some(Piece { color, kind: PieceKind::King }) if color == by_color)
            {
                return true;
            }
        }
    }

    for (df, dr) in BISHOP_DIRS {
        if ray_attacked(
            board,
            file,
            rank,
            df,
            dr,
            by_color,
            &[PieceKind::Bishop, PieceKind::Queen],
        ) {
            return true;
        }
    }
    for (df, dr) in ROOK_DIRS {
        if ray_attacked(
            board,
            file,
            rank,
            df,
            dr,
            by_color,
            &[PieceKind::Rook, PieceKind::Queen],
        ) {
            return true;
        }
    }

    false
}

pub fn perft(board: &Board, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }
    let moves = generate_legal_moves(board);
    if depth == 1 {
        return moves.len() as u64;
    }
    let mut nodes = 0_u64;
    for mv in moves {
        if let Some(next) = board.apply_move(mv) {
            nodes += perft(&next, depth - 1);
        }
    }
    nodes
}

fn generate_pseudo_legal_moves(board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();

    for sq in 0_u8..64 {
        let Some(piece) = board.piece_at(sq) else {
            continue;
        };
        if piece.color != board.side_to_move {
            continue;
        }
        match piece.kind {
            PieceKind::Pawn => generate_pawn_moves(board, sq, piece.color, &mut moves),
            PieceKind::Knight => {
                generate_jumper_moves(board, sq, piece.color, &KNIGHT_DELTAS, &mut moves)
            }
            PieceKind::Bishop => {
                generate_slider_moves(board, sq, piece.color, &BISHOP_DIRS, &mut moves)
            }
            PieceKind::Rook => {
                generate_slider_moves(board, sq, piece.color, &ROOK_DIRS, &mut moves)
            }
            PieceKind::Queen => {
                generate_slider_moves(board, sq, piece.color, &BISHOP_DIRS, &mut moves);
                generate_slider_moves(board, sq, piece.color, &ROOK_DIRS, &mut moves);
            }
            PieceKind::King => generate_king_moves(board, sq, piece.color, &mut moves),
        }
    }

    moves
}

fn generate_pawn_moves(board: &Board, from: u8, color: Color, out: &mut Vec<Move>) {
    let (file, rank) = to_coord(from);
    let (step, start_rank, promo_rank) = match color {
        Color::White => (1_i8, 1_i8, 6_i8),
        Color::Black => (-1_i8, 6_i8, 1_i8),
    };

    if let Some(one_step) = from_coord(file, rank + step) {
        if board.piece_at(one_step).is_none() {
            if rank == promo_rank {
                for promotion in [
                    PieceKind::Queen,
                    PieceKind::Rook,
                    PieceKind::Bishop,
                    PieceKind::Knight,
                ] {
                    out.push(Move {
                        from,
                        to: one_step,
                        promotion: Some(promotion),
                        is_en_passant: false,
                        is_castling: false,
                    });
                }
            } else {
                out.push(Move {
                    from,
                    to: one_step,
                    promotion: None,
                    is_en_passant: false,
                    is_castling: false,
                });
                if rank == start_rank {
                    if let Some(two_step) = from_coord(file, rank + (2 * step)) {
                        if board.piece_at(two_step).is_none() {
                            out.push(Move {
                                from,
                                to: two_step,
                                promotion: None,
                                is_en_passant: false,
                                is_castling: false,
                            });
                        }
                    }
                }
            }
        }
    }

    for df in [-1_i8, 1_i8] {
        if let Some(target) = from_coord(file + df, rank + step) {
            if let Some(captured) = board.piece_at(target) {
                if captured.color != color {
                    if rank == promo_rank {
                        for promotion in [
                            PieceKind::Queen,
                            PieceKind::Rook,
                            PieceKind::Bishop,
                            PieceKind::Knight,
                        ] {
                            out.push(Move {
                                from,
                                to: target,
                                promotion: Some(promotion),
                                is_en_passant: false,
                                is_castling: false,
                            });
                        }
                    } else {
                        out.push(Move {
                            from,
                            to: target,
                            promotion: None,
                            is_en_passant: false,
                            is_castling: false,
                        });
                    }
                }
            } else if Some(target) == board.en_passant {
                out.push(Move {
                    from,
                    to: target,
                    promotion: None,
                    is_en_passant: true,
                    is_castling: false,
                });
            }
        }
    }
}

fn generate_jumper_moves(
    board: &Board,
    from: u8,
    color: Color,
    deltas: &[(i8, i8)],
    out: &mut Vec<Move>,
) {
    let (file, rank) = to_coord(from);
    for &(df, dr) in deltas {
        if let Some(to) = from_coord(file + df, rank + dr) {
            if !matches!(board.piece_at(to), Some(Piece { color: c, .. }) if c == color) {
                out.push(Move {
                    from,
                    to,
                    promotion: None,
                    is_en_passant: false,
                    is_castling: false,
                });
            }
        }
    }
}

fn generate_slider_moves(
    board: &Board,
    from: u8,
    color: Color,
    dirs: &[(i8, i8)],
    out: &mut Vec<Move>,
) {
    let (file, rank) = to_coord(from);
    for &(df, dr) in dirs {
        let mut f = file + df;
        let mut r = rank + dr;
        while let Some(to) = from_coord(f, r) {
            if let Some(piece) = board.piece_at(to) {
                if piece.color != color {
                    out.push(Move {
                        from,
                        to,
                        promotion: None,
                        is_en_passant: false,
                        is_castling: false,
                    });
                }
                break;
            }
            out.push(Move {
                from,
                to,
                promotion: None,
                is_en_passant: false,
                is_castling: false,
            });
            f += df;
            r += dr;
        }
    }
}

fn generate_king_moves(board: &Board, from: u8, color: Color, out: &mut Vec<Move>) {
    generate_jumper_moves(board, from, color, &KING_DELTAS, out);

    if in_check(board, color) {
        return;
    }

    match color {
        Color::White => {
            if board.castling.white_king_side
                && board.piece_at(5).is_none()
                && board.piece_at(6).is_none()
                && !is_square_attacked(board, 5, Color::Black)
                && !is_square_attacked(board, 6, Color::Black)
            {
                out.push(Move {
                    from: 4,
                    to: 6,
                    promotion: None,
                    is_en_passant: false,
                    is_castling: true,
                });
            }
            if board.castling.white_queen_side
                && board.piece_at(1).is_none()
                && board.piece_at(2).is_none()
                && board.piece_at(3).is_none()
                && !is_square_attacked(board, 3, Color::Black)
                && !is_square_attacked(board, 2, Color::Black)
            {
                out.push(Move {
                    from: 4,
                    to: 2,
                    promotion: None,
                    is_en_passant: false,
                    is_castling: true,
                });
            }
        }
        Color::Black => {
            if board.castling.black_king_side
                && board.piece_at(61).is_none()
                && board.piece_at(62).is_none()
                && !is_square_attacked(board, 61, Color::White)
                && !is_square_attacked(board, 62, Color::White)
            {
                out.push(Move {
                    from: 60,
                    to: 62,
                    promotion: None,
                    is_en_passant: false,
                    is_castling: true,
                });
            }
            if board.castling.black_queen_side
                && board.piece_at(57).is_none()
                && board.piece_at(58).is_none()
                && board.piece_at(59).is_none()
                && !is_square_attacked(board, 59, Color::White)
                && !is_square_attacked(board, 58, Color::White)
            {
                out.push(Move {
                    from: 60,
                    to: 58,
                    promotion: None,
                    is_en_passant: false,
                    is_castling: true,
                });
            }
        }
    }
}

fn ray_attacked(
    board: &Board,
    file: i8,
    rank: i8,
    df: i8,
    dr: i8,
    by_color: Color,
    valid_kinds: &[PieceKind],
) -> bool {
    let mut f = file + df;
    let mut r = rank + dr;
    while let Some(sq) = from_coord(f, r) {
        if let Some(piece) = board.piece_at(sq) {
            return piece.color == by_color && valid_kinds.contains(&piece.kind);
        }
        f += df;
        r += dr;
    }
    false
}

fn to_coord(square: u8) -> (i8, i8) {
    ((square % 8) as i8, (square / 8) as i8)
}

fn from_coord(file: i8, rank: i8) -> Option<u8> {
    if (0..8).contains(&file) && (0..8).contains(&rank) {
        Some((rank * 8 + file) as u8)
    } else {
        None
    }
}
