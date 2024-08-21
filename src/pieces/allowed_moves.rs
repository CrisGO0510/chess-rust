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
