use crate::{play::Game, play::MoveEval, predicted_eval::get_truncated_eval};
use cozy_chess::{Board, Color, GameStatus, Move};
use std::{cmp::max, cmp::min, collections::HashMap, sync::atomic::Ordering};

/// Returns a legal move that places the player to move in the worst position
/// Additionally returns the evaluation after said move
pub fn get_move(game: Game) -> Game {
    let older_binding = game.current_best_move.clone().unwrap();
    let mut binding = older_binding.lock().unwrap();
    let mut move_list = get_move_depth_1(game.clone());
    let side_to_move = game.board.side_to_move();
    let tmp_best_move = match side_to_move {
        Color::White => move_list.first().unwrap().clone(),
        Color::Black => move_list.last().unwrap().clone(),
    };

    *binding = Option::Some(tmp_best_move);
    drop(binding);
    for depth in 1..=3 {
        if game.stop_search.load(Ordering::SeqCst) {
            break;
        }
        move_list = get_move_iterative(game.clone(), depth, move_list);
        let mut binding = older_binding.lock().unwrap();
        let side_to_move = game.board.side_to_move();
        let tmp_best_move = match side_to_move {
            Color::White => move_list.first().unwrap().clone(),
            Color::Black => move_list.last().unwrap().clone(),
        };
        *binding = Option::Some(tmp_best_move);
        drop(binding);
    }
    game
}

fn get_move_iterative(game: Game, depth: i32, mut move_list: Vec<MoveEval>) -> Vec<MoveEval> {
    let side_to_move = game.board.side_to_move();
    if side_to_move == Color::Black {
        move_list.reverse()
    }
    let mut alpha = i32::MIN;
    let mut beta = i32::MAX;
    let mut new_move_list = Vec::new();
    for move_to_play in move_list {
        let evaluation = get_move_evaluation(
            move_to_play.best_move,
            &game.board,
            depth,
            alpha,
            beta,
            &game.previous_boards,
            game.forced_capture,
        );
        new_move_list.push(MoveEval {
            evaluation: evaluation,
            best_move: move_to_play.best_move,
        });
        match side_to_move {
            Color::Black => {
                alpha = max(alpha, evaluation);
                if evaluation >= beta {
                    continue;
                }
            }
            Color::White => {
                beta = min(beta, evaluation);
                if evaluation <= alpha {
                    continue;
                }
            }
        }
    }
    new_move_list.sort_by(|a, b| a.evaluation.cmp(&b.evaluation));
    return new_move_list;
}

fn get_move_depth_1(game: Game) -> Vec<MoveEval> {
    let side_to_move = game.board.side_to_move();
    let enemy_pieces = game.board.colors(!side_to_move);
    let mut capture_moves: Vec<Move> = Vec::new();
    let mut move_list: Vec<MoveEval> = Vec::new();
    if game.forced_capture {
        game.board.generate_moves(|moves| {
            let mut captures = moves.clone();
            captures.to &= enemy_pieces;
            for mv in captures {
                capture_moves.push(mv);
            }
            false
        });
        if !capture_moves.is_empty() {
            for mv in capture_moves {
                let cur_eval = get_move_evaluation(
                    mv,
                    &game.board,
                    0,
                    i32::MIN,
                    i32::MAX,
                    &game.previous_boards,
                    game.forced_capture,
                );
                let move_eval = MoveEval {
                    evaluation: cur_eval,
                    best_move: mv,
                };
                move_list.push(move_eval);
            }
            move_list.sort_by(|a, b| a.evaluation.cmp(&b.evaluation));
            return move_list;
        }
    }

    game.board.generate_moves(|moves| {
        for mv in moves {
            let cur_eval = get_move_evaluation(
                mv,
                &game.board,
                0,
                i32::MIN,
                i32::MAX,
                &game.previous_boards,
                game.forced_capture,
            );
            let move_eval = MoveEval {
                evaluation: cur_eval,
                best_move: mv,
            };
            move_list.push(move_eval);
        }
        false
    });
    move_list.sort_by(|a, b| a.evaluation.cmp(&b.evaluation));
    return move_list;
}

/// Returns the evaluation for a specific move. Assumes players will pick the move that hurts them the most
/// depth = the amount of ply to search down. 0 is base case, 1 makes opponent move and stops
/// alpha = minimum score that the maximizing player is assured of
/// beta = maximum score that the minimizing player is assured of.
fn get_move_evaluation(
    piece_move: Move,
    board: &Board,
    depth: i32,
    alpha: i32,
    beta: i32,
    previous_boards: &HashMap<u64, i32>,
    forced_capture: bool,
) -> i32 {
    let mut board_with_move = board.clone();
    let mut prev_boards = previous_boards.clone(); //TODO: Make / Unmake
    board_with_move.play(piece_move);
    let board_hash = board_with_move.hash();
    prev_boards
        .entry(board_hash)
        .and_modify(|board_hash| *board_hash += 1)
        .or_insert(1);
    get_board_evaluation(
        &board_with_move,
        depth,
        alpha,
        beta,
        &prev_boards,
        forced_capture,
    )
}

/// Returns the evaluation for white for a specific move. Assumes players will pick the move that hurts them the most
/// depth = the amount of ply to search down. 0 is base case, 1 makes opponent move and stops
/// alpha = minimum score that the maximizing player is assured of
/// beta = maximum score that the minimizing player is assured of.
fn get_board_evaluation(
    board: &Board,
    depth: i32,
    mut alpha: i32,
    mut beta: i32,
    previous_boards: &HashMap<u64, i32>,
    forced_capture: bool,
) -> i32 {
    // Deal with game ending evaluation
    for val in previous_boards.values() {
        if *val >= 3 {
            return 0; // 3 fold reprition, stop
        }
    }

    match board.status() {
        GameStatus::Drawn => return 0,
        GameStatus::Won => match board.side_to_move() {
            // Recall loser is current side to move
            Color::Black => {
                return i32::MAX;
            }
            Color::White => {
                return i32::MIN;
            }
        },
        GameStatus::Ongoing => (),
    }
    if board.halfmove_clock() >= 50 {
        return 0;
    }
    if depth == 0 {
        let mut evaluation = 0;
        evaluation += get_truncated_eval(board);
        return evaluation;
    }

    // Recursive evaluation

    let side_to_move = board.side_to_move();
    let enemy_pieces = board.colors(!side_to_move);
    let mut capture_moves: Vec<Move> = Vec::new();

    let mut evaluation = match side_to_move {
        Color::Black => i32::MIN,
        Color::White => i32::MAX,
    };

    // Handle case of forced capture
    if forced_capture {
        board.generate_moves(|moves| {
            let mut captures = moves.clone();
            captures.to &= enemy_pieces;
            for mv in captures {
                capture_moves.push(mv);
            }
            false
        });
        if !capture_moves.is_empty() {
            for mv in capture_moves {
                let cur_eval = get_move_evaluation(
                    mv,
                    board,
                    depth - 1,
                    alpha,
                    beta,
                    previous_boards,
                    forced_capture,
                );
                match side_to_move {
                    Color::Black => {
                        evaluation = max(cur_eval, evaluation);
                        alpha = max(alpha, evaluation);
                        if evaluation >= beta {
                            return evaluation;
                        }
                    }
                    Color::White => {
                        evaluation = min(cur_eval, evaluation);
                        beta = min(beta, evaluation);
                        if evaluation <= alpha {
                            return evaluation;
                        }
                    }
                }
            }
            return evaluation;
        }
    }

    // Handle case of any legal move is valid
    board.generate_moves(|moves| {
        for mv in moves {
            let cur_eval = get_move_evaluation(
                mv,
                board,
                depth - 1,
                alpha,
                beta,
                previous_boards,
                forced_capture,
            );
            match side_to_move {
                Color::Black => {
                    evaluation = max(cur_eval, evaluation);
                    alpha = max(alpha, evaluation);
                    if evaluation >= beta {
                        return true;
                    }
                }
                Color::White => {
                    evaluation = min(cur_eval, evaluation);
                    beta = min(beta, evaluation);
                    if evaluation <= alpha {
                        return true;
                    }
                }
            }
        }
        false
    });
    return evaluation;
}
