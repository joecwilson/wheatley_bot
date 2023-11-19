use std::{cmp::max, cmp::min, collections::HashMap};

use crate::{predicted_eval::get_truncated_eval, play::Game};
use cozy_chess::{Board, Color, GameStatus, Move};

/// Returns a legal move that places the player to move in the worst position
/// Additionally returns the evaluation after said move
pub fn get_move(game: Game) -> (Option<Move>, i32) {
    let side_to_move = game.board.side_to_move();
    match side_to_move {
        Color::White => get_white_move(game),
        Color::Black => get_black_move(game)
    }
}


fn get_white_move(game: Game) -> (Option<Move>, i32) {
    assert_eq!(game.board.side_to_move(), Color::White);
    let mut best_eval = i32::MAX;
    let mut cur_move = Option::None;
    game.board.generate_moves(|moves| {
        for mv in moves {
            let cur_eval = get_move_evaluation(mv, &game.board, 3, i32::MIN, i32::MAX, &game.previous_boards);
            if cur_eval <= best_eval {
                cur_move = Some(mv);
                best_eval = cur_eval;
            }
        }
        false
    });
    (cur_move, best_eval)
}

fn get_black_move(game: Game) -> (Option<Move>, i32) {
    assert_eq!(game.board.side_to_move(), Color::Black);
    let mut best_eval = i32::MIN;
    let mut cur_move = Option::None;
    game.board.generate_moves(|moves| {
        for mv in moves {
            let cur_eval = get_move_evaluation(mv, &game.board, 3, i32::MIN, i32::MAX, &game.previous_boards);
            if cur_eval >= best_eval {
                cur_move = Some(mv);
                best_eval = cur_eval;
            }
        }
        false
    });
    (cur_move, best_eval)
}

/// Returns the evaluation for a specific move. Assumes players will pick the move that hurts them the most
/// depth = the amount of ply to search down. 0 is base case, 1 makes opponent move and stops
/// alpha = minimum score that the maximizing player is assured of
/// beta = maximum score that the minimizing player is assured of.
fn get_move_evaluation(piece_move: Move, board: &Board, depth: i32, alpha: i32, beta: i32, previous_boards: &HashMap<u64, i32>) -> i32 {
    let mut board_with_move = board.clone();
    let mut prev_boards = previous_boards.clone();
    board_with_move.play_unchecked(piece_move);
    let board_hash = board_with_move.hash();
    prev_boards.entry(board_hash).and_modify(|board_hash| *board_hash += 1).or_insert(1);
    get_board_evaluation(&board_with_move, depth, alpha, beta, &prev_boards)
}

/// Returns the evaluation for white for a specific move. Assumes players will pick the move that hurts them the most
/// depth = the amount of ply to search down. 0 is base case, 1 makes opponent move and stops
/// alpha = minimum score that the maximizing player is assured of
/// beta = maximum score that the minimizing player is assured of.
fn get_board_evaluation(board: &Board, depth: i32, alpha: i32, beta: i32, previous_boards: &HashMap<u64, i32>) -> i32 {
    for val in previous_boards.values() {
        if *val >= 3 {
            return 0; // 3 fold reprition, stop
        }
    }
    match board.status() {
        GameStatus::Drawn => return 0,
        GameStatus::Won => match board.side_to_move() {
            Color::Black => return i32::MAX,
            Color::White => return i32::MIN,
        },
        GameStatus::Ongoing => (),
    }
    if depth == 0 {
        let mut evaluation = 0;
        evaluation += get_truncated_eval(board);
        return evaluation;
    }
    let mut evaluation;
    let mut new_alpha = alpha;
    let mut new_beta = beta;
    if board.side_to_move() == Color::Black {
        evaluation = i32::MIN;
        board.generate_moves(|moves| {
            for mv in moves {
                let potential_eval =
                    get_move_evaluation(mv, &board, depth - 1, new_alpha, new_beta, previous_boards);
                evaluation = max(potential_eval, evaluation);
                if evaluation > new_beta {
                    return true;
                }
                new_alpha = max(new_alpha, evaluation);
            }
            false
        });
    } else {
        evaluation = i32::MAX;
        board.generate_moves(|moves| {
            for mv in moves {
                let potential_eval =
                    get_move_evaluation(mv, &board, depth - 1, new_alpha, new_beta, previous_boards);
                evaluation = min(potential_eval, evaluation);
                if evaluation < new_alpha {
                    return true;
                }
                new_beta = min(new_alpha, evaluation)
            }
            false
        });
    }
    return evaluation;
}
