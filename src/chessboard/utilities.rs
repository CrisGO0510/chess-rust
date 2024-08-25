use std::process::Command;

use crate::pieces::piece_type::{ChessPiece, ChessPieceColor, ChessPieceType};

use super::chessboard::Chessboard;

pub fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Error al limpiar la consola");
    } else {
        Command::new("clear")
            .status()
            .expect("Error al limpiar la consola");
    }
}

pub fn get_coordinates(message: &str) -> [usize; 2] {
    loop {
        println!("{}", message);
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Error al leer la entrada.");
            continue;
        }

        let chess_coords: Vec<&str> = input.trim().split_whitespace().collect();

        if chess_coords.len() == 2 {
            let column_char = chess_coords[0].chars().next();
            if let (Some(chess_y), Ok(chess_x)) = (column_char, chess_coords[1].parse::<usize>()) {
                if chess_x < 1 || chess_x > 8 {
                    println!("El número debe estar entre 1 y 8.");
                    continue;
                }

                let x = 8 - chess_x;
                let y = match chess_y {
                    'a' => 0,
                    'b' => 1,
                    'c' => 2,
                    'd' => 3,
                    'e' => 4,
                    'f' => 5,
                    'g' => 6,
                    'h' => 7,
                    _ => 8,
                };

                if x < 8 && y < 8 {
                    return [x, y];
                }
            }
        }

        println!("Entrada inválida. Por favor ingrese dos valores: una letra (a-h) seguida de un número (1-8). Ejemplo: a 2.");
    }
}

pub fn new_chessboard_instance_after_move(
    chessboard: &Chessboard,
    from_piece: &ChessPiece,
    to: [usize; 2],
) -> Chessboard {
    // Clonamos el tablero y la pieza para trabajar con copias temporales
    let mut temp_chessboard = chessboard.clone();
    let mut temp_piece = from_piece.clone();

    // Guardamos la posición original de la pieza y actualizamos su nueva posición
    let from = temp_piece.position;

    // Realizamos el movimiento a la ficha
    temp_piece.before_position = Some(from);
    temp_piece.position = to;

    // Si la pieza es un rey, actualizamos su posición en el estado del jugador
    if temp_piece.piece == ChessPieceType::King {
        match temp_piece.color {
            ChessPieceColor::White => temp_chessboard.player1.king_position = to,
            ChessPieceColor::Black => temp_chessboard.player2.king_position = to,
        }
    }

    // Si la pieza es un peón, verificamos si se puede promocionar
    if temp_piece.piece == ChessPieceType::Pawn {
        temp_piece = pawn_promotion(temp_piece);
    }

    // Actualizamos el tablero: movemos la pieza y vaciamos su posición anterior
    temp_chessboard.board[to[0]][to[1]] = Some(temp_piece);
    temp_chessboard.board[from[0]][from[1]] = None;

    // Cambiamos el turno del jugador
    temp_chessboard.player_turn = match temp_chessboard.player_turn {
        ChessPieceColor::White => ChessPieceColor::Black,
        ChessPieceColor::Black => ChessPieceColor::White,
    };

    // temp_chessboard.print_board("copia".to_string());

    temp_chessboard
}

fn get_piece_type(message: &str) -> ChessPieceType {
    loop {
        println!("Seleccione la pieza que desea coronar:\n{}", message);
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Error al leer la entrada.");
            continue;
        }

        let chess_coords: Vec<&str> = input.trim().split_whitespace().collect();

        if chess_coords.len() == 1 {
            let piece_char = chess_coords[0].chars().next();
            let chess_piece = match piece_char {
                Some('T') => ChessPieceType::Rook,
                Some('A') => ChessPieceType::Bishop,
                Some('D') => ChessPieceType::Queen,
                Some('C') => ChessPieceType::Knight,
                _ => ChessPieceType::King,
            };

            if chess_piece != ChessPieceType::King {
                return chess_piece;
            }
        }

        println!("Entrada inválida, ingrese una pieza valida.");
    }
}

fn pawn_promotion(pawn_piece: ChessPiece) -> ChessPiece {
    let mut temp_pawn_piece = pawn_piece.clone();

    match temp_pawn_piece.color {
        ChessPieceColor::White => {
            if temp_pawn_piece.position[0] == 0 {
                // Preguntamos al usuario que ficha quiere
                let piece_type =
                    get_piece_type("Reina: \"D\"\nAlfil: \"A\"\nTorre: \"T\"\nCaballo: \"C\"");

                temp_pawn_piece.piece = piece_type;
            }
        }
        ChessPieceColor::Black => {
            if temp_pawn_piece.position[0] == 7 {
                // Preguntamos al usuario que ficha quiere
                let piece_type =
                    get_piece_type("Reina: \"D\"\nAlfil: \"A\"\nTorre: \"T\"\nCaballo: \"C\"");

                temp_pawn_piece.piece = piece_type;
            }
        }

    }
    
    temp_pawn_piece
}
