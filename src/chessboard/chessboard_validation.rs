use super::{chessboard::Chessboard, utilities::new_chessboard_instance_after_move};
use crate::pieces::allowed_moves::king_allowed_moves;
use crate::pieces::piece_actions::PieceActions;
use crate::pieces::piece_type::{ChessPiece, ChessPieceColor, ChessPieceType, Message};

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
 - `Err(Message)`: Un mensaje de error en caso de que el movimiento no sea válido, explicando la razón del error.
*/
pub fn validate_move(
    chessboard: &Chessboard,
    from_position: &Option<ChessPiece>,
    to_position: &Option<ChessPiece>,
    to: [usize; 2],
    _is_computer: bool,
) -> Result<Chessboard, Message> {
    match from_position {
        Some(from_piece) => {
            // Validamos si es el turno del jugador
            if from_piece.color != chessboard.player_turn {
                return Err(Message::PieceNotYourColor);
            }

            // Validamos enroque
            if from_piece.piece == ChessPieceType::King
                && (from_piece.position[1] as i32 - to[1] as i32).abs() == 2
            {
                return castling_validate(chessboard, to);
            }

            match to_position {
                Some(to_piece) => {
                    // Validamos si la pieza es del mismo color

                    if from_piece.color == to_piece.color {
                        return Err(Message::CannotMoveToOccupiedSameColor);
                    }

                    // Validamos si la pieza es un rey
                    if to_piece.piece == ChessPieceType::King {
                        return Err(Message::CannotCaptureKing);
                    }

                    let moves = from_piece.capture_piece(to);

                    // Validamos si la pieza puede realizar el movimiento
                    if moves.is_empty() {
                        return Err(Message::CannotMovePieceToPosition);
                    }

                    // Validamos si hay una pieza en el camino
                    if validate_piece_in_path(chessboard, moves, to) {
                        return Err(Message::PieceBlockingTheWay);
                    }
                }
                None => {
                    let moves = from_piece.move_piece(to);

                    // Validamos si la pieza puede realizar el movimiento
                    if moves.is_empty() {
                        return Err(Message::CannotMovePieceToPosition);
                    }

                    // Validamos si hay una pieza en el camino
                    if validate_piece_in_path(chessboard, moves, to) {
                        return Err(Message::PieceBlockingTheWay);
                    }
                }
            }

            // Creamos una instancia temporal de chessboard
            let temp_chessboard =
                new_chessboard_instance_after_move(chessboard, from_piece, to, _is_computer);

            // Validamos que no quede en jaque después del movimiento
            if is_check(&temp_chessboard, chessboard.player_turn).is_some() {
                return Err(Message::CannotLeaveKingInCheck);
            }

            Ok(temp_chessboard)
        }
        None => Err(Message::NoPieceInStartingPosition),
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

                // // En caso de no haber una pieza en el camino, notificamos que el rey está en jaque
                // println!(
                //     "¡El rey está en jaque! por la pieza {} en la posición {:?}",
                //     from_piece.to_char(),
                //     from_piece.position
                // );

                // Retornamos la posición de la pieza que pone en jaque al rey
                return Some(from_piece.position);
            }
        }
    }

    // En caso de que no haya jaque, retornamos un none
    return None;
}

/**
 Valida si hay jaque mate.
 # Arguments
 * `chessboard` - Una referencia al tablero de ajedrez actual.
 * `player_color` - El color del usuario al cual vamos a analizar el jaquemate
 * `attacker_position` - La posición destino de la ficha que esta amaenazando al rey.
 # Returns
 Retorna un `bool` que puede ser:
 - `True`: Dando a entender que hay jaque mate.
 - `False`: Dando a entender que no hay jaque mate.
*/
pub fn is_checkmate(
    chessboard: &Chessboard,
    player_color: ChessPieceColor,
    attacker_position: [usize; 2],
    _is_computer: bool,
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
        let temp_chessboard = new_chessboard_instance_after_move(
            chessboard,
            &attacking_piece,
            move_position,
            _is_computer,
        );

        if is_check(&temp_chessboard, player_color).is_none() {
            return false;
        }
    }

    // Iteramos cada una de las piezas del jugador
    for i in 0..8 {
        for j in 0..8 {
            if let Some(from_piece) = chessboard.board[i][j] {
                // Si la pieza no es del mismo color, continuamos
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
                        _is_computer,
                    );

                    // Si no hay jaque después de capturar la pieza atacante, retornamos false
                    if is_check(&temp_chessboard, player_color).is_none() {
                        return false;
                    }
                }

                // Si no se puede capturar a la pieza atacante, validamos si la pieza puede interponerse
                for move_position in attack_route.iter() {
                    let moves = from_piece.move_piece(*move_position);

                    if !moves.is_empty() {
                        // Validamos si hay una pieza en el camino
                        if validate_piece_in_path(chessboard, moves, *move_position) {
                            continue;
                        }

                        // Creamos una instancia temporal de chessboard, para analizar si el rey sigue en jaque después de interponerse
                        let temp_chessboard = new_chessboard_instance_after_move(
                            chessboard,
                            &from_piece,
                            *move_position,
                            _is_computer,
                        );

                        // Si no hay jaque después de interponerse, retornamos false
                        if is_check(&temp_chessboard, player_color).is_none() {
                            return false;
                        }
                    }
                }
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

/**
 Valida si se puede realizar el enroque.
 # Arguments
 * `chessboard` - Una referencia al tablero de ajedrez actual.
 * `to` - La posición destino del movimiento del rey`.
 # Returns
 Retorna un `Result` que puede ser:
 - `Ok(Chessboard)`: Un nuevo tablero de ajedrez después de realizar el movimiento de enroque si es válido.
 - `Err(Message)`: Un mensaje de error en caso de que el movimiento no sea válido, explicando la razón del error.
*/
fn castling_validate(chessboard: &Chessboard, to: [usize; 2]) -> Result<Chessboard, Message> {
    // Obtenemos la posición del rey
    let king_position = match chessboard.player_turn {
        ChessPieceColor::White => chessboard.player1.king_position,
        ChessPieceColor::Black => chessboard.player2.king_position,
    };

    // Validamos si el rey esta a 2 posiciones de la torre
    if (king_position[1] as i32 - to[1] as i32).abs() != 2 {
        return Err(Message::CannotCastle);
    }

    // Validamos si el rey se ha movido
    let king_piece = chessboard.board[king_position[0]][king_position[1]].unwrap();

    if king_piece.before_position.is_some() {
        return Err(Message::CannotCastle);
    }

    // Obtenemos la posición de la torre
    let rook_position = match chessboard.player_turn {
        ChessPieceColor::White => {
            if to == [7, 6] {
                [7, 7]
            } else {
                [7, 0]
            }
        }
        ChessPieceColor::Black => {
            if to == [0, 6] {
                [0, 7]
            } else {
                [0, 0]
            }
        }
    };

    // Validamos que la torre exista y no se haya movido
    if let Some(rook_piece) = chessboard.board[rook_position[0]][rook_position[1]] {
        if rook_piece.before_position.is_some() {
            return Err(Message::CannotCastle);
        }
    } else {
        return Err(Message::CannotCastle);
    }

    // Validamos que no hayan fichas entre el rey y la torre
    for y in (king_position[1] + 1)..rook_position[1] {
        match chessboard.board[king_position[0]][y] {
            Some(_) => return Err(Message::CannotCastle),
            None => continue,
        };
    }

    // Creamos una instancia temporal de chessboard y movemos el rey
    let mut temp_chessboard =
        new_chessboard_instance_after_move(chessboard, &king_piece, to, false);

    // Obtenemos la posición final de la torre después del enroque
    let rook_position_to = match chessboard.player_turn {
        ChessPieceColor::White => {
            if to == [7, 6] {
                [7, 5]
            } else {
                [7, 3]
            }
        }
        ChessPieceColor::Black => {
            if to == [0, 6] {
                [0, 5]
            } else {
                [0, 3]
            }
        }
    };

    // Movemos la torre
    temp_chessboard = new_chessboard_instance_after_move(
        &temp_chessboard,
        &chessboard.board[rook_position[0]][rook_position[1]].unwrap(),
        rook_position_to,
        false,
    );

    // Cambiamos de turno
    temp_chessboard.player_turn = match temp_chessboard.player_turn {
        ChessPieceColor::White => ChessPieceColor::Black,
        ChessPieceColor::Black => ChessPieceColor::White,
    };

    return Ok(temp_chessboard);
}
