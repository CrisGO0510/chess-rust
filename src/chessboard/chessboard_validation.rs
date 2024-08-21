use crate::pieces::allowed_moves::king_allowed_moves;
use crate::pieces::piece_actions::PieceActions;
use crate::pieces::piece_type::{ChessPiece, ChessPieceColor, ChessPieceType};

use super::{chessboard::Chessboard, utilities::new_chessboard_instance_after_move};
/**
 Valida si un movimiento es legal en el tablero de ajedrez
 # Arguments
 * `chessboard` - Una referencia al tablero de ajedrez actual.
 * `from_position` - Una referencia a una opción que contiene la pieza que se va a mover, si existe.
 * `to_position` - Una referencia a una opción que contiene la pieza en la posición destino, si existe.
 * `to` - La posición destino del movimiento, representada como un arreglo de dos elementos `[usize; 2]`.
 # Returns
 Retorna un `Result` que puede ser:
 - `Ok(Chessboard)`: Un nuevo tablero de ajedrez después de realizar el movimiento si es válido.
 - `Err(String)`: Un mensaje de error en caso de que el movimiento no sea válido, explicando la razón del error.
*/
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
                    if validate_piece_in_path(chessboard, moves, to) {
                        return Err("Hay una pieza en el camino.".to_string());
                    }
                }
                None => {
                    let moves = from_piece.move_piece(to);

                    // Validamos si la pieza puede realizar el movimiento
                    if moves.is_empty() {
                        return Err("No puedes mover la pieza a esa posición.".to_string());
                    }

                    // Validamos si hay una pieza en el camino
                    if validate_piece_in_path(chessboard, moves, to) {
                        return Err("Hay una pieza en el camino.".to_string());
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

/**
 Función que valida si el rey está en jaque.
 # Arguments
 * `chessboard` - Instancia del tablero de ajedrez de la cual se quiere validar.
 * `player_color` - El color del jugador actual.
 # Returns
  `Option<[usize; 2]>`: Retorna la posición de la pieza que pone en jaque al rey, si la hay.
*/
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

                // Si hay una pieza en el camino, continuamos buscando
                if validate_piece_in_path(chessboard, moves, king_position) {
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

pub fn is_checkmate(
    chessboard: &Chessboard,
    player_color: ChessPieceColor,
    attacker_position: [usize; 2],
) -> bool {
    let king_position = match player_color {
        ChessPieceColor::White => chessboard.player1.king_position,
        ChessPieceColor::Black => chessboard.player2.king_position,
    };

    // Obtenemos la información de la pieza que pone en jaque al rey
    let attacking_piece = chessboard.board[attacker_position[0]][attacker_position[1]].unwrap();
    let attack_route = attacking_piece.capture_piece(king_position);

    // Analizamos si el rey puede moverse a una posición segura
    let king_moves = king_allowed_moves(king_position);

    // Iteramos cada uno de los movimientos del rey
    for move_position in king_moves {
        // Validamos si el rey puede moverse a una posición segura
        if attack_route.contains(&move_position) {
            continue;
        }

        // Creamos una instancia temporal de chessboard, para analizar si el rey sigue en jaque después del movimiento
        let temp_chessboard =
            new_chessboard_instance_after_move(chessboard, &attacking_piece, move_position);

        if is_check(&temp_chessboard, player_color).is_none() {
            return false;
        }
    }

    // Iteramos cada una de las piezas del jugador
    for i in 0..8 {
        for j in 0..8 {
            if let Some(from_piece) = chessboard.board[i][j] {
                // Si la pieza es del mismo color, continuamos
                if from_piece.color != player_color {
                    continue;
                }

                // Si la pieza es un rey, continuamos, ya que ya validamos sus posibles casos anteriormente
                if from_piece.piece == ChessPieceType::King {
                    continue;
                }

                // Validamos si la pieza puede capturar a la pieza atacante
                let moves = from_piece.capture_piece(attacker_position);

                if !moves.is_empty() {
                    // Validamos si hay una pieza en el camino
                    if validate_piece_in_path(chessboard, moves, attacker_position) {
                        continue;
                    }

                    // Creamos una instancia temporal de chessboard, para analizar si el rey sigue en jaque después de capturar la pieza atacante
                    let temp_chessboard = new_chessboard_instance_after_move(
                        chessboard,
                        &from_piece,
                        attacker_position,
                    );

                    // Si no hay jaque después de capturar la pieza atacante, retornamos false
                    if is_check(&temp_chessboard, player_color).is_none() {
                        return false;
                    }
                }

                // Si no se puede capturar a la pieza atacante, validamos si la pieza puede interponerse
                
            }
        }
    }

    return true;
}

/**
Valida si hay una pieza en el camino de un movimiento.
# Arguments
* `chessboard` - Una referencia al tablero de ajedrez actual.
* `moves` - Un vector de posiciones `[usize; 2]` que representa el camino de movimiento de la pieza.
* `to` - La posición destino del movimiento, representada como un arreglo de dos elementos `[usize; 2]`.
# Returns
`bool` - Retorna `true` si hay una pieza en el camino, excluyendo la posición de destino. Retorna `false` si no hay piezas en el camino.
*/
fn validate_piece_in_path(chessboard: &Chessboard, moves: Vec<[usize; 2]>, to: [usize; 2]) -> bool {
    // Validamos si hay una pieza en el camino
    for move_position in moves {
        if move_position == to {
            continue;
        }

        if chessboard.board[move_position[0]][move_position[1]].is_some() {
            return true;
        }
    }

    return false;
}
