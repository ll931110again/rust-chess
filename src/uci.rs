use std::io::{self, BufRead, Write};

use crate::board::{Board, CLASSIC_STARTPOS_FEN};
use crate::movegen::{generate_legal_moves, perft};
use crate::opening_book::pick_book_move;
use crate::search::{SearchLimits, Searcher};

pub fn run_uci() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut board = Board::default();
    let mut searcher = Searcher::new();
    let mut own_book_enabled = true;

    // Single-threaded UCI read-eval loop.
    for line in stdin.lock().lines() {
        let Ok(cmd_line) = line else {
            continue;
        };
        let cmd = cmd_line.trim();
        if cmd.is_empty() {
            continue;
        }

        if cmd == "uci" {
            println!("id name rustchess-basic");
            println!("id author Cursor");
            println!(
                "option name Threads type spin default {} min 1 max 256",
                searcher.threads()
            );
            println!("option name OwnBook type check default true");
            println!("uciok");
            continue;
        }

        if cmd == "isready" {
            println!("readyok");
            continue;
        }

        if let Some(rest) = cmd.strip_prefix("setoption ") {
            apply_setoption_command(&mut searcher, &mut own_book_enabled, rest);
            continue;
        }

        if cmd == "ucinewgame" {
            board = Board::default();
            continue;
        }

        if cmd == "quit" {
            break;
        }

        if let Some(rest) = cmd.strip_prefix("position ") {
            if let Err(err) = apply_position_command(&mut board, rest) {
                println!("info string position parse error: {err}");
            }
            continue;
        }

        if let Some(rest) = cmd.strip_prefix("go ") {
            if let Some(depth_str) = rest.strip_prefix("perft ") {
                // Non-standard helper for movegen validation from a GUI/CLI.
                let depth = depth_str.parse::<u32>().unwrap_or(1);
                let nodes = perft(&board, depth);
                println!("info string perft depth {} nodes {}", depth, nodes);
                println!("bestmove 0000");
                continue;
            }

            let limits = parse_go_limits(rest);
            if own_book_enabled {
                if let Some(book_move) = pick_book_move(&board) {
                    println!("info string book move {}", book_move.to_uci());
                    println!("bestmove {}", book_move.to_uci());
                    continue;
                }
            }
            let result = searcher.iterative_deepening(&board, limits);
            println!(
                "info string search done depth {} score_cp {} nodes {} time {}",
                result.depth, result.score_cp, result.nodes, result.elapsed_ms
            );
            if let Some(best) = result.best_move {
                println!("bestmove {}", best.to_uci());
            } else {
                println!("bestmove 0000");
            }
            continue;
        }

        if cmd == "stop" {
            continue;
        }

        if cmd == "d" {
            println!(
                "info string side {:?} halfmove {} fullmove {}",
                board.side_to_move, board.halfmove_clock, board.fullmove_number
            );
            continue;
        }

        println!("info string unknown command: {cmd}");
        let _ = stdout.flush();
    }
}

fn apply_position_command(board: &mut Board, args: &str) -> Result<(), String> {
    let tokens: Vec<&str> = args.split_whitespace().collect();
    if tokens.is_empty() {
        return Err("missing position arguments".to_string());
    }

    let mut idx = 0;
    if tokens[idx] == "startpos" {
        *board = Board::from_fen(CLASSIC_STARTPOS_FEN)?;
        idx += 1;
    } else if tokens[idx] == "fen" {
        idx += 1;
        if idx >= tokens.len() {
            return Err("fen payload missing".to_string());
        }
        let fen_end = tokens
            .iter()
            .enumerate()
            .skip(idx)
            .find_map(|(i, t)| if *t == "moves" { Some(i) } else { None })
            .unwrap_or(tokens.len());
        let fen = tokens[idx..fen_end].join(" ");
        *board = Board::from_fen(&fen)?;
        idx = fen_end;
    } else {
        return Err("position must start with startpos or fen".to_string());
    }

    // Rebuild game state by replaying the provided UCI move list.
    if idx < tokens.len() && tokens[idx] == "moves" {
        idx += 1;
        while idx < tokens.len() {
            let mv_str = tokens[idx];
            let legal = generate_legal_moves(board);
            let Some(mv) = legal.into_iter().find(|m| m.to_uci() == mv_str) else {
                return Err(format!("illegal move in position: {mv_str}"));
            };
            *board = board
                .apply_move(mv)
                .ok_or_else(|| format!("failed to apply move: {mv_str}"))?;
            idx += 1;
        }
    }
    Ok(())
}

fn parse_go_limits(args: &str) -> SearchLimits {
    let mut limits = SearchLimits::default();
    let tokens: Vec<&str> = args.split_whitespace().collect();
    let mut i = 0;
    while i < tokens.len() {
        match tokens[i] {
            "depth" => {
                if let Some(v) = tokens.get(i + 1).and_then(|s| s.parse::<u32>().ok()) {
                    limits.depth = Some(v.max(1));
                }
                i += 2;
            }
            "movetime" => {
                if let Some(v) = tokens.get(i + 1).and_then(|s| s.parse::<u64>().ok()) {
                    limits.movetime_ms = Some(v.max(1));
                }
                i += 2;
            }
            "wtime" => {
                limits.wtime_ms = tokens.get(i + 1).and_then(|s| s.parse::<u64>().ok());
                i += 2;
            }
            "btime" => {
                limits.btime_ms = tokens.get(i + 1).and_then(|s| s.parse::<u64>().ok());
                i += 2;
            }
            "winc" => {
                limits.winc_ms = tokens.get(i + 1).and_then(|s| s.parse::<u64>().ok());
                i += 2;
            }
            "binc" => {
                limits.binc_ms = tokens.get(i + 1).and_then(|s| s.parse::<u64>().ok());
                i += 2;
            }
            "movestogo" => {
                limits.movestogo = tokens.get(i + 1).and_then(|s| s.parse::<u32>().ok());
                i += 2;
            }
            "infinite" => {
                // Current engine has no async stop thread; use a practical fallback depth.
                limits.depth = Some(10);
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    limits
}

fn apply_setoption_command(searcher: &mut Searcher, own_book_enabled: &mut bool, args: &str) {
    let tokens: Vec<&str> = args.split_whitespace().collect();
    if tokens.is_empty() || tokens[0] != "name" {
        return;
    }

    let value_idx = tokens.iter().position(|t| *t == "value");
    let name_tokens = match value_idx {
        Some(v) if v > 1 => &tokens[1..v],
        Some(_) => &[][..],
        None if tokens.len() > 1 => &tokens[1..],
        None => &[][..],
    };
    let value_tokens = value_idx
        .and_then(|v| tokens.get(v + 1..))
        .unwrap_or(&[][..]);

    let name = name_tokens.join(" ");
    if name.eq_ignore_ascii_case("threads") {
        if let Some(raw) = value_tokens.first() {
            if let Ok(v) = raw.parse::<usize>() {
                searcher.set_threads(v);
                println!("info string threads set to {}", searcher.threads());
            }
        }
    } else if name.eq_ignore_ascii_case("ownbook") {
        if let Some(raw) = value_tokens.first() {
            if raw.eq_ignore_ascii_case("true") {
                *own_book_enabled = true;
                println!("info string ownbook enabled");
            } else if raw.eq_ignore_ascii_case("false") {
                *own_book_enabled = false;
                println!("info string ownbook disabled");
            }
        }
    }
}
