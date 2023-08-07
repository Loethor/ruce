use crate::board::{moves::Move, Board, BOARD_SIZE};

use super::{Color, Piece, PieceType};

pub fn generate_sliding_moves(piece: Piece, board: &Board, row: u8, col: u8) -> Option<Vec<Move>> {
    let mut moves: Vec<Move> = Vec::new();

    match piece.piece_type {
        PieceType::Bishop => {
            let diagonal_moves = generate_diagonal_moves(board, row, col, piece.color);
            if let Some(mut diagonal_moves) = diagonal_moves {
                moves.append(&mut diagonal_moves);
            }
        }
        PieceType::Rook => {
            let horizontal_moves = generate_horizontal_moves(board, row, col, piece.color);
            if let Some(mut horizontal_moves) = horizontal_moves {
                moves.append(&mut horizontal_moves);
            }
            let vertical_moves = generate_vertical_moves(board, row, col, piece.color);
            if let Some(mut vertical_moves) = vertical_moves {
                moves.append(&mut vertical_moves);
            }
        }
        PieceType::Queen => {
            let diagonal_moves = generate_diagonal_moves(board, row, col, piece.color);
            if let Some(mut diagonal_moves) = diagonal_moves {
                moves.append(&mut diagonal_moves);
            }
            let horizontal_moves = generate_horizontal_moves(board, row, col, piece.color);
            if let Some(mut horizontal_moves) = horizontal_moves {
                moves.append(&mut horizontal_moves);
            }
            let vertical_moves = generate_vertical_moves(board, row, col, piece.color);
            if let Some(mut vertical_moves) = vertical_moves {
                moves.append(&mut vertical_moves);
            }
        }
        _ => return None,
    }

    None
}

fn generate_diagonal_moves(board: &Board, row: u8, col: u8, color: Color) -> Option<Vec<Move>> {
    let mut moves: Vec<Move> = Vec::new();

    // up diagonal moves
    let up_moves = generate_up_diagonal_moves(board, row, col, color);
    if let Some(mut up_moves) = up_moves {
        moves.append(&mut up_moves);
    }

    // down diagonal moves
    let down_moves = generate_down_diagonal_moves(board, row, col, color);
    if let Some(mut down_moves) = down_moves {
        moves.append(&mut down_moves);
    }

    if moves.is_empty() {
        return None;
    }
    Some(moves)
}
fn generate_vertical_moves(board: &Board, row: u8, col: u8, color: Color) -> Option<Vec<Move>> {
    let mut moves: Vec<Move> = Vec::new();
    let square: u8 = row * BOARD_SIZE + col;

    // down moves
    for i in 1..BOARD_SIZE {
        if i > row {
            break;
        }
        let target_row = row - i;
        let target_square = target_row * BOARD_SIZE + col;
        if board.get_piece(target_square).is_none() {
            moves.push(Move {
                initial_square: square,
                target_square,
            });
        } else {
            // Direction blocked by enemy piece
            if (board.get_piece(target_square).unwrap().color != color) {
                moves.push(Move {
                    initial_square: square,
                    target_square,
                });
            }
            // We break here because the direction is blocked
            break;
        }
    }

    // up moves
    for i in 1..BOARD_SIZE {
        if i + row >= BOARD_SIZE {
            break;
        }
        let target_row = row + i;
        let target_square = target_row * BOARD_SIZE + col;
        if board.get_piece(target_square).is_none() {
            moves.push(Move {
                initial_square: square,
                target_square,
            });
        } else {
            // Direction blocked by enemy piece
            if board.get_piece(target_square).unwrap().color != color {
                moves.push(Move {
                    initial_square: square,
                    target_square,
                });
            }
            // We break here because the direction is blocked
            break;
        }
    }

    if moves.is_empty() {
        None
    } else {
        Some(moves)
    }
}

fn generate_up_diagonal_moves(board: &Board, row: u8, col: u8, color: Color) -> Option<Vec<Move>> {
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
        if board.get_piece(target_square).is_none() {
            moves.push(Move {
                initial_square: square,
                target_square,
            });
        } else {
            // Direction blocked by enemy piece
            if board.get_piece(target_square).unwrap().color != color {
                moves.push(Move {
                    initial_square: square,
                    target_square,
                });
            }
            // We break here because the direction is blocked
            break;
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
        if board.get_piece(target_square).is_none() {
            moves.push(Move {
                initial_square: square,
                target_square,
            });
        } else {
            // Direction blocked by enemy piece
            if board.get_piece(target_square).unwrap().color != color {
                moves.push(Move {
                    initial_square: square,
                    target_square,
                });
            }
            // We break here because the direction is blocked
            break;
        }
    }

    if moves.is_empty() {
        None
    } else {
        Some(moves)
    }
}

fn generate_horizontal_moves(board: &Board, row: u8, col: u8, color: Color) -> Option<Vec<Move>> {
    let mut moves: Vec<Move> = Vec::new();
    let square: u8 = row * BOARD_SIZE + col;

    // left moves
    for i in 1..BOARD_SIZE {
        if i > col {
            break;
        }
        let target_col = col - i;
        let target_square = row * BOARD_SIZE + target_col;
        if board.get_piece(target_square).is_none() {
            moves.push(Move {
                initial_square: square,
                target_square,
            });
        } else {
            // Direction blocked by enemy piece
            if (board.get_piece(target_square).unwrap().color != color) {
                moves.push(Move {
                    initial_square: square,
                    target_square,
                });
            }
            // We break here because the direction is blocked
            break;
        }
    }

    // right moves
    for i in 1..BOARD_SIZE {
        if col + i >= BOARD_SIZE {
            break;
        }
        let target_col = col + i;
        let target_square = row * BOARD_SIZE + target_col;
        if board.get_piece(target_square).is_none() {
            moves.push(Move {
                initial_square: square,
                target_square,
            });
        } else {
            // Direction blocked by enemy piece
            if (board.get_piece(target_square).unwrap().color != color) {
                moves.push(Move {
                    initial_square: square,
                    target_square,
                });
            }
            // We break here because the direction is blocked
            break;
        }
    }

    if !moves.is_empty() {
        Some(moves)
    } else {
        None
    }
}

fn generate_down_diagonal_moves(
    board: &Board,
    row: u8,
    col: u8,
    color: Color,
) -> Option<Vec<Move>> {
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
        if board.get_piece(target_square).is_none() {
            moves.push(Move {
                initial_square: square,
                target_square,
            });
        } else {
            // Direction blocked by enemy piece
            if board.get_piece(target_square).unwrap().color != color {
                moves.push(Move {
                    initial_square: square,
                    target_square,
                });
            }
            // We break here because the direction is blocked
            break;
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
        if board.get_piece(target_square).is_none() {
            moves.push(Move {
                initial_square: square,
                target_square,
            });
        } else {
            // Direction blocked by enemy piece
            if board.get_piece(target_square).unwrap().color != color {
                moves.push(Move {
                    initial_square: square,
                    target_square,
                });
            }
            // We break here because the direction is blocked
            break;
        }
    }

    if moves.is_empty() {
        None
    } else {
        Some(moves)
    }
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
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 7);
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
        assert!(moves.is_none());
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
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 6);
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
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 7);
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
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 6);
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
        assert!(moves.is_none());
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
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 3);
    }

    #[test]
    fn test_vertical_move() {
        let board = Board::new_empty_board();
        let moves = generate_vertical_moves(&board, 0, 0, Color::White);
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 7);
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
        assert!(moves.is_none());
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
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 1);
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
        assert!(moves.is_none());
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
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 2);
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
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 2);
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
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 4);
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
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 7);
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
        assert!(moves.is_none());
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
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 1);
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
        assert!(moves.is_none());
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
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 2);
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
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 7);
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
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 6);
    }

    #[test]
    fn test_diagonal_down_move() {
        let board = Board::new_empty_board();
        let moves = generate_down_diagonal_moves(&board, 0, 0, Color::White);
        assert!(moves.is_none());
    }

    #[test]
    fn test_diagonal_down_move_top_row() {
        let board = Board::new_empty_board();
        let moves = generate_diagonal_moves(&board, 7, 4, Color::White);
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 7);
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
        assert!(moves.is_none());
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
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 3);
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
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 4);
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
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 7);
    }

    #[test]
    fn test_diagonal_move_no_pieces() {
        let board = Board::new_empty_board();
        let moves = generate_diagonal_moves(&board, 3, 3, Color::White);
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 13);
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
        assert!(moves.is_none());
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
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 4);
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
        assert!(moves.is_some());
        assert_eq!(moves.unwrap().len(), 9);
    }
}
