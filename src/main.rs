use game_state::GameState;
use std::str::FromStr;

mod board;
mod game_state;

fn main() {
    let game_state =
        GameState::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    if let Err(e) = game_state {
        println!("Error: {}", e);
        return;
    }
    let game_state = game_state.unwrap();
    game_state.board.print_board();
    let moves = game_state.generate_moves();
}
