use std::str::FromStr;
use thiserror::Error;

use crate::board::Board;
use crate::board::piece::Color;
use crate::board::moves::Move;

pub struct GameState {
    pub board: Board,
    pub current_player: Color,
    pub turn: u32,
}

impl GameState {
    pub fn generate_moves(&self) -> Vec<Move>{
        self.board.generate_moves(self.current_player)
    }
    
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseFenError{
    #[error("Invalid FEN: {0}")]
    InvalidPiecePlacement(String),
}
    

// The FEN string is parsed into a GameState struct
// The FEN string is a space-separated string of 6 fields
// currently only board is raising errors if FEN string is invalid
// TODO: add error handling for other fields
// TODO: Error type should be properly defined  
impl FromStr for GameState{
    type Err = ParseFenError;

    fn from_str(fen : &str) -> Result<Self, Self::Err> {

        let mut iter = fen.split_whitespace();

        // Parse the FEN string
        let piece_placement = iter.next().expect("Invalid FEN: missing piece placement");
        let active_color = iter.next().expect("Invalid FEN: missing active color");
        let _castling_availability = iter.next().expect("Invalid FEN: missing castling availability");
        let _en_passant_target = iter.next().expect("Invalid FEN: missing en passant target");
        let _half_move_clock = iter.next().expect("Invalid FEN: missing half-move clock");
        let full_move_number = iter.next().expect("Invalid FEN: missing full move number");

        let mut game_state: GameState = GameState {
            board: Board::from_str(piece_placement)?,
            current_player: Color::White,
            turn: 1,
        };

        // Parse the active color 
        match active_color {
            "b" => game_state.current_player = Color::Black,
            "w" => game_state.current_player = Color::White,
            _ => panic!("Invalid active color in FEN string: {}", active_color)
        }

        // Parse current turn
        game_state.turn = full_move_number.parse().expect("Invalid FEN: invalid full move number");

        // TODO missing other rules:
        // castling
        // en passant
        // half move clock


        Ok(game_state)
    }
}