use crate::engine;
use cozy_chess::{Board, Color, GameStatus, Move};

#[derive(Debug, Copy, Clone)]
pub enum GameState {
    PlayerWin,
    PlayerLose,
    Draw,
    InProgress,
    AttemptedIllegalMove,
}

#[derive(Debug)]
pub struct Game {
    board: Board,
    player_is_white: bool,
}

impl Game {
    // Goal:
    //  Hold the chess board in a function
    //  Call out to the engine to get its move
    //  Call out to the player to get its move

    // Setup:
    //  Get what color the player wants
    //  Get how the player wants the bot to play
    //      Lose vs a bot that also wants to love
    //      Lose vs a bot that wants to win
    //  Get if the bot should follow a "book" of bad openings
    //  Make first bot move if player is black
    pub fn new() -> Self {
        println!("Got here");
        Self {
            board: Board::default(),
            player_is_white: false,
        }
    }
    // Game:
    //  Get player move
    //  Get bot move
    //  Return status of game, Player Win, Player Lose, Draw, Stalemate, In Progress
    pub fn play_game(&mut self, player_move: &str) -> GameState {
        let player_move_result = player_move.parse::<Move>();
        let player_move_actual = match player_move_result {
            Ok(known_good_move) => known_good_move,
            Err(e) => {
                return GameState::AttemptedIllegalMove;
            }
        };
        let legal = self.board.is_legal(player_move_actual);
        if !legal {
            return GameState::AttemptedIllegalMove;
        }
        self.board.play_unchecked(player_move_actual);
        self.get_status()
    }

    /// Gets the current status of a game.
    /// TODO Get it to return draw for 3 fold repitition, and Insufficent Material
    fn get_status(&self) -> GameState {
        let cozy_status = self.board.status();
        match cozy_status {
            GameStatus::Drawn => GameState::Draw,
            GameStatus::Ongoing => GameState::InProgress,
            GameStatus::Won => {
                if self.player_is_white {
                    if self.board.side_to_move() == Color::White {
                        return GameState::PlayerLose;
                    }
                    return GameState::PlayerWin;
                } else {
                    if self.board.side_to_move() == Color::Black {
                        return GameState::PlayerLose;
                    }
                    return GameState::PlayerWin;
                }
            }
        }
    }
}
