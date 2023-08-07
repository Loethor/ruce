use crate::board::{moves::Move, Board, BOARD_SIZE};

use super::{Color, Piece, PieceType};

pub fn generate_sliding_moves(piece: Piece, board: &Board, row: u8, col: u8) -> Option<Vec<Move>> {
    let mut moves: Vec<Move> = Vec::new();

    match piece.piece_type {
        PieceType::Bishop => {
            moves.append(&mut generate_diagonal_moves(board, row, col, piece.color));
        }
        PieceType::Rook => {
            moves.append(&mut generate_horizontal_moves(board, row, col, piece.color));
            moves.append(&mut generate_vertical_moves(board, row, col, piece.color));
        }
        PieceType::Queen => {
            moves.append(&mut generate_diagonal_moves(board, row, col, piece.color));
            moves.append(&mut generate_horizontal_moves(board, row, col, piece.color));
            moves.append(&mut generate_vertical_moves(board, row, col, piece.color));
        }
        _ => return None,
    }

    Some(moves)
}

fn generate_diagonal_moves(board: &Board, row: u8, col: u8, color: Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    // up diagonal moves
    moves.append(&mut generate_up_diagonal_moves(board, row, col, color));

    // down diagonal moves
    moves.append(&mut generate_down_diagonal_moves(board, row, col, color));

    moves
}

fn generate_vertical_moves(board: &Board, row: u8, col: u8, color: Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let square: u8 = row * BOARD_SIZE + col;

    // down moves
    for i in 1..BOARD_SIZE {
        if i > row {
            break;
        }
        let target_row = row - i;
        let target_square = target_row * BOARD_SIZE + col;

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
        } else {
            moves.push(Move {
                initial_square: square,
                target_square,
            });
        }
    }

    // up moves
    for i in 1..BOARD_SIZE {
        if i + row >= BOARD_SIZE {
            break;
        }
        let target_row = row + i;
        let target_square = target_row * BOARD_SIZE + col;
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
        } else {
            moves.push(Move {
                initial_square: square,
                target_square,
            });
        }
    }

    moves
}

fn generate_up_diagonal_moves(board: &Board, row: u8, col: u8, color: Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let square: u8 = row * BOARD_SIZE + col;

    // up left moves
    for i in 1..BOARD_SIZE {
        if row + i >= BOARD_SIZE || i > col {
            break;
        }
        let target_row = row + i;
        let target_col = col - i;
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
        } else {
            moves.push(Move {
                initial_square: square,
                target_square,
            });
        }
    }

    // up right moves
    for i in 1..BOARD_SIZE {
        if row + i >= BOARD_SIZE || col + i >= BOARD_SIZE {
            break;
        }
        let target_row = row + i;
        let target_col = col + i;
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
        } else {
            moves.push(Move {
                initial_square: square,
                target_square,
            });
        }
    }

    moves
}

fn generate_horizontal_moves(board: &Board, row: u8, col: u8, color: Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let square: u8 = row * BOARD_SIZE + col;

    // left moves
    for i in 1..col + 1 {
        let target_col = col - i;
        let target_square = row * BOARD_SIZE + target_col;

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
        } else {
            moves.push(Move {
                initial_square: square,
                target_square,
            });
        }
    }

    // right moves
    for i in 1..BOARD_SIZE - col {
        let target_col = col + i;
        let target_square = row * BOARD_SIZE + target_col;

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
        } else {
            moves.push(Move {
                initial_square: square,
                target_square,
            });
        }
    }

    moves
}

fn generate_down_diagonal_moves(board: &Board, row: u8, col: u8, color: Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let square: u8 = row * BOARD_SIZE + col;

    // down left moves
    for i in 1..BOARD_SIZE {
        if row < i || i > col {
            break;
        }
        let target_row = row - i;
        let target_col = col - i;
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
        } else {
            moves.push(Move {
                initial_square: square,
                target_square,
            });
        }
    }

    // down right moves
    for i in 1..BOARD_SIZE {
        if row < i || col + i >= BOARD_SIZE {
            break;
        }
        let target_row = row - i;
        let target_col = col + i;
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
        } else {
            moves.push(Move {
                initial_square: square,
                target_square,
            });
        }
    }

    moves
}

#[cfg(test)]
mod horizontal_move_tests {
    use super::*;
    use crate::board::Board;
    use crate::board::{Color, Piece, PieceType};

    #[test]
    fn test_horizontal_move() {
        let board = Board::new_empty_board();
        let moves = generate_horizontal_moves(&board, 0, 0, Color::White);
        assert_eq!(moves.len(), 7);
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
        let moves = generate_horizontal_moves(&board, 0, 0, Color::White);
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
        let moves = generate_horizontal_moves(&board, 0, 1, Color::White);
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
        let moves = generate_horizontal_moves(&board, 0, 1, Color::White);
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
        let moves = generate_horizontal_moves(&board, 0, 6, Color::White);
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
        let moves = generate_horizontal_moves(&board, 0, 1, Color::White);
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
        let moves = generate_horizontal_moves(&board, 0, 1, Color::White);
        assert_eq!(moves.len(), 3);
    }

    #[test]
    fn test_vertical_move() {
        let board = Board::new_empty_board();
        let moves = generate_vertical_moves(&board, 0, 0, Color::White);
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
        let moves = generate_vertical_moves(&board, 0, 0, Color::White);
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
        let moves = generate_vertical_moves(&board, 0, 0, Color::White);
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
        let moves = generate_vertical_moves(&board, 1, 0, Color::White);
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

        let moves = generate_vertical_moves(&board, 1, 0, Color::White);
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

        let moves = generate_vertical_moves(&board, 1, 0, Color::Black);
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

        let moves = generate_vertical_moves(&board, 1, 0, Color::White);
        assert_eq!(moves.len(), 4);
    }
}

#[cfg(test)]
mod diagonal_move_tests {

    use super::*;
    use crate::board::Board;
    use crate::board::{Color, Piece, PieceType};

    #[test]
    fn test_diagonal_up_move() {
        let board = Board::new_empty_board();
        let moves = generate_diagonal_moves(&board, 0, 0, Color::White);
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
        let moves = generate_diagonal_moves(&board, 0, 0, Color::White);
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
        let moves = generate_diagonal_moves(&board, 0, 0, Color::White);
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
        let moves = generate_diagonal_moves(&board, 0, 4, Color::White);
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

        let moves = generate_diagonal_moves(&board, 0, 4, Color::White);
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

        let moves = generate_diagonal_moves(&board, 0, 4, Color::White);
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

        let moves = generate_diagonal_moves(&board, 0, 4, Color::White);
        assert_eq!(moves.len(), 6);
    }

    #[test]
    fn test_diagonal_down_move() {
        let board = Board::new_empty_board();
        let moves = generate_down_diagonal_moves(&board, 0, 0, Color::White);
        assert!(moves.is_empty());
    }

    #[test]
    fn test_diagonal_down_move_top_row() {
        let board = Board::new_empty_board();
        let moves = generate_diagonal_moves(&board, 7, 4, Color::White);
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
        let moves = generate_diagonal_moves(&board, 7, 4, Color::White);
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

        let moves = generate_diagonal_moves(&board, 7, 4, Color::White);
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

        let moves = generate_diagonal_moves(&board, 7, 4, Color::White);
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

        let moves = generate_diagonal_moves(&board, 7, 4, Color::White);
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn test_diagonal_move_no_pieces() {
        let board = Board::new_empty_board();
        let moves = generate_diagonal_moves(&board, 3, 3, Color::White);
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
        let moves = generate_diagonal_moves(&board, 3, 3, Color::White);
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
        let moves = generate_diagonal_moves(&board, 3, 3, Color::Black);
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
        let moves = generate_diagonal_moves(&board, 3, 3, Color::Black);
        assert_eq!(moves.len(), 9);
    }
}
