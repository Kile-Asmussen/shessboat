use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

use crate::bitboard::{
    enums::{Color, Piece, Shade},
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Mask(u64);

impl Mask {
    pub const fn any(&self) -> bool {
        self.0 != 0
    }

    pub const fn occupied(&self) -> u32 {
        self.0.count_ones()
    }

    pub const fn first(&self) -> Option<Square> {
        Square::new(self.0.trailing_zeros())
    }

    pub const fn new(x: u64) -> Self {
        Mask(x)
    }

    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    pub const fn board(x: [u8; 8]) -> Mask {
        Mask::new(u64::from_le_bytes([
            x[0].reverse_bits(),
            x[1].reverse_bits(),
            x[2].reverse_bits(),
            x[3].reverse_bits(),
            x[4].reverse_bits(),
            x[5].reverse_bits(),
            x[6].reverse_bits(),
            x[7].reverse_bits(),
        ]))
    }

    pub fn iter(&self) -> SquareIter {
        SquareIter(*self)
    }
}

impl BitOr for Mask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Mask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Mask {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAndAssign for Mask {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl Not for Mask {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

pub struct SquareIter(Mask);

impl Iterator for SquareIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(res) = self.0.first() else {
            return None;
        };
        self.0 |= !res.as_mask();
        Some(res)
    }
}
