use crate::piece::Piece;
use crate::moves::Move;


pub struct Board{
    pub squares: Vec<Option<Piece>>
}

impl Board {
    
    pub fn generate_moves() -> Vec<Move>{
        let moves:Vec<Move> = Vec::new();
        // for loop all squares in board
        // get piece
        // calculate moves
        // add them into the list
        moves
    }
}