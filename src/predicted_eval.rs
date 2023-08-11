use cozy_chess::{Board, Color, Piece, Square};

/// Piece Square tables copied from https://www.chessprogramming.org/Simplified_Evaluation_Function#Piece-Square_Tables
const PAWN_PIECE_SQUARE_TABLE: [[i8; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [10, 10, 20, 30, 30, 20, 10, 10],
    [5, 5, 10, 25, 25, 10, 5, 5],
    [0, 0, 0, 20, 20, 0, 0, 0],
    [5, -5, -10, 0, 0, -10, -5, 5],
    [5, 10, 10, -20, -20, 10, 10, 5],
    [0, 0, 0, 0, 0, 0, 0, 0],
];
const KNIGHT_PIECE_SQUARE_TABLE: [[i8; 8]; 8] = [
    [-50, -40, -30, -30, -30, -30, -40, -50],
    [-40, -20, 0, 0, 0, 0, -20, -40],
    [-40, -20, 0, 0, 0, 0, -20, -40],
    [-30, 5, 15, 20, 20, 15, 5, -30],
    [-30, 0, 15, 20, 20, 15, 0, -30],
    [-30, 5, 10, 15, 15, 10, 5, -30],
    [-40, -20, 0, 5, 5, 0, -20, -40],
    [-50, -40, -30, -30, -30, -30, -40, -50],
];
const BISHIOP_PIECE_SQUARE_TABLE: [[i8; 8]; 8] = [
    [-20, -10, -10, -10, -10, -10, -10, -20],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-10, 0, 5, 10, 10, 5, 0, -10],
    [-10, 5, 5, 10, 10, 5, 5, -10],
    [-10, 0, 10, 10, 10, 10, 0, -10],
    [-10, 10, 10, 10, 10, 10, 10, -10],
    [-10, 5, 0, 0, 0, 0, 5, -10],
    [-20, -10, -10, -10, -10, -10, -10, -20],
];
const ROOK_PIECE_SQUARE_TABLE: [[i8; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [5, 10, 10, 10, 10, 10, 10, 5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [0, 0, 0, 5, 5, 0, 0, 0],
];
const QUEEN_PIECE_SQUARE_TABLE: [[i8; 8]; 8] = [
    [-20, -10, -10, -5, -5, -10, -10, -20],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-10, 0, 5, 5, 5, 5, 0, -10],
    [-5, 0, 5, 5, 5, 5, 0, -5],
    [0, 0, 5, 5, 5, 5, 0, -5],
    [-10, 5, 5, 5, 5, 5, 0, -10],
    [-10, 0, 5, 0, 0, 0, 0, -10],
    [-20, -10, -10, -5, -5, -10, -10, -20],
];
const KING_MIDDLE_GAME_PIECE_SQUARE_TABLE: [[i8; 8]; 8] = [
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-20, -30, -30, -40, -40, -30, -30, -20],
    [-10, -20, -20, -20, -20, -20, -20, -10],
    [20, 20, 0, 0, 0, 0, 20, 20],
    [20, 30, 10, 0, 0, 10, 30, 20],
];
const KING_END_GAME_PIECE_SQUARE_TABLE: [[i8; 8]; 8] = [
    [-50, -40, -30, -20, -20, -30, -40, -50],
    [-30, -20, -10, 0, 0, -10, -20, -30],
    [-30, -10, 20, 30, 30, 20, -10, -30],
    [-30, -10, 30, 40, 40, 30, -10, -30],
    [-30, -10, 30, 40, 40, 30, -10, -30],
    [-30, -10, 20, 30, 30, 20, -10, -30],
    [-30, -30, 0, 0, 0, 0, -30, -30],
    [-50, -30, -30, -30, -30, -30, -30, -50],
];

pub fn get_truncated_eval(board: &Board) -> i32 {
    get_material_evaluation(board) + get_piece_square_evaluation(board)
}

struct RowAndCol {
    row: usize,
    col: usize,
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
        let index = index_to_row_and_col(pawn);
        pawn_value += PAWN_PIECE_SQUARE_TABLE[index.row][index.col] as i32;
    }
    let black_pawns = board.colored_pieces(Color::Black, Piece::Pawn).into_iter();
    for pawn in black_pawns {
        let index = index_to_row_and_col(pawn);
        pawn_value -= PAWN_PIECE_SQUARE_TABLE[7 - index.row][7 - index.col] as i32;
    }
    pawn_value
}

fn get_bishiop_piece_square_evaluation(board: &Board) -> i32 {
    let mut bishop_value = 0;
    let white_bishiops = board
        .colored_pieces(Color::White, Piece::Bishop)
        .into_iter();
    for bishiop in white_bishiops {
        let index = index_to_row_and_col(bishiop);
        bishop_value += BISHIOP_PIECE_SQUARE_TABLE[7 - index.row][7 - index.col] as i32;
    }
    let black_bishiops = board
        .colored_pieces(Color::Black, Piece::Bishop)
        .into_iter();
    for bishiop in black_bishiops {
        let index = index_to_row_and_col(bishiop);
        bishop_value -= BISHIOP_PIECE_SQUARE_TABLE[7 - index.row][7 - index.col] as i32;
    }
    bishop_value
}

fn get_knight_piece_square_evaluation(board: &Board) -> i32 {
    let mut knight_value = 0;
    let white_knights = board
        .colored_pieces(Color::White, Piece::Knight)
        .into_iter();
    for knight in white_knights {
        let index = index_to_row_and_col(knight);
        knight_value += KNIGHT_PIECE_SQUARE_TABLE[index.row][index.col] as i32;
    }
    let black_knights = board
        .colored_pieces(Color::Black, Piece::Knight)
        .into_iter();
    for knight in black_knights {
        let index = index_to_row_and_col(knight);
        knight_value -= KNIGHT_PIECE_SQUARE_TABLE[7 - index.row][7 - index.col] as i32;
    }
    knight_value
}

fn get_rook_piece_square_evaluation(board: &Board) -> i32 {
    let mut rook_value = 0;
    let white_rooks = board.colored_pieces(Color::White, Piece::Rook).into_iter();
    for rook in white_rooks {
        let index = index_to_row_and_col(rook);
        rook_value += ROOK_PIECE_SQUARE_TABLE[index.row][index.col] as i32;
    }
    let black_rooks = board.colored_pieces(Color::Black, Piece::Rook).into_iter();
    for rook in black_rooks {
        let index = index_to_row_and_col(rook);
        rook_value -= ROOK_PIECE_SQUARE_TABLE[7 - index.row][7 - index.col] as i32;
    }
    rook_value
}

fn get_queen_piece_square_evaluation(board: &Board) -> i32 {
    let mut queen_value = 0;
    let white_queens = board.colored_pieces(Color::White, Piece::Rook).into_iter();
    for queen in white_queens {
        let index = index_to_row_and_col(queen);
        queen_value += QUEEN_PIECE_SQUARE_TABLE[index.row][index.col] as i32;
    }
    let black_queens = board.colored_pieces(Color::Black, Piece::Rook).into_iter();
    for queen in black_queens {
        let index = index_to_row_and_col(queen);
        queen_value -= QUEEN_PIECE_SQUARE_TABLE[7 - index.row][7 - index.col] as i32;
    }
    queen_value
}

/// Returns the evaluation in centipawns of the position of the kings on the board.
/// Like all piece square evaluations, this evaluation is from the view of the player playing white
fn get_king_piece_square_evaluation(board: &Board) -> i32 {
    let mut king_value = 0;
    // Decide if we are in the endgame. Lets just assume always middlegame
    //TODO - Determine endgame
    let white_king = board.colored_pieces(Color::White, Piece::Rook).into_iter();
    for king in white_king {
        let index = index_to_row_and_col(king);
        king_value += KING_MIDDLE_GAME_PIECE_SQUARE_TABLE[index.row][index.col] as i32;
    }
    let black_king = board.colored_pieces(Color::Black, Piece::Rook).into_iter();
    for king in black_king {
        let index = index_to_row_and_col(king);
        king_value -= KING_MIDDLE_GAME_PIECE_SQUARE_TABLE[7 - index.row][7 - index.col] as i32;
    }
    king_value
}

/// Converts an index to a row and column, the row is the first parameter in the tuple, while the column is the second parameter
fn index_to_row_and_col(square: Square) -> RowAndCol {
    let index = square as usize;
    RowAndCol {
        row: 7 - (index / 8),
        col: 7 - (index % 8),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_square_evaluation_default() {
        assert_eq!(get_piece_square_evaluation(&Board::default()), 0);
    }

    #[test]
    fn test_pawn_piece_square_evaluation() {
        let board1 = Board::from_fen("3k4/8/8/8/8/5P2/8/3K4 w - - 0 1", false).unwrap();
        assert_eq!(
            get_pawn_piece_square_evaluation(&board1),
            -10,
            "failed test 1"
        );
        let board2 = "3k4/8/8/8/8/2P5/8/3K4 w - - 0 1".parse().unwrap();
        assert_eq!(
            get_pawn_piece_square_evaluation(&board2),
            -10,
            "failed test 2"
        );
        let board3 = "3k4/8/1P6/8/8/8/8/3K4 w - - 0 1".parse().unwrap();
        assert_eq!(
            get_pawn_piece_square_evaluation(&board3),
            10,
            "failed test 3"
        );
        let board4 = "3k4/8/2p5/8/8/8/8/3K4 w - - 0 1".parse().unwrap();
        assert_eq!(
            get_pawn_piece_square_evaluation(&board4),
            10,
            "failed test 4"
        );
    }
}
