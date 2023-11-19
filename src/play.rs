use std::collections::HashMap;

use cozy_chess::Board;

#[derive(Clone)]
pub struct Game {
    /// Weigher to print more output when printing a move
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
}
