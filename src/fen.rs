use crate::piece::{Piece, PieceColor, PieceType};
use crate::board::{Board};


// Function to parse FEN and populate the board
// Function to parse FEN and populate the board
pub fn fen_to_board(fen: &str) -> Board {
    let mut board: Board = Board { squares: vec![None; 64]};
    let mut rank = 7;
    let mut file = 0;

    for c in fen.chars() {
        match c {
            '0'..='8' => {
                let empty_squares = c.to_digit(10).unwrap() as usize;
                file += empty_squares;
            }
            '/' => {
                rank -= 1;
                file = 0;
            }
            'a'..='z' => {
                let new_piece = match c {
                    'p' => Some(Piece { piece_type: PieceType::Pawn, color: PieceColor::Black}),
                    'b' => Some(Piece { piece_type: PieceType::Bishop, color: PieceColor::Black}),
                    'n' => Some(Piece { piece_type: PieceType::Knight, color: PieceColor::Black}),
                    'r' => Some(Piece { piece_type: PieceType::Rook, color: PieceColor::Black}),
                    'q' => Some(Piece { piece_type: PieceType::Queen, color: PieceColor::Black}),
                    'k' => Some(Piece { piece_type: PieceType::King, color: PieceColor::Black}),
                    _ => None,
                };
                board.squares[rank * 8 + file] = new_piece;
                file += 1;
            }
            'A'..='Z' => {
                let new_piece = match c {
                    'P' => Some(Piece { piece_type: PieceType::Pawn, color: PieceColor::White}),
                    'B' => Some(Piece { piece_type: PieceType::Bishop, color: PieceColor::White}),
                    'N' => Some(Piece { piece_type: PieceType::Knight, color: PieceColor::White}),
                    'R' => Some(Piece { piece_type: PieceType::Rook, color: PieceColor::White}),
                    'Q' => Some(Piece { piece_type: PieceType::Queen, color: PieceColor::White}),
                    'K' => Some(Piece { piece_type: PieceType::King, color: PieceColor::White}),
                    _ => None,
                };
                board.squares[rank * 8 + file] = new_piece;
                file += 1;
            }
            _ => break,
        }
    }

    board
}
