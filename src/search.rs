use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;
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
    threads: usize,
}

impl Searcher {
    pub fn new() -> Self {
        Self {
            threads: thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1),
        }
    }

    pub fn threads(&self) -> usize {
        self.threads
    }

    pub fn set_threads(&mut self, threads: usize) {
        self.threads = threads.max(1);
    }

    pub fn iterative_deepening(&mut self, board: &Board, limits: SearchLimits) -> SearchResult {
        let start = Instant::now();
        let ctx = SearchCtx {
            stop_at: compute_stop_time(board.side_to_move, limits).map(|d| start + d),
            aborted: AtomicBool::new(false),
            nodes: AtomicU64::new(0),
        };

        let max_depth = limits.depth.unwrap_or(6);
        let mut best_move = None;
        let mut best_score = 0;
        let mut last_completed_depth = 0;

        // Iterative deepening: progressively search deeper and keep last complete result.
        for depth in 1..=max_depth {
            let mut root_moves = generate_legal_moves(board);
            order_moves(board, &mut root_moves);

            if root_moves.is_empty() {
                break;
            }

            if !ctx.aborted.load(Ordering::Relaxed) {
                if let Some((mv, score)) =
                    self.search_root_parallel(board, &root_moves, depth, &ctx)
                {
                    best_move = Some(mv);
                    best_score = score;
                    last_completed_depth = depth;
                    let elapsed = start.elapsed().as_millis();
                    println!(
                        "info depth {} score cp {} nodes {} time {} pv {}",
                        depth,
                        best_score,
                        ctx.nodes.load(Ordering::Relaxed),
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
            nodes: ctx.nodes.load(Ordering::Relaxed),
            elapsed_ms: start.elapsed().as_millis(),
        }
    }

    fn search_root_parallel(
        &self,
        board: &Board,
        root_moves: &[Move],
        depth: u32,
        ctx: &SearchCtx,
    ) -> Option<(Move, i32)> {
        let workers = self.threads.min(root_moves.len()).max(1);
        let next_index = Arc::new(AtomicUsize::new(0));
        let (tx, rx) = mpsc::channel::<(usize, Move, i32)>();

        thread::scope(|scope| {
            for _ in 0..workers {
                let tx = tx.clone();
                let next_index = Arc::clone(&next_index);
                scope.spawn(move || {
                    loop {
                        if ctx.aborted.load(Ordering::Relaxed) {
                            break;
                        }
                        let idx = next_index.fetch_add(1, Ordering::Relaxed);
                        if idx >= root_moves.len() {
                            break;
                        }
                        let mv = root_moves[idx];
                        let Some(next_board) = board.apply_move(mv) else {
                            continue;
                        };
                        let score = -negamax(&next_board, depth - 1, -INF, INF, 1, ctx);
                        let _ = tx.send((idx, mv, score));
                    }
                });
            }
            drop(tx);
        });

        let mut best: Option<(usize, Move, i32)> = None;
        for (idx, mv, score) in rx {
            match best {
                Some((best_idx, _, best_score)) => {
                    // Stable tie-break by move order to keep output deterministic-ish.
                    if score > best_score || (score == best_score && idx < best_idx) {
                        best = Some((idx, mv, score));
                    }
                }
                None => best = Some((idx, mv, score)),
            }
        }
        best.map(|(_, mv, score)| (mv, score))
    }
}

struct SearchCtx {
    stop_at: Option<Instant>,
    aborted: AtomicBool,
    nodes: AtomicU64,
}

fn negamax(board: &Board, depth: u32, mut alpha: i32, beta: i32, ply: i32, ctx: &SearchCtx) -> i32 {
    if ctx.aborted.load(Ordering::Relaxed) {
        return 0;
    }
    if should_stop(ctx.stop_at) {
        ctx.aborted.store(true, Ordering::Relaxed);
        return 0;
    }

    ctx.nodes.fetch_add(1, Ordering::Relaxed);

    // Leaf evaluation (no quiescence yet).
    if depth == 0 {
        return evaluate(board);
    }

    let mut moves = generate_legal_moves(board);
    if moves.is_empty() {
        // No legal moves means checkmate or stalemate.
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
        let score = -negamax(&next, depth - 1, -beta, -alpha, ply + 1, ctx);
        if ctx.aborted.load(Ordering::Relaxed) {
            return 0;
        }
        if score > best {
            best = score;
        }
        if score > alpha {
            alpha = score;
        }
        // Alpha-beta cutoff: no need to search siblings that opponent avoids.
        if alpha >= beta {
            break;
        }
    }
    best
}

fn should_stop(stop_at: Option<Instant>) -> bool {
    stop_at.is_some_and(|deadline| Instant::now() >= deadline)
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
    // Basic time policy: allocate roughly 1/N of remaining clock plus half increment.
    let moves_left = limits.movestogo.unwrap_or(30).max(1) as u64;
    let budget = (base_time / moves_left) + (inc / 2);
    Some(Duration::from_millis(budget.clamp(5, base_time.max(5))))
}

fn order_moves(board: &Board, moves: &mut [Move]) {
    // Simple MVV-like ordering: prioritize captures and promotions.
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
    // Material-only eval from side-to-move perspective.
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
