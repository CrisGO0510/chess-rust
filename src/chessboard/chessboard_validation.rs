use crate::pieces::{
    piece_actions::PieceActions,
    piece_type::{ChessPiece, ChessPieceColor, ChessPieceType},
};

use super::chessboard::Chessboard;

pub fn validate_move(
    chessboard: &Chessboard,
    from_position: &Option<ChessPiece>,
    to_position: &Option<ChessPiece>,
    to: [usize; 2],
) -> Result<(), String> {
    match from_position {
        Some(from_piece) => {
            // Validamos si es el turno del jugador
            if from_piece.color != chessboard.player_turn {
                return Err("La pieza no es de tu color.".to_string());
            }

            match to_position {
                Some(to_piece) => {
                    // Validamos si la pieza es del mismo color
                    if from_piece.color == to_piece.color {
                        return Err("No puedes mover una pieza a una posición ocupada por una pieza del mismo color.".to_string());
                    }

                    // Validamos si la pieza es un rey
                    if from_piece.piece == ChessPieceType::King {
                        return Err("No puedes capturar al rey.".to_string());
                    }

                    let moves = from_piece.capture_piece(to);

                    // Validamos si la pieza puede realizar el movimiento
                    if moves.is_empty() {
                        return Err("No puedes mover la pieza a esa posición.".to_string());
                    }

                    // Validamos si hay una pieza en el camino
                    for move_position in moves {
                        if move_position == to {
                            continue;
                        }

                        if chessboard.board[move_position[0]][move_position[1]].is_some() {
                            return Err("Hay una pieza en el camino.".to_string());
                        }
                    }
                }
                None => {
                    let moves = from_piece.move_piece(to);

                    // Validamos si la pieza puede realizar el movimiento
                    if moves.is_empty() {
                        return Err("No puedes mover la pieza a esa posición.".to_string());
                    }

                    // Validamos si hay una pieza en el camino
                    for move_position in moves {
                        if chessboard.board[move_position[0]][move_position[1]].is_some() {
                            return Err("Hay una pieza en el camino.".to_string());
                        }
                    }
                }
            }

            // Validamos que no quede en jaque después del movimiento
            if is_check(chessboard) {
                return Err("No puedes dejar al rey en jaque.".to_string());
            }

            Ok(())
        }
        None => Err("No hay una pieza en la posición de origen.".to_string()),
    }
}

pub fn is_check(chessboard: &Chessboard) -> bool {
    // let king_position = match chessboard.player_turn {
    //     ChessPieceColor::White => chessboard.player1.king_position,
    //     ChessPieceColor::Black => chessboard.player2.king_position,
    // };

    // for i in 0..8 {
    //     for j in 0..8 {
    //         let from_position = chessboard.board[i][j];

    //         if let Some(from_piece) = from_position {
    //             if from_piece.color != chessboard.player_turn {
    //                 continue;
    //             }

    //             let moves = from_piece.capture_piece(king_position);

    //             // Validamos si la pieza puede realizar el movimiento
    //             if moves.is_empty() {
    //                 continue;
    //             }

    //             // Validamos si hay una pieza en el camino
    //             for move_position in moves {
    //                 if chessboard.board[move_position[0]][move_position[1]].is_some() {
    //                     continue;
    //                 }
    //             }

    //             println!(
    //                 "¡El rey está en jaque! por la pieza {} en la posición {:?}",
    //                 from_piece.to_char(),
    //                 from_piece.position
    //             );
    //             return true;
    //         }
    //     }
    // }

    return false;
}
