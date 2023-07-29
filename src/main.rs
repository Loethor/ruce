mod piece;
mod fen;
mod game_state;
mod moves;

fn main() {
    
    let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = fen::fen_to_board(fen_string);
    board.print_board();
    let moves = board.generate_moves();
}
