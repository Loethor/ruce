pub mod moves;
pub mod piece;

use std::collections::HashMap;

use crate::board::moves::Move;
use crate::board::piece::{Color, Piece, PieceType};

use crate::board::piece::knight::{generate_knight_moves, precalculate_knight_moves};

pub const BOARD_SIZE: usize = 8;

pub struct Board {
    pub squares: Vec<Option<Piece>>,
    pub knight_moves_map: HashMap<u8, Vec<u8>>,
}

impl Board {
    pub fn new_empty_board() -> Self {
        let squares = vec![None; 64]; // Initialize the board with empty squares
        Board {
            squares,
            knight_moves_map: precalculate_knight_moves(),
        }
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
                    if piece.color != current_player {
                        continue;
                    }

                    let piece_moves = match piece.piece_type {
                        PieceType::Pawn => self.generate_pawn_moves(row, col, piece.color),
                        PieceType::Bishop => self.generate_bishop_moves(row, col),
                        PieceType::Knight => {
                            generate_knight_moves(&self, row, col, &self.knight_moves_map)
                        }
                        PieceType::Rook => self.generate_rook_moves(row, col),
                        PieceType::Queen => self.generate_queen_moves(row, col),
                        PieceType::King => self.generate_king_moves(row, col),
                    };
                    // Add it to the list if there is a move
                    if let Some(valid_move) = piece_moves {
                        moves.extend(valid_move);
                    }
                }
            }
        }
        println!("There are {} moves in this position.", moves.len());
        moves
    }

    fn generate_pawn_moves(&self, row: usize, col: usize, piece_color: Color) -> Option<Vec<Move>> {
        // TODO create pawn.rs and move it there?

        let mut moves: Vec<Move> = Vec::new();

        // Calculate the direction based on the piece color
        let direction = match piece_color {
            Color::White => 1,  // Moving up (increasing row index)
            Color::Black => -1, // Moving down (decreasing row index)
        };

        // Calculate the initial square number
        let initial_square = (row * BOARD_SIZE + col) as u8;

        // Calculate the destination row for the pawn's move
        let new_row = (row as isize + direction) as usize;

        // Ensure that the destination row is within bounds
        if new_row >= BOARD_SIZE {
            return None; // Invalid move, the pawn is off the board
        }

        // One square movement
        // Calculate the target square number
        let target_square = (new_row * BOARD_SIZE + col) as u8;

        let mut is_one_move_allowed = false;
        // Check if the destination square is empty
        if self.get_piece(target_square.into()).is_none() {
            // The destination square is empty, so it's a valid non-capturing move
            is_one_move_allowed = true;
            moves.push(Move {
                initial_square,
                target_square,
            });
        }

        // Two square movement
        // Check if it's the pawn's first move and if the two-square move is available
        if (row == 1 && piece_color == Color::White) || (row == 6 && piece_color == Color::Black) {
            // Calculate the target square number for the two-square move
            let two_square_target =
                ((row as isize + 2 * direction) * BOARD_SIZE as isize + col as isize) as u8;

            // Check if the two-square move destination is empty and also the one-square move was empty
            if self.get_piece(two_square_target.into()).is_none() && is_one_move_allowed {
                // The two-square move is available, add it to the list of valid moves
                moves.push(Move {
                    initial_square,
                    target_square: two_square_target,
                });
            }
        }

        // Captures
        let left_new_col = col as isize - 1;
        let right_new_col = col as isize + 1;

        // Check if there's a capture on the left diagonal
        if left_new_col >= 0 {
            let left_diagonal_target =
                ((row as isize + direction) * BOARD_SIZE as isize + left_new_col) as u8;
            if let Some(piece) = self.get_piece(left_diagonal_target.into()) {
                if piece.color != piece_color {
                    // The left diagonal has an opponent's piece, so it's a valid capturing move
                    moves.push(Move {
                        initial_square,
                        target_square: left_diagonal_target,
                    });
                }
            }
        }

        // Check if there's a capture on the right diagonal
        if right_new_col < BOARD_SIZE as isize {
            let right_diagonal_target =
                ((row as isize + direction) * BOARD_SIZE as isize + right_new_col) as u8;
            if let Some(piece) = self.get_piece(right_diagonal_target.into()) {
                if piece.color != piece_color {
                    // The right diagonal has an opponent's piece, so it's a valid capturing move
                    moves.push(Move {
                        initial_square,
                        target_square: right_diagonal_target,
                    });
                }
            }
        }

        // TODO: Implement en passant
        // TODO: Implement promotion

        if !moves.is_empty() {
            return Some(moves);
        }
        None
    }

    fn generate_bishop_moves(&self, row: usize, col: usize) -> Option<Vec<Move>> {
        // TODO
        None
    }

    fn generate_knight_moves(&self, row: usize, col: usize) -> Option<Vec<Move>> {
        // TODO
        None
    }

    fn generate_rook_moves(&self, row: usize, col: usize) -> Option<Vec<Move>> {
        // TODO
        None
    }

    fn generate_queen_moves(&self, row: usize, col: usize) -> Option<Vec<Move>> {
        // TODO
        None
    }

    fn generate_king_moves(&self, row: usize, col: usize) -> Option<Vec<Move>> {
        // TODO
        None
    }

    pub fn print_board(&self) {
        println!("  +------------------------+");
        for row in (0..BOARD_SIZE).rev() {
            print!("{} |", row + 1);

            for col in 0..BOARD_SIZE {
                let square = row * BOARD_SIZE + col;
                if let Some(piece) = self.get_piece(square) {
                    let piece_char = piece.piece_as_char();
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
    fn create_test_board_with_piece(
        piece_type: PieceType,
        piece_color: Color,
        row: usize,
        col: usize,
    ) -> Board {
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
        let piece = Piece {
            piece_type: Pawn,
            color: Color::White,
        };
        board.set_piece(square as u8, Some(piece));

        let moves = board.generate_pawn_moves(1, 3, piece.color).unwrap();
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
        let piece = Piece {
            piece_type: Pawn,
            color: Color::Black,
        };
        board.set_piece(square as u8, Some(piece));

        let moves = board.generate_pawn_moves(6, 3, piece.color).unwrap();
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
        let piece = Piece {
            piece_type: Pawn,
            color: Color::Black,
        };
        board.set_piece(square as u8, Some(piece));

        // piece blocking
        let row = 5;
        let col = 3;
        let square = row * BOARD_SIZE + col;
        let blocking_piece = Piece {
            piece_type: Pawn,
            color: Color::Black,
        };
        board.set_piece(square as u8, Some(blocking_piece));

        let moves = board.generate_pawn_moves(6, 3, piece.color);
        assert!(moves.is_none());
    }

    #[test]
    fn test_generate_pawn_moves_captures() {
        let mut board = Board::new_empty_board();
        let row = 6;
        let col = 3;
        let square = row * BOARD_SIZE + col;
        let piece = Piece {
            piece_type: Pawn,
            color: Color::Black,
        };
        board.set_piece(square as u8, Some(piece));

        // capturable piece
        let row = 5;
        let col = 2;
        let square = row * BOARD_SIZE + col;
        let capturable_piece = Piece {
            piece_type: Pawn,
            color: Color::White,
        };
        board.set_piece(square as u8, Some(capturable_piece));

        let moves = board.generate_pawn_moves(6, 3, piece.color).unwrap();
        assert_eq!(moves.len(), 3);

        // Check that the pawn can move one square forward
        assert!(moves.contains(&Move {
            initial_square: 51,
            target_square: 43,
        }));
    }

    // TODO add test cases for en passant, and promotion
    // End Pawn tests

    #[test]
    fn test_generate_knight_moves_in_center() {
        let mut board = Board::new_empty_board();
        let row = 4;
        let col = 4;
        let square = row * BOARD_SIZE + col;
        let piece = Piece {
            piece_type: PieceType::Knight,
            color: Color::Black,
        };
        board.set_piece(square as u8, Some(piece));

        let moves = board.generate_moves(Color::Black);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn test_generate_knight_moves_in_corner() {
        let mut board = Board::new_empty_board();
        let row = 0;
        let col = 0;
        let square = row * BOARD_SIZE + col;
        let piece = Piece {
            piece_type: PieceType::Knight,
            color: Color::Black,
        };
        board.set_piece(square as u8, Some(piece));

        let moves = board.generate_moves(Color::Black);
        assert_eq!(moves.len(), 2);
    }
}
