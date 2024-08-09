use std::process::Command;

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

        println!("Entrada inválida. Por favor ingrese dos valores: una letra (a-h) seguida de un número (1-8). Ejemplo: a2.");
    }
}
