use crate::piece::{Piece, PieceColor, PieceType};

// Function to parse FEN and populate the board
// Function to parse FEN and populate the board
pub fn fen_to_board(fen: &str) -> Vec<Option<Piece>> {
    let mut board: Vec<Option<Piece>> = vec![None; 64];
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
                let piece_type = match c {
                    'p' => Some(PieceType::Pawn),
                    'b' => Some(PieceType::Bishop),
                    'n' => Some(PieceType::Knight),
                    'r' => Some(PieceType::Rook),
                    'q' => Some(PieceType::Queen),
                    'k' => Some(PieceType::King),
                    _ => None,
                };
                board[rank * 8 + file] = Some(Piece {
                    piece_type,
                    color: Some(PieceColor::Black),
                });
                file += 1;
            }
            'A'..='Z' => {
                let piece_type = match c {
                    'P' => Some(PieceType::Pawn),
                    'B' => Some(PieceType::Bishop),
                    'N' => Some(PieceType::Knight),
                    'R' => Some(PieceType::Rook),
                    'Q' => Some(PieceType::Queen),
                    'K' => Some(PieceType::King),
                    _ => None,
                };
                board[rank * 8 + file] = Some(Piece {
                    piece_type,
                    color: Some(PieceColor::White),
                });
                file += 1;
            }
            _ => break,
        }
    }

    board
}
