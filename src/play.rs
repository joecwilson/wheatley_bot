use cozy_chess::{Board, Move};
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MoveEval {
    pub evaluation: i32,
    pub best_move: Move,
}

#[derive(Clone)]
pub struct Game {
    /// Weither to print more output when printing a move
    pub debug_mode: bool,
    /// If captures should be able to be forced
    pub forced_capture: bool,
    /// The current state of the board in the game
    pub board: Board,
    /// A field to aid in collecting 3 and 5 move repetition
    /// The keys of this hashmap are the hashes of the board
    /// This is not competly safe from collisions, but is likely
    /// good enough for our needs
    /// The values show the number of times that this
    /// move has been collected
    pub previous_boards: HashMap<u64, i32>,
    /// Weither This game is searching, if this is true
    pub is_searching: bool,
    /// The move the engine currently thinks is best
    pub current_best_move: Option<Arc<Mutex<Option<MoveEval>>>>,
    /// Weither to stop a search
    pub stop_search: Arc<AtomicBool>,
    /// If a move has been set
    pub move_set: Arc<AtomicBool>,
}
