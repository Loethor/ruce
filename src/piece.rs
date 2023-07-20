

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub piece_type: Option<PieceType>,
    pub color: Option<PieceColor>
}

impl Piece {

    pub fn to_char(&self) -> char {
        match (self.piece_type, self.color) {
            (Some(PieceType::Pawn), Some(PieceColor::White)) => 'P',
            (Some(PieceType::Pawn), Some(PieceColor::Black)) => 'p',
            (Some(PieceType::Bishop), Some(PieceColor::White)) => 'B',
            (Some(PieceType::Bishop), Some(PieceColor::Black)) => 'b',
            (Some(PieceType::Knight), Some(PieceColor::White)) => 'N',
            (Some(PieceType::Knight), Some(PieceColor::Black)) => 'n',
            (Some(PieceType::Rook), Some(PieceColor::White)) => 'R',
            (Some(PieceType::Rook), Some(PieceColor::Black)) => 'r',
            (Some(PieceType::Queen), Some(PieceColor::White)) => 'Q',
            (Some(PieceType::Queen), Some(PieceColor::Black)) => 'q',
            (Some(PieceType::King), Some(PieceColor::White)) => 'K',
            (Some(PieceType::King), Some(PieceColor::Black)) => 'k',
            _ => '0',
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PieceType{
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