use cozy_chess::Square::A1;
use cozy_chess::{Board, Move};

/// Returns a legal move that places the color in the worst position
pub fn get_move(board: &Board) -> Move {
    let mut min_eval: f32 = f32::NEG_INFINITY;
    let mut cur_move = Move {
        from: A1,
        to: A1,
        promotion: None,
    };
    board.generate_moves(|moves| {
        for mv in moves {
            let cur_eval = get_evaluation(&mv, &board);
            if cur_eval >= min_eval {
                cur_move = mv;
                min_eval = cur_eval;
            }
        }
        false
    });
    cur_move
}

/// Returns the evaluation for a specific move.
/// Higher evaluation == Good for the current player
/// We want to pick the lowest evaluation
/// INFINITY = Engine Wins
/// -INFINITY = Player Wins
fn get_evaluation(piece_move: &Move, board: &Board) -> f32 {
    return 1.0;
}
