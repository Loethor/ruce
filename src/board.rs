//! Module containing chess board related logic and structures.

mod board_tests;
pub mod moves;
pub mod piece;

use std::collections::HashMap;

use crate::board::moves::Move;
use crate::board::piece::{Color, Piece, PieceType};
use crate::game_state::ParseFenError;
use std::str::FromStr;

use self::piece::knight::precalculate_knight_moves;

/// Represents the size of the chess board (number of rows and columns).
pub const BOARD_SIZE: u8 = 8;

/// Represents the chess board, containing squares with optional pieces.
#[derive(PartialEq, Eq, Debug)]
pub struct Board {
    pub squares: Vec<Option<Piece>>,
    pub knight_moves_map: HashMap<u8, Vec<u8>>,
}

impl Board {
    /// Creates a new empty chess board.
    ///
    /// This function initializes the board with empty squares, represented by `None`, and
    /// precalculates the knight moves for each square using the `precalculate_knight_moves`
    /// function. The knight moves are stored in a `HashMap` named `knight_moves_map`, where
    /// each square's index (0 to 63) maps to a vector containing the indices of the squares
    /// that a knight can move to from that square.
    ///
    /// # Example
    ///
    /// ```
    /// use chess_engine::board::Board;
    ///
    /// let board = Board::new_empty_board();
    /// ```
    ///
    /// # Returns
    ///
    /// A new `Board` struct representing an empty chess board.
    pub fn new_empty_board() -> Self {
        let squares = vec![None; 64]; // Initialize the board with empty squares
        Board {
            squares,
            knight_moves_map: precalculate_knight_moves(),
        }
    }

    /// Retrieves the piece located at the specified square on the board.
    ///
    /// # Arguments
    ///
    /// * `square`: The index of the square to retrieve the piece from.
    ///             It is represented as a `u8` value, where the index starts from 0 (top-left square)
    ///             and increases sequentially from left to right and top to bottom.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing a reference to the piece at the specified square.
    /// - If the square is empty, it returns `None`.
    /// - If the square contains a piece, it returns `Some(&Piece)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_engine::board::{Board, Color, Piece, PieceType};
    ///
    /// let mut board = Board::new_empty_board();
    /// let piece = Piece {
    ///     piece_type: PieceType::Pawn,
    ///     color: Color::White,
    /// };
    /// board.set_piece(8, Some(piece)); // Set a pawn piece at square 8
    ///
    /// // Retrieve the piece at square 8
    /// let retrieved_piece = board.get_piece(8);
    /// assert_eq!(retrieved_piece, Some(&Piece {
    ///     piece_type: PieceType::Pawn,
    ///     color: Color::White,
    /// }));
    /// ```
    pub fn get_piece(&self, square: u8) -> Option<&Piece> {
        self.squares[square as usize].as_ref()
    }

    /// Sets a piece at a specific square on the board.
    ///
    /// # Arguments
    ///
    /// * `square` - The index of the square (0 to 63) where the piece will be set.
    /// * `piece` - The piece to be placed at the specified square.
    pub fn set_piece(&mut self, square: u8, piece: Piece) {
        self.squares[square as usize] = Some(piece);
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

    /// Prints the current state of the chess board.
    ///
    /// This function will print the chess board, displaying each piece's symbol at its respective square.
    /// An empty square is represented by a dot (".") character. The board is printed with row numbers and column labels.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_engine::board::{Board, Color, Piece, PieceType};
    ///
    /// let mut board = Board::new_empty_board();
    /// let piece = Piece {
    ///     piece_type: PieceType::Pawn,
    ///     color: Color::White,
    /// };
    /// board.set_piece(8, Some(piece)); // Set a pawn piece at square 8
    ///
    /// board.print_board();
    /// // The output should be:
    /// //   +------------------------+
    /// // 8 | .  .  .  .  .  .  .  . |
    /// // 7 | .  .  .  .  .  .  .  . |
    /// // 6 | .  .  .  .  .  .  .  . |
    /// // 5 | .  .  .  .  .  .  .  . |
    /// // 4 | .  .  .  .  .  .  .  . |
    /// // 3 | .  .  .  .  .  .  .  . |
    /// // 2 | .  .  .  .  .  .  .  . |
    /// // 1 | P  .  .  .  .  .  .  . |
    /// //   +------------------------+
    /// //     a  b  c  d  e  f  g  h
    /// ```
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

// Errors are raised if the FEN string is invalid
// i.e., different that pbnrqk
fn char_to_piece(piece: &str, color: Color) -> Result<Piece, ParseFenError> {
    match piece {
        "p" => Ok(Piece {
            piece_type: PieceType::Pawn,
            color,
        }),
        "b" => Ok(Piece {
            piece_type: PieceType::Bishop,
            color,
        }),
        "n" => Ok(Piece {
            piece_type: PieceType::Knight,
            color,
        }),
        "r" => Ok(Piece {
            piece_type: PieceType::Rook,
            color,
        }),
        "q" => Ok(Piece {
            piece_type: PieceType::Queen,
            color,
        }),
        "k" => Ok(Piece {
            piece_type: PieceType::King,
            color,
        }),
        _ => Err(ParseFenError::InvalidPiecePlacement(piece.to_string())),
    }
}

impl FromStr for Board {
    type Err = ParseFenError;

    /// Errors are raised if the FEN string is invalid
    /// char_to_piece is responsible for raising the error
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::board::Board;
    /// use std::str::FromStr;
    ///
    /// let board = Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
    /// ```
    fn from_str(piece_placement: &str) -> Result<Self, Self::Err> {
        let mut board = Board::new_empty_board();

        let mut rank: u8 = 7;
        let mut file: u8 = 0;

        for c in piece_placement.chars() {
            match c {
                '0'..='8' => {
                    let empty_squares: u8 = c.to_digit(10).unwrap() as u8;
                    file += empty_squares;
                }
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                'a'..='z' => {
                    let new_piece = char_to_piece(&c.to_lowercase().to_string(), Color::Black)?;
                    board.set_piece(rank * BOARD_SIZE + file, new_piece);
                    file += 1;
                }
                'A'..='Z' => {
                    let new_piece = char_to_piece(&c.to_lowercase().to_string(), Color::White)?;
                    board.set_piece(rank * BOARD_SIZE + file, new_piece);
                    file += 1;
                }
                _ => break,
            }
        }
        Ok(board)
    }
}
