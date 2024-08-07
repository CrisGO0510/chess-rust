use super::piece_type::{ChessPiece, ChessPieceType};

pub trait PieceActions {
  fn move_piece(&self);
  fn capture_piece(&self);
}

trait KingActions {
  fn king_move(&self);
  fn king_capture(&self);
}

trait RookActions {
  fn rook_move(&self);
  fn rook_capture(&self);
}


impl KingActions for ChessPiece {
  fn king_move(&self) {
      println!("Moviendo el Rey.");
  }

  fn king_capture(&self) {
      println!("Capturando con el Rey.");
  }
}

impl RookActions for ChessPiece {
  fn rook_move(&self) {
      println!("Moviendo la Torre.");
  }

  fn rook_capture(&self) {
      println!("Capturando con la Torre.");
  }
}


impl PieceActions for ChessPiece {
  fn move_piece(&self) {
      match self.piece {
          ChessPieceType::King => self.king_move(),
          ChessPieceType::Rook => self.rook_move(),
          _ => (),
      }
  }

  fn capture_piece(&self) {
      match self.piece {
          ChessPieceType::King => self.king_capture(),
          ChessPieceType::Rook => self.rook_capture(),
          _ => (),
      }
  }
}