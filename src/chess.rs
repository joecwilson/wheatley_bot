use std::fmt;
const ROWS:usize = 8;
const COLS:usize = ROWS;
const AREA:usize = ROWS * COLS;

enum PossibleChessPieces {
    Empty,
    Pawn,
    Knight,
    Bishiop,
    Rook,
    Queen,
    King,
}

enum OwnerColor {
    Unowned,
    White,
    Black,
}

struct ChessPiece {
    piece: PossibleChessPieces,
    color: OwnerColor,
    hasMoved: bool,
}

pub struct ChessBoard {
    board: [ChessPiece; AREA],
}

impl ChessBoard {
    fn getPiece(&self, row: usize, col: usize) -> Option<&ChessPiece>{
        let index = row * COLS + col;
        let full = &self.board[index];
        let piece = &full.piece;
        match piece{
            PossibleChessPieces::Empty => {
                return None;
            }
            _ => {
                return Some(full);
            }
        }
    }

    pub fn new() -> Self{
        Self {
            board: [
                ChessPiece {
                    piece: PossibleChessPieces::Rook,
                    color: OwnerColor::Black,
                    hasMoved: false,
                },
                ChessPiece {
                    piece: PossibleChessPieces::Knight,
                    color: OwnerColor::Black,
                    hasMoved: false,
                },
                ChessPiece {
                    piece: PossibleChessPieces::Bishiop,
                    color: OwnerColor::Black,
                    hasMoved: false,
                },
                ChessPiece {
                    piece: PossibleChessPieces::Queen,
                    color: OwnerColor::Black,
                    hasMoved: false,
                },
                ChessPiece {
                    piece: PossibleChessPieces::King,
                    color: OwnerColor::Black,
                    hasMoved: false,
                },
                ChessPiece {
                    piece: PossibleChessPieces::Bishiop,
                    color: OwnerColor::Black,
                    hasMoved: false,
                },
                ChessPiece {
                    piece: PossibleChessPieces::Knight,
                    color: OwnerColor::Black,
                    hasMoved: false,
                },
                ChessPiece {
                    piece: PossibleChessPieces::Rook,
                    color: OwnerColor::Black,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::Black,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::Black,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::Black,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::Black,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::Black,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::Black,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::Black,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::Black,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::White,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::White,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::White,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::White,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::White,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::White,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::White,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::White,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Rook,
                    color: OwnerColor::White,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Knight,
                    color: OwnerColor::White,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Bishiop,
                    color: OwnerColor::White,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Queen,
                    color: OwnerColor::White,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::King,
                    color: OwnerColor::White,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Bishiop,
                    color: OwnerColor::White,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Knight,
                    color: OwnerColor::White,
                    hasMoved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Rook,
                    color: OwnerColor::White,
                    hasMoved: false,
                },
            ],
        }
    }

    

}

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       for row in 0..ROWS {
        for  col in 0..COLS{
            let index = row * COLS + col;
            self.board[index].fmt(f);
        }
        writeln!(f,"");
       }
       write!(f,"")
    }
    
}

impl fmt::Display for ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.color {
            OwnerColor::White => {
                match self.piece {
                    PossibleChessPieces::Bishiop => {write!(f, "♗")}
                    PossibleChessPieces::King => {write!(f,"♔")}
                    PossibleChessPieces::Knight => {write!(f,"♘")}
                    PossibleChessPieces::Pawn => {write!(f,"♙")}
                    PossibleChessPieces::Queen => {write!(f,"♕")}
                    PossibleChessPieces::Rook => {write!(f,"♖")}
                    PossibleChessPieces::Empty => {write!(f," ")}
                }
            }
            OwnerColor::Black => {
                match self.piece {
                    PossibleChessPieces::Bishiop => {write!(f, "♝")}
                    PossibleChessPieces::King => {write!(f,"♚")}
                    PossibleChessPieces::Knight => {write!(f,"♞")}
                    PossibleChessPieces::Pawn => {write!(f,"♟︎")}
                    PossibleChessPieces::Queen => {write!(f,"♛")}
                    PossibleChessPieces::Rook => {write!(f,"♜")}
                    PossibleChessPieces::Empty => {write!(f," ")}
            }}
            OwnerColor::Unowned => {write!(f," ")}    
        }
    }
}