use super::colors;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ChessPieceType {
    King,
    Rook,
    Bishop,
    Queen,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ChessPieceColor {
    White,
    Black,
  }
  #[derive(Clone, Copy, Debug)]
  pub struct ChessPiece {
    pub piece: ChessPieceType,
    pub color: ChessPieceColor,
    pub position: [usize; 2],
}

impl ChessPiece {
    // Devuelve el carácter asociado a cada tipo de pieza
    pub fn to_char(&self) -> String {
        let color = match self.color {
            ChessPieceColor::White => colors::BRIGHT_WHITE.to_string(),
            ChessPieceColor::Black => colors::BRIGHT_BLACK.to_string(),
        };
    
        let letter = match self.piece {
            ChessPieceType::King => "R", // Rey
            ChessPieceType::Rook => "T", // Torre
            ChessPieceType::Bishop => "A", // Alfil
            ChessPieceType::Queen => "D", // Dama
            ChessPieceType::Knight => "C", // Caballo
            ChessPieceType::Pawn => "P", // Peón
        };
    
        color + letter + colors::RESET
    }
}
