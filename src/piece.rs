#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
}

impl Piece {
    pub fn to_char(&self) -> char {
        match (self.piece_type, self.color) {
            (PieceType::Pawn, PieceColor::White) => 'P',
            (PieceType::Pawn, PieceColor::Black) => 'p',
            (PieceType::Bishop, PieceColor::White) => 'B',
            (PieceType::Bishop, PieceColor::Black) => 'b',
            (PieceType::Knight, PieceColor::White) => 'N',
            (PieceType::Knight, PieceColor::Black) => 'n',
            (PieceType::Rook, PieceColor::White) => 'R',
            (PieceType::Rook, PieceColor::Black) => 'r',
            (PieceType::Queen, PieceColor::White) => 'Q',
            (PieceType::Queen, PieceColor::Black) => 'q',
            (PieceType::King, PieceColor::White) => 'K',
            (PieceType::King, PieceColor::Black) => 'k',
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

#[derive(Debug, Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}
