mod piece;
mod fen;

use fen::fen_to_board;

fn main() {
    
    let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = fen_to_board(fen_string);

    // Print the board in the standard orientation
    for rank in (0..8).rev() {
        for file in 0..8 {
            let piece = board[rank * 8 + file];
            match piece {
                Some(p) => print!(" {} ", p.to_char()),
                None => print!(" 0 "),
            }
        }
        println!();
    }
}
