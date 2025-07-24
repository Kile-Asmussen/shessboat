use std::{
    fmt::Debug,
    iter::{Product, Sum},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

use crate::shessboard::{
    boardmap::BoardMap,
    enums::{Color, ColorPiece, Rank, Shade},
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
        Mask::new(0)
    }

    pub const fn full() -> Self {
        Mask::new(u64::MAX)
    }

    pub const fn new(x: u64) -> Self {
        Mask(x)
    }

    pub const fn mirror(&self) -> Self {
        Mask::new(self.as_u64().swap_bytes())
    }

    pub const fn rotate(&self) -> Self {
        Mask::new(self.as_u64().reverse_bits())
    }

    pub const fn overlap(&self, other: Mask) -> Mask {
        Mask::new(self.as_u64() & other.as_u64())
    }

    pub const fn overlay(&self, other: Mask) -> Mask {
        Mask::new(self.as_u64() | other.as_u64())
    }

    pub const fn differences(&self, other: Mask) -> Mask {
        Mask::new(self.as_u64() ^ other.as_u64())
    }

    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    pub const fn as_u8(&self, rank: Rank) -> u8 {
        self.as_u64().to_le_bytes()[(rank as u8 - 1) as usize]
    }

    pub const fn new_rank(rank: Rank, val: u8) -> Self {
        let mut res = [0u8; 8];
        res[rank.as_rank() as usize] = val;
        Self::new(u64::from_le_bytes(res))
    }

    pub const fn any(&self) -> bool {
        self.as_u64() != 0
    }

    pub const fn occupied(&self) -> u32 {
        self.as_u64().count_ones()
    }

    pub const fn inverse(&self) -> Mask {
        Self::new(!self.as_u64())
    }

    pub const fn set(self, sq: Square) -> Self {
        self.overlay(sq.as_mask())
    }

    pub const fn unset(self, sq: Square) -> Self {
        self.overlap(sq.as_mask().inverse())
    }

    pub const fn first(self) -> Option<Square> {
        Square::new(self.0.trailing_zeros() as i8)
    }

    pub const fn last(self) -> Option<Square> {
        Square::new(self.0.leading_zeros() as i8)
    }

    pub const fn sans_first(self) -> Self {
        let Some(sq) = self.first() else {
            return self;
        };
        let mut res = self;
        res.unset(sq)
    }

    pub const fn contains(&self, sq: Square) -> bool {
        self.as_u64() & sq.as_mask().as_u64() != 0
    }

    pub const fn visboard(x: [u8; 8]) -> Mask {
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

    pub const fn iter(&self) -> SquareIter {
        SquareIter(*self)
    }

    pub fn render(&self, highlight: &mut BoardMap<bool>) {
        for sq in self.iter() {
            highlight.set(sq, true);
        }
    }
}

impl BitOr for Mask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.overlay(rhs)
    }
}

impl BitOrAssign for Mask {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.overlay(rhs)
    }
}

impl BitAnd for Mask {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.overlap(rhs)
    }
}

impl BitAndAssign for Mask {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.overlap(rhs)
    }
}

impl BitXor for Mask {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.differences(rhs)
    }
}

impl BitXorAssign for Mask {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.differences(rhs)
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

impl SquareIter {
    pub const fn next(&mut self) -> Option<Square> {
        let Some(res) = self.0.first() else {
            return None;
        };
        self.0 = self.0.sans_first();
        Some(res)
    }
}

impl Iterator for SquareIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        SquareIter::next(self)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.occupied() as usize, Some(self.0.occupied() as usize))
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
