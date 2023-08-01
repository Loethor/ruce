use super::piece::PieceType;
use super::piece::Piece;
use crate::board::Board;

#[derive(PartialEq, Debug)]
pub struct Move {
    pub initial_square:u8,
    pub target_square:u8,
}