//! Module containing pawn related logic.

use crate::board::{moves::Move, Board, BOARD_SIZE};

use super::Color;

fn one_square_move(board: &Board, initial_square: u8, col: u8, new_row: u8) -> Option<Move> {
    // Calculate the target square number
    let target_square = new_row * BOARD_SIZE + col;

    // Check if destination is empty
    if board.get_piece(target_square).is_none() {
        return Some(Move {
            initial_square,
            target_square,
        });
    }
    None
}

fn two_square_move(
    board: &Board,
    initial_square: u8,
    row: u8,
    col: u8,
    is_one_move_allowed: bool,
    direction: isize,
    color: Color,
) -> Option<Move> {
    // Check if it's the pawn's first move and if the two-square move is available
    if (row == 1 && color == Color::White) || (row == 6 && color == Color::Black) {
        // Calculate the target square number for the two-square move
        let two_square_target =
            ((row as isize + 2 * direction) * BOARD_SIZE as isize + col as isize) as u8;

        // Check if the two-square move destination is empty and also the one-square move was empty
        if board.get_piece(two_square_target).is_none() && is_one_move_allowed {
            // The two-square move is available, add it to the list of valid moves
            return Some(Move {
                initial_square,
                target_square: two_square_target,
            });
        }
    }
    None
}

fn capture_moves(
    board: &Board,
    initial_square: u8,
    row: u8,
    direction: isize,
    new_col: isize,
    color: Color,
) -> Option<Move> {
    // Check if there's a capture on the left diagonal
    if new_col >= 0 && new_col < BOARD_SIZE as isize {
        let diagonal_target = ((row as isize + direction) * BOARD_SIZE as isize + new_col) as u8;
        if let Some(piece) = board.get_piece(diagonal_target) {
            if piece.color != color {
                // The left diagonal has an opponent's piece, so it's a valid capturing move
                return Some(Move {
                    initial_square,
                    target_square: diagonal_target,
                });
            }
        }
    }
    None
}

/// Generate pawn moves for a specific position on the board.
///
/// Given a chess `board`, `row`, `col`, and the pawn's `color`, this function calculates all valid
/// pawn moves for the pawn located at the specified position (`row`, `col`) on the board.
/// It takes into consideration the pawn's color to determine the direction of movement.
/// The function considers one-square moves, two-square moves for the pawn's first move,
/// and capturing moves diagonally.
/// The function also handles special moves like en passant and promotion, but these are not yet implemented.
///
/// The function returns an `Option<Vec<Move>>`, where `Some(moves)` contains a vector of `Move`
/// structs representing the valid moves that the pawn can make. If no valid moves are found,
/// `None` is returned.
///
/// # Arguments
///
/// * `board`: The chess board on which the pawn moves will be calculated.
/// * `row`: The row coordinate (0-based) of the square where the pawn is located.
/// * `col`: The column coordinate (0-based) of the square where the pawn is located.
/// * `color`: The color of the pawn (either `Color::White` or `Color::Black`).
///
/// # Returns
///
/// An `Option<Vec<Move>>`, where `Some(moves)` contains the valid moves that the pawn can make,
/// and `None` if no valid moves are found.
///
/// # Example
///
/// ```
/// use chess_engine::board::{Board, generate_pawn_moves};
/// use chess_engine::board::piece::{Color, Piece, PieceType};
/// use chess_engine::board::moves::Move;
///
/// // Create an empty chess board
/// let mut board = Board::new_empty_board();
///
/// // Place a white pawn on square 'd2' (index 11)
/// let pawn = Piece {
///     piece_type: PieceType::Pawn,
///     color: Color::White,
/// };
/// board.set_piece(11, Some(pawn));
///
/// // Generate pawn moves for the pawn on 'd2'
/// let moves = generate_pawn_moves(&board, 1, 3, Color::White);
///
/// // The valid pawn moves for 'd2' are 'd3' (one square move) and 'd4' (two square move).
/// let expected_moves = vec![
///     Move { initial_square: 11, target_square: 19 },
///     Move { initial_square: 11, target_square: 27 },
/// ];
/// assert_eq!(moves.unwrap(), expected_moves);
/// ```
pub fn generate_pawn_moves(board: &Board, row: u8, col: u8, color: Color) -> Option<Vec<Move>> {
    let mut moves: Vec<Move> = Vec::new();

    // Calculate the direction based on the piece color
    let direction = match color {
        Color::White => 1,  // Moving up (increasing row index)
        Color::Black => -1, // Moving down (decreasing row index)
    };

    // Calculate the initial square number
    let initial_square = row * BOARD_SIZE + col;

    // Calculate the destination row for the pawn's move
    let new_row = (row as isize + direction) as u8;

    // Ensure that the destination row is within bounds
    if new_row >= BOARD_SIZE {
        return None; // Invalid move, the pawn is off the board
    }

    // Info needed for two square move
    let mut is_one_move_allowed = false;

    // One square movement
    if let Some(move_) = one_square_move(board, initial_square, col, new_row) {
        is_one_move_allowed = true;
        moves.push(move_);
    }

    // Two square movement
    if let Some(move_) = two_square_move(
        board,
        initial_square,
        row,
        col,
        is_one_move_allowed,
        direction,
        color,
    ) {
        moves.push(move_);
    }

    // Captures
    if let Some(move_) = capture_moves(
        board,
        initial_square,
        row,
        direction,
        col as isize - 1,
        color,
    ) {
        moves.push(move_);
    }
    if let Some(move_) = capture_moves(
        board,
        initial_square,
        row,
        direction,
        col as isize + 1,
        color,
    ) {
        moves.push(move_);
    }

    // TODO: Implement en passant, for this we need to know the previous move

    // TODO: Implement promotion, hmm what is the purpose of this?
    // this is a move that ends either in the last row or the first row

    if !moves.is_empty() {
        return Some(moves);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::piece::Color::Black;
    use crate::board::piece::Color::White;
    use crate::board::piece::Piece;
    use crate::board::piece::PieceType::Pawn;

    #[test]
    fn test_generate_pawn_moves_white() {
        let mut board = Board::new_empty_board();
        let row = 1;
        let col = 3;
        let square = row * BOARD_SIZE + col;
        let pawn = Piece {
            piece_type: Pawn,
            color: White,
        };
        board.set_piece(square, pawn);
        let moves = pawn.generate_moves(&board, row, col).unwrap();

        assert_eq!(moves.len(), 2);

        // Check that the pawn can move one square forward
        assert!(moves.contains(&Move {
            initial_square: 11,
            target_square: 19,
        }));

        // Check that the pawn can move two squares forward
        assert!(moves.contains(&Move {
            initial_square: 11,
            target_square: 27,
        }));
    }

    #[test]
    fn test_generate_pawn_moves_black() {
        let mut board = Board::new_empty_board();
        let row = 6;
        let col = 3;
        let square = row * BOARD_SIZE + col;
        let piece = Piece {
            piece_type: Pawn,
            color: Black,
        };
        board.set_piece(square, piece);
        let moves = piece.generate_moves(&board, row, col).unwrap();

        assert_eq!(moves.len(), 2);

        // Check that the pawn can move one square forward
        assert!(moves.contains(&Move {
            initial_square: 51,
            target_square: 43,
        }));

        // Check that the pawn can move two squares forward
        assert!(moves.contains(&Move {
            initial_square: 51,
            target_square: 35,
        }));
    }

    #[test]
    fn test_generate_pawn_moves_white_only_one_square_move_allowed() {
        let mut board = Board::new_empty_board();
        let row = 1;
        let col = 3;

        let white_pawn_square = row * BOARD_SIZE + col;
        let black_pawn_square = (row + 2) * BOARD_SIZE + col;

        let white_pawn = Piece {
            piece_type: Pawn,
            color: White,
        };
        let black_pawn = Piece {
            piece_type: Pawn,
            color: Black,
        };

        board.set_piece(white_pawn_square, white_pawn);
        board.set_piece(black_pawn_square, black_pawn);

        let moves = white_pawn.generate_moves(&board, row, col).unwrap();

        assert_eq!(moves.len(), 1);

        // Check that the pawn can move one square forward
        assert!(moves.contains(&Move {
            initial_square: 11,
            target_square: 19,
        }));

        // Check that the pawn can't move two squares forward
        assert!(!moves.contains(&Move {
            initial_square: 11,
            target_square: 27,
        }));
    }

    #[test]
    fn test_generate_pawn_move_black_only_one_square_allowed() {
        let mut board = Board::new_empty_board();
        let row = 6;
        let col = 3;

        let white_pawn_square = (row - 2) * BOARD_SIZE + col;
        let black_pawn_square = row * BOARD_SIZE + col;

        let white_pawn = Piece {
            piece_type: Pawn,
            color: White,
        };
        let black_pawn = Piece {
            piece_type: Pawn,
            color: Black,
        };

        board.set_piece(white_pawn_square, white_pawn);
        board.set_piece(black_pawn_square, black_pawn);

        let moves = black_pawn.generate_moves(&board, row, col).unwrap();

        assert_eq!(moves.len(), 1);

        // Check that the pawn can move one square forward
        assert!(moves.contains(&Move {
            initial_square: 51,
            target_square: 43,
        }));

        // Check that the pawn can't move two squares forward
        assert!(!moves.contains(&Move {
            initial_square: 51,
            target_square: 35,
        }));
    }

    #[test]
    fn test_generate_pawn_move_white_takes_piece_from_starting_row() {
        let mut board = Board::new_empty_board();
        let row = 1;
        let col = 3;

        let white_pawn_square = row * BOARD_SIZE + col;
        let black_pawn_square = (row + 1) * BOARD_SIZE + col + 1;

        let white_pawn = Piece {
            piece_type: Pawn,
            color: White,
        };
        let black_pawn = Piece {
            piece_type: Pawn,
            color: Black,
        };

        board.set_piece(white_pawn_square, white_pawn);
        board.set_piece(black_pawn_square, black_pawn);

        let moves = white_pawn.generate_moves(&board, row, col).unwrap();

        assert_eq!(moves.len(), 3);

        // Check that the pawn can move one square forward
        assert!(moves.contains(&Move {
            initial_square: 11,
            target_square: 19,
        }));

        // Check that the pawn can take the black pawn
        assert!(moves.contains(&Move {
            initial_square: 11,
            target_square: 20,
        }));

        // Check that pawn can move two square forward
        assert!(moves.contains(&Move {
            initial_square: 11,
            target_square: 27,
        }));
    }

    #[test]
    fn test_generate_pawn_move_white_takes_piece() {
        let mut board = Board::new_empty_board();
        let row = 3;
        let col = 3;

        let white_pawn_square = row * BOARD_SIZE + col;
        let black_pawn_square = (row + 1) * BOARD_SIZE + col + 1;

        let white_pawn = Piece {
            piece_type: Pawn,
            color: White,
        };
        let black_pawn = Piece {
            piece_type: Pawn,
            color: Black,
        };

        board.set_piece(white_pawn_square, white_pawn);
        board.set_piece(black_pawn_square, black_pawn);

        let moves = white_pawn.generate_moves(&board, row, col).unwrap();

        assert_eq!(moves.len(), 2);

        // Check that the pawn can move one square forward
        assert!(moves.contains(&Move {
            initial_square: 27,
            target_square: 35,
        }));

        // Check that the pawn can take the black pawn
        assert!(moves.contains(&Move {
            initial_square: 27,
            target_square: 36,
        }));
    }

    #[test]
    fn test_generate_pawn_move_black_takes_piece_from_starting_row() {
        let mut board = Board::new_empty_board();
        let row = 6;
        let col = 3;

        let white_pawn_square = (row - 1) * BOARD_SIZE + col + 1;
        let black_pawn_square = row * BOARD_SIZE + col;

        let white_pawn = Piece {
            piece_type: Pawn,
            color: White,
        };
        let black_pawn = Piece {
            piece_type: Pawn,
            color: Black,
        };

        board.set_piece(white_pawn_square, white_pawn);
        board.set_piece(black_pawn_square, black_pawn);

        let moves = black_pawn.generate_moves(&board, row, col).unwrap();

        assert_eq!(moves.len(), 3);

        // Check that the pawn can move one square forward
        assert!(moves.contains(&Move {
            initial_square: 51,
            target_square: 43,
        }));

        // Check that the pawn can take the white pawn
        assert!(moves.contains(&Move {
            initial_square: 51,
            target_square: 44,
        }));

        // Check that pawn can move two square forward
        assert!(moves.contains(&Move {
            initial_square: 51,
            target_square: 35,
        }));
    }

    #[test]
    fn test_generate_pawn_move_black_takes_piece() {
        let mut board = Board::new_empty_board();
        let row = 4;
        let col = 3;

        let white_pawn_square = (row - 1) * BOARD_SIZE + col - 1;
        let black_pawn_square = row * BOARD_SIZE + col;

        let white_pawn = Piece {
            piece_type: Pawn,
            color: White,
        };
        let black_pawn = Piece {
            piece_type: Pawn,
            color: Black,
        };

        board.set_piece(white_pawn_square, white_pawn);
        board.set_piece(black_pawn_square, black_pawn);

        let moves = black_pawn.generate_moves(&board, row, col).unwrap();

        assert_eq!(moves.len(), 2);

        // Check that the pawn can move one square forward
        assert!(moves.contains(&Move {
            initial_square: 35,
            target_square: 27,
        }));

        // Check that the pawn can take the white pawn
        assert!(moves.contains(&Move {
            initial_square: 35,
            target_square: 26,
        }));
    }

    #[test]
    fn test_generate_pawn_move_white_on_first_col() {
        let mut board = Board::new_empty_board();
        let row = 3;
        let col = 0;

        let white_pawn_square = row * BOARD_SIZE + col;
        let black_pawn_square = (row + 1) * BOARD_SIZE + col + 1;

        let white_pawn = Piece {
            piece_type: Pawn,
            color: White,
        };
        let black_pawn = Piece {
            piece_type: Pawn,
            color: Black,
        };

        board.set_piece(white_pawn_square, white_pawn);
        board.set_piece(black_pawn_square, black_pawn);

        let moves = white_pawn.generate_moves(&board, row, col).unwrap();

        assert_eq!(moves.len(), 2);

        // Check that the pawn can move one square forward
        assert!(moves.contains(&Move {
            initial_square: 24,
            target_square: 32,
        }));

        // Check that the pawn can take the black pawn
        assert!(moves.contains(&Move {
            initial_square: 24,
            target_square: 33,
        }));
    }

    #[test]
    fn test_generate_pawn_move_white_on_last_col() {
        let mut board = Board::new_empty_board();
        let row = 3;
        let col = 7;

        let white_pawn_square = row * BOARD_SIZE + col;
        let black_pawn_square = (row + 1) * BOARD_SIZE + col - 1;

        let white_pawn = Piece {
            piece_type: Pawn,
            color: White,
        };
        let black_pawn = Piece {
            piece_type: Pawn,
            color: Black,
        };

        board.set_piece(white_pawn_square, white_pawn);
        board.set_piece(black_pawn_square, black_pawn);

        let moves = white_pawn.generate_moves(&board, row, col).unwrap();

        assert_eq!(moves.len(), 2);

        // Check that the pawn can move one square forward
        assert!(moves.contains(&Move {
            initial_square: 31,
            target_square: 39,
        }));

        // Check that the pawn can take the black pawn
        assert!(moves.contains(&Move {
            initial_square: 31,
            target_square: 38,
        }));
    }

    #[test]
    fn test_generate_pawn_move_black_on_first_col() {
        let mut board = Board::new_empty_board();
        let row = 6;
        let col = 0;

        let white_pawn_square = (row - 1) * BOARD_SIZE + col + 1;
        let black_pawn_square = row * BOARD_SIZE + col;

        let white_pawn = Piece {
            piece_type: Pawn,
            color: White,
        };
        let black_pawn = Piece {
            piece_type: Pawn,
            color: Black,
        };

        board.set_piece(white_pawn_square, white_pawn);
        board.set_piece(black_pawn_square, black_pawn);

        let moves = black_pawn.generate_moves(&board, row, col).unwrap();

        assert_eq!(moves.len(), 3);

        // Check that the pawn can move one square forward
        assert!(moves.contains(&Move {
            initial_square: 48,
            target_square: 40,
        }));

        // Check that the pawn can take the white pawn
        assert!(moves.contains(&Move {
            initial_square: 48,
            target_square: 41,
        }));

        // Check that pawn can move two square forward
        assert!(moves.contains(&Move {
            initial_square: 48,
            target_square: 32,
        }));
    }

    #[test]
    fn test_generate_pawn_move_black_on_last_col() {
        let mut board = Board::new_empty_board();
        let row = 6;
        let col = 7;

        let white_pawn_square = (row - 1) * BOARD_SIZE + col - 1;
        let black_pawn_square = row * BOARD_SIZE + col;

        let white_pawn = Piece {
            piece_type: Pawn,
            color: White,
        };
        let black_pawn = Piece {
            piece_type: Pawn,
            color: Black,
        };

        board.set_piece(white_pawn_square, white_pawn);
        board.set_piece(black_pawn_square, black_pawn);

        let moves = black_pawn.generate_moves(&board, row, col).unwrap();

        assert_eq!(moves.len(), 3);

        // Check that the pawn can move one square forward
        assert!(moves.contains(&Move {
            initial_square: 55,
            target_square: 47,
        }));

        // Check that the pawn can take the white pawn
        assert!(moves.contains(&Move {
            initial_square: 55,
            target_square: 46,
        }));

        // Check that pawn can move two square forward
        assert!(moves.contains(&Move {
            initial_square: 55,
            target_square: 39,
        }));
    }

    #[test]
    fn test_generate_pawn_move_white_blocked() {
        let mut board = Board::new_empty_board();
        let row = 3;
        let col = 0;

        let white_pawn_square = row * BOARD_SIZE + col;
        let black_pawn_square = (row + 1) * BOARD_SIZE + col;

        let white_pawn = Piece {
            piece_type: Pawn,
            color: White,
        };
        let black_pawn = Piece {
            piece_type: Pawn,
            color: Black,
        };

        board.set_piece(white_pawn_square, white_pawn);
        board.set_piece(black_pawn_square, black_pawn);

        let moves = white_pawn.generate_moves(&board, row, col);

        assert_eq!(moves, None);
    }

    #[test]
    fn test_generate_pawn_move_black_blocked() {
        let mut board = Board::new_empty_board();
        let row = 6;
        let col = 0;

        let white_pawn_square = (row - 1) * BOARD_SIZE + col;
        let black_pawn_square = row * BOARD_SIZE + col;

        let white_pawn = Piece {
            piece_type: Pawn,
            color: White,
        };
        let black_pawn = Piece {
            piece_type: Pawn,
            color: Black,
        };

        board.set_piece(white_pawn_square, white_pawn);
        board.set_piece(black_pawn_square, black_pawn);

        let moves = black_pawn.generate_moves(&board, row, col);

        assert_eq!(moves, None);
    }
}
