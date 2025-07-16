use crate::bitboard::{
    boardmap::BoardMap,
    enums::{Color, ColorPiece, Piece},
    masks::Mask,
    pieces::Micropawns,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Queens(Mask);

impl Queens {
    pub const fn nil() -> Self {
        Self(Mask::nil())
    }

    pub const fn new(mask: Mask) -> Self {
        Self(mask)
    }

    pub const fn materiel(&self) -> Micropawns {
        self.0.occupied() as i64 * 9_000_000
    }

    pub const fn as_mask(&self) -> Mask {
        self.0
    }

    pub const fn mut_mask(&mut self) -> &mut Mask {
        &mut self.0
    }

    pub fn render(&self, board: &mut BoardMap<Option<ColorPiece>>, color: Color) {
        for sq in self.0.iter() {
            board.set(sq, Some(ColorPiece::new(color, Piece::Queen)));
        }
    }
}
