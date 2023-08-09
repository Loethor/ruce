//! Module containing chess piece related logic and structures.
use crate::board::moves::Move;
use crate::board::Board;

use self::king::generate_king_moves;
use self::knight::generate_knight_moves;
use self::pawn::generate_pawn_moves;
use self::sliding_pieces::generate_sliding_moves;

pub mod king;
pub mod knight;
pub mod pawn;
mod sliding_pieces;

/// Represents a chess piece, containing its type and color.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

    pub fn generate_moves(&self, board: &Board, row: u8, col: u8) -> Option<Vec<Move>> {
        match self.piece_type {
            PieceType::Pawn => generate_pawn_moves(board, row, col, self.color),
            PieceType::Bishop => generate_sliding_moves(*self, board, row, col),
            PieceType::Knight => generate_knight_moves(board, row, col),
            PieceType::Rook => generate_sliding_moves(*self, board, row, col),
            PieceType::Queen => generate_sliding_moves(*self, board, row, col),
            PieceType::King => generate_king_moves(board, row, col),
        }
    }
}

/// Represents the type of a chess piece.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

/// Represents the color of a chess piece.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}
