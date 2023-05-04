use cozy_chess::Board;
use cozy_chess::Color;
use engine::get_move;
use std::io;

use crate::play::GameState;
mod engine;
mod play;
#[derive(Debug, Copy, Clone)]
struct ChessPiece {
    piece: PossibleChessPiece,
    color: Color,
}
#[derive(Debug, Copy, Clone)]
enum PossibleChessPiece {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

fn main() {
    let mut game = play::Game::new();
    print_board(&game.board);
    let mut state: GameState;
    // Lets actually get a game
    loop {
        state = GameState::AttemptedIllegalMove;
        let engine_prediction = get_move(&game.board);
        println!("I would make {engine_prediction}");
        while state == GameState::AttemptedIllegalMove {
            // We need input
            println!("Enter the starting position, followed by the position to move");
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            let trimmed = line.trim();
            state = game.play_game(&trimmed);
        }
        print_board(&game.board);
        if state != GameState::InProgress {
            break;
        }
        println!("My turn");
        let engine_move = get_move(&game.board);
        game.board.play(engine_move);
        print_board(&game.board);
        if state != GameState::InProgress {
            break;
        }
    }
    match state {
        GameState::Draw => println!("Draw. Good Game"),
        GameState::PlayerLose => println!("I win!"),
        GameState::PlayerWin => println!("You win T_T"),
        _ => panic!("Unexpected state"),
    }
}

fn print_board(board: &Board) {
    // We need to determine what the board looks like
    // Then we can display the board
    // Lets get the board into an array of ChessPieces

    // board.
    let fen = format!("{:#}", board);
    let chess_pieces = parse_fen(&fen);

    // Print all of the pieces
    for index in 0..64 {
        if index % 8 == 0 {
            println!();
        }
        print_piece(&chess_pieces[index]);
    }
    println!();
}

fn parse_fen(fen: &str) -> [ChessPiece; 64] {
    let mut chess_pieces = [ChessPiece {
        piece: PossibleChessPiece::None,
        color: Color::White,
    }; 64];
    let mut index = 0;
    for char in fen.chars() {
        if char == '/' {
            continue;
        }
        match char {
            'p' => {
                chess_pieces[index] = ChessPiece {
                    piece: PossibleChessPiece::Pawn,
                    color: Color::Black,
                }
            }
            'n' => {
                chess_pieces[index] = ChessPiece {
                    piece: PossibleChessPiece::Knight,
                    color: Color::Black,
                }
            }
            'b' => {
                chess_pieces[index] = ChessPiece {
                    piece: PossibleChessPiece::Bishop,
                    color: Color::Black,
                }
            }
            'r' => {
                chess_pieces[index] = ChessPiece {
                    piece: PossibleChessPiece::Rook,
                    color: Color::Black,
                }
            }
            'q' => {
                chess_pieces[index] = ChessPiece {
                    piece: PossibleChessPiece::Queen,
                    color: Color::Black,
                }
            }
            'k' => {
                chess_pieces[index] = ChessPiece {
                    piece: PossibleChessPiece::King,
                    color: Color::Black,
                }
            }
            'P' => {
                chess_pieces[index] = ChessPiece {
                    piece: PossibleChessPiece::Pawn,
                    color: Color::White,
                }
            }
            'N' => {
                chess_pieces[index] = ChessPiece {
                    piece: PossibleChessPiece::Knight,
                    color: Color::White,
                }
            }
            'B' => {
                chess_pieces[index] = ChessPiece {
                    piece: PossibleChessPiece::Bishop,
                    color: Color::White,
                }
            }
            'R' => {
                chess_pieces[index] = ChessPiece {
                    piece: PossibleChessPiece::Rook,
                    color: Color::White,
                }
            }
            'Q' => {
                chess_pieces[index] = ChessPiece {
                    piece: PossibleChessPiece::Queen,
                    color: Color::White,
                }
            }
            'K' => {
                chess_pieces[index] = ChessPiece {
                    piece: PossibleChessPiece::King,
                    color: Color::White,
                }
            }
            '1' => {
                chess_pieces[index] = ChessPiece {
                    piece: PossibleChessPiece::None,
                    color: Color::White,
                }
            }
            '2' => {
                for i in 0..2 {
                    chess_pieces[index + i] = ChessPiece {
                        piece: PossibleChessPiece::None,
                        color: Color::White,
                    }
                }
                index += 1;
            }
            '3' => {
                for i in 0..3 {
                    chess_pieces[index + i] = ChessPiece {
                        piece: PossibleChessPiece::None,
                        color: Color::White,
                    }
                }
                index += 2;
            }
            '4' => {
                for i in 0..4 {
                    chess_pieces[index + i] = ChessPiece {
                        piece: PossibleChessPiece::None,
                        color: Color::White,
                    }
                }
                index += 3;
            }
            '5' => {
                for i in 0..5 {
                    chess_pieces[index + i] = ChessPiece {
                        piece: PossibleChessPiece::None,
                        color: Color::White,
                    }
                }
                index += 4;
            }
            '6' => {
                for i in 0..6 {
                    chess_pieces[index + i] = ChessPiece {
                        piece: PossibleChessPiece::None,
                        color: Color::White,
                    }
                }
                index += 5;
            }
            '7' => {
                for i in 0..7 {
                    chess_pieces[index + i] = ChessPiece {
                        piece: PossibleChessPiece::None,
                        color: Color::White,
                    }
                }
                index += 6;
            }
            '8' => {
                for i in 0..8 {
                    chess_pieces[index + i] = ChessPiece {
                        piece: PossibleChessPiece::None,
                        color: Color::White,
                    }
                }
                index += 7;
            }
            other => {
                println!("The unexpected character = {other}");
                panic!("Unexpected character in FEN string")
            }
        }
        index += 1;
        if index == 64 {
            break;
        }
    }
    return chess_pieces;
}

fn print_piece(piece: &ChessPiece) {
    match piece.color {
        Color::White => print_white_piece(piece),
        Color::Black => print_black_piece(piece),
    }
}

fn print_white_piece(piece: &ChessPiece) {
    match piece.piece {
        PossibleChessPiece::None => print!("   "),
        PossibleChessPiece::Pawn => print!(" ♙ "),
        PossibleChessPiece::Knight => print!(" ♘ "),
        PossibleChessPiece::Bishop => print!(" ♗ "),
        PossibleChessPiece::Rook => print!(" ♖ "),
        PossibleChessPiece::Queen => print!(" ♕ "),
        PossibleChessPiece::King => print!(" ♔ "),
    }
}

fn print_black_piece(piece: &ChessPiece) {
    match piece.piece {
        PossibleChessPiece::None => print!("   "),
        PossibleChessPiece::Pawn => print!(" ♟︎ "),
        PossibleChessPiece::Knight => print!(" ♞ "),
        PossibleChessPiece::Bishop => print!(" ♝ "),
        PossibleChessPiece::Rook => print!(" ♜ "),
        PossibleChessPiece::Queen => print!(" ♛ "),
        PossibleChessPiece::King => print!(" ♚ "),
    }
}
