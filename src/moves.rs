use crate::elements::{Piece, PieceColor, PieceValue, Square};

pub trait ChessMove {
    fn uci(self) -> String;
    fn piece(self) -> Piece;
    fn from(self) -> Square;
    fn to(self) -> Square;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Castling {
    pub color: PieceColor,
    pub king_from: Square,
    pub king_to: Square,
    pub rook_from: Square,
    pub rook_to: Square,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PawnPush {
    pub color: PieceColor,
    pub from: Square,
    pub torpedo: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PawnCapture {
    pub color: PieceColor,
    pub from: Square,
    pub to: Square,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EnPassantCapture {
    pub color: PieceColor,
    pub from: Square,
    pub to: Square,
}

pub struct KnightMove {
    pub color: PieceColor,
    pub from: Square,
    pub to: Square,
}

pub struct DiagonalSlide {
    pub piece: Piece,
    pub from: Square,
    pub to: Square,
}

pub struct OrthogonalSlide {
    pub piece: Piece,
    pub from: Square,
    pub to: Square,
}
