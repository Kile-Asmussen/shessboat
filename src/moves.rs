use crate::{
    pieces::{Color, Piece},
    squares::{EpsSquare, Square},
};

pub enum Move {
    Castling(CastlingMove),
    Pawn(PawnMove),
    Promotion(PromotionMove),
    Standard(StandardMove),
}

pub struct StandardMove {
    pub color: Color,
    pub piece: Piece,
    pub from: Square,
    pub to: Square,
    pub capture: Option<Piece>,
}

pub struct PawnMove {
    pub color: Color,
    pub from: Square,
    pub to: Square,
    pub capture: Option<(Piece, Square)>,
}

impl PawnMove {
    pub fn exposes_en_passent(&self) -> Option<Square> {
        let (f1, r1) = self.from.file_and_rank();
        let (f2, r2) = self.to.file_and_rank();

        if f1 == f2 && r1.abs_diff(r2) == 2 {
            Some(Square::from_file_and_rank(f1, (r1 + r2) / 2))
        } else {
            None
        }
    }
}

pub struct PromotionMove {
    pub color: Color,
    pub from: Square,
    pub to: Square,
    pub into: Piece,
}

pub struct CastlingMove {
    pub color: Color,
    pub king_from: Square,
    pub king_to: Square,
    pub rook_from: Square,
    pub rook_to: Square,
}
