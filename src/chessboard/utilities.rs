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
    // Creamos una instancia temporal de chessboard y de la pieza
    let mut temp_chessboard = chessboard.clone();
    let mut temp_piece = from_piece.clone();

    // Obtenemos la posición de la pieza
    let from = temp_piece.position;

    // Realizamos el movimiento
    temp_piece.position = to;
    temp_chessboard.board[to[0]][to[1]] = Some(temp_piece);
    temp_chessboard.board[from[0]][from[1]] = None;

    // Actualizamos la posición del rey
    if Some(temp_piece.piece) == Some(ChessPieceType::King) {
        match temp_piece.color {
            ChessPieceColor::White => temp_chessboard.player1.king_position = to,
            ChessPieceColor::Black => temp_chessboard.player2.king_position = to,
        }
    }

    // Cambiamos el turno del jugador
    temp_chessboard.player_turn = match temp_chessboard.player_turn {
        ChessPieceColor::White => ChessPieceColor::Black,
        ChessPieceColor::Black => ChessPieceColor::White,
    };
    
    // temp_chessboard.print_board("copia".to_string());

    temp_chessboard
}
