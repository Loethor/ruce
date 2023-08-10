use crate::board::{moves::Move, Board, BOARD_SIZE};

use super::{Color, Piece, PieceType};

/// This is the public API for generating moves for sliding pieces.
/// Expected pieces are `Bishop`, `Rook`, and `Queen`.
/// Based on the piece type, this function will generate moves in the
/// diagonal directions, linear directions, or both.
pub fn generate_sliding_moves(piece: Piece, board: &Board, row: u8, col: u8) -> Option<Vec<Move>> {
    let mut moves: Vec<Move> = Vec::new();

    match piece.piece_type {
        PieceType::Bishop => {
            moves.append(&mut generate_moves(
                board,
                row,
                col,
                piece.color,
                &DIAGONAL_DIRECTIONS,
            ));
        }
        PieceType::Rook => {
            moves.append(&mut generate_moves(
                board,
                row,
                col,
                piece.color,
                &LINEAR_DIRECTIONS,
            ));
        }
        PieceType::Queen => {
            moves.append(&mut generate_moves(
                board,
                row,
                col,
                piece.color,
                &DIAGONAL_DIRECTIONS,
            ));
            moves.append(&mut generate_moves(
                board,
                row,
                col,
                piece.color,
                &LINEAR_DIRECTIONS,
            ));
        }
        _ => return None,
    }

    Some(moves)
}

/// Generate moves in the given directions.
/// The directions are given as a tuple of functions.
/// The first function is the row offset function, the second is the column offset function,
/// and the third is the stopping condition function.
/// The stopping condition function takes the current row, column, and offset as arguments.
/// It returns true if the stopping condition is met, false otherwise.
fn generate_moves(
    board: &Board,
    row: u8,
    col: u8,
    color: Color,
    directions: &[Direction],
) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let square: u8 = row * BOARD_SIZE + col;

    for dir in directions {
        for i in 1..BOARD_SIZE {
            let (row_offset, col_offset, stopping_condition) = dir;

            if stopping_condition(row, col, i) {
                break;
            }

            let target_row = row_offset(row, i);
            let target_col = col_offset(col, i);
            let target_square = target_row * BOARD_SIZE + target_col;

            if let Some(piece) = board.get_piece(target_square) {
                // Direction blocked by enemy piece
                if piece.color != color {
                    moves.push(Move {
                        initial_square: square,
                        target_square,
                    });
                }
                // We break here because the direction is blocked
                break;
            }
            moves.push(Move {
                initial_square: square,
                target_square,
            });
        }
    }

    moves
}

/// Define the possible directions and their associated functions and stopping conditions
/// for sliding pieces.
/// The first function is the row offset function, the second is the column offset function,
/// and the third is the stopping condition function.
type Direction = (fn(u8, u8) -> u8, fn(u8, u8) -> u8, fn(u8, u8, u8) -> bool);

/// The DIAGONAL_DIRECTIONS consist of the diagonal up directions and the diagonal down directions.
static DIAGONAL_DIRECTIONS: [Direction; 4] = [
    DIAGONAL_UP_DIRECTIONS[0],
    DIAGONAL_UP_DIRECTIONS[1],
    DIAGONAL_DOWN_DIRECTIONS[0],
    DIAGONAL_DOWN_DIRECTIONS[1],
];

/// The DIAGONAL_UP_DIRECTIONS consist of the diagonal up right direction and the diagonal up left direction.
/// The diagonal up right direction is defined as the direction from the current square to the top right corner of the board.
/// The diagonal up left direction is defined as the direction from the current square to the top left corner of the board.
static DIAGONAL_UP_DIRECTIONS: [Direction; 2] = [
    (
        |row, i| row + i,
        |col, i| col + i,
        |row, col, i| row + i >= BOARD_SIZE || col + i >= BOARD_SIZE,
    ),
    (
        |row, i| row + i,
        |col, i| col - i,
        |row, col, i| row + i >= BOARD_SIZE || i > col,
    ),
];

/// The DIAGONAL_DOWN_DIRECTIONS consist of the diagonal down right direction and the diagonal down left direction.
/// The diagonal down right direction is defined as the direction from the current square to the bottom right corner of the board.
/// The diagonal down left direction is defined as the direction from the current square to the bottom left corner of the board.
static DIAGONAL_DOWN_DIRECTIONS: [Direction; 2] = [
    (
        |row, i| row - i,
        |col, i| col + i,
        |row, col, i| row < i || col + i >= BOARD_SIZE,
    ),
    (
        |row, i| row - i,
        |col, i| col - i,
        |row, col, i| row < i || i > col,
    ),
];

/// The LINEAR_DIRECTIONS consist of the horizontal directions and the vertical directions.
static LINEAR_DIRECTIONS: [Direction; 4] = [
    HORIZONTAL_DIRECTIONS[0],
    HORIZONTAL_DIRECTIONS[1],
    VERTICAL_DIRECTIONS[0],
    VERTICAL_DIRECTIONS[1],
];

/// The HORIZONTAL_DIRECTIONS consist of the horizontal right direction and the horizontal left direction.
/// The horizontal right direction is defined as the direction from the current square to the right edge of the board.
/// The horizontal left direction is defined as the direction from the current square to the left edge of the board.
static HORIZONTAL_DIRECTIONS: [Direction; 2] = [
    (
        |row, _| row,
        |col, i| col + i,
        |_, col, i| col + i >= BOARD_SIZE,
    ),
    (|row, _| row, |col, i| col - i, |_, col, i| col < i),
];

/// The VERTICAL_DIRECTIONS consist of the vertical up direction and the vertical down direction.
/// The vertical up direction is defined as the direction from the current square to the top edge of the board.
/// The vertical down direction is defined as the direction from the current square to the bottom edge of the board.
static VERTICAL_DIRECTIONS: [Direction; 2] = [
    (
        |row, i| row + i,
        |col, _| col,
        |row, _, i| row + i >= BOARD_SIZE,
    ),
    (|row, i| row - i, |col, _| col, |row, _, i| row < i),
];

/// This module tests the linear move generation.
#[cfg(test)]
mod linear_move_tests {
    use super::*;
    use crate::board::{self, Board};
    use crate::board::{Color, Piece, PieceType};

    #[test]
    fn test_horizontal_move() {
        let board = Board::new_empty_board();
        let moves = generate_moves(&board, 0, 0, Color::White, &HORIZONTAL_DIRECTIONS);
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn test_linear_moves() {
        let board = Board::new_empty_board();
        let moves = generate_moves(&board, 0, 0, Color::White, &LINEAR_DIRECTIONS);
        assert_eq!(moves.len(), 14);
    }

    #[test]
    fn test_horizontal_move_blocked() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            1,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        let moves = generate_moves(&board, 0, 0, Color::White, &HORIZONTAL_DIRECTIONS);
        assert!(moves.is_empty());
    }

    #[test]
    fn test_horizontal_move_blocked_from_left() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            0,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        let moves = generate_moves(&board, 0, 1, Color::White, &HORIZONTAL_DIRECTIONS);
        assert_eq!(moves.len(), 6);
    }

    #[test]
    fn test_horizontal_move_may_capture_left() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            0,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );
        let moves = generate_moves(&board, 0, 1, Color::White, &HORIZONTAL_DIRECTIONS);
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn test_horizontal_move_blocked_from_right() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            0,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        let moves = generate_moves(&board, 0, 6, Color::White, &HORIZONTAL_DIRECTIONS);
        assert_eq!(moves.len(), 6);
    }

    #[test]
    fn test_horizontal_move_blocked_from_both_side() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            0,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        board.set_piece(
            2,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        let moves = generate_moves(&board, 0, 1, Color::White, &HORIZONTAL_DIRECTIONS);
        assert!(moves.is_empty());
    }

    #[test]
    fn test_horizontal_move_pieces_on_both_sides() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            0,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );
        board.set_piece(
            3,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );
        let moves = generate_moves(&board, 0, 1, Color::White, &HORIZONTAL_DIRECTIONS);
        assert_eq!(moves.len(), 3);
    }

    #[test]
    fn test_vertical_move() {
        let board = Board::new_empty_board();
        let moves = generate_moves(&board, 0, 0, Color::White, &VERTICAL_DIRECTIONS);
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn test_vertical_move_blocked() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            8,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        let moves = generate_moves(&board, 0, 0, Color::White, &VERTICAL_DIRECTIONS);
        assert!(moves.is_empty());
    }

    #[test]
    fn test_vertical_move_blocked_from_top() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            16,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        let moves = generate_moves(&board, 0, 0, Color::White, &VERTICAL_DIRECTIONS);
        assert_eq!(moves.len(), 1);
    }

    #[test]
    fn test_vertical_move_blocked_from_both() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            0,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        board.set_piece(
            16,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        let moves = generate_moves(&board, 1, 0, Color::White, &VERTICAL_DIRECTIONS);
        assert!(moves.is_empty());
    }

    #[test]
    fn test_vertical_move_blocked_from_top_capture_bottom() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            24,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );
        board.set_piece(
            0,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );

        let moves = generate_moves(&board, 1, 0, Color::White, &VERTICAL_DIRECTIONS);
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn test_vertical_move_blocked_from_bottom_capture_top() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            0,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );
        board.set_piece(
            24,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );

        let moves = generate_moves(&board, 1, 0, Color::Black, &VERTICAL_DIRECTIONS);
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn test_vertical_move_blocked_may_capture_from_both_sides() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            0,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );
        board.set_piece(
            32,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );

        let moves = generate_moves(&board, 1, 0, Color::White, &VERTICAL_DIRECTIONS);
        assert_eq!(moves.len(), 4);
    }

    #[test]
    fn test_linear_moves_blocked_everywhere() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            1,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        board.set_piece(
            8,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        board.set_piece(
            10,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        board.set_piece(
            17,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );

        let moves = generate_moves(&board, 1, 1, Color::White, &LINEAR_DIRECTIONS);
        assert!(moves.is_empty());
    }

    #[test]
    fn test_liner_move_capture_everywhere() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            1,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );
        board.set_piece(
            8,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );
        board.set_piece(
            10,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );
        board.set_piece(
            17,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );

        let moves = generate_moves(&board, 1, 1, Color::White, &LINEAR_DIRECTIONS);
        assert_eq!(moves.len(), 4);
    }

    #[test]
    fn test_linear_capture_on_the_edge() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            15,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );

        let moves = generate_moves(&board, 1, 1, Color::White, &LINEAR_DIRECTIONS);
        assert_eq!(moves.len(), 14);
    }
}

/// This module tests the diagonal move generation.
#[cfg(test)]
mod diagonal_move_tests {

    use super::*;
    use crate::board::Board;
    use crate::board::{Color, Piece, PieceType};

    #[test]
    fn test_diagonal_up_move() {
        let board = Board::new_empty_board();
        let moves = generate_moves(&board, 0, 0, Color::White, &DIAGONAL_UP_DIRECTIONS);
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn test_diagonal_up_move_blocked() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            9,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        let moves = generate_moves(&board, 0, 0, Color::White, &DIAGONAL_UP_DIRECTIONS);
        assert!(moves.is_empty());
    }

    #[test]
    fn test_diagonal_up_move_blocked_with_one_move() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            18,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        let moves = generate_moves(&board, 0, 0, Color::White, &DIAGONAL_UP_DIRECTIONS);
        assert_eq!(moves.len(), 1);
    }

    #[test]
    fn test_diagonal_up_move_blocked_from_both() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            11,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        board.set_piece(
            13,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        let moves = generate_moves(&board, 0, 4, Color::White, &DIAGONAL_UP_DIRECTIONS);
        assert!(moves.is_empty());
    }

    #[test]
    fn test_diagonal_up_move_blocked_from_top_capture_both() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            11,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );
        board.set_piece(
            13,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );

        let moves = generate_moves(&board, 0, 4, Color::White, &DIAGONAL_UP_DIRECTIONS);
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn test_diagonal_up_move_capture_both_on_edges() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            32,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );
        board.set_piece(
            31,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );

        let moves = generate_moves(&board, 0, 4, Color::White, &DIAGONAL_UP_DIRECTIONS);
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn test_diagonal_up_move_capture_on_left_edge_blocked_on_right() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            32,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );
        board.set_piece(
            31,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        board.set_piece(
            33,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );

        let moves = generate_moves(&board, 0, 4, Color::White, &DIAGONAL_UP_DIRECTIONS);
        assert_eq!(moves.len(), 6);
    }

    #[test]
    fn test_diagonal_down_move() {
        let board = Board::new_empty_board();
        let moves = generate_moves(&board, 0, 0, Color::White, &DIAGONAL_DOWN_DIRECTIONS);
        assert!(moves.is_empty());
    }

    #[test]
    fn test_diagonal_down_move_top_row() {
        let board = Board::new_empty_board();
        let moves = generate_moves(&board, 7, 4, Color::White, &DIAGONAL_DOWN_DIRECTIONS);
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn test_diagonal_down_move_blocked_from_both() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            51,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        board.set_piece(
            53,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        let moves = generate_moves(&board, 7, 4, Color::White, &DIAGONAL_DOWN_DIRECTIONS);
        assert!(moves.is_empty());
    }

    #[test]
    fn test_diagonal_down_move_blocked_from_left() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            51,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );

        let moves = generate_moves(&board, 7, 4, Color::White, &DIAGONAL_DOWN_DIRECTIONS);
        assert_eq!(moves.len(), 3);
    }

    #[test]
    fn test_diagonal_down_move_blocked_from_right() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            53,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );

        let moves = generate_moves(&board, 7, 4, Color::White, &DIAGONAL_DOWN_DIRECTIONS);
        assert_eq!(moves.len(), 4);
    }

    #[test]
    fn test_diagonal_down_move_capture_both() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            24,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );
        board.set_piece(
            39,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );

        let moves = generate_moves(&board, 7, 4, Color::White, &DIAGONAL_DOWN_DIRECTIONS);
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn test_diagonal_move_no_pieces() {
        let board = Board::new_empty_board();
        let moves = generate_moves(&board, 3, 3, Color::White, &DIAGONAL_DIRECTIONS);
        assert_eq!(moves.len(), 13);
    }

    #[test]
    fn test_diagonal_move_fully_blocked() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            34,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        board.set_piece(
            36,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        board.set_piece(
            18,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        board.set_piece(
            20,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        let moves = generate_moves(&board, 3, 3, Color::White, &DIAGONAL_DIRECTIONS);
        assert!(moves.is_empty());
    }

    #[test]
    fn test_diagonal_move_capture_everywhere() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            34,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        board.set_piece(
            36,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        board.set_piece(
            18,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        board.set_piece(
            20,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        let moves = generate_moves(&board, 3, 3, Color::Black, &DIAGONAL_DIRECTIONS);
        assert_eq!(moves.len(), 4);
    }

    #[test]
    fn test_diagonal_move_capture_at_distance() {
        let mut board = Board::new_empty_board();
        board.set_piece(
            41,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        board.set_piece(
            45,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        board.set_piece(
            0,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        board.set_piece(
            6,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );
        let moves = generate_moves(&board, 3, 3, Color::Black, &DIAGONAL_DIRECTIONS);
        assert_eq!(moves.len(), 9);
    }
}

/// This module tests the public API
#[cfg(test)]
mod piece_move_tests {

    use super::*;
    use crate::board::Board;
    use crate::board::{Color, Piece, PieceType};

    #[test]
    fn test_queen_moves_no_pieces() {
        let board = Board::new_empty_board();
        let queen = Piece {
            piece_type: PieceType::Queen,
            color: Color::White,
        };
        let moves = generate_sliding_moves(queen, &board, 3, 3);
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 27);
    }

    #[test]
    fn test_queen_moves_blocked_on_one_direction() {
        let mut board = Board::new_empty_board();
        let queen = Piece {
            piece_type: PieceType::Queen,
            color: Color::White,
        };
        board.set_piece(
            34,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        let moves = generate_sliding_moves(queen, &board, 3, 3);
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 24);
    }

    #[test]
    fn test_queen_moves_captures_on_two_directions() {
        let mut board = Board::new_empty_board();
        let queen = Piece {
            piece_type: PieceType::Queen,
            color: Color::White,
        };
        board.set_piece(
            34,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );
        board.set_piece(
            36,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );
        let moves = generate_sliding_moves(queen, &board, 3, 3);
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 22);
    }

    #[test]
    fn test_rook_moves_no_pieces() {
        let board = Board::new_empty_board();
        let rook = Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        };
        let moves = generate_sliding_moves(rook, &board, 3, 3);
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 14);
    }

    #[test]
    fn test_rook_moves_capture_one() {
        let mut board = Board::new_empty_board();
        let rook = Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        };
        board.set_piece(
            26,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );
        let moves = generate_sliding_moves(rook, &board, 3, 3);
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 12);
    }

    #[test]
    fn test_rook_moves_blocked_two() {
        let mut board = Board::new_empty_board();
        let rook = Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        };
        board.set_piece(
            24,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        board.set_piece(
            28,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        let moves = generate_sliding_moves(rook, &board, 3, 3);
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 9);
    }

    #[test]
    fn test_bishop_moves_no_pieces() {
        let board = Board::new_empty_board();
        let bishop = Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        };
        let moves = generate_sliding_moves(bishop, &board, 3, 3);
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 13);
    }

    #[test]
    fn test_bishop_moves_capture_one_piece() {
        let mut board = Board::new_empty_board();
        let bishop = Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        };
        board.set_piece(
            36,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::Black,
            },
        );
        let moves = generate_sliding_moves(bishop, &board, 3, 3);
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 10);
    }

    #[test]
    fn test_bishop_moves_blocked_by_two() {
        let mut board = Board::new_empty_board();
        let bishop = Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        };
        board.set_piece(
            41,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        board.set_piece(
            45,
            Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            },
        );
        let moves = generate_sliding_moves(bishop, &board, 3, 3);
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 8);
    }
}
