use cozy_chess::Square::A1;
use cozy_chess::{Board, Color, GameStatus, Move, Piece};
use std::cmp::max;

/// Returns a legal move that places the player to move in the worst position
pub fn get_move(board: &Board) -> Move {
    let mut max_eval: f32 = f32::INFINITY;
    let mut cur_move = Move {
        from: A1,
        to: A1,
        promotion: None,
    };
    board.generate_moves(|moves| {
        for mv in moves {
            let cur_eval = get_evaluation(&mv, &board, 3, f32::NEG_INFINITY, f32::INFINITY);
            if cur_eval <= max_eval {
                cur_move = mv;
                max_eval = cur_eval;
            }
            println!("The current evaluation is {cur_eval}");
        }
        false
    });
    cur_move
}

/// Returns the evaluation for a specific move. Assumes players will pick the move that hurts them the most
/// depth = the amount of ply to search down. 0 is base case, 1 makes opponent move and stops
/// alpha = minimum score that the maximizing player is assured of
/// beta = maximum score that the minimizing player is assured of.
fn get_evaluation(piece_move: &Move, board: &Board, depth: i32, alpha: f32, beta: f32) -> f32 {
    let mut board_with_move = board.clone();
    board_with_move.play_unchecked(piece_move.clone());
    match board_with_move.status() {
        GameStatus::Drawn => return 0.0,
        GameStatus::Won => return f32::NEG_INFINITY,
        GameStatus::Ongoing => (),
    }
    if depth == 0 {
        let mut evaluation: f32 = 0.0;
        evaluation += get_material_evaluation(&board_with_move) as f32;
        return evaluation;
    }
    let mut evaluation;
    let mut new_alpha = alpha;
    let mut new_beta = beta;
    if board.side_to_move() == Color::Black {
        evaluation = f32::NEG_INFINITY;
        board.generate_moves(|moves| {
            for mv in moves {
                let potential_eval = get_evaluation(&mv, &board, depth - 1, new_alpha, new_beta);
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
                let potential_eval = get_evaluation(&mv, &board, depth - 1, new_alpha, new_beta);
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

/// Returns the material evaluation of a particular board for white
fn get_material_evaluation(board: &Board) -> i32 {
    let mut material: i32 = 0;
    material += board.colored_pieces(Color::White, Piece::Pawn).len() as i32;
    material += board.colored_pieces(Color::White, Piece::Knight).len() as i32 * 3;
    material += board.colored_pieces(Color::White, Piece::Bishop).len() as i32 * 3;
    material += board.colored_pieces(Color::White, Piece::Rook).len() as i32 * 5;
    material += board.colored_pieces(Color::White, Piece::Queen).len() as i32 * 9;

    material -= board.colored_pieces(Color::Black, Piece::Pawn).len() as i32;
    material -= board.colored_pieces(Color::Black, Piece::Knight).len() as i32 * 3;
    material -= board.colored_pieces(Color::Black, Piece::Bishop).len() as i32 * 3;
    material -= board.colored_pieces(Color::Black, Piece::Rook).len() as i32 * 5;
    material -= board.colored_pieces(Color::Black, Piece::Queen).len() as i32 * 9;
    return material;
}
