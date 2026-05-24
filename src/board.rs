use std::fmt;

pub const CLASSIC_STARTPOS_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CastlingRights {
    pub white_king_side: bool,
    pub white_queen_side: bool,
    pub black_king_side: bool,
    pub black_queen_side: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub promotion: Option<PieceKind>,
    pub is_en_passant: bool,
    pub is_castling: bool,
}

impl Move {
    pub fn to_uci(self) -> String {
        let mut out = format!("{}{}", square_to_coord(self.from), square_to_coord(self.to));
        if let Some(promo) = self.promotion {
            out.push(match promo {
                PieceKind::Knight => 'n',
                PieceKind::Bishop => 'b',
                PieceKind::Rook => 'r',
                PieceKind::Queen => 'q',
                PieceKind::Pawn | PieceKind::King => 'q',
            });
        }
        out
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_uci())
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    // Flat 8x8 board: index = rank * 8 + file, with a1 = 0 and h8 = 63.
    pub squares: [Option<Piece>; 64],
    pub side_to_move: Color,
    pub castling: CastlingRights,
    pub en_passant: Option<u8>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

impl Default for Board {
    fn default() -> Self {
        Self::from_fen(CLASSIC_STARTPOS_FEN).expect("valid starting FEN")
    }
}

impl Board {
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 6 {
            return Err("FEN must have 6 fields".to_string());
        }

        let mut squares = [None; 64];
        let ranks: Vec<&str> = parts[0].split('/').collect();
        if ranks.len() != 8 {
            return Err("FEN board must have 8 ranks".to_string());
        }

        // FEN ranks are listed from rank 8 to rank 1; map them to our a1-based indexing.
        for (fen_rank, rank_str) in ranks.iter().enumerate() {
            let board_rank = 7_u8.saturating_sub(fen_rank as u8);
            let mut file = 0_u8;
            for c in rank_str.chars() {
                if c.is_ascii_digit() {
                    file = file.saturating_add(c.to_digit(10).unwrap_or(0) as u8);
                } else {
                    let piece =
                        piece_from_fen_char(c).ok_or_else(|| format!("invalid piece: {c}"))?;
                    if file >= 8 {
                        return Err("FEN rank overflows 8 files".to_string());
                    }
                    let sq = board_rank * 8 + file;
                    squares[sq as usize] = Some(piece);
                    file += 1;
                }
            }
            if file != 8 {
                return Err("FEN rank does not contain 8 files".to_string());
            }
        }

        let side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("invalid side-to-move in FEN".to_string()),
        };

        let mut castling = CastlingRights::default();
        if parts[2] != "-" {
            for c in parts[2].chars() {
                match c {
                    'K' => castling.white_king_side = true,
                    'Q' => castling.white_queen_side = true,
                    'k' => castling.black_king_side = true,
                    'q' => castling.black_queen_side = true,
                    _ => return Err("invalid castling rights".to_string()),
                }
            }
        }

        let en_passant = if parts[3] == "-" {
            None
        } else {
            Some(coord_to_square(parts[3]).ok_or_else(|| "invalid en-passant square".to_string())?)
        };

        let halfmove_clock = parts[4]
            .parse::<u32>()
            .map_err(|_| "invalid halfmove clock".to_string())?;
        let fullmove_number = parts[5]
            .parse::<u32>()
            .map_err(|_| "invalid fullmove number".to_string())?;

        Ok(Self {
            squares,
            side_to_move,
            castling,
            en_passant,
            halfmove_clock,
            fullmove_number,
        })
    }

    pub fn piece_at(&self, sq: u8) -> Option<Piece> {
        self.squares[sq as usize]
    }

    pub fn king_square(&self, color: Color) -> Option<u8> {
        self.squares
            .iter()
            .enumerate()
            .find(|(_, piece)| matches!(piece, Some(p) if p.color == color && p.kind == PieceKind::King))
            .map(|(idx, _)| idx as u8)
    }

    pub fn apply_move(&self, mv: Move) -> Option<Self> {
        let mut next = self.clone();
        let moving_piece = next.piece_at(mv.from)?;
        if moving_piece.color != self.side_to_move {
            return None;
        }

        let mut captured_piece = next.piece_at(mv.to);
        next.squares[mv.from as usize] = None;

        // En-passant captures a pawn on the adjacent file, not on the destination square.
        if mv.is_en_passant {
            let capture_sq = if moving_piece.color == Color::White {
                mv.to.checked_sub(8)?
            } else {
                mv.to.checked_add(8)?
            };
            captured_piece = next.piece_at(capture_sq);
            next.squares[capture_sq as usize] = None;
        }

        let placed_piece = if let Some(promo) = mv.promotion {
            Piece {
                color: moving_piece.color,
                kind: promo,
            }
        } else {
            moving_piece
        };
        next.squares[mv.to as usize] = Some(placed_piece);

        // Castling is encoded as king move; this block repositions the rook.
        if mv.is_castling && moving_piece.kind == PieceKind::King {
            match (moving_piece.color, mv.to) {
                (Color::White, 6) => {
                    next.squares[7] = None;
                    next.squares[5] = Some(Piece {
                        color: Color::White,
                        kind: PieceKind::Rook,
                    });
                }
                (Color::White, 2) => {
                    next.squares[0] = None;
                    next.squares[3] = Some(Piece {
                        color: Color::White,
                        kind: PieceKind::Rook,
                    });
                }
                (Color::Black, 62) => {
                    next.squares[63] = None;
                    next.squares[61] = Some(Piece {
                        color: Color::Black,
                        kind: PieceKind::Rook,
                    });
                }
                (Color::Black, 58) => {
                    next.squares[56] = None;
                    next.squares[59] = Some(Piece {
                        color: Color::Black,
                        kind: PieceKind::Rook,
                    });
                }
                _ => return None,
            }
        }

        // Moving king/rook (or capturing rook) can permanently remove castling rights.
        match moving_piece.kind {
            PieceKind::King => match moving_piece.color {
                Color::White => {
                    next.castling.white_king_side = false;
                    next.castling.white_queen_side = false;
                }
                Color::Black => {
                    next.castling.black_king_side = false;
                    next.castling.black_queen_side = false;
                }
            },
            PieceKind::Rook => match mv.from {
                0 => next.castling.white_queen_side = false,
                7 => next.castling.white_king_side = false,
                56 => next.castling.black_queen_side = false,
                63 => next.castling.black_king_side = false,
                _ => {}
            },
            _ => {}
        }

        if let Some(captured) = captured_piece {
            if captured.kind == PieceKind::Rook {
                match mv.to {
                    0 => next.castling.white_queen_side = false,
                    7 => next.castling.white_king_side = false,
                    56 => next.castling.black_queen_side = false,
                    63 => next.castling.black_king_side = false,
                    _ => {}
                }
            }
        }

        next.en_passant = None;
        if moving_piece.kind == PieceKind::Pawn {
            let delta = mv.to.abs_diff(mv.from);
            if delta == 16 {
                // Store the square "passed over" so opponent can capture en-passant next ply.
                next.en_passant = Some(if moving_piece.color == Color::White {
                    mv.from + 8
                } else {
                    mv.from - 8
                });
            }
        }

        if moving_piece.kind == PieceKind::Pawn || captured_piece.is_some() {
            next.halfmove_clock = 0;
        } else {
            next.halfmove_clock += 1;
        }

        if self.side_to_move == Color::Black {
            next.fullmove_number += 1;
        }
        next.side_to_move = self.side_to_move.opposite();
        Some(next)
    }
}

pub fn square_to_coord(square: u8) -> String {
    let file = (square % 8) as usize;
    let rank = (square / 8) as usize;
    let file_char = (b'a' + file as u8) as char;
    let rank_char = (b'1' + rank as u8) as char;
    format!("{file_char}{rank_char}")
}

pub fn coord_to_square(coord: &str) -> Option<u8> {
    let bytes = coord.as_bytes();
    if bytes.len() != 2 {
        return None;
    }
    let file = bytes[0];
    let rank = bytes[1];
    if !(b'a'..=b'h').contains(&file) || !(b'1'..=b'8').contains(&rank) {
        return None;
    }
    let f = file - b'a';
    let r = rank - b'1';
    Some(r * 8 + f)
}

fn piece_from_fen_char(c: char) -> Option<Piece> {
    let color = if c.is_ascii_uppercase() {
        Color::White
    } else {
        Color::Black
    };
    let kind = match c.to_ascii_lowercase() {
        'p' => PieceKind::Pawn,
        'n' => PieceKind::Knight,
        'b' => PieceKind::Bishop,
        'r' => PieceKind::Rook,
        'q' => PieceKind::Queen,
        'k' => PieceKind::King,
        _ => return None,
    };
    Some(Piece { color, kind })
}
