use crate::board::{Board, Color, PieceKind};

const BISHOP_PAIR_BONUS: i32 = 30;
const TEMPO_BONUS: i32 = 10;
const DOUBLED_PAWN_PENALTY: i32 = 12;
const ISOLATED_PAWN_PENALTY: i32 = 10;

const PAWN_PST: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, -20, -20, 10, 10, 5, 5, -5, -10, 0, 0, -10, -5, 5, 0, 0, 0,
    20, 20, 0, 0, 0, 5, 5, 10, 25, 25, 10, 5, 5, 10, 10, 20, 30, 30, 20, 10, 10, 50, 50, 50, 50,
    50, 50, 50, 50, 0, 0, 0, 0, 0, 0, 0, 0,
];
const KNIGHT_PST: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15, 10,
    0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15, 15, 10,
    5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
];
const BISHOP_PST: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5, 0,
    -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10, 10, 10,
    -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
];
const ROOK_PST: [i32; 64] = [
    0, 0, 0, 5, 5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0,
    0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 5, 10, 10, 10, 10, 10, 10, 5, 0, 0,
    0, 0, 0, 0, 0, 0,
];
const QUEEN_PST: [i32; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0, -10,
    -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0, 5, 0, 0,
    0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
];
const KING_PST: [i32; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40,
    -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40, -40, -30,
    -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0,
    10, 30, 20,
];

pub fn evaluate(board: &Board) -> i32 {
    // Classical static evaluation: material + piece-square tables + pawn structure.
    let mut score = 0_i32;
    let mut white_pawns_per_file = [0_u8; 8];
    let mut black_pawns_per_file = [0_u8; 8];
    let mut white_bishops = 0_u8;
    let mut black_bishops = 0_u8;

    for sq in 0_u8..64 {
        if let Some(piece) = board.piece_at(sq) {
            let mut val = piece_value(piece.kind) + piece_square_value(piece.kind, piece.color, sq);
            if piece.kind == PieceKind::Pawn {
                let file = file_of(sq);
                let rank_adv = pawn_advancement(piece.color, sq);
                val += rank_adv * 3;
                match piece.color {
                    Color::White => white_pawns_per_file[file] += 1,
                    Color::Black => black_pawns_per_file[file] += 1,
                }
                if is_passed_pawn(board, piece.color, sq) {
                    val += 20 + rank_adv * 6;
                }
            } else if piece.kind == PieceKind::Bishop {
                if piece.color == Color::White {
                    white_bishops += 1;
                } else {
                    black_bishops += 1;
                }
            }

            if piece.color == Color::White {
                score += val;
            } else {
                score -= val;
            }
        }
    }

    score += pawn_structure_score(&white_pawns_per_file, &black_pawns_per_file);
    score -= pawn_structure_score(&black_pawns_per_file, &white_pawns_per_file);

    if white_bishops >= 2 {
        score += BISHOP_PAIR_BONUS;
    }
    if black_bishops >= 2 {
        score -= BISHOP_PAIR_BONUS;
    }

    // Small initiative bonus for side to move.
    if board.side_to_move == Color::White {
        score += TEMPO_BONUS;
    } else {
        score -= TEMPO_BONUS;
    }

    if board.side_to_move == Color::White {
        score
    } else {
        -score
    }
}

pub fn piece_value(kind: PieceKind) -> i32 {
    match kind {
        PieceKind::Pawn => 100,
        PieceKind::Knight => 320,
        PieceKind::Bishop => 330,
        PieceKind::Rook => 500,
        PieceKind::Queen => 900,
        PieceKind::King => 0,
    }
}

fn piece_square_value(kind: PieceKind, color: Color, sq: u8) -> i32 {
    let idx = match color {
        Color::White => sq as usize,
        Color::Black => mirror_square(sq),
    };
    match kind {
        PieceKind::Pawn => PAWN_PST[idx],
        PieceKind::Knight => KNIGHT_PST[idx],
        PieceKind::Bishop => BISHOP_PST[idx],
        PieceKind::Rook => ROOK_PST[idx],
        PieceKind::Queen => QUEEN_PST[idx],
        PieceKind::King => KING_PST[idx],
    }
}

fn pawn_structure_score(own: &[u8; 8], opp: &[u8; 8]) -> i32 {
    let mut score = 0_i32;
    for file in 0..8 {
        let count = own[file] as i32;
        if count > 1 {
            score -= (count - 1) * DOUBLED_PAWN_PENALTY;
        }
        if count > 0 {
            let left = if file > 0 { own[file - 1] } else { 0 };
            let right = if file < 7 { own[file + 1] } else { 0 };
            if left == 0 && right == 0 {
                score -= count * ISOLATED_PAWN_PENALTY;
            }
            if opp[file] == 0 {
                score += 4 * count;
            }
        }
    }

    // Slightly reward healthier pawn masses in center files.
    score += ((own[3] + own[4]) as i32) * 2;
    score
}

fn is_passed_pawn(board: &Board, color: Color, sq: u8) -> bool {
    let file = file_of(sq) as i32;
    let rank = rank_of(sq) as i32;
    let enemy = color.opposite();

    let mut f = file - 1;
    while f <= file + 1 {
        if (0..8).contains(&f) {
            let mut r = if color == Color::White {
                rank + 1
            } else {
                rank - 1
            };
            while (0..8).contains(&r) {
                let idx = (r * 8 + f) as u8;
                if let Some(piece) = board.piece_at(idx) {
                    if piece.color == enemy && piece.kind == PieceKind::Pawn {
                        return false;
                    }
                }
                if color == Color::White {
                    r += 1;
                } else {
                    r -= 1;
                }
            }
        }
        f += 1;
    }
    true
}

fn mirror_square(sq: u8) -> usize {
    let file = sq % 8;
    let rank = sq / 8;
    ((7 - rank) * 8 + file) as usize
}

fn file_of(sq: u8) -> usize {
    (sq % 8) as usize
}

fn rank_of(sq: u8) -> usize {
    (sq / 8) as usize
}

fn pawn_advancement(color: Color, sq: u8) -> i32 {
    let rank = rank_of(sq) as i32;
    match color {
        Color::White => rank,
        Color::Black => 7 - rank,
    }
}
