use crate::elements::{Piece, PieceColor, PieceValue, Square};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CastlingMove {
    pub color: PieceColor,
    pub king_from: Square,
    pub king_to: Square,
    pub rook_from: Square,
    pub rook_to: Square,
}

impl CastlingMove {
    fn uci(self) -> String {
        format!(
            "{}{}",
            self.king_from.algebraic(),
            self.rook_from.algebraic()
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PawnPromotion {
    pub color: PieceColor,
    pub from: Square,
    pub to: Square,
    pub into: PieceValue,
}

impl PawnPromotion {
    fn uci(self) -> String {
        format!(
            "{}{}={}",
            self.from.algebraic(),
            self.to.algebraic(),
            self.into.letter()
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EnPassantCapture {
    pub color: PieceColor,
    pub from: Square,
    pub to: Square,
    pub capture: Square,
}

impl EnPassantCapture {
    fn uci(self) -> String {
        format!("{}{}", self.from.algebraic(), self.to.algebraic())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StandardMove {
    pub piece: Piece,
    pub from: Square,
    pub to: Square,
    pub capture: Option<Piece>,
}

impl StandardMove {
    fn uci(self) -> String {
        format!("{}{}", self.from.algebraic(), self.to.algebraic())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move {
    Castling(CastlingMove),
    EnPassant(EnPassantCapture),
    Promotion(PawnPromotion),
    Standard(StandardMove),
}

impl Move {
    pub fn uci(self) -> String {
        match self {
            Move::Castling(c) => c.uci(),
            Move::EnPassant(e) => e.uci(),
            Move::Promotion(p) => p.uci(),
            Move::Standard(s) => s.uci(),
        }
    }
}
