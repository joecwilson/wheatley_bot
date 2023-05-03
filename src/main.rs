use engine::get_move;

mod engine;
mod play;
fn main() {
    let mut game = play::Game::new();
    game.play_game("e2e4");
    get_move(&game.board);
}
