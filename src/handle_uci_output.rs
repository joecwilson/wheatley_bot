use cozy_chess::Move;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Implements the response to the id command
pub fn id() {
    println!("id name wheatleybot v.{VERSION}");
    println!("id author Joseph Wilson");
    get_options();
    println!("uciok");
}

fn get_options() {
    println!("option name ForceCapture type check default true");
}

/// Responds to the isready command
pub fn ready_ok() {
    println!("readyok");
}

/// Sends information to the GUI
pub fn send_info(info: &str) {
    println!("info {info}");
}

pub fn best_move(requested_move: &Move) {
    println!("bestmove {requested_move}");
}
