use crate::pieces::routes_moves::{
    bishop_route_moves, king_route_moves, knight_route_moves, pawn_route_capture, pawn_route_move,
    queen_route_moves, rook_route_moves,
};

use super::piece_type::{ChessPiece, ChessPieceType};

pub trait PieceActions {
    fn move_piece(&self, new_position: [usize; 2]) -> Vec<[usize; 2]>;
    fn capture_piece(&self, new_position: [usize; 2]) -> Vec<[usize; 2]>;
}

trait KingActions {
    fn king_move(&self, new_position: [usize; 2]) -> Vec<[usize; 2]>;
    fn king_capture(&self, new_position: [usize; 2]) -> Vec<[usize; 2]>;
}

trait RookActions {
    fn rook_move(&self, new_position: [usize; 2]) -> Vec<[usize; 2]>;
    fn rook_capture(&self, new_position: [usize; 2]) -> Vec<[usize; 2]>;
}

trait BishopActions {
    fn bishop_move(&self, new_position: [usize; 2]) -> Vec<[usize; 2]>;
    fn bishop_capture(&self, new_position: [usize; 2]) -> Vec<[usize; 2]>;
}

trait QueenActions {
    fn queen_move(&self, new_position: [usize; 2]) -> Vec<[usize; 2]>;
    fn queen_capture(&self, new_position: [usize; 2]) -> Vec<[usize; 2]>;
}

trait KnightActions {
    fn knight_move(&self, new_position: [usize; 2]) -> Vec<[usize; 2]>;
    fn knight_capture(&self, new_position: [usize; 2]) -> Vec<[usize; 2]>;
}

trait PawnActions {
    fn pawn_move(&self, new_position: [usize; 2]) -> Vec<[usize; 2]>;
    fn pawn_capture(&self, new_position: [usize; 2]) -> Vec<[usize; 2]>;
}

impl KingActions for ChessPiece {
    fn king_move(&self, new_position: [usize; 2]) -> Vec<[usize; 2]> {
        println!("Moviendo el Rey a la nueva posición {:?}", new_position);
        king_route_moves(self.position, new_position)
    }

    fn king_capture(&self, new_position: [usize; 2]) -> Vec<[usize; 2]> {
        println!("Capturando con el Rey.");
        king_route_moves(self.position, new_position)
    }
}

impl RookActions for ChessPiece {
    fn rook_move(&self, new_position: [usize; 2]) -> Vec<[usize; 2]> {
        println!("Moviendo la Torre a la nueva posición {:?}", new_position);
        rook_route_moves(self.position, new_position)
    }

    fn rook_capture(&self, new_position: [usize; 2]) -> Vec<[usize; 2]> {
        println!("Capturando con la Torre.");
        rook_route_moves(self.position, new_position)
    }
}

impl BishopActions for ChessPiece {
    fn bishop_move(&self, new_position: [usize; 2]) -> Vec<[usize; 2]> {
        println!("Moviendo el Alfil a la nueva posición {:?}", new_position);
        bishop_route_moves(self.position, new_position)
    }

    fn bishop_capture(&self, new_position: [usize; 2]) -> Vec<[usize; 2]> {
        println!("Capturando con el Alfil.");
        bishop_route_moves(self.position, new_position)
    }
}

impl QueenActions for ChessPiece {
    fn queen_move(&self, new_position: [usize; 2]) -> Vec<[usize; 2]> {
        println!("Moviendo la Reina a la nueva posición {:?}", new_position);
        queen_route_moves(self.position, new_position)
    }

    fn queen_capture(&self, new_position: [usize; 2]) -> Vec<[usize; 2]> {
        println!("Capturando con la Reina.");
        queen_route_moves(self.position, new_position)
    }
}

impl KnightActions for ChessPiece {
    fn knight_move(&self, new_position: [usize; 2]) -> Vec<[usize; 2]> {
        println!("Moviendo el Caballo a la nueva posición {:?}", new_position);
        knight_route_moves(self.position, new_position)
    }

    fn knight_capture(&self, new_position: [usize; 2]) -> Vec<[usize; 2]> {
        println!("Capturando con el Caballo.");
        knight_route_moves(self.position, new_position)
    }
}

impl PawnActions for ChessPiece {
    fn pawn_move(&self, new_position: [usize; 2]) -> Vec<[usize; 2]> {
        println!("Moviendo el Peón a la nueva posición {:?}", new_position);
        pawn_route_move(self.position, new_position, self.color)
    }

    fn pawn_capture(&self, new_position: [usize; 2]) -> Vec<[usize; 2]> {
        println!("Capturando con el Peón.");
        pawn_route_capture(self.position, new_position ,self.color)
    }
}

impl PieceActions for ChessPiece {
    fn move_piece(&self, new_position: [usize; 2]) -> Vec<[usize; 2]> {
        match self.piece {
            ChessPieceType::King => self.king_move(new_position),
            ChessPieceType::Rook => self.rook_move(new_position),
            ChessPieceType::Bishop => self.bishop_move(new_position),
            ChessPieceType::Queen => self.queen_move(new_position),
            ChessPieceType::Knight => self.knight_move(new_position),
            ChessPieceType::Pawn => self.pawn_move(new_position),
        }
    }

    fn capture_piece(&self, new_position: [usize; 2]) -> Vec<[usize; 2]> {
        match self.piece {
            ChessPieceType::King => self.king_capture(new_position),
            ChessPieceType::Rook => self.rook_capture(new_position),
            ChessPieceType::Bishop => self.bishop_capture(new_position),
            ChessPieceType::Queen => self.queen_capture(new_position),
            ChessPieceType::Knight => self.knight_capture(new_position),
            ChessPieceType::Pawn => self.pawn_capture(new_position),
        }
    }
}
