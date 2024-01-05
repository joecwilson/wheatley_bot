use cozy_chess::{Board, Move};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self};

use crate::engine::get_move;
use crate::handle_uci_output;
use crate::play::Game;

pub fn debug(game: Game) -> Game {
    Game {
        debug_mode: true,
        ..game
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
        forced_capture: forced_capture,
        ..game
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
        board: board,
        ..game
    }
}

pub fn go(_input_tokens: &Vec<&str>, game: Game) -> Game {
    let changed_game = Game {
        is_searching: true,
        current_best_move: Option::Some(Arc::new(Mutex::new(Option::None))),
        stop_search: Arc::new(AtomicBool::new(false)),
        ..game
    };
    let new_game = changed_game.clone();
    let _thread = thread::spawn(move || {
        let even_newer_game = get_move(new_game);
        let stopped = even_newer_game.stop_search.load(Ordering::SeqCst);
        if !stopped {
            let older_binding = even_newer_game.current_best_move.unwrap();
            let binding = older_binding.lock().unwrap();
            let move_evaluation = binding.as_ref().unwrap();
            handle_uci_output::best_move(&move_evaluation.best_move)
        }
    });
    changed_game
}

pub fn stop(game: Game) -> Game {
    game.stop_search.store(true, Ordering::SeqCst);
    // Note: To Avoid panics with valid UCI commands, the worker thread must maintain
    // The lock on current best move untill it has written data into the field
    let older_binding = game.current_best_move.unwrap();
    let binding = older_binding.lock().unwrap();
    let move_evaluation = binding.as_ref().unwrap();

    // let move_evaluation = game.current_best_move.unwrap().lock().unwrap().unwrap();
    handle_uci_output::best_move(&move_evaluation.best_move);

    Game {
        current_best_move: Option::None,
        is_searching: false,
        ..game
    }
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
        is_searching: false,
        current_best_move: Option::None,
        stop_search: Arc::new(AtomicBool::new(false)),
    }
}
