pub mod moves;
pub mod piece;

use crate::board;
use crate::board::piece::{Piece, PieceType, Color};
use crate::board::moves::Move;


pub const BOARD_SIZE:usize = 8;
pub struct Board {
    pub squares: Vec<Option<Piece>>,
}

impl Board {

    pub fn new_empty_board() -> Self {
        let squares = vec![None; 64]; // Initialize the board with empty squares
        Board { squares }
    }

    // Method to get a piece at a specific square on the board
    pub fn get_piece(&self, square: usize) -> Option<&Piece> {
        self.squares[square].as_ref()
    }

    pub fn set_piece(&mut self, square: u8, piece: Option<Piece>) {
        self.squares[square as usize] = piece;
    }

    pub fn generate_moves(&self, current_player: Color) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let square = row * BOARD_SIZE + col;
                if let Some(piece) = self.get_piece(square) {

                    // Only check moves of the pieces that match the current player
                    if piece.color != current_player{continue;}
                    let piece_moves = piece.generate_moves(self, row, col);
                    
                    // Add it to the list if there is a move
                    if let Some(valid_move) = piece_moves{
                        moves.extend(valid_move);
                    }
                }
            }
        }
        println!("There are {} moves in this position.", moves.len());
        moves
    }

    pub fn print_board(&self) {
        println!("  +------------------------+");
        for row in (0..BOARD_SIZE).rev() {
            print!("{} |", row + 1);

            for col in 0..BOARD_SIZE {
                let square = row * BOARD_SIZE + col;
                if let Some(piece) = self.get_piece(square) {
                    let piece_char = piece.to_char();
                    print!(" {} ", piece_char);
                } else {
                    print!(" . ");
                }
            }

            println!("|");
        }
        println!("  +------------------------+");
        println!("    a  b  c  d  e  f  g  h ");
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::piece::Piece;
    use crate::board::piece::PieceType::Pawn;

    // Helper function to create a test board with a given piece at a specific position
    fn create_test_board_with_piece(piece_type: PieceType, piece_color: Color, row: usize, col: usize) -> Board {
        let mut board = Board::new_empty_board();
        let piece = Piece {
            color: piece_color,
            piece_type,
        };
        let square_index = row * BOARD_SIZE + col;
        board.squares[square_index] = Some(piece);
        board
    }

    // Pawn tests
    #[test]
    fn test_generate_pawn_moves_white() {
        let mut board = Board::new_empty_board();
        let row = 1;
        let col = 3;
        let square = row * BOARD_SIZE + col;
        let piece = Piece {piece_type:Pawn, color:Color::White};
        board.set_piece(square as u8, Some(piece));
        let moves = piece.generate_moves(&board, 1, 3).unwrap();

        assert_eq!(moves.len(), 2);

        // Check that the pawn can move one square forward
        assert!(moves.contains(&Move {
            initial_square: 11,
            target_square: 19,
        }));

        // Check that the pawn can move two squares forward from its starting position
        assert!(moves.contains(&Move {
            initial_square: 11,
            target_square: 27,
        }));
    }

    #[test]
    fn test_generate_pawn_moves_black() {
        let mut board = Board::new_empty_board();
        let row = 6;
        let col = 3;
        let square = row * BOARD_SIZE + col;
        let piece = Piece {piece_type:Pawn, color:Color::Black};
        board.set_piece(square as u8, Some(piece));

        let moves = piece.generate_moves(&board, 6, 3).unwrap();

        assert_eq!(moves.len(), 2);

        // Check that the pawn can move one square forward
        assert!(moves.contains(&Move {
            initial_square: 51,
            target_square: 43,
        }));

        // Check that the pawn can move two squares forward from its starting position
        assert!(moves.contains(&Move {
            initial_square: 51,
            target_square: 35,
        }));
    }

    #[test]
    fn test_generate_pawn_moves_no_moves() {
        let mut board = Board::new_empty_board();
        let row = 6;
        let col = 3;
        let square = row * BOARD_SIZE + col;
        let piece = Piece {piece_type:Pawn, color:Color::Black};
        board.set_piece(square as u8, Some(piece));

        // piece blocking 
        let row = 5;
        let col = 3;
        let square = row * BOARD_SIZE + col;
        let blocking_piece = Piece {piece_type:Pawn, color:Color::Black};
        board.set_piece(square as u8, Some(blocking_piece));

        let moves = piece.generate_moves(&board, 6, 3);
        // let moves = board.generate_pawn_moves(6, 3, piece.color);
        assert!(moves.is_none()); 
    }

    #[test]
    fn test_generate_pawn_moves_captures() {

        let mut board = Board::new_empty_board();
        let row = 6;
        let col = 3;
        let square = row * BOARD_SIZE + col;
        let piece = Piece {piece_type:Pawn, color:Color::Black};
        board.set_piece(square as u8, Some(piece));

        // capturable piece 
        let row = 5;
        let col = 2;
        let square = row * BOARD_SIZE + col;
        let capturable_piece = Piece {piece_type:Pawn, color:Color::White};
        board.set_piece(square as u8, Some(capturable_piece));

        let moves = piece.generate_moves(&board, 6, 3).unwrap();
        assert_eq!(moves.len(), 3);

        // Check that the pawn can move one square forward
        assert!(moves.contains(&Move {
            initial_square: 51,
            target_square: 43,
        }));
    }

    // TODO add test cases for en passant, and promotion
    // End Pawn tests

}