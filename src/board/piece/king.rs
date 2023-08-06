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

    if !moves.is_empty() {
        Some(moves)
    } else {
        None
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
        let row = 3;
        let col = 3;
        let square = row * BOARD_SIZE + col;

        let king = Piece {
            piece_type: PieceType::King,
            color: Color::White,
        };
        board.set_piece(square, king);

        let moves = generate_king_moves(&board, row, col).unwrap();

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
        let row = 0;
        let col = 0;
        let square = row * BOARD_SIZE + col;

        let king = Piece {
            piece_type: PieceType::King,
            color: Color::Black,
        };
        board.set_piece(square, king);

        let moves = generate_king_moves(&board, row, col).unwrap();

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
        let row = 2;
        let col = 7;
        let square = row * BOARD_SIZE + col;

        let king = Piece {
            piece_type: PieceType::King,
            color: Color::White,
        };
        board.set_piece(square, king);

        let moves = generate_king_moves(&board, row, col).unwrap();

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
}
