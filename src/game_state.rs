use std::str::FromStr;
use thiserror::Error;

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
    /// The current result of the game
    pub game_result: GameResult,
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

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseFenError {
    #[error("Invalid character(s) in FEN! First invalid character: {0}")]
    InvalidPiecePlacement(String),
}

impl FromStr for GameState {
    type Err = ParseFenError;

    /// The FEN string is parsed into a GameState struct
    /// The FEN string is a space-separated string of 6 fields
    /// currently only board is raising errors if FEN string is invalid
    /// TODO: add error handling for other fields
    /// TODO: Error type should be properly defined
    ///
    /// # Examples
    /// ```
    /// use chess_engine::game_state::GameState;
    /// use std::str::FromStr;
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let game_state = GameState::from_str(fen).unwrap();
    /// ```
    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        let mut iter = fen.split_whitespace();

        // Parse the FEN string
        let piece_placement = iter.next().expect("Invalid FEN: missing piece placement");
        let active_color = iter.next().expect("Invalid FEN: missing active color");
        let castling_availability = iter
            .next()
            .expect("Invalid FEN: missing castling availability");
        let _en_passant_target = iter.next().expect("Invalid FEN: missing en passant target");
        let _half_move_clock = iter.next().expect("Invalid FEN: missing half-move clock");
        let full_move_number = iter.next().expect("Invalid FEN: missing full move number");

        let mut game_state: GameState = GameState {
            board: Board::from_str(piece_placement)?,
            current_player: Color::White,
            turn: full_move_number.parse().unwrap(),
            game_result: GameResult::Undecided,
        };

        // Parse the active color
        match active_color {
            "b" => game_state.current_player = Color::Black,
            "w" => game_state.current_player = Color::White,
            _ => panic!("Invalid active color in FEN string: {}", active_color),
        }

        // Parse current turn
        game_state.turn = full_move_number
            .parse()
            .expect("Invalid FEN: invalid full move number");

        // castling availabilty
        game_state.board.castling_availability = parse_castling_availablity(castling_availability);

        // TODO missing other rules:
        // en passant
        // half move clock

        Ok(game_state)
    }
}

pub enum GameResult {
    WhiteWon,
    BlackWon,
    Draw,
    Undecided,
}

/// Parses a castling availability string and returns a tuple representing castling availability.
///
/// The castling availability string is a sequence of characters representing the availability of
/// castling for both white and black players. The characters used are:
/// - 'K': King-side castling for white.
/// - 'Q': Queen-side castling for white.
/// - 'k': King-side castling for black.
/// - 'q': Queen-side castling for black.
///
/// # Arguments
///
/// * `castling_availabilyt_str` - The castling availability string to parse.
///
/// # Returns
///
/// A tuple with four elements representing the castling availability:
/// * Element 0: Availability of King-side castling for white.
/// * Element 1: Availability of Queen-side castling for white.
/// * Element 2: Availability of King-side castling for black.
/// * Element 3: Availability of Queen-side castling for black.
///
/// # Examples
///
/// ```
/// let castling_str = "KQkq";
/// let castling_availability = parse_castling_availablity(castling_str);
/// assert_eq!(castling_availability, (true, true, true, true));
/// ```
///
/// ```
/// let castling_str = "Kq";
/// let castling_availability = parse_castling_availablity(castling_str);
/// assert_eq!(castling_availability, (true, false, false, true));
/// ```
fn parse_castling_availablity(castling_availabilyt_str: &str) -> (bool, bool, bool, bool) {
    let mut castling_availablity = (false, false, false, false);
    if castling_availabilyt_str.contains('K') {
        castling_availablity.0 = true;
    }
    if castling_availabilyt_str.contains('Q') {
        castling_availablity.1 = true;
    }
    if castling_availabilyt_str.contains('k') {
        castling_availablity.2 = true;
    }
    if castling_availabilyt_str.contains('q') {
        castling_availablity.3 = true;
    }
    castling_availablity
}
