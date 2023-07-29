mod piece;
mod fen;
mod game_state;
mod moves;
mod board;

fn main() {
    
    let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let game_state = fen::fen_to_board(fen_string);
    game_state.board.print_board();
    let moves = game_state.generate_moves();
}
