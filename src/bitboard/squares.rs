use std::num::NonZeroU64;

use crate::bitboard::masks::Mask;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Square(NonZeroU64);

impl Square {
    pub const fn as_mask(&self) -> Mask {
        Mask::new(self.0.get())
    }

    pub const fn index(ix: u32) -> Option<Self> {
        let Some(n) = 1u64.checked_shl(ix) else {
            return None;
        };
        let Some(n) = NonZeroU64::new(n) else {
            return None;
        };
        Some(Square(n))
    }
}
