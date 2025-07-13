use std::{
    fmt::Debug,
    iter::{Product, Sum},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not},
};

use crate::bitboard::{
    enums::{Color, Piece, Shade},
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Mask(u64);

impl Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mask({:#064b})", self.0)
    }
}

impl Mask {
    pub const fn nil() -> Self {
        Mask(0)
    }

    pub const fn full() -> Self {
        Mask(u64::MAX)
    }

    pub const fn new(x: u64) -> Self {
        Mask(x)
    }

    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    pub const fn any(&self) -> bool {
        self.0 != 0
    }

    pub const fn occupied(&self) -> u32 {
        self.0.count_ones()
    }

    pub const fn first(&self) -> Option<Square> {
        Square::new(self.0.trailing_zeros() as i8)
    }

    pub const fn sans_first(&self) -> Self {
        let Some(sq) = self.first() else {
            return *self;
        };
        Self::new(self.0 & !sq.as_mask().as_u64())
    }

    pub const fn contains(&self, sq: Square) -> bool {
        self.as_u64() & sq.as_mask().as_u64() != 0
    }

    pub const fn board(x: [u8; 8]) -> Mask {
        Mask::new(u64::from_be_bytes([
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

    pub fn render(&self, highlight: &mut [bool; 64]) {
        for sq in self.iter() {
            highlight[sq.index() as usize] = true;
        }
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
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Mask {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl Not for Mask {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl IntoIterator for Mask {
    type Item = Square;

    type IntoIter = SquareIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Sum for Mask {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = Mask::nil();
        for m in iter {
            res |= m;
        }
        res
    }
}

impl Product for Mask {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = Mask::nil();
        for m in iter {
            res &= m;
        }
        res
    }
}

pub struct SquareIter(Mask);

impl Iterator for SquareIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(res) = self.0.first() else {
            return None;
        };
        self.0 = self.0.sans_first();
        Some(res)
    }
}

#[test]
fn test_mask_first_and_iteration() {
    assert_eq!(Mask::new(0x1).first(), Square::new(0));
    assert_eq!(Mask::new(0x2).first(), Square::new(1));
    assert_eq!(Mask::new(0).first(), None);
    assert_eq!(Mask::new(0x3).first(), Square::new(0));
    assert_eq!(Mask::new(1 << 63).first(), Square::new(63));

    let mut iter = Mask::new(0x10F).iter();
    assert_eq!(iter.next(), Square::new(0));
    assert_eq!(iter.next(), Square::new(1));
    assert_eq!(iter.next(), Square::new(2));
    assert_eq!(iter.next(), Square::new(3));
    assert_eq!(iter.next(), Square::new(8));
    assert_eq!(iter.next(), None);
}
