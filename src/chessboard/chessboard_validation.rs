use crate::pieces::{
    piece_actions::PieceActions,
    piece_type::{ChessPiece, ChessPieceColor, ChessPieceType},
};

use super::{chessboard::Chessboard, utilities::new_chessboard_instance_after_move};

pub fn validate_move(
    chessboard: &Chessboard,
    from_position: &Option<ChessPiece>,
    to_position: &Option<ChessPiece>,
    to: [usize; 2],
) -> Result<Chessboard, String> {
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
                    if to_piece.piece == ChessPieceType::King {
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

            // Creamos una instancia temporal de chessboard
            let temp_chessboard = new_chessboard_instance_after_move(chessboard, from_piece, to);

            // Validamos que no quede en jaque después del movimiento
            if is_check(&temp_chessboard, chessboard.player_turn).is_some() {
                return Err("No puedes dejar al rey en jaque.".to_string());
            }

            Ok(temp_chessboard)
        }
        None => Err("No hay una pieza en la posición de origen.".to_string()),
    }
}

pub fn is_check(chessboard: &Chessboard, player_color: ChessPieceColor) -> Option<[usize; 2]> {
    let king_position = match player_color {
        ChessPieceColor::White => chessboard.player1.king_position,
        ChessPieceColor::Black => chessboard.player2.king_position,
    };

    for i in 0..8 {
        for j in 0..8 {
            if let Some(from_piece) = chessboard.board[i][j] {
                // Si la pieza es del mismo color, continuamos
                if from_piece.color == player_color {
                    continue;
                }

                // Obtenemos los movimientos de la pieza para capturar al rey
                let moves = from_piece.capture_piece(king_position);

                // Validamos si la pieza puede realizar el movimiento
                if moves.is_empty() {
                    continue;
                }

                // Variable para validar si hay una pieza en el camino
                let mut piece_in_path: bool = false;

                // Validamos si hay una pieza en el camino, siendo el caso continuamos
                for move_position in moves {
                    if move_position == king_position {
                        continue;
                    }

                    if chessboard.board[move_position[0]][move_position[1]].is_some() {
                        println!("Hay algo en el camino {:?}", move_position);

                        piece_in_path = true;
                        break;
                    }
                }

                // Si hay una pieza en el camino, continuamos buscando
                if piece_in_path {
                    continue;
                }

                // En caso de no haber una pieza en el camino, notificamos que el rey está en jaque
                println!(
                    "¡El rey está en jaque! por la pieza {} en la posición {:?}",
                    from_piece.to_char(),
                    from_piece.position
                );

                // Retornamos la posición de la pieza que pone en jaque al rey
                return Some(from_piece.position);
            }
        }
    }

    // En caso de que no haya jaque, retornamos un none
    return None;
}
