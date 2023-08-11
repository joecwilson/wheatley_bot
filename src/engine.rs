use std::ops::Neg;

use crate::predicted_eval::get_truncated_eval;
use cozy_chess::Square::A1;
use cozy_chess::{Board, Color, GameStatus, Move};

/// Returns a legal move that places the player to move in the worst position
/// Additionally returns the evaluation after said move
pub fn get_move(board: &Board) -> (Move, f32) {
    let mut max_eval: f32 = f32::INFINITY;
    let mut cur_move = Move {
        from: A1,
        to: A1,
        promotion: None,
    };
    board.generate_moves(|moves| {
        for mv in moves {
            let cur_eval = get_move_evaluation(&mv, &board, 3, f32::NEG_INFINITY, f32::INFINITY);
            if cur_eval <= max_eval {
                cur_move = mv;
                max_eval = cur_eval;
            }
        }
        false
    });
    (cur_move, max_eval)
}

/// Returns the evaluation for a specific move. Assumes players will pick the move that hurts them the most
/// depth = the amount of ply to search down. 0 is base case, 1 makes opponent move and stops
/// alpha = minimum score that the maximizing player is assured of
/// beta = maximum score that the minimizing player is assured of.
fn get_move_evaluation(piece_move: &Move, board: &Board, depth: i32, alpha: f32, beta: f32) -> f32 {
    let mut board_with_move = board.clone();
    board_with_move.play_unchecked(piece_move.clone());
    get_board_evaluation(&board_with_move, depth, alpha, beta)
}

/// Returns the evaluation for a specific move. Assumes players will pick the move that hurts them the most
/// depth = the amount of ply to search down. 0 is base case, 1 makes opponent move and stops
/// alpha = minimum score that the maximizing player is assured of
/// beta = maximum score that the minimizing player is assured of.
fn get_board_evaluation(board: &Board, depth: i32, alpha: f32, beta: f32) -> f32 {
    match board.status() {
        GameStatus::Drawn => return 0.0,
        GameStatus::Won => return f32::INFINITY,
        GameStatus::Ongoing => (),
    }
    if depth == 0 {
        let mut evaluation: f32 = 0.0;
        evaluation += get_truncated_eval(board) as f32;
        if board.side_to_move() == Color::White {
            evaluation = evaluation.neg();
        }
        return evaluation;
    }
    let mut evaluation;
    let mut new_alpha = alpha;
    let mut new_beta = beta;
    if board.side_to_move() == Color::Black {
        evaluation = f32::NEG_INFINITY;
        board.generate_moves(|moves| {
            for mv in moves {
                let potential_eval =
                    get_move_evaluation(&mv, &board, depth - 1, new_alpha, new_beta);
                if potential_eval > evaluation {
                    evaluation = potential_eval;
                }
                if evaluation > new_beta {
                    return true;
                }
                if new_alpha > evaluation {
                    new_alpha = evaluation;
                }
            }
            false
        });
    } else {
        evaluation = f32::INFINITY;
        board.generate_moves(|moves| {
            for mv in moves {
                let potential_eval =
                    get_move_evaluation(&mv, &board, depth - 1, new_alpha, new_beta);
                if potential_eval < evaluation {
                    evaluation = potential_eval;
                }
                if evaluation < new_alpha {
                    return true;
                }
                if evaluation < new_beta {
                    new_beta = evaluation;
                }
            }
            false
        });
    }
    return evaluation;
}
