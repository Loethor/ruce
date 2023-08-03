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
