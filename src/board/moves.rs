//! Module containing chess moves related logic and structures.

use std::collections::HashSet;
use std::iter::FromIterator;

/// Represents a move from an initial square to a target square on the chess board.
///
/// The `Move` struct is used to represent a valid move made by a chess piece. It contains
/// the indices of the initial square and the target square on the chess board. Each square
/// is represented by a numeric value between 0 and 63 (inclusive), where 0 corresponds to
/// the bottom-left square (a1) and 63 corresponds to the top-right square (h8) of the board.
///
/// # Example
///
/// ```
/// use chess_engine::Move;
///
/// let initial_square = 8; // a2 square
/// let target_square = 24; // a4 square
///
/// let chess_move = Move {
///     initial_square,
///     target_square,
/// };
///
/// assert_eq!(chess_move.initial_square, 8);
/// assert_eq!(chess_move.target_square, 24);
/// ```
#[derive(PartialEq, Debug, Eq, Hash, Clone)]
pub struct Move {
    /// The index of the initial square where the move starts.
    pub initial_square: u8,

    /// The index of the target square where the move ends.
    pub target_square: u8,
}

// Needed to compare two list of moves with different order of moves
// Allow dead code because test is not enough to realise it is being used
#[allow(dead_code)]
pub fn are_moves_equal(moves1: &[Move], moves2: &[Move]) -> bool {
    let set1: HashSet<_> = HashSet::from_iter(moves1.iter().cloned());
    let set2: HashSet<_> = HashSet::from_iter(moves2.iter().cloned());

    set1 == set2
}
