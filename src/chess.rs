use std::fmt;
const ROWS:usize = 8;
const COLS:usize = ROWS;
const AREA:usize = ROWS * COLS;

#[derive(Debug, Copy, Clone)]
enum PossibleChessPieces {
    Empty,
    Pawn,
    Knight,
    Bishiop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone)]

enum OwnerColor {
    Unowned,
    White,
    Black,
}

#[derive(Debug, Copy, Clone)]
struct ChessPiece {
    piece: PossibleChessPieces,
    color: OwnerColor,
    has_moved: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct ChessBoard {
    board: [ChessPiece; AREA],
}

impl ChessBoard {
    fn get_piece(&self, row: usize, col: usize) -> Option<&ChessPiece>{
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

    fn get_mut_piece(& mut self, row: usize, col: usize) -> Option<& mut ChessPiece>{
        let index = row * COLS + col;
        let full = & mut self.board[index];
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

    /// Moves the piece at the start and end colum to the place in end_row and end_col
    /// If the move is not a legal move returns false, and does not update the board
    fn move_piece(& mut self, start_row: usize, start_col:usize, end_row:usize, end_col:usize)->bool{
        // Lets just move right now.
        // We need to have a copy of the piece in the end position
        let start_index = start_row * COLS + start_col;
        let mut start_piece = self.board[start_index];
        let end_index = end_row * COLS + end_col;
        let end_piece = self.board[end_index];
        self.board[end_index] = start_piece;
        if !self.check_legality(start_row, start_col, end_row, end_col){
            self.board[end_index] = end_piece;
            return false;
        }
        start_piece.has_moved = true;
        return true;
    }

    /// Checks to see if moving the piece would be legal or not. 
    /// Returns true if the move is legal, false otherwise.
    fn check_legality(&self, start_row: usize, start_col:usize, end_row:usize, end_col:usize)->bool {
        if (self.in_check()){
            return false;
        }
        return true;
    }

    /// Returns true if a king is in check, false otherwise
    fn in_check(&self)->bool{
        return false;
    }

    pub fn new() -> Self{
        Self {
            board: [
                ChessPiece {
                    piece: PossibleChessPieces::Rook,
                    color: OwnerColor::Black,
                    has_moved: false,
                },
                ChessPiece {
                    piece: PossibleChessPieces::Knight,
                    color: OwnerColor::Black,
                    has_moved: false,
                },
                ChessPiece {
                    piece: PossibleChessPieces::Bishiop,
                    color: OwnerColor::Black,
                    has_moved: false,
                },
                ChessPiece {
                    piece: PossibleChessPieces::Queen,
                    color: OwnerColor::Black,
                    has_moved: false,
                },
                ChessPiece {
                    piece: PossibleChessPieces::King,
                    color: OwnerColor::Black,
                    has_moved: false,
                },
                ChessPiece {
                    piece: PossibleChessPieces::Bishiop,
                    color: OwnerColor::Black,
                    has_moved: false,
                },
                ChessPiece {
                    piece: PossibleChessPieces::Knight,
                    color: OwnerColor::Black,
                    has_moved: false,
                },
                ChessPiece {
                    piece: PossibleChessPieces::Rook,
                    color: OwnerColor::Black,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::Black,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::Black,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::Black,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::Black,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::Black,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::Black,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::Black,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::Black,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Empty,
                    color: OwnerColor::Unowned,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::White,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::White,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::White,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::White,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::White,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::White,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::White,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Pawn,
                    color: OwnerColor::White,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Rook,
                    color: OwnerColor::White,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Knight,
                    color: OwnerColor::White,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Bishiop,
                    color: OwnerColor::White,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Queen,
                    color: OwnerColor::White,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::King,
                    color: OwnerColor::White,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Bishiop,
                    color: OwnerColor::White,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Knight,
                    color: OwnerColor::White,
                    has_moved: false,
                },
                ChessPiece{
                    piece: PossibleChessPieces::Rook,
                    color: OwnerColor::White,
                    has_moved: false,
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