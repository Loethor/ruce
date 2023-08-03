/// Represents a chess piece, containing its type and color.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    /// Converts the piece into a character representation.
    ///
    /// # Returns
    ///
    /// A character representing the piece according to the FEN standard.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_engine::piece::{Piece, PieceType, Color};
    ///
    /// let white_pawn = Piece { piece_type: PieceType::Pawn, color: Color::White };
    /// assert_eq!(white_pawn.to_char(), 'P');
    ///
    /// let black_knight = Piece { piece_type: PieceType::Knight, color: Color::Black };
    /// assert_eq!(black_knight.to_char(), 'n');
    /// ```
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
