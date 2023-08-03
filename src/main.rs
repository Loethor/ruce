use std::str::FromStr;
use game_state::GameState;

mod game_state;
mod board;

fn main() {
    let game_state = GameState::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    game_state.board.print_board();
    let moves = game_state.generate_moves();
}
