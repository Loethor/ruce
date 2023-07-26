use crate::piece::{Piece, PieceType, Color};
use crate::moves::Move;

const BOARD_SIZE:usize = 8;


pub struct GameState {
    pub board: Vec<Option<Piece>>,
    pub current_player: Color,
    pub turn: u32,
}

impl GameState {
    
    pub fn generate_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.board[row * BOARD_SIZE + col] {
                    let piece_moves = match piece.piece_type {
                        PieceType::Pawn => self.generate_pawn_moves(row, col),
                        PieceType::Bishop => self.generate_bishop_moves(row, col),
                        PieceType::Knight => self.generate_knight_moves(row, col),
                        PieceType::Rook => self.generate_rook_moves(row, col),
                        PieceType::Queen => self.generate_queen_moves(row, col),
                        PieceType::King => self.generate_king_moves(row, col),
                    };
                    moves.extend(piece_moves);
                }
            }
        }

        moves
    }

    fn generate_pawn_moves(&self, row: usize, col: usize) -> Vec<Move> {
        // TODO
        Vec::new()
    }

    fn generate_bishop_moves(&self, row: usize, col: usize) -> Vec<Move> {
        // TODO
        Vec::new()
    }

    fn generate_knight_moves(&self, row: usize, col: usize) -> Vec<Move> {
        // TODO
        Vec::new()
    }

    fn generate_rook_moves(&self, row: usize, col: usize) -> Vec<Move> {
        // TODO
        Vec::new()
    }

    fn generate_queen_moves(&self, row: usize, col: usize) -> Vec<Move> {
        // TODO
        Vec::new()
    }

    fn generate_king_moves(&self, row: usize, col: usize) -> Vec<Move> {
        // TODO
        Vec::new()
    }



    pub fn print_board(&self) {
        let board_size = 8; // Assuming an 8x8 chessboard

        println!("  +------------------------+");

        for row in (0..BOARD_SIZE).rev() {
            print!("{} |", row);

            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.board[row * BOARD_SIZE + col] {
                    let piece_char = piece.to_char();
                    print!(" {} ", piece_char);
                } else {
                    print!(" . ");
                }
            }

            println!("|");
        }

        println!("  +------------------------+");
        println!("    a  b  c  d  e  f  g  h ");

    }
}