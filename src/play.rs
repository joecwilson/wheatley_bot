use cozy_chess::Board;
use cozy_chess::Move;

#[derive(Debug, Copy, Clone)]
pub enum GameState{
    PlayerWin,
    PlayerLose,
    Draw,
    InProgress,
    AttemptedIllegalMove,
}

#[derive(Debug)]
pub struct Game{
    board:Board
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
    pub fn new() -> Self{
        println!("Got here");
        Self{
            board : Board::default()
        }
    }
    // Game:
    //  Get player move
    //  Get bot move
    //  Return status of game, Player Win, Player Lose, Draw, Stalemate, In Progress
    pub fn play_game(&mut self, player_move: &str) -> GameState {
        let player_move_result = player_move.parse::<Move>();
        let player_move_actual = match player_move_result{
            Ok(known_good_move) => {
                known_good_move
            }
            Err(e) => {
                return GameState::AttemptedIllegalMove;
            }
        };
        let legal = self.board.is_legal(player_move_actual);
        if !legal{
            return GameState::AttemptedIllegalMove;
        }
        self.board.play_unchecked(player_move_actual);

        return GameState::InProgress
    }
}
