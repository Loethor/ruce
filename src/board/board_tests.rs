use crate::board::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_starting_board_from_str() {
        let board_from_fen =
            Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
        let board_hardcoded = starting_position();
        assert_eq!(board_from_fen, board_hardcoded);
    }

    #[test]
    fn test_error_fen_string() {
        let board_from_fen =
            Board::from_str("rnbqkbna/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap_err();
        assert_eq!(
            board_from_fen,
            ParseFenError::InvalidPiecePlacement("a".to_string())
        );
    }
}

#[cfg(test)]
fn starting_position() -> Board {
    let mut board_hardcoded = Board::new_empty_board();
    board_hardcoded.set_piece(
        0,
        Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        },
    );
    board_hardcoded.set_piece(
        1,
        Piece {
            piece_type: PieceType::Knight,
            color: Color::White,
        },
    );
    board_hardcoded.set_piece(
        2,
        Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        },
    );
    board_hardcoded.set_piece(
        3,
        Piece {
            piece_type: PieceType::Queen,
            color: Color::White,
        },
    );
    board_hardcoded.set_piece(
        4,
        Piece {
            piece_type: PieceType::King,
            color: Color::White,
        },
    );
    board_hardcoded.set_piece(
        5,
        Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        },
    );
    board_hardcoded.set_piece(
        6,
        Piece {
            piece_type: PieceType::Knight,
            color: Color::White,
        },
    );
    board_hardcoded.set_piece(
        7,
        Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        },
    );
    board_hardcoded.set_piece(
        8,
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        },
    );
    board_hardcoded.set_piece(
        9,
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        },
    );
    board_hardcoded.set_piece(
        10,
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        },
    );
    board_hardcoded.set_piece(
        11,
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        },
    );
    board_hardcoded.set_piece(
        12,
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        },
    );
    board_hardcoded.set_piece(
        13,
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        },
    );
    board_hardcoded.set_piece(
        14,
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        },
    );
    board_hardcoded.set_piece(
        15,
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        },
    );

    board_hardcoded.set_piece(
        48,
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        },
    );
    board_hardcoded.set_piece(
        49,
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        },
    );
    board_hardcoded.set_piece(
        50,
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        },
    );
    board_hardcoded.set_piece(
        51,
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        },
    );
    board_hardcoded.set_piece(
        52,
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        },
    );
    board_hardcoded.set_piece(
        53,
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        },
    );
    board_hardcoded.set_piece(
        54,
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        },
    );
    board_hardcoded.set_piece(
        55,
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        },
    );
    board_hardcoded.set_piece(
        56,
        Piece {
            piece_type: PieceType::Rook,
            color: Color::Black,
        },
    );
    board_hardcoded.set_piece(
        57,
        Piece {
            piece_type: PieceType::Knight,
            color: Color::Black,
        },
    );
    board_hardcoded.set_piece(
        58,
        Piece {
            piece_type: PieceType::Bishop,
            color: Color::Black,
        },
    );
    board_hardcoded.set_piece(
        59,
        Piece {
            piece_type: PieceType::Queen,
            color: Color::Black,
        },
    );
    board_hardcoded.set_piece(
        60,
        Piece {
            piece_type: PieceType::King,
            color: Color::Black,
        },
    );
    board_hardcoded.set_piece(
        61,
        Piece {
            piece_type: PieceType::Bishop,
            color: Color::Black,
        },
    );
    board_hardcoded.set_piece(
        62,
        Piece {
            piece_type: PieceType::Knight,
            color: Color::Black,
        },
    );
    board_hardcoded.set_piece(
        63,
        Piece {
            piece_type: PieceType::Rook,
            color: Color::Black,
        },
    );
    board_hardcoded
}

#[cfg(test)]
mod move_tests {
    use super::*;
    use crate::board::piece::Piece;
    use crate::board::piece::PieceType::Pawn;

    /// Helper function to create a test board with a given piece at a specific position
    fn create_test_board_with_piece(
        piece_type: PieceType,
        piece_color: Color,
        row: usize,
        col: usize,
    ) -> Board {
        let mut board = Board::new_empty_board();
        let piece = Piece {
            color: piece_color,
            piece_type,
        };
        let square_index = row * BOARD_SIZE + col;
        board.squares[square_index] = Some(piece);
        board
    }

    // Pawn tests
    #[test]
    fn test_generate_pawn_moves_white() {
        let mut board = Board::new_empty_board();
        let row = 1;
        let col = 3;
        let square = row * BOARD_SIZE + col;
        let piece = Piece {
            piece_type: Pawn,
            color: Color::White,
        };
        board.set_piece(square as u8, piece);

        let moves = board.generate_pawn_moves(1, 3, piece.color).unwrap();
        assert_eq!(moves.len(), 2);

        // Check that the pawn can move one square forward
        assert!(moves.contains(&Move {
            initial_square: 11,
            target_square: 19,
        }));

        // Check that the pawn can move two squares forward from its starting position
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
            color: Color::Black,
        };
        board.set_piece(square as u8, piece);

        let moves = board.generate_pawn_moves(6, 3, piece.color).unwrap();
        assert_eq!(moves.len(), 2);

        // Check that the pawn can move one square forward
        assert!(moves.contains(&Move {
            initial_square: 51,
            target_square: 43,
        }));

        // Check that the pawn can move two squares forward from its starting position
        assert!(moves.contains(&Move {
            initial_square: 51,
            target_square: 35,
        }));
    }

    #[test]
    fn test_generate_pawn_moves_no_moves() {
        let mut board = Board::new_empty_board();
        let row = 6;
        let col = 3;
        let square = row * BOARD_SIZE + col;
        let piece = Piece {
            piece_type: Pawn,
            color: Color::Black,
        };
        board.set_piece(square as u8, piece);

        // piece blocking
        let row = 5;
        let col = 3;
        let square = row * BOARD_SIZE + col;
        let blocking_piece = Piece {
            piece_type: Pawn,
            color: Color::Black,
        };
        board.set_piece(square as u8, blocking_piece);

        let moves = board.generate_pawn_moves(6, 3, piece.color);
        assert!(moves.is_none());
    }

    #[test]
    fn test_generate_pawn_moves_captures() {
        let mut board = Board::new_empty_board();
        let row = 6;
        let col = 3;
        let square = row * BOARD_SIZE + col;
        let piece = Piece {
            piece_type: Pawn,
            color: Color::Black,
        };
        board.set_piece(square as u8, piece);

        // capturable piece
        let row = 5;
        let col = 2;
        let square = row * BOARD_SIZE + col;
        let capturable_piece = Piece {
            piece_type: Pawn,
            color: Color::White,
        };
        board.set_piece(square as u8, capturable_piece);

        let moves = board.generate_pawn_moves(6, 3, piece.color).unwrap();
        assert_eq!(moves.len(), 3);

        // Check that the pawn can move one square forward
        assert!(moves.contains(&Move {
            initial_square: 51,
            target_square: 43,
        }));
    }

    // TODO add test cases for en passant, and promotion
    // End Pawn tests
}
