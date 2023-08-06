//! Module containing knight related logic.

use std::collections::HashMap;

use crate::board::{moves::Move, Board, BOARD_SIZE};

/// Array of possible relative knight moves from a given square.
const KNIGHT_MOVES: [(isize, isize); 8] = [
    (-2, 1),
    (-2, -1),
    (-1, 2),
    (-1, -2),
    (1, 2),
    (1, -2),
    (2, 1),
    (2, -1),
];

/// Precalculate all knight moves for each square on the board.
///
/// This function precalculates all possible knight moves for each square on the chess board and
/// returns a hashmap that maps each square's index to a vector of indices representing the valid
/// squares the knight can move to from that square.
///
/// # Returns
///
/// A hashmap where the key is the index of the initial square, and the value is a vector containing
/// the indices of the target squares the knight can move to from the initial square.
///
/// # Example
///
/// ```
/// use chess_engine::board::precalculate_knight_moves;
///
/// let knight_moves_map = precalculate_knight_moves();
///
/// // Get the possible knight moves for square 'a1' (index 0)
/// let possible_moves_a1 = knight_moves_map[&0];
/// assert_eq!(possible_moves_a1, vec![10, 17]);
/// ```
pub fn precalculate_knight_moves() -> HashMap<u8, Vec<u8>> {
    let mut knight_moves_map: HashMap<u8, Vec<u8>> = HashMap::new();

    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            let square_index = row * BOARD_SIZE + col;
            let mut knight_moves: Vec<u8> = Vec::new();

            // Calculate all possible knight moves from the current square
            for &(dr, dc) in &KNIGHT_MOVES {
                let new_row = (row as isize + dr) as u8;
                let new_col = (col as isize + dc) as u8;

                // Check if the new coordinates are within the board boundaries
                if new_row < BOARD_SIZE && new_col < BOARD_SIZE {
                    let target_square = new_row * BOARD_SIZE + new_col;
                    knight_moves.push(target_square);
                }
            }

            knight_moves_map.insert(square_index, knight_moves);
        }
    }

    knight_moves_map
}

/// Generate knight moves based on precalculated moves.
///
/// Given a chess `board`, `row`, and `col`, this function calculates all valid knight moves
/// for the knight piece located at the specified position (`row`, `col`) on the board.
/// It uses the precalculated knight moves from the `board`'s `knight_moves_map` to efficiently
/// determine the valid target squares for the knight.
///
/// The function returns an `Option<Vec<Move>>`, where `Some(moves)` contains a vector of `Move`
/// structs representing the valid moves that the knight can make. If no valid moves are found,
/// `None` is returned.
///
/// # Arguments
///
/// * `board`: The chess board on which the knight moves will be calculated.
/// * `row`: The row coordinate (0-based) of the square where the knight is located.
/// * `col`: The column coordinate (0-based) of the square where the knight is located.
///
/// # Returns
///
/// An `Option<Vec<Move>>`, where `Some(moves)` contains the valid moves that the knight can make,
/// and `None` if no valid moves are found.
///
/// # Example
///
/// ```
/// use chess_engine::board::{Board, generate_knight_moves};
/// use chess_engine::board::piece::{Color, Piece, PieceType};
/// use chess_engine::board::moves::Move;
///
/// // Create an empty chess board
/// let mut board = Board::new_empty_board();
///
/// // Place a white knight on square 'd4' (index 27)
/// let knight = Piece {
///     piece_type: PieceType::Knight,
///     color: Color::White,
/// };
/// board.set_piece(27, Some(knight));
///
/// // Generate knight moves for the knight on 'd4'
/// let moves = generate_knight_moves(&board, 3, 3);
///
/// // The valid knight moves for 'd4' are 'b5', 'f5', 'c2', 'e2', 'b3', 'f3', 'c6', and 'e6'.
/// let expected_moves = vec![
///     Move { initial_square: 27, target_square: 17 },
///     Move { initial_square: 27, target_square: 37 },
///     Move { initial_square: 27, target_square: 12 },
///     Move { initial_square: 27, target_square: 42 },
///     Move { initial_square: 27, target_square: 19 },
///     Move { initial_square: 27, target_square: 39 },
///     Move { initial_square: 27, target_square: 14 },
///     Move { initial_square: 27, target_square: 44 },
/// ];
/// assert_eq!(moves.unwrap(), expected_moves);
/// ```
pub fn generate_knight_moves(board: &Board, row: u8, col: u8) -> Option<Vec<Move>> {
    let mut moves: Vec<Move> = Vec::new();
    let square: u8 = row * BOARD_SIZE + col;

    if let Some(possible_moves) = board.knight_moves_map.get(&square) {
        for &target_square in possible_moves {
            // Check if the target square is empty or occupied by an opponent's piece
            if board.get_piece(target_square).map_or(true, |piece| {
                piece.color != board.get_piece(square).unwrap().color
            }) {
                let initial_square = row * BOARD_SIZE + col;
                moves.push(Move {
                    initial_square,
                    target_square,
                });
            }
        }
    }

    if !moves.is_empty() {
        Some(moves)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Auxiliary function to sort a vector and return the sorted vector
    fn sort_vector(vec: &[u8]) -> Vec<u8> {
        let mut sorted_vec = vec.to_owned();
        sorted_vec.sort();
        sorted_vec
    }

    #[test]
    fn test_precalculate_knight_moves() {
        let knight_moves_map = precalculate_knight_moves();

        // Test some specific squares

        // Test for square 'a1'
        assert_eq!(
            sort_vector(&knight_moves_map[&0]),
            sort_vector(&vec![10, 17])
        );

        // Test for square 'd4'
        assert_eq!(
            sort_vector(&knight_moves_map[&27]),
            sort_vector(&vec![10, 17, 12, 21, 33, 37, 44, 42])
        );

        // Test for square 'h8'
        assert_eq!(
            sort_vector(&knight_moves_map[&63]),
            sort_vector(&vec![46, 53])
        );
    }

    #[test]
    fn test_knight_moves_valid_indices() {
        let knight_moves_map = precalculate_knight_moves();

        // Test all squares on the board to ensure the target squares are within valid indices.
        for (_square_index, knight_moves) in knight_moves_map.iter() {
            for target_square in knight_moves {
                assert!(
                    (0..64).contains(target_square),
                    "Invalid target square index: {}",
                    target_square
                );
            }
        }
    }
}
