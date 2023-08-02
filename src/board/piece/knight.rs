use std::collections::HashMap;

use crate::board::{moves::Move, Board, BOARD_SIZE};

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

/// Precalculate all knight moves for each square on the board
pub fn precalculate_knight_moves() -> HashMap<u8, Vec<u8>> {
    let mut knight_moves_map: HashMap<u8, Vec<u8>> = HashMap::new();

    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            let square_index = (row * BOARD_SIZE + col) as u8;
            let mut knight_moves: Vec<u8> = Vec::new();

            // Calculate all possible knight moves from the current square
            for &(dr, dc) in &KNIGHT_MOVES {
                let new_row = (row as isize + dr) as usize;
                let new_col = (col as isize + dc) as usize;

                // Check if the new coordinates are within the board boundaries
                if new_row < BOARD_SIZE && new_col < BOARD_SIZE {
                    let target_square = (new_row * BOARD_SIZE + new_col) as u8;
                    knight_moves.push(target_square);
                }
            }

            knight_moves_map.insert(square_index, knight_moves);
        }
    }

    knight_moves_map
}

// Function to generate knight moves based on precalculated moves
pub fn generate_knight_moves(
    board: &Board,
    row: usize,
    col: usize,
    precalculated_moves: &HashMap<u8, Vec<u8>>,
) -> Option<Vec<Move>> {
    let mut moves: Vec<Move> = Vec::new();
    let square: u8 = (row * BOARD_SIZE + col) as u8;

    if let Some(possible_moves) = precalculated_moves.get(&square) {
        for &target_square in possible_moves {
            let target_row = (target_square / BOARD_SIZE as u8) as usize;
            let target_col = (target_square % BOARD_SIZE as u8) as usize;

            // Check if the target square is empty or occupied by an opponent's piece
            if board.get_piece(target_square.into()).map_or(true, |piece| {
                piece.color != board.get_piece(square.into()).unwrap().color
            }) {
                let initial_square = (row * BOARD_SIZE + col) as u8;
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
    fn sort_vector(vec: &Vec<u8>) -> Vec<u8> {
        let mut sorted_vec = vec.clone();
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
        for (square_index, knight_moves) in knight_moves_map.iter() {
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
