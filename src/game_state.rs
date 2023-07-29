use crate::board::Board;
use crate::piece::Color;
use crate::moves::Move;


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
    