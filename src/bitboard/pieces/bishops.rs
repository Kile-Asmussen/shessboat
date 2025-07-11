use crate::bitboard::{
    colorfault::Colorfault,
    enums::{Color, Piece},
    masks::Mask,
    pieces::Millipawns,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Bishops(Mask);

impl Bishops {
    pub fn materiel(&self) -> Millipawns {
        Millipawns(self.0.occupied() as usize * 3333)
    }
}

impl Colorfault for Bishops {
    fn colorfault(c: Color) -> Self {
        Self(c.as_mask() & Piece::Bishop.as_mask())
    }
}
