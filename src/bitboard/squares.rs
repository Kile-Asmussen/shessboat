use std::num::NonZeroU64;

use crate::bitboard::{
    enums::{File, Rank},
    masks::Mask,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Square(NonZeroU64);

impl Square {
    pub const fn as_mask(&self) -> Mask {
        Mask::new(self.0.get())
    }

    pub const fn new(ix: u32) -> Option<Self> {
        let Some(n) = 1u64.checked_shl(ix) else {
            return None;
        };
        let Some(n) = NonZeroU64::new(n) else {
            return None;
        };
        Some(Square(n))
    }

    pub const fn index(&self) -> u32 {
        self.0.trailing_zeros()
    }

    pub const fn rank_and_file(&self) -> (Rank, File) {
        (
            Rank::rank(self.index() % 8).unwrap(),
            File::file(self.index() / 8).unwrap(),
        )
    }
}
