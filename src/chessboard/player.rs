use crate::pieces::piece_type::ChessPieceColor;

pub struct Player {
    pub name: String,
    pub color: ChessPieceColor,
    pub king_position: [usize; 2]
}
