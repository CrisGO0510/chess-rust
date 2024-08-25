use super::piece_type::ChessPieceColor;

pub fn king_route_moves(current_position: [usize; 2], new_position: [usize; 2]) -> Vec<[usize; 2]> {
    let mut route_moves: Vec<[usize; 2]> = Vec::new();

    let (cx, cy) = (current_position[0] as i32, current_position[1] as i32);
    let (nx, ny) = (new_position[0] as i32, new_position[1] as i32);

    // Verificar si la nueva posición está dentro del tablero
    if nx < 0 || nx > 7 || ny < 0 || ny > 7 {
        return route_moves;
    }

    // Verificar si el movimiento es válido para el Rey (movimiento a una casilla adyacente)
    if (cx - nx).abs() <= 1 && (cy - ny).abs() <= 1 {
        // Solo agrega el movimiento si la nueva posición es diferente de la actual
        if current_position != new_position {
            route_moves.push(new_position);
        }
    }

    route_moves
}

pub fn rook_route_moves(current_position: [usize; 2], new_position: [usize; 2]) -> Vec<[usize; 2]> {
    let mut route_moves: Vec<[usize; 2]> = Vec::new();

    let (cx, cy) = (current_position[0], current_position[1]);
    let (nx, ny) = (new_position[0], new_position[1]);

    // Verificar si la nueva posición está dentro del tablero
    if nx > 7 || ny > 7 {
        return route_moves;
    }

    // Verificar si se puede alcanzar la nueva posición
    if (cx == nx && cy == ny) || (cx != nx && cy != ny) {
        return route_moves;
    }

    // Validamos si necesitamos un movimiento horizontal o vertical
    if cx == nx {
        // Movimiento horizontal
        let start = if cy < ny { cy } else { ny };
        let end = if cy < ny { ny } else { cy };

        for y in start..(end + 1) {
            // Ignoramos la posición actual
            if y == cy {
                continue;
            }

            route_moves.push([cx, y]);
        }
    } else if cy == ny {
        // Movimiento vertical
        let start = if cx < nx { cx } else { nx };
        let end = if cx < nx { nx } else { cx };

        for x in start..(end + 1) {
            // Ignoramos la posición actual
            if x == cx {
                continue;
            }

            route_moves.push([x, cy]);
        }
    }

    // Retornamos los movimientos posibles
    route_moves
}

pub fn bishop_route_moves(
    current_position: [usize; 2],
    new_position: [usize; 2],
) -> Vec<[usize; 2]> {
    let mut route_moves: Vec<[usize; 2]> = Vec::new();

    let (cx, cy) = (current_position[0] as i32, current_position[1] as i32);
    let (nx, ny) = (new_position[0] as i32, new_position[1] as i32);

    // Verificar si la nueva posición está dentro del tablero
    if nx < 0 || nx > 7 || ny < 0 || ny > 7 {
        return route_moves;
    }

    // Verificar si el movimiento es diagonal
    if (cx - nx).abs() == (cy - ny).abs() {
        // Determinar la dirección del movimiento
        let dx = if nx > cx { 1 } else { -1 };
        let dy = if ny > cy { 1 } else { -1 };

        // Calcular las posiciones intermedias
        let mut x = cx + dx;
        let mut y = cy + dy;

        while x != nx && y != ny {
            route_moves.push([x as usize, y as usize]);
            x += dx;
            y += dy;
        }

        // Agregar la posición final
        route_moves.push([nx as usize, ny as usize]);
    }

    route_moves
}

pub fn queen_route_moves(
    current_position: [usize; 2],
    new_position: [usize; 2],
) -> Vec<[usize; 2]> {
    if rook_route_moves(current_position, new_position).len() > 0 {
        return rook_route_moves(current_position, new_position);
    }

    if bishop_route_moves(current_position, new_position).len() > 0 {
        return bishop_route_moves(current_position, new_position);
    }

    vec![]
}

pub fn knight_route_moves(
    current_position: [usize; 2],
    new_position: [usize; 2],
) -> Vec<[usize; 2]> {
    let mut route_moves: Vec<[usize; 2]> = Vec::new();
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

    // Calcular los posibles movimientos válidos
    for (dx, dy) in moves.iter() {
        let (new_x, new_y) = (cx + dx, cy + dy);

        // Verificar si el nuevo movimiento está dentro del tablero
        if new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8 {
            route_moves.push([new_x as usize, new_y as usize]);
        }
    }

    // Verificar si la nueva posición es un movimiento válido
    if route_moves.contains(&new_position) {
        return vec![new_position];
    }

    vec![]
}

pub fn pawn_route_move(
    current_position: [usize; 2],
    new_position: [usize; 2],
    color: ChessPieceColor,
) -> Vec<[usize; 2]> {
    let mut route_moves: Vec<[usize; 2]> = Vec::new();
    let (cx, cy) = (current_position[0] as i32, current_position[1] as i32);
    let (nx, ny) = (new_position[0] as i32, new_position[1] as i32);

    // Si el peón no se mueve o se mueve en diagonal (otros métodos)
    if cx == nx || cy != ny {
        return route_moves;
    }

    // Verificar si la nueva posición está dentro del tablero
    if nx < 0 || nx > 7 || ny < 0 || ny > 7 {
        return route_moves;
    }

    match color {
        ChessPieceColor::Black => {
            // Movimiento hacia adelante
            if nx == cx + 1 && ny == cy {
                route_moves.push([nx as usize, ny as usize]);
            }
            // Movimiento doble hacia adelante desde la fila 1
            if cx == 1 && nx == cx + 2 && ny == cy {
                route_moves.push([(nx - 1) as usize, ny as usize]);
                route_moves.push([nx as usize, ny as usize]);
            }
        }
        ChessPieceColor::White => {
            // Movimiento hacia adelante
            if nx == cx - 1 && ny == cy {
                route_moves.push([nx as usize, ny as usize]);
            }
            // Movimiento doble hacia adelante desde la fila 6
            if cx == 6 && nx == cx - 2 && ny == cy {
                route_moves.push([(nx + 1) as usize, ny as usize]);
                route_moves.push([nx as usize, ny as usize]);
            }
        }
    }

    route_moves
}

pub fn pawn_route_capture(
    current_position: [usize; 2],
    new_position: [usize; 2],
    color: ChessPieceColor,
) -> Vec<[usize; 2]> {
    let mut route_moves: Vec<[usize; 2]> = Vec::new();
    let (cx, cy) = (current_position[0] as i32, current_position[1] as i32);
    let (nx, ny) = (new_position[0] as i32, new_position[1] as i32);

    // Verificar si la nueva posición está dentro del tablero
    if nx < 0 || nx > 7 || ny < 0 || ny > 7 {
        return route_moves;
    }

    match color {
        ChessPieceColor::Black => {
            // Captura en diagonal
            if nx == cx + 1 && (ny == cy - 1 || ny == cy + 1) {
                route_moves.push([nx as usize, ny as usize]);
            }
        }
        ChessPieceColor::White => {
            // Captura en diagonal
            if nx == cx - 1 && (ny == cy - 1 || ny == cy + 1) {
                route_moves.push([nx as usize, ny as usize]);
            }
        }
    }

    route_moves
}
