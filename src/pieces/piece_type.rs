use super::colors;

#[derive(Clone, Copy, PartialEq)]
pub enum ChessPieceType {
    King,
    Rook,
    Bishop,
    Queen,
    Knight,
    Pawn,
}

#[derive(PartialEq, Debug)]
pub enum Message {
    Success,
    Check,
    CheckMate,
    // Validate Err
    PieceNotYourColor,
    CannotMoveToOccupiedSameColor,
    CannotCaptureKing,
    CannotMovePieceToPosition,
    PieceBlockingTheWay,
    CannotLeaveKingInCheck,
    NoPieceInStartingPosition,
    CannotCastle,
}

impl Message {
    pub fn get_message(&self) -> String {
        match self {
            Message::Success => "Movimiento realizado".to_string(),
            Message::Check => "Jaque!".to_string(),
            Message::CheckMate => "Jaque Mate".to_string(),
            Message::PieceNotYourColor => "La pieza no es de tu color".to_string(),
            Message::CannotMoveToOccupiedSameColor => {
                "No puedes mover una pieza a una posición ocupada por una pieza del mismo color"
                    .to_string()
            }
            Message::CannotCaptureKing => "No puedes capturar al rey".to_string(),
            Message::CannotMovePieceToPosition => {
                "No puedes mover la pieza a esa posición".to_string()
            }
            Message::PieceBlockingTheWay => "Hay una pieza en el camino".to_string(),
            Message::CannotLeaveKingInCheck => "No puedes dejar al rey en jaque".to_string(),
            Message::NoPieceInStartingPosition => {
                "No hay una pieza en la posición de inicio".to_string()
            }
            Message::CannotCastle => "No puedes realizar enroque".to_string(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ChessPieceColor {
    White,
    Black,
}
#[derive(Clone, Copy)]
pub struct ChessPiece {
    pub piece: ChessPieceType,
    pub color: ChessPieceColor,
    pub position: [usize; 2],
    pub before_position: Option<[usize; 2]>,
}

impl ChessPiece {
    // Devuelve el carácter asociado a cada tipo de pieza
    pub fn to_char(&self) -> String {
        let color = match self.color {
            ChessPieceColor::White => colors::BRIGHT_WHITE.to_string(),
            ChessPieceColor::Black => colors::BRIGHT_BLACK.to_string(),
        };

        let letter = match self.piece {
            ChessPieceType::King => "R",   // Rey
            ChessPieceType::Rook => "T",   // Torre
            ChessPieceType::Bishop => "A", // Alfil
            ChessPieceType::Queen => "D",  // Dama
            ChessPieceType::Knight => "C", // Caballo
            ChessPieceType::Pawn => "P",   // Peón
        };

        color + letter + colors::RESET
    }
}
