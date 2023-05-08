use std::ops::Neg;

use cozy_chess::Square::A1;
use cozy_chess::{Board, Color, GameStatus, Move, Piece};

/// Piece Square tables copied from https://www.chessprogramming.org/Simplified_Evaluation_Function#Piece-Square_Tables
const PAWN_PIECE_SQUARE_TABLE: [i8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5, 5,
    10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10, -20,
    -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
];
const KNIGHT_PIECE_SQUARE_TABLE: [i8; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -40, -20, 0, 0, 0, 0,
    -20, -40, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15, 15,
    10, 5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
];
const BISHIOP_PIECE_SQUARE_TABLE: [i8; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5, 0,
    -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10, 10, 10,
    -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
];
const ROOK_PIECE_SQUARE_TABLE: [i8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
    0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0,
    0, 5, 5, 0, 0, 0,
];
const QUEEN_PIECE_SQUARE_TABLE: [i8; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0, -10,
    -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0, 5, 0, 0,
    0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
];
const KING_MIDDLE_GAME_PIECE_SQUARE_TABLE: [i8; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40,
    -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40, -40, -30,
    -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0,
    10, 30, 20,
];
const KING_END_GAME_PIECE_SQUARE_TABLE: [i8; 64] = [
    -50, -40, -30, -20, -20, -30, -40, -50, -30, -20, -10, 0, 0, -10, -20, -30, -30, -10, 20, 30,
    30, 20, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30,
    -10, 20, 30, 30, 20, -10, -30, -30, -30, 0, 0, 0, 0, -30, -30, -50, -30, -30, -30, -30, -30,
    -30, -50,
];

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
            let cur_eval = get_move_evaluation(&mv, &board, 0, f32::NEG_INFINITY, f32::INFINITY);
            if cur_eval <= max_eval {
                cur_move = mv;
                max_eval = cur_eval;
            }
            println!("The evaluation after making {mv} is {cur_eval}")
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
        GameStatus::Won => return f32::NEG_INFINITY,
        GameStatus::Ongoing => (),
    }
    if depth == 0 {
        let mut evaluation: f32 = 0.0;
        evaluation += get_material_evaluation(&board) as f32;
        evaluation += get_piece_square_evaluation(&board) as f32;
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

/// Returns the material evaluation of a particular board for white in centipawns
/// Uses standard material weights
///     Pawns == 100 centipawns
///     Knights == 300 centipawns
///     Bishiops == 300 centipawns
///     Rooks == 500 centipawns
///     Queens == 900 centipawns
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
    material *= 100;
    return material;
}

/// Returns the value of the positions that the pieces are in
/// The percice piece square tables are declared
/// Again evaluates with respect to white, negate for black
fn get_piece_square_evaluation(board: &Board) -> i32 {
    let mut value: i32 = 0;
    value += get_pawn_piece_square_evaluation(board);
    value += get_rook_piece_square_evaluation(board);
    value += get_bishiop_piece_square_evaluation(board);
    value += get_knight_piece_square_evaluation(board);
    value += get_queen_piece_square_evaluation(board);
    value += get_king_piece_square_evaluation(board);
    return value;
}

fn get_pawn_piece_square_evaluation(board: &Board) -> i32 {
    let mut pawn_value = 0;
    let white_pawns = board.colored_pieces(Color::White, Piece::Pawn).into_iter();
    for pawn in white_pawns {
        let index = pawn as usize;
        pawn_value += PAWN_PIECE_SQUARE_TABLE[index] as i32;
    }
    let black_pawns = board.colored_pieces(Color::Black, Piece::Pawn).into_iter();
    for pawn in black_pawns {
        let index = pawn as usize;
        pawn_value -= PAWN_PIECE_SQUARE_TABLE[(7 - (index / 8)) * 8 + (index % 8)] as i32;
    }
    pawn_value
}

fn get_bishiop_piece_square_evaluation(board: &Board) -> i32 {
    let mut bishop_value = 0;
    let white_bishiops = board
        .colored_pieces(Color::White, Piece::Bishop)
        .into_iter();
    for bishiop in white_bishiops {
        let index = bishiop as usize;
        bishop_value += BISHIOP_PIECE_SQUARE_TABLE[index] as i32;
    }
    let black_bishiops = board
        .colored_pieces(Color::Black, Piece::Bishop)
        .into_iter();
    for bishiop in black_bishiops {
        let index = bishiop as usize;
        bishop_value -= BISHIOP_PIECE_SQUARE_TABLE[(7 - (index / 8)) * 8 + (index % 8)] as i32;
    }
    bishop_value
}

fn get_knight_piece_square_evaluation(board: &Board) -> i32 {
    let mut knight_value = 0;
    let white_knights = board
        .colored_pieces(Color::White, Piece::Knight)
        .into_iter();
    for knight in white_knights {
        let index = knight as usize;
        knight_value += KNIGHT_PIECE_SQUARE_TABLE[index] as i32;
    }
    let black_knights = board
        .colored_pieces(Color::Black, Piece::Knight)
        .into_iter();
    for knight in black_knights {
        let index = knight as usize;
        knight_value -= KNIGHT_PIECE_SQUARE_TABLE[(7 - (index / 8)) * 8 + (index % 8)] as i32;
    }
    knight_value
}

fn get_rook_piece_square_evaluation(board: &Board) -> i32 {
    let mut rook_value = 0;
    let white_rooks = board.colored_pieces(Color::White, Piece::Rook).into_iter();
    for rook in white_rooks {
        let index = rook as usize;
        rook_value += ROOK_PIECE_SQUARE_TABLE[index] as i32;
    }
    let black_rooks = board.colored_pieces(Color::Black, Piece::Rook).into_iter();
    for rook in black_rooks {
        let index = rook as usize;
        rook_value -= ROOK_PIECE_SQUARE_TABLE[(7 - (index / 8)) * 8 + (index % 8)] as i32;
    }
    rook_value
}

fn get_queen_piece_square_evaluation(board: &Board) -> i32 {
    let mut queen_value = 0;
    let white_queens = board.colored_pieces(Color::White, Piece::Rook).into_iter();
    for queen in white_queens {
        let index = queen as usize;
        queen_value += QUEEN_PIECE_SQUARE_TABLE[index] as i32;
    }
    let black_queens = board.colored_pieces(Color::Black, Piece::Rook).into_iter();
    for queen in black_queens {
        let index = queen as usize;
        queen_value -= QUEEN_PIECE_SQUARE_TABLE[(7 - (index / 8)) * 8 + (index % 8)] as i32;
    }
    queen_value
}

fn get_king_piece_square_evaluation(board: &Board) -> i32 {
    let mut king_value = 0;
    // Decide if we are in the endgame. Lets just assume always middlegame
    //TODO - Determine endgame
    let white_king = board.colored_pieces(Color::White, Piece::Rook).into_iter();
    for king in white_king {
        let index = king as usize;
        king_value += KING_MIDDLE_GAME_PIECE_SQUARE_TABLE[index] as i32;
    }
    let black_king = board.colored_pieces(Color::Black, Piece::Rook).into_iter();
    for king in black_king {
        let index = king as usize;
        king_value -=
            KING_MIDDLE_GAME_PIECE_SQUARE_TABLE[(7 - (index / 8)) * 8 + (index % 8)] as i32;
    }
    king_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_square_evaluation_default() {
        assert_eq!(get_piece_square_evaluation(&Board::default()), 0);
    }
}
