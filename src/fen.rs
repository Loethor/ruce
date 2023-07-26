use crate::piece::{Piece, Color, PieceType};
use crate::game_state::{GameState};


// Function to parse FEN and populate the board
pub fn fen_to_board(fen: &str) -> GameState {
    let mut game_state: GameState = GameState {
        board: vec![None; 64],
        current_player: Color::White,
        turn: 1,
    };
    let mut rank = 7;
    let mut file = 0;

    let mut iter = fen.split_whitespace();

    // Parse the FEN string
    let piece_placement = iter.next().expect("Invalid FEN: missing piece placement");
    let active_color = iter.next().expect("Invalid FEN: missing active color");
    let _castling_availability = iter.next().expect("Invalid FEN: missing castling availability");
    let _en_passant_target = iter.next().expect("Invalid FEN: missing en passant target");
    let _half_move_clock = iter.next().expect("Invalid FEN: missing half-move clock");
    let full_move_number = iter.next().expect("Invalid FEN: missing full move number");

    // Parse the active color and current turn
    if active_color == "b" {
        game_state.current_player = Color::Black;
    }

    game_state.turn = full_move_number.parse().expect("Invalid FEN: invalid full move number");

    for c in piece_placement.chars() {
        match c {
            '0'..='8' => {
                let empty_squares = c.to_digit(10).unwrap() as usize;
                file += empty_squares;
            }
            '/' => {
                rank -= 1;
                file = 0;
            }
            'a'..='z' => {
                let new_piece = match c {
                    'p' => Some(Piece {
                        piece_type: PieceType::Pawn,
                        color: Color::Black,
                    }),
                    'b' => Some(Piece {
                        piece_type: PieceType::Bishop,
                        color: Color::Black,
                    }),
                    'n' => Some(Piece {
                        piece_type: PieceType::Knight,
                        color: Color::Black,
                    }),
                    'r' => Some(Piece {
                        piece_type: PieceType::Rook,
                        color: Color::Black,
                    }),
                    'q' => Some(Piece {
                        piece_type: PieceType::Queen,
                        color: Color::Black,
                    }),
                    'k' => Some(Piece {
                        piece_type: PieceType::King,
                        color: Color::Black,
                    }),
                    _ => None,
                };
                game_state.board[rank * 8 + file] = new_piece;
                file += 1;
            }
            'A'..='Z' => {
                let new_piece = match c {
                    'P' => Some(Piece {
                        piece_type: PieceType::Pawn,
                        color: Color::White,
                    }),
                    'B' => Some(Piece {
                        piece_type: PieceType::Bishop,
                        color: Color::White,
                    }),
                    'N' => Some(Piece {
                        piece_type: PieceType::Knight,
                        color: Color::White,
                    }),
                    'R' => Some(Piece {
                        piece_type: PieceType::Rook,
                        color: Color::White,
                    }),
                    'Q' => Some(Piece {
                        piece_type: PieceType::Queen,
                        color: Color::White,
                    }),
                    'K' => Some(Piece {
                        piece_type: PieceType::King,
                        color: Color::White,
                    }),
                    _ => None,
                };
                game_state.board[rank * 8 + file] = new_piece;
                file += 1;
            }
            _ => break,
        }
    }

    game_state
}