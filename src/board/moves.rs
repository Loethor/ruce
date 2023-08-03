/// Represents a chess move, containing the initial and target squares.
#[derive(PartialEq)]
pub struct Move {
    /// The index of the square (0 to 63) where the piece moves from.
    pub initial_square: u8,
    /// The index of the square (0 to 63) where the piece moves to.
    pub target_square: u8,
}
