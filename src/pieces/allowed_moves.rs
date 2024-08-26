use super::piece_type::ChessPieceColor;

pub fn king_allowed_moves(current_position: [usize; 2]) -> Vec<[usize; 2]> {
    let mut allowed_moves: Vec<[usize; 2]> = Vec::new();

    // Itera sobre los posibles movimientos del rey
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue; // El rey no se mueve a la misma posición
            }

            let position_x = current_position[0] as i32 + dx;
            let position_y = current_position[1] as i32 + dy;

            if position_x >= 0 && position_x < 8 && position_y >= 0 && position_y < 8 {
                allowed_moves.push([position_x as usize, position_y as usize]);
            }
        }
    }

    allowed_moves
}

pub fn rook_allowed_moves(current_position: [usize; 2]) -> Vec<[usize; 2]> {
    let mut allowed_moves: Vec<[usize; 2]> = Vec::new();

    // Iteramos los posibles movimientos de la torre en x
    for dx in 0..8 {
        if dx != current_position[0] {
            allowed_moves.push([dx, current_position[1]]);
        }
    }

    // Iteramos los posibles movimientos de la torre en y
    for dy in 0..8 {
        if dy != current_position[1] {
            allowed_moves.push([current_position[0], dy]);
        }
    }

    allowed_moves
}

pub fn bishop_allowed_moves(current_position: [usize; 2]) -> Vec<[usize; 2]> {
    let mut allowed_moves: Vec<[usize; 2]> = Vec::new();
    let (cx, cy) = (current_position[0], current_position[1]);

    // Diagonal: arriba-izquierda a abajo-derecha
    let mut x = cx as i32;
    let mut y = cy as i32;
    while x >= 0 && y >= 0 {
        if x != cx as i32 || y != cy as i32 {
            allowed_moves.push([x as usize, y as usize]);
        }
        x -= 1;
        y -= 1;
    }

    let mut x = cx as i32;
    let mut y = cy as i32;
    while x < 8 && y < 8 {
        if x != cx as i32 || y != cy as i32 {
            allowed_moves.push([x as usize, y as usize]);
        }
        x += 1;
        y += 1;
    }

    // Diagonal: abajo-izquierda a arriba-derecha
    let mut x = cx as i32;
    let mut y = cy as i32;
    while x >= 0 && y < 8 {
        if x != cx as i32 || y != cy as i32 {
            allowed_moves.push([x as usize, y as usize]);
        }
        x -= 1;
        y += 1;
    }

    let mut x = cx as i32;
    let mut y = cy as i32;
    while x < 8 && y >= 0 {
        if x != cx as i32 || y != cy as i32 {
            allowed_moves.push([x as usize, y as usize]);
        }
        x += 1;
        y -= 1;
    }

    allowed_moves
}

pub fn knight_allowed_moves(current_position: [usize; 2]) -> Vec<[usize; 2]> {
    let mut allowed_moves: Vec<[usize; 2]> = Vec::new();
    let (cx, cy) = (current_position[0] as i32, current_position[1] as i32);

    // Posibles movimientos del caballo
    let moves = [
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];

    for (dx, dy) in moves.iter() {
        let (new_x, new_y) = (cx + dx, cy + dy);

        // Verificamos y agregamos los movimientos permitidos
        if new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8 {
            allowed_moves.push([new_x as usize, new_y as usize]);
        }
    }

    allowed_moves
}

pub fn pawn_allowed_move(current_position: [usize; 2], color: ChessPieceColor) -> Vec<[usize; 2]> {
    let mut allowed_moves: Vec<[usize; 2]> = Vec::new();
    let (x, y) = (current_position[0], current_position[1]);

    match color {
        ChessPieceColor::Black => {
            if x < 7 {
                allowed_moves.push([x + 1, y]);
                if x == 1 {
                    allowed_moves.push([x + 2, y]);
                }
            }
        }
        ChessPieceColor::White => {
            if x > 0 {
                allowed_moves.push([x - 1, y]);
                if x == 6 {
                    allowed_moves.push([x - 2, y]);
                }
            }
        }
    }
    allowed_moves
}

pub fn pawn_allowed_capture(
    current_position: [usize; 2],
    color: ChessPieceColor,
) -> Vec<[usize; 2]> {
    let mut allowed_moves: Vec<[usize; 2]> = Vec::new();
    let (x, y) = (current_position[0], current_position[1]);

    match color {
        ChessPieceColor::Black => {
            if x < 7 {
                if y > 0 {
                    allowed_moves.push([x + 1, y - 1]);
                }
                if y < 7 {
                    allowed_moves.push([x + 1, y + 1]);
                }
            }
        }
        ChessPieceColor::White => {
            if x > 0 {
                if y > 0 {
                    allowed_moves.push([x - 1, y - 1]);
                }
                if y < 7 {
                    allowed_moves.push([x - 1, y + 1]);
                }
            }
        }
    }

    allowed_moves
}

pub fn queen_allowed_moves(current_position: [usize; 2]) -> Vec<[usize; 2]> {
    let mut allowed_moves: Vec<[usize; 2]> = Vec::new();

    // Movimientos de la reina: combinación de torre y alfil
    let rook_moves = rook_allowed_moves(current_position);
    let bishop_moves = bishop_allowed_moves(current_position);

    for move_ in rook_moves.iter() {
        allowed_moves.push(*move_);
    }

    for move_ in bishop_moves.iter() {
        allowed_moves.push(*move_);
    }

    allowed_moves
}

pub fn pawn_all_allowed_move(
    current_position: [usize; 2],
    color_piece: ChessPieceColor,
) -> Vec<[usize; 2]> {
    let mut allowed_moves: Vec<[usize; 2]> = Vec::new();

    // Movimientos de la reina: combinación de torre y alfil
    let pawn_allowed_move = pawn_allowed_move(current_position, color_piece);
    let pawn_allowed_capture = pawn_allowed_capture(current_position, color_piece);

    for move_ in pawn_allowed_move.iter() {
        allowed_moves.push(*move_);
    }

    for move_ in pawn_allowed_capture.iter() {
        allowed_moves.push(*move_);
    }

    allowed_moves
}
