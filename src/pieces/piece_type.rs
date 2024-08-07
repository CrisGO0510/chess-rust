pub enum ChessPieceType {
  King,
  Rook,
  Bishop,
  Queen,
  Knight,
  Pawn,
}

pub enum ChessPieceColor {
  White,
  Black,
}

pub struct ChessPiece {
  pub piece: ChessPieceType,
  pub color: ChessPieceColor,
  pub position: [i32; 2],
}
