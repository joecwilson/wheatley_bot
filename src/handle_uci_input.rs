use std::thread::{self};

use cozy_chess::{Board, Move};

use crate::engine::get_move;
use crate::handle_uci_output;
use crate::play::Game;

pub fn debug(game: Game) -> Game {
    Game {
        debug_mode: true,
        forced_capture: game.forced_capture,
        board: game.board,
        move_history: game.move_history,
    }
}

pub fn is_ready(game: Game) -> Game {
    handle_uci_output::ready_ok();
    game
}

pub fn set_option(_input_tokens: &Vec<&str>, game: Game) -> Game {
    game
}

pub fn position(input_tokens: &Vec<&str>, game: Game) -> Game {
    let mut move_history: Vec<Move> = Vec::new();
    // First token is the position command
    if input_tokens[1] != "startpos" {
        handle_uci_output::send_info("Requires a starting point of startpos");
        panic!("Invalid UCI Command")
    }
    // if input_tokens[2] != "moves" {
    //     handle_uci_output::send_info("After startpos requires moves");
    //     panic!("Invalid UCI Command")
    // }
    let mut board = Board::startpos();
    for i in 3..input_tokens.len() {
        let played_move = input_tokens[i].parse::<Move>().unwrap();
        move_history.push(played_move);
        board.play(played_move);
    }
    Game {
        debug_mode: true,
        forced_capture: game.forced_capture,
        board: board,
        move_history: move_history,
    }
}

pub fn go(_input_tokens: &Vec<&str>, game: Game) -> Game {
    let new_game = game.clone();
    let _thread = thread::spawn(move || {
        let engine_response = get_move(&game.board);
        handle_uci_output::best_move(&engine_response.0.unwrap())
    });
    new_game
}

pub fn stop(game: Game) -> Game {
    game
}

pub fn uci_new_game() -> Game {
    default_game()
}

pub fn register(game: Game) -> Game {
    game
}

pub fn ponderhit(game: Game) -> Game {
    game
}

pub fn default_game() -> Game {
    Game {
        debug_mode: false,
        board: Board::startpos(),
        forced_capture: false,
        move_history: Vec::new(),
    }
}
