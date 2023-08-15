use cozy_chess::{Board, Move};

#[derive(Clone)]
pub struct Game {
    pub debug_mode: bool,
    pub forced_capture: bool,
    pub board: Board,
    pub move_history: Vec<Move>,
}
