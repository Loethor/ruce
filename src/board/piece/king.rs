//! Module containing king related logic.

use crate::board::{moves::Move, Board, BOARD_SIZE};

// Define the possible offsets for king's moves
const KING_MOVES: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn generate_king_moves(board: &Board, row: u8, col: u8) -> Option<Vec<Move>> {
    let mut moves: Vec<Move> = Vec::new();
    let square: u8 = row * BOARD_SIZE + col;

    for &(dr, dc) in &KING_MOVES {
        let new_row = (row as isize + dr) as u8;
        let new_col = (col as isize + dc) as u8;

        // Check if the new coordinates are within the board boundaries
        if new_row < BOARD_SIZE && new_col < BOARD_SIZE {
            let target_square = new_row * BOARD_SIZE + new_col;

            // Check if the target square is empty or occupied by an opponent's piece
            if board.get_piece(target_square).map_or(true, |piece| {
                piece.color != board.get_piece(square).unwrap().color
            }) {
                moves.push(Move {
                    initial_square: square,
                    target_square,
                });
            }
        }
    }
    generate_white_castling_moves(square, board, &mut moves);
    generate_black_castling_moves(square, board, &mut moves);

    if !moves.is_empty() {
        Some(moves)
    } else {
        None
    }
}

fn generate_black_castling_moves(square: u8, board: &Board, moves: &mut Vec<Move>) {
    // checking for black castling
    // king original position
    if square == 60 {
        // king side
        if board.castling_availability.2
            && board.get_piece(61).is_none()
            && board.get_piece(62).is_none()
        {
            moves.push(Move {
                initial_square: square,
                target_square: 62,
            });
        }
        // queen side
        if board.castling_availability.3
            && board.get_piece(59).is_none()
            && board.get_piece(58).is_none()
        {
            moves.push(Move {
                initial_square: square,
                target_square: 58,
            });
        }
    }
}

fn generate_white_castling_moves(square: u8, board: &Board, moves: &mut Vec<Move>) {
    // checking for white castling
    // king original position
    if square == 4 {
        // king side
        if board.castling_availability.0
            && board.get_piece(5).is_none()
            && board.get_piece(6).is_none()
        {
            moves.push(Move {
                initial_square: square,
                target_square: 6,
            });
        }
        // queen side
        if board.castling_availability.1
            && board.get_piece(3).is_none()
            && board.get_piece(2).is_none()
        {
            moves.push(Move {
                initial_square: square,
                target_square: 2,
            });
        }
    }
}

#[cfg(test)]
mod king_tests {
    use super::*;
    use crate::board::{
        moves::are_moves_equal,
        piece::{Color, Piece, PieceType},
    };

    #[test]
    fn test_generate_king_moves_middle() {
        let mut board = Board::new_empty_board();
        let white_king_square = 27;
        let current_player = Color::White;

        let white_king = Piece {
            piece_type: PieceType::King,
            color: Color::White,
        };
        board.set_piece(white_king_square, white_king);

        let moves = board.generate_moves(current_player);

        let expected_moves = vec![
            Move {
                initial_square: 27,
                target_square: 18,
            },
            Move {
                initial_square: 27,
                target_square: 19,
            },
            Move {
                initial_square: 27,
                target_square: 20,
            },
            Move {
                initial_square: 27,
                target_square: 26,
            },
            Move {
                initial_square: 27,
                target_square: 28,
            },
            Move {
                initial_square: 27,
                target_square: 34,
            },
            Move {
                initial_square: 27,
                target_square: 35,
            },
            Move {
                initial_square: 27,
                target_square: 36,
            },
        ];

        assert_eq!(moves.len(), expected_moves.len());
        assert!(are_moves_equal(&moves, &expected_moves));
    }

    #[test]
    fn test_generate_king_moves_corner() {
        let mut board = Board::new_empty_board();
        let current_player = Color::White;
        let white_king_square = 0;

        let white_king = Piece {
            piece_type: PieceType::King,
            color: Color::White,
        };
        board.set_piece(white_king_square, white_king);

        let moves = board.generate_moves(current_player);

        let expected_moves = vec![
            Move {
                initial_square: 0,
                target_square: 8,
            },
            Move {
                initial_square: 0,
                target_square: 9,
            },
            Move {
                initial_square: 0,
                target_square: 1,
            },
        ];

        assert_eq!(moves.len(), expected_moves.len());
        assert!(are_moves_equal(&moves, &expected_moves));
    }

    #[test]
    fn test_generate_king_moves_side() {
        let mut board = Board::new_empty_board();
        let current_player = Color::White;
        let white_king_square = 23;

        let white_king = Piece {
            piece_type: PieceType::King,
            color: Color::White,
        };
        board.set_piece(white_king_square, white_king);

        let moves = board.generate_moves(current_player);

        let expected_moves = vec![
            Move {
                initial_square: 23,
                target_square: 14,
            },
            Move {
                initial_square: 23,
                target_square: 15,
            },
            Move {
                initial_square: 23,
                target_square: 22,
            },
            Move {
                initial_square: 23,
                target_square: 30,
            },
            Move {
                initial_square: 23,
                target_square: 31,
            },
        ];

        assert_eq!(moves.len(), expected_moves.len());
        assert!(are_moves_equal(&moves, &expected_moves));
    }

    #[test]
    fn test_white_king_castling_both_sides() {
        let mut board = Board::new_empty_board();
        let current_player = Color::White;
        let white_king_square = 4;

        let white_king = Piece {
            piece_type: PieceType::King,
            color: Color::White,
        };
        board.set_piece(white_king_square, white_king);
        board.castling_availability = (true, true, false, false);

        let moves = board.generate_moves(current_player);

        let expected_moves = vec![
            Move {
                initial_square: 4,
                target_square: 3,
            },
            Move {
                initial_square: 4,
                target_square: 11,
            },
            Move {
                initial_square: 4,
                target_square: 12,
            },
            Move {
                initial_square: 4,
                target_square: 13,
            },
            Move {
                initial_square: 4,
                target_square: 5,
            },
            Move {
                initial_square: 4,
                target_square: 2,
            },
            Move {
                initial_square: 4,
                target_square: 6,
            },
        ];

        assert_eq!(moves.len(), expected_moves.len());
        assert!(are_moves_equal(&moves, &expected_moves));
    }

    #[test]
    fn test_white_king_castling_queen_side() {
        let mut board = Board::new_empty_board();
        let current_player = Color::White;
        let white_king_square = 4;

        let white_king = Piece {
            piece_type: PieceType::King,
            color: Color::White,
        };
        board.set_piece(white_king_square, white_king);
        board.castling_availability = (false, true, false, false);

        let moves = board.generate_moves(current_player);

        let expected_moves = vec![
            Move {
                initial_square: 4,
                target_square: 3,
            },
            Move {
                initial_square: 4,
                target_square: 11,
            },
            Move {
                initial_square: 4,
                target_square: 12,
            },
            Move {
                initial_square: 4,
                target_square: 13,
            },
            Move {
                initial_square: 4,
                target_square: 5,
            },
            Move {
                initial_square: 4,
                target_square: 2,
            },
        ];

        assert_eq!(moves.len(), expected_moves.len());
        assert!(are_moves_equal(&moves, &expected_moves));
    }

    #[test]
    fn test_white_king_castling_king_side() {
        let mut board = Board::new_empty_board();
        let current_player = Color::White;
        let white_king_square = 4;

        let white_king = Piece {
            piece_type: PieceType::King,
            color: Color::White,
        };
        board.set_piece(white_king_square, white_king);
        board.castling_availability = (true, false, false, false);

        let moves = board.generate_moves(current_player);

        let expected_moves = vec![
            Move {
                initial_square: 4,
                target_square: 3,
            },
            Move {
                initial_square: 4,
                target_square: 11,
            },
            Move {
                initial_square: 4,
                target_square: 12,
            },
            Move {
                initial_square: 4,
                target_square: 13,
            },
            Move {
                initial_square: 4,
                target_square: 5,
            },
            Move {
                initial_square: 4,
                target_square: 6,
            },
        ];

        assert_eq!(moves.len(), expected_moves.len());
        assert!(are_moves_equal(&moves, &expected_moves));
    }

    #[test]
    fn test_black_king_castling_both_sides() {
        let mut board = Board::new_empty_board();
        let current_player = Color::Black;
        let white_king_square = 60;

        let white_king = Piece {
            piece_type: PieceType::King,
            color: Color::Black,
        };
        board.set_piece(white_king_square, white_king);
        board.castling_availability = (false, false, true, true);

        let moves = board.generate_moves(current_player);

        let expected_moves = vec![
            Move {
                initial_square: 60,
                target_square: 59,
            },
            Move {
                initial_square: 60,
                target_square: 61,
            },
            Move {
                initial_square: 60,
                target_square: 51,
            },
            Move {
                initial_square: 60,
                target_square: 52,
            },
            Move {
                initial_square: 60,
                target_square: 53,
            },
            Move {
                initial_square: 60,
                target_square: 62,
            },
            Move {
                initial_square: 60,
                target_square: 58,
            },
        ];

        assert_eq!(moves.len(), expected_moves.len());
        assert!(are_moves_equal(&moves, &expected_moves));
    }

    #[test]
    fn test_black_king_castling_queen_side() {
        let mut board = Board::new_empty_board();
        let current_player = Color::Black;
        let white_king_square = 60;

        let white_king = Piece {
            piece_type: PieceType::King,
            color: Color::Black,
        };
        board.set_piece(white_king_square, white_king);
        board.castling_availability = (false, false, false, true);

        let moves = board.generate_moves(current_player);

        let expected_moves = vec![
            Move {
                initial_square: 60,
                target_square: 59,
            },
            Move {
                initial_square: 60,
                target_square: 61,
            },
            Move {
                initial_square: 60,
                target_square: 51,
            },
            Move {
                initial_square: 60,
                target_square: 52,
            },
            Move {
                initial_square: 60,
                target_square: 53,
            },
            Move {
                initial_square: 60,
                target_square: 58,
            },
        ];

        assert_eq!(moves.len(), expected_moves.len());
        assert!(are_moves_equal(&moves, &expected_moves));
    }

    #[test]
    fn test_black_king_castling_king_side() {
        let mut board = Board::new_empty_board();
        let current_player = Color::Black;
        let white_king_square = 60;

        let white_king = Piece {
            piece_type: PieceType::King,
            color: Color::Black,
        };
        board.set_piece(white_king_square, white_king);
        board.castling_availability = (false, false, true, false);

        let moves = board.generate_moves(current_player);

        let expected_moves = vec![
            Move {
                initial_square: 60,
                target_square: 59,
            },
            Move {
                initial_square: 60,
                target_square: 61,
            },
            Move {
                initial_square: 60,
                target_square: 51,
            },
            Move {
                initial_square: 60,
                target_square: 52,
            },
            Move {
                initial_square: 60,
                target_square: 53,
            },
            Move {
                initial_square: 60,
                target_square: 62,
            },
        ];

        assert_eq!(moves.len(), expected_moves.len());
        assert!(are_moves_equal(&moves, &expected_moves));
    }
}
