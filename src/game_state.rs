use crate::piece::{Piece, PieceType, Color};
use crate::moves::Move;

const BOARD_SIZE:usize = 8;


pub struct GameState {
    pub board: Vec<Option<Piece>>,
    pub current_player: Color,
    pub turn: u32,
}

impl GameState {

    fn get_piece(&self, square: u8) -> Option<&Piece> {
        self.board.get(square as usize).and_then(|piece| piece.as_ref())
    }
    
    pub fn generate_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Some(piece) = self.board[row * BOARD_SIZE + col] {

                    // Only check moves of the pieces that match the current player
                    if piece.color != self.current_player{continue;}

                    let piece_moves = match piece.piece_type {
                        PieceType::Pawn => self.generate_pawn_moves(row, col, piece.color),
                        PieceType::Bishop => self.generate_bishop_moves(row, col),
                        PieceType::Knight => self.generate_knight_moves(row, col),
                        PieceType::Rook => self.generate_rook_moves(row, col),
                        PieceType::Queen => self.generate_queen_moves(row, col),
                        PieceType::King => self.generate_king_moves(row, col),
                    };
                    // Add it to the list if there is a move
                    if let Some(valid_move) = piece_moves{
                        moves.extend(valid_move);
                    }
                }
            }
        }
        println!("There are {} moves in this position.", moves.len());
        moves
    }

    fn generate_pawn_moves(&self, row: usize, col: usize, piece_color: Color) -> Option<Vec<Move>> {
        // TODO create pawn.rs and move it there?

        let mut moves: Vec<Move> = Vec::new();


        // Calculate the direction based on the piece color
        let direction = match piece_color {
            Color::White => 1, // Moving up (increasing row index)
            Color::Black => -1,  // Moving down (decreasing row index)
        };

        // Calculate the initial square number
        let initial_square = (row * BOARD_SIZE + col) as u8;

        // Calculate the destination row for the pawn's move
        let new_row = (row as isize + direction) as usize;

        // Ensure that the destination row is within bounds
        if new_row >= BOARD_SIZE {
            return None; // Invalid move, the pawn is off the board
        }

        // One square movement
        // Calculate the target square number
        let target_square = (new_row * BOARD_SIZE + col) as u8;

        // Check if the destination square is empty
        if self.get_piece(target_square).is_none() {
            // The destination square is empty, so it's a valid non-capturing move
            moves.push(Move {
                initial_square,
                target_square,
            });
        }

        // Two square movement
        // Check if it's the pawn's first move and if the two-square move is available
        if (row == 1 && piece_color == Color::White) || (row == 6 && piece_color == Color::Black) {
            // Calculate the target square number for the two-square move
            let two_square_target = ((row as isize + 2 * direction) * BOARD_SIZE as isize + col as isize) as u8;

            // Check if the two-square move destination is empty
            if self.get_piece(two_square_target).is_none() {
                // The two-square move is available, add it to the list of valid moves
                moves.push(Move {
                    initial_square,
                    target_square: two_square_target,
                });
            }
        }

        // Captures TODO
        // let mut left_new_col = BOARD_SIZE;
        // let mut right_new_col = BOARD_SIZE;

        // if col - 1 >= 0 {
        //     left_new_col = col - 1
        // }
        // if col + 1 < BOARD_SIZE {
        //     right_new_col = col + 1;
        // }        

        // TODO: Implement capturing moves (diagonally) if there's an opponent's piece
        // TODO: Implement en passant
        // TODO: Implement promotion

        if moves.len() > 0{
            return Some(moves);
        }
        None
    }

    fn generate_bishop_moves(&self, row: usize, col: usize) -> Option<Vec<Move>> {
        // TODO
        None
    }

    fn generate_knight_moves(&self, row: usize, col: usize) -> Option<Vec<Move>> {
        // TODO
        None
    }

    fn generate_rook_moves(&self, row: usize, col: usize) -> Option<Vec<Move>> {
        // TODO
        None
    }

    fn generate_queen_moves(&self, row: usize, col: usize) -> Option<Vec<Move>> {
        // TODO
        None
    }

    fn generate_king_moves(&self, row: usize, col: usize) -> Option<Vec<Move>> {
        // TODO
        None
    }



    pub fn print_board(&self) {
        let board_size = 8; // Assuming an 8x8 chessboard

        println!("  +------------------------+");

        for row in (0..BOARD_SIZE).rev() {
            print!("{} |", row + 1);

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