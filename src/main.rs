use chessboard::chessboard::Chessboard;
use chessboard::utilities::{get_coordinates, is_valid_move, new_chessboard_instance_after_move};
use pieces::move_analysis::get_best_move;
use pieces::piece_type::Message;

mod chessboard;
mod pieces;

fn main() {
    let mut board = Chessboard::new();

    board.print_board("Bienvenido al juego de ajedrez".to_string());

    loop {
        let from = get_coordinates("Ingrese la posición de la pieza que desea mover (a h):");
        let to = get_coordinates("Ingrese la posición hacia donde desea mover (a h):");

        let message = board.move_piece(from, to, false);

        board.print_board(message.get_message());

        if !is_valid_move(&message){
            continue;
        }

        // Validamos si es jaque mate
        if message == Message::CheckMate {
            println!("Juego terminado");
            break;
        }

        // Analizamos el mejor movimiento para la computadora
        let (from, to) = get_best_move(&board, 4);
    
        println!("La computadora mueve de {:?}, {:?}", from, to);

        let from_piece = board.board[from[0]][from[1]].unwrap();

        board = new_chessboard_instance_after_move(&board, &from_piece, to, true);

        board.print_board(message.get_message());
    }

}
