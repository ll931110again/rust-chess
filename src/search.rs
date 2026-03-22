use std::time::{Duration, Instant};

use crate::board::{Board, Color, Move, PieceKind};
use crate::movegen::{generate_legal_moves, in_check};

const CHECKMATE_SCORE: i32 = 30_000;
const INF: i32 = 1_000_000;

#[derive(Clone, Copy, Debug, Default)]
pub struct SearchLimits {
    pub depth: Option<u32>,
    pub movetime_ms: Option<u64>,
    pub wtime_ms: Option<u64>,
    pub btime_ms: Option<u64>,
    pub winc_ms: Option<u64>,
    pub binc_ms: Option<u64>,
    pub movestogo: Option<u32>,
}

#[derive(Clone, Copy, Debug)]
pub struct SearchResult {
    pub best_move: Option<Move>,
    pub score_cp: i32,
    pub depth: u32,
    pub nodes: u64,
    pub elapsed_ms: u128,
}

pub struct Searcher {
    start: Instant,
    stop_at: Option<Instant>,
    nodes: u64,
    aborted: bool,
}

impl Searcher {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            stop_at: None,
            nodes: 0,
            aborted: false,
        }
    }

    pub fn iterative_deepening(&mut self, board: &Board, limits: SearchLimits) -> SearchResult {
        self.start = Instant::now();
        self.nodes = 0;
        self.aborted = false;
        self.stop_at = compute_stop_time(board.side_to_move, limits).map(|d| self.start + d);

        let max_depth = limits.depth.unwrap_or(6);
        let mut best_move = None;
        let mut best_score = 0;
        let mut last_completed_depth = 0;

        for depth in 1..=max_depth {
            let mut alpha = -INF;
            let beta = INF;
            let mut depth_best_move = None;
            let mut depth_best_score = -INF;

            let mut root_moves = generate_legal_moves(board);
            order_moves(board, &mut root_moves);

            if root_moves.is_empty() {
                break;
            }

            for mv in root_moves {
                if self.should_stop() {
                    self.aborted = true;
                    break;
                }
                let Some(next) = board.apply_move(mv) else {
                    continue;
                };
                let score = -self.negamax(&next, depth - 1, -beta, -alpha, 1);
                if score > depth_best_score {
                    depth_best_score = score;
                    depth_best_move = Some(mv);
                }
                if score > alpha {
                    alpha = score;
                }
            }

            if !self.aborted {
                if let Some(mv) = depth_best_move {
                    best_move = Some(mv);
                    best_score = depth_best_score;
                    last_completed_depth = depth;
                    let elapsed = self.start.elapsed().as_millis();
                    println!(
                        "info depth {} score cp {} nodes {} time {} pv {}",
                        depth,
                        best_score,
                        self.nodes,
                        elapsed,
                        mv.to_uci()
                    );
                }
            } else {
                break;
            }
        }

        SearchResult {
            best_move,
            score_cp: best_score,
            depth: last_completed_depth,
            nodes: self.nodes,
            elapsed_ms: self.start.elapsed().as_millis(),
        }
    }

    fn negamax(&mut self, board: &Board, depth: u32, mut alpha: i32, beta: i32, ply: i32) -> i32 {
        if self.should_stop() {
            self.aborted = true;
            return 0;
        }

        self.nodes += 1;

        if depth == 0 {
            return evaluate(board);
        }

        let mut moves = generate_legal_moves(board);
        if moves.is_empty() {
            if in_check(board, board.side_to_move) {
                return -(CHECKMATE_SCORE - ply);
            }
            return 0;
        }

        order_moves(board, &mut moves);

        let mut best = -INF;
        for mv in moves {
            let Some(next) = board.apply_move(mv) else {
                continue;
            };
            let score = -self.negamax(&next, depth - 1, -beta, -alpha, ply + 1);
            if self.aborted {
                return 0;
            }
            if score > best {
                best = score;
            }
            if score > alpha {
                alpha = score;
            }
            if alpha >= beta {
                break;
            }
        }
        best
    }

    fn should_stop(&self) -> bool {
        self.stop_at
            .is_some_and(|deadline| Instant::now() >= deadline)
    }
}

fn compute_stop_time(side_to_move: Color, limits: SearchLimits) -> Option<Duration> {
    if let Some(ms) = limits.movetime_ms {
        return Some(Duration::from_millis(ms.max(1)));
    }

    let base_time = match side_to_move {
        Color::White => limits.wtime_ms?,
        Color::Black => limits.btime_ms?,
    };
    let inc = match side_to_move {
        Color::White => limits.winc_ms.unwrap_or(0),
        Color::Black => limits.binc_ms.unwrap_or(0),
    };
    let moves_left = limits.movestogo.unwrap_or(30).max(1) as u64;
    let budget = (base_time / moves_left) + (inc / 2);
    Some(Duration::from_millis(budget.clamp(5, base_time.max(5))))
}

fn order_moves(board: &Board, moves: &mut [Move]) {
    moves.sort_by_key(|mv| {
        let capture_value = board
            .piece_at(mv.to)
            .map(|piece| piece_value(piece.kind))
            .unwrap_or(0);
        let promo_bonus = mv.promotion.map(piece_value).unwrap_or(0);
        -(capture_value + promo_bonus)
    });
}

fn evaluate(board: &Board) -> i32 {
    let mut score = 0;
    for sq in 0_u8..64 {
        if let Some(piece) = board.piece_at(sq) {
            let val = piece_value(piece.kind);
            if piece.color == Color::White {
                score += val;
            } else {
                score -= val;
            }
        }
    }
    if board.side_to_move == Color::White {
        score
    } else {
        -score
    }
}

fn piece_value(kind: PieceKind) -> i32 {
    match kind {
        PieceKind::Pawn => 100,
        PieceKind::Knight => 320,
        PieceKind::Bishop => 330,
        PieceKind::Rook => 500,
        PieceKind::Queen => 900,
        PieceKind::King => 0,
    }
}
