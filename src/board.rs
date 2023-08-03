//! Module containing chess board related logic and structures.

/// Module containing chess move related logic and structures.
pub mod moves;
/// Module containing chess piece related logic and structures.
pub mod piece;

use std::collections::HashMap;

use crate::board::moves::Move;
use crate::board::piece::{Color, Piece};

use self::piece::knight::precalculate_knight_moves;

pub const BOARD_SIZE: u8 = 8;
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
    pub fn get_piece(&self, square: u8) -> Option<&Piece> {
        self.squares[square as usize].as_ref()
    }

    /// Sets a piece at a specific square on the board.
    ///
    /// # Arguments
    ///
    /// * `square` - The index of the square (0 to 63) where the piece will be set.
    /// * `piece` - The piece to be placed at the specified square. Use `None` to clear the square.
    pub fn set_piece(&mut self, square: u8, piece: Option<Piece>) {
        self.squares[square as usize] = piece;
    }

    /// Generates all possible moves for the pieces of the specified player.
    ///
    /// # Arguments
    ///
    /// * `current_player` - The color of the current player (either `Color::White` or `Color::Black`).
    ///
    /// # Returns
    ///
    /// A vector containing all valid moves for the pieces of the current player.
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
                    let piece_moves = piece.generate_moves(self, row, col);

                    // Add it to the list if there is a move
                    if let Some(valid_move) = piece_moves {
                        moves.extend(valid_move);
                    }
                }
            }
        }
        moves
    }

    pub fn print_board(&self) {
        println!("  +------------------------+");
        for row in (0..BOARD_SIZE).rev() {
            print!("{} |", row + 1);

            for col in 0..BOARD_SIZE {
                let square = row * BOARD_SIZE + col;
                if let Some(piece) = self.get_piece(square) {
                    let piece_char = piece.as_char();
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
