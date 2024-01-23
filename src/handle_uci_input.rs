use cozy_chess::{Board, Color, Move};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self};
use std::time::Duration;

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

pub fn go(input_tokens: &Vec<&str>, game: Game) -> Game {
    let changed_game = Game {
        is_searching: true,
        current_best_move: Option::Some(Arc::new(Mutex::new(Option::None))),
        stop_search: Arc::new(AtomicBool::new(false)),
        move_set: Arc::new(AtomicBool::new(false)),
        ..game
    };
    let side_to_move = changed_game.board.side_to_move();
    let new_game = changed_game.clone();
    let timer_game = changed_game.clone();
    let mut white_time: Option<u64> = Option::None;
    let mut black_time: Option<u64> = Option::None;
    let mut infinite = false;
    let mut time_to_move: Option<u64> = Option::None;
    for token_idx in 0..input_tokens.len() {
        match input_tokens[token_idx] {
            "wtime" => {
                white_time = Some(input_tokens[token_idx + 1].parse().unwrap());
                // let white_unwrapped_time = white_time.unwrap();
                // eprintln!("White time = {white_unwrapped_time}");
            }
            "btime" => {
                black_time = Some(input_tokens[token_idx + 1].parse().unwrap());
                // let black_unwrapped_time = black_time.unwrap();
                // eprintln!("Black time = {black_unwrapped_time}")
            }
            "infinite" => {
                infinite = true;
                // eprintln!("Told to go infinitely now");
            }
            "movetime" => time_to_move = Some(input_tokens[token_idx + 1].parse().unwrap()),
            _ => (),
        }
    }
    let _running_thread = thread::spawn(move || {
        get_move(new_game);
    });
    let _timer_thread = thread::spawn(move || {
        if time_to_move == Option::None && !infinite {
            match side_to_move {
                Color::White => {
                    time_to_move = Option::Some(white_time.unwrap_or(10) / 50);
                }
                Color::Black => {
                    time_to_move = Option::Some(black_time.unwrap_or(10) / 50);
                }
            }
        }
        if !infinite {
            // let unwraped_time = time_to_move.unwrap();
            // eprintln!("Time to move = {unwraped_time}");
            let time_to_move_dur = Duration::from_millis(time_to_move.unwrap());
            thread::sleep(time_to_move_dur);
            // eprintln!("Telling to stop");
            stop(timer_game);
        }
    });
    changed_game
}

pub fn stop(game: Game) -> Game {
    // Need to prevent stopping a stopped search
    // let game_board = game.board.clone();
    // eprintln!("Told to stop now, board = {game_board}");
    game.stop_search.store(true, Ordering::SeqCst);
    while !game.move_set.load(Ordering::SeqCst){
        thread::sleep(Duration::from_millis(1));
    }
    // Note: To Avoid panics with valid UCI commands, the worker thread must maintain
    // The lock on current best move until it has written data into the field
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
        move_set: Arc::new(AtomicBool::new(false)),
    }
}
