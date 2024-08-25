use chessboard::chessboard::Chessboard;
use chessboard::utilities::get_coordinates;
use pieces::piece_type::Message;

mod pieces;
mod chessboard;

fn main() {
    let mut board = Chessboard::new();

    board.print_board("Bienvenido al juego de ajedrez".to_string());

    loop {
        let from = get_coordinates("Ingrese la posición de la pieza que desea mover (a h):");
        let to = get_coordinates("Ingrese la posición hacia donde desea mover (a h):");

        let message = board.move_piece(from, to);

        board.print_board(message.get_message());

        // Validamoss si es jaque mate
        if message == Message::CheckMate {
            println!("Juego terminado");
            break;
        }
    }
}