use cozy_chess::Square::A1;
use cozy_chess::{Board, Color, GameStatus, Move, Piece};

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
/// Higher evaluation == Good for the side to move
/// We want to pick the lowest evaluation
/// INFINITY = Size to move Wins
/// -INFINITY = Other side Wins
fn get_evaluation(piece_move: &Move, board: &Board) -> f32 {
    // Get how many pieces exist
    let mut board_with_move = board.clone();
    board_with_move.play_unchecked(piece_move.clone());
    match board_with_move.status() {
        GameStatus::Drawn => return 0.0,
        GameStatus::Won => return f32::NEG_INFINITY,
        GameStatus::Ongoing => (),
    }
    // TODO: Implement 3 move repition

    let mut evaluation: f32 = 0.0;
    evaluation += get_material_evaluation(&board_with_move) as f32;
    return evaluation;
}

/// Returns the material evaluation of a particular move
///
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
    // if black is current move, we need to flip this

    if board.side_to_move() == Color::Black {
        material *= -1;
    }
    return material;
}
