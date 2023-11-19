use std::collections::HashMap;
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
        previous_boards: game.previous_boards,
    }
}

pub fn is_ready(game: Game) -> Game {
    handle_uci_output::ready_ok();
    game
}

pub fn set_option(input_tokens: &Vec<&str>, game: Game) -> Game {
    if input_tokens[1] != "name" {
        handle_uci_output::send_info("Invalid option sent, name not present");
        panic!("Invalid UCI Command");
    }
    if input_tokens[2] != "ForcedCapture" {
        handle_uci_output::send_info("Invalid option sent, id not reckonised");
        panic!("Invalid UCI Command");
    }
    let forced_capture = input_tokens[4].parse().unwrap();
    Game {
        debug_mode: game.debug_mode,
        forced_capture: forced_capture,
        board: game.board,
        previous_boards: game.previous_boards,
    }
}

pub fn position(input_tokens: &Vec<&str>, game: Game) -> Game {
    let mut previous_boards: HashMap<u64, i32> = HashMap::new();
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
    previous_boards.insert(board.hash(), 1);
    for i in 3..input_tokens.len() {
        let played_move = input_tokens[i].parse::<Move>().unwrap();
        // move_history.push(played_move);
        board.play(played_move);
        let board_hash = board.hash();
        previous_boards
            .entry(board_hash)
            .and_modify(|board_hash| *board_hash += 1)
            .or_insert(1);
    }
    Game {
        debug_mode: game.debug_mode,
        forced_capture: game.forced_capture,
        board: board,
        previous_boards: previous_boards,
    }
}

pub fn go(_input_tokens: &Vec<&str>, game: Game) -> Game {
    let new_game = game.clone();
    let _thread = thread::spawn(move || {
        let engine_response = get_move(new_game);
        handle_uci_output::best_move(&engine_response.0.unwrap())
    });
    game
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
        forced_capture: true,
        previous_boards: HashMap::new(),
    }
}
