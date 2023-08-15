use cozy_chess::Move;

/// Implements the response to the id command
pub fn id() {
    println!("id name wheatleybot Alpha.08.15.2023");
    println!("id author Joseph Wilson");
    println!("uciok");
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
