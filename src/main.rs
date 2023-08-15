use std::{io, process::ExitCode};

mod engine;
mod handle_uci_input;
mod handle_uci_output;
mod play;
mod predicted_eval;

fn main() -> ExitCode {
    println!("wheatleybot by Joseph Wilson");
    let mut uci_input = String::new();
    let std_in = io::stdin();
    std_in
        .read_line(&mut uci_input)
        .expect("Error When awaiting UCI signal");

    if uci_input != "uci\n" {
        // println!("Expected uci command to be given");
        handle_uci_output::send_info("Expected UCI Command to be given");
        return ExitCode::FAILURE;
    }
    handle_uci_output::id();
    let mut game = handle_uci_input::default_game();
    loop {
        let mut uci_command = String::new();
        std_in
            .read_line(&mut uci_command)
            .expect("Error When awaiting UCI signal");
        let uci_tokens: Vec<&str> = uci_command.split_whitespace().into_iter().collect();
        game = match uci_tokens[0] {
            "debug" => handle_uci_input::debug(game),
            "isready" => handle_uci_input::is_ready(game),
            "setoption" => handle_uci_input::set_option(&uci_tokens, game),
            "position" => handle_uci_input::position(&uci_tokens, game),
            "go" => handle_uci_input::go(&uci_tokens, game),
            "stop" => handle_uci_input::stop(&uci_tokens, game),
            "ucinewgame" => handle_uci_input::uci_new_game(&uci_tokens),
            "quit" => return ExitCode::SUCCESS,
            _ => {
                let first_word = uci_tokens[0];
                let info_to_send = format!(
                    "UCI Command [{first_word}] not recognized. Input string was {uci_command}"
                );
                handle_uci_output::send_info(&info_to_send);
                game
            }
        }
    }
}
