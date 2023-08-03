#[cfg(test)]
use crate::board::*;

#[cfg(test)]
mod board_tests{
    use super::*;
    #[test]
    fn test_starting_board_from_str(){
        let board_from_fen = Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
        let board_hardcoded = starting_position();
        assert_eq!(board_from_fen, board_hardcoded);
    }
}
#[cfg(test)]
fn starting_position() -> Board {
    let mut board_hardcoded = Board::new_empty_board();
        board_hardcoded.set_piece(0, Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        }));
        board_hardcoded.set_piece(1, Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::White,
        }));
        board_hardcoded.set_piece(2, Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        }));
        board_hardcoded.set_piece(3, Some(Piece {
            piece_type: PieceType::Queen,
            color: Color::White,
        }));
        board_hardcoded.set_piece(4, Some(Piece {
            piece_type: PieceType::King,
            color: Color::White,
        }));
        board_hardcoded.set_piece(5, Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        }));
        board_hardcoded.set_piece(6, Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::White,
        }));
        board_hardcoded.set_piece(7, Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        }));
        board_hardcoded.set_piece(8, Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        }));
        board_hardcoded.set_piece(9, Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        }));
        board_hardcoded.set_piece(10, Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        }));
        board_hardcoded.set_piece(11, Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        }));
        board_hardcoded.set_piece(12, Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        }));
        board_hardcoded.set_piece(13, Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        }));
        board_hardcoded.set_piece(14, Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        }));
        board_hardcoded.set_piece(15, Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        }));

        board_hardcoded.set_piece(48, Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        }));
        board_hardcoded.set_piece(49, Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        }));
        board_hardcoded.set_piece(50, Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        }));
        board_hardcoded.set_piece(51, Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        }));
        board_hardcoded.set_piece(52, Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        }));
        board_hardcoded.set_piece(53, Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        }));
        board_hardcoded.set_piece(54, Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        }));
        board_hardcoded.set_piece(55, Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        }));
        board_hardcoded.set_piece(56, Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::Black,
        }));
        board_hardcoded.set_piece(57, Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::Black,
        }));
        board_hardcoded.set_piece(58, Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::Black,
        }));
        board_hardcoded.set_piece(59, Some(Piece {
            piece_type: PieceType::Queen,
            color: Color::Black,
        }));
        board_hardcoded.set_piece(60, Some(Piece {
            piece_type: PieceType::King,
            color: Color::Black,
        }));
        board_hardcoded.set_piece(61, Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::Black,
        }));
        board_hardcoded.set_piece(62, Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::Black,
        }));
        board_hardcoded.set_piece(63, Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::Black,
        }));
        board_hardcoded
}