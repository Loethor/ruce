pub mod knight;

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn to_char(&self) -> char {
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
