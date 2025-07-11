use crate::bitboard::{
    colorfault::Colorfault,
    enums::{Color, Piece},
    masks::Mask,
    pieces::Micropawns,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Queens(Mask);

impl Queens {
    pub fn materiel(&self) -> Micropawns {
        Micropawns(self.0.occupied() as usize * 9_000_000)
    }

    pub fn as_mask(&self) -> Mask {
        self.0
    }
}

impl Colorfault for Queens {
    fn colorfault(c: Color) -> Self {
        Self(Piece::Queen.as_mask() & c.as_mask())
    }
}
