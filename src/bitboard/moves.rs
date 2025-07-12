use crate::bitboard::{masks::Mask, squares::Square};

pub struct MoveDb([Mask; 64]);

impl MoveDb {
    pub const fn new(it: [Mask; 64]) -> Self {
        Self(it)
    }

    pub const fn lookup(&self, sq: Square) -> Mask {
        self.0[sq.index() as usize]
    }
}
