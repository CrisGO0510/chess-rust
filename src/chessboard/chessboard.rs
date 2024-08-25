use crate::chessboard::utilities::clear_console;
use crate::pieces::colors;
use crate::pieces::piece_type::{ChessPiece, ChessPieceColor, ChessPieceType, Message};

use super::chessboard_validation::{is_check, is_checkmate, validate_move};
use super::player::Player;

#[derive(Clone)]
pub struct Chessboard {
    pub board: [[Option<ChessPiece>; 8]; 8],
    pub player_turn: ChessPieceColor,
    pub player1: Player,
    pub player2: Player,
}

impl Chessboard {
    #[rustfmt::skip]
    pub fn new() -> Self {
        let mut board: [[Option<ChessPiece>; 8]; 8] = [[None; 8]; 8];

        // Piezas negros
        board[0][0] = Some(ChessPiece { piece: ChessPieceType::Rook, color: ChessPieceColor::Black, position: [0, 0] , before_position: None});
        board[0][1] = Some(ChessPiece { piece: ChessPieceType::Knight, color: ChessPieceColor::Black, position: [0, 1] , before_position: None});
        board[0][2] = Some(ChessPiece { piece: ChessPieceType::Bishop, color: ChessPieceColor::Black, position: [0, 2] , before_position: None});
        board[0][3] = Some(ChessPiece { piece: ChessPieceType::Queen, color: ChessPieceColor::Black, position: [0, 3] , before_position: None});
        board[0][4] = Some(ChessPiece { piece: ChessPieceType::King, color: ChessPieceColor::Black, position: [0, 4] , before_position: None});
        board[0][5] = Some(ChessPiece { piece: ChessPieceType::Bishop, color: ChessPieceColor::Black, position: [0, 5] , before_position: None});
        board[0][6] = Some(ChessPiece { piece: ChessPieceType::Knight, color: ChessPieceColor::Black, position: [0, 6] , before_position: None});
        board[0][7] = Some(ChessPiece { piece: ChessPieceType::Rook, color: ChessPieceColor::Black, position: [0, 7] , before_position: None});

        // Peones negros
        for col in 0..8 {
            board[1][col] = Some(ChessPiece { piece: ChessPieceType::Pawn, color: ChessPieceColor::Black, position: [1, col] , before_position: None});
        }

        // Piezas blanca
        board[7][0] = Some(ChessPiece { piece: ChessPieceType::Rook, color: ChessPieceColor::White, position: [7, 0] , before_position: None});
        board[7][1] = Some(ChessPiece { piece: ChessPieceType::Knight, color: ChessPieceColor::White, position: [7, 1] , before_position: None});
        board[7][2] = Some(ChessPiece { piece: ChessPieceType::Bishop, color: ChessPieceColor::White, position: [7, 2] , before_position: None});
        board[7][3] = Some(ChessPiece { piece: ChessPieceType::Queen, color: ChessPieceColor::White, position: [7, 3] , before_position: None});
        board[7][4] = Some(ChessPiece { piece: ChessPieceType::King, color: ChessPieceColor::White, position: [7, 4] , before_position: None});
        // board[7][5] = Some(ChessPiece { piece: ChessPieceType::Bishop, color: ChessPieceColor::White, position: [7, 5] , before_position: None});
        // board[7][6] = Some(ChessPiece { piece: ChessPieceType::Knight, color: ChessPieceColor::White, position: [7, 6] , before_position: None});
        board[7][7] = Some(ChessPiece { piece: ChessPieceType::Rook, color: ChessPieceColor::White, position: [7, 7] , before_position: None});

        // Peones blancos
        for col in 0..8 {
            board[6][col] = Some(ChessPiece { piece: ChessPieceType::Pawn, color: ChessPieceColor::White, position: [6, col] , before_position: None});
        }

        Chessboard { board , player_turn: ChessPieceColor::White,
            player1: Player {
                name: "Blanco".to_string(),
                king_position: [7, 4],
            },
            player2: Player {
                name: "Negro".to_string(),
                king_position: [0, 4],
            },
        }
    }

    pub fn print_board(&self, message: String) {
        let player_turn = match self.player_turn {
            ChessPieceColor::White => {
                colors::BRIGHT_WHITE.to_string()
                    + "Sigue jugador "
                    + &self.player1.name
                    + colors::RESET
            }
            ChessPieceColor::Black => {
                colors::BRIGHT_BLACK.to_string()
                    + "Sigue jugador "
                    + &self.player2.name
                    + colors::RESET
            }
        };

        // clear_console();

        let mut row_index = 8;
        println!("      a   b   c   d   e   f   g   h");
        print!("    ┌───┬───┬───┬───┬───┬───┬───┬───┐\n  {} ", row_index);

        for row in self.board.iter() {
            for cell in row.iter() {
                match cell {
                    // Some(piece) => print!("│ {:?} ", piece.position),
                    Some(piece) => print!("│ {} ", piece.to_char()),
                    None => print!("│   "),
                }
            }
            row_index -= 1;

            println!("│");

            if row_index != 0 {
                print!(
                    "    ├───┼───┼───┼───┼───┼───┼───┼───┤{}\n  {} ",
                    if row_index == 3 {
                        format!("\t{}", player_turn)
                    } else if row_index == 4 {
                        format!("\t{}", message)
                    } else {
                        String::new()
                    },
                    row_index
                );
            }
        }
        println!("    └───┴───┴───┴───┴───┴───┴───┴───┘");
    }

    pub fn move_piece(&mut self, from: [usize; 2], to: [usize; 2]) -> Message {
        let from_position = self.board[from[0]][from[1]];
        let to_position = self.board[to[0]][to[1]];

        // Validamos el movimiento
        let validation_result = validate_move(self, &from_position, &to_position, to);

        match validation_result {
            Ok(new_chessboard) => {
                // Actualizamos el tablero con la nueva instancia
                *self = new_chessboard;

                // Determinamos si hay jaque
                if let Some(attacker_position) = is_check(self, self.player_turn) {
                    // Determinamos si el jaque es jaque mate
                    if is_checkmate(self, self.player_turn, attacker_position) {
                        return Message::CheckMate;
                    }

                    return Message::Check;
                }

                Message::Success
            }
            Err(message) => message,
        }
    }
}
