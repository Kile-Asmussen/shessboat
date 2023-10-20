use crate::elements::{Piece, PieceColor, PieceValue, Square};

pub trait Move: Debug + PartialEq + Eq + Clone + Copy {
    fn uci(self) -> String;
    fn color(self) -> PieceColor;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CastlingMove {
    pub color: PieceColor,
    pub king_from: Square,
    pub king_to: Square,
    pub rook_from: Square,
    pub rook_to: Square,
}

impl Move for CastlingMove {
    fn uci(self) -> String {
        format!(
            "{}{}",
            self.king_from.algebraic(),
            self.rook_from.algebraic()
        )
    }

    fn color(self) -> PieceColor {
        self.color
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PawnPromotion {
    pub color: PieceColor,
    pub from: Square,
    pub to: Square,
    pub into: PieceValue,
}

impl Move for PawnPromotion {
    fn uci(self) -> String {
        format!(
            "{}{}={}",
            self.from.algebraic(),
            self.to.algebraic(),
            self.into.letter()
        )
    }

    fn color(self) -> PieceColor {
        self.color
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EnPassantCapture {
    pub color: PieceColor,
    pub from: Square,
    pub to: Square,
    pub capture: Square,
}

impl Move for EnPassantCapture {
    fn uci(self) -> String {
        format!("{}{}", self.from.algebraic(), self.to.algebraic())
    }

    fn color(self) -> PieceColor {
        self.color
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StandardMove {
    pub piece: Piece,
    pub from: Square,
    pub to: Square,
    pub capture: Option<Piece>,
}

impl Move for StandardMove {
    fn uci(self) -> String {
        format!("{}{}", self.from.algebraic(), self.to.algebraic())
    }

    fn color(self) -> PieceColor {
        self.piece.color()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GeneralMove {
    Castling(CastlingMove),
    EnPassant(EnPassantCapture),
    Promotion(PawnPromotion),
    Standard(StandardMove),
}

impl Move for GeneralMove {
    fn uci(self) -> String {
        use GeneralMove::*;
        match self {
            Castling(c) => c.uci(),
            EnPassant(e) => e.uci(),
            Promotion(p) => p.uci(),
            Standard(s) => s.uci(),
        }
    }

    fn color(self) -> PieceColor {
        use GeneralMove::*;
        match self {
            Castling(c) => c.color(),
            EnPassant(e) => e.color(),
            Promotion(p) => p.color(),
            Standard(s) => s.color(),
        }
    }
}
