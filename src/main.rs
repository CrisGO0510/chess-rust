use pieces::piece_type::{ChessPiece, ChessPieceType, ChessPieceColor};
use pieces::piece_actions::PieceActions;

mod pieces;

fn main() {
    let king = ChessPiece {
        piece: ChessPieceType::King,
        color: ChessPieceColor::Black,
        position: [0, 0],
    };

    king.move_piece();
    king.capture_piece();
}
