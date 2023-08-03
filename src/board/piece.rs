use crate::board::moves::Move;
use crate::board::Board;

use self::knight::generate_knight_moves;
use self::pawn::generate_pawn_moves;

pub mod knight;
pub mod pawn;

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn as_char(&self) -> char {
        match (self.piece_type, self.color) {
            (PieceType::Pawn, Color::White) => 'P',
            (PieceType::Pawn, Color::Black) => 'p',
            (PieceType::Bishop, Color::White) => 'B',
            (PieceType::Bishop, Color::Black) => 'b',
            (PieceType::Knight, Color::White) => 'N',
            (PieceType::Knight, Color::Black) => 'n',
            (PieceType::Rook, Color::White) => 'R',
            (PieceType::Rook, Color::Black) => 'r',
            (PieceType::Queen, Color::White) => 'Q',
            (PieceType::Queen, Color::Black) => 'q',
            (PieceType::King, Color::White) => 'K',
            (PieceType::King, Color::Black) => 'k',
        }
    }

    pub fn generate_moves(&self, board: &Board, row: usize, col: usize) -> Option<Vec<Move>> {
        match self.piece_type {
            PieceType::Pawn => generate_pawn_moves(board, row, col, self.color),
            PieceType::Bishop => generate_knight_moves(board, row, col),
            PieceType::Knight => None,
            PieceType::Rook => None,
            PieceType::Queen => None,
            PieceType::King => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}
