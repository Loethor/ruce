use crate::board::moves::Move;
use crate::board::piece::Color;
use crate::board::Board;

/// Represents the state of a chess game, including the chess board, the current player, and the turn number.
pub struct GameState {
    /// The chess board containing the arrangement of pieces.
    pub board: Board,
    /// The color of the current player (either `Color::White` or `Color::Black`).
    pub current_player: Color,
    /// The current turn number of the game.
    pub turn: u32,
}

impl GameState {
    /// Generates all possible moves for the pieces of the current player.
    ///
    /// This method calls the `generate_moves` method of the `Board` to calculate all possible moves
    /// for the pieces of the current player on the chess board.
    ///
    /// # Returns
    ///
    /// A vector containing all valid moves for the pieces of the current player.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess_engine::board::piece::Color;
    /// use chess_engine::game_state::GameState;
    ///
    /// // Assume `game_state` is initialized with the chess board and the current player.
    /// let moves = game_state.generate_moves();
    /// // Now `moves` contains all valid moves for the current player's pieces on the board.
    /// ```
    pub fn generate_moves(&self) -> Vec<Move> {
        // Delegates the move generation to the `Board` struct's `generate_moves` method.
        self.board.generate_moves(self.current_player)
    }
}
