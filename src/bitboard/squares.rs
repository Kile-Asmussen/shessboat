use std::{
    fmt::{Debug, Display},
    num::NonZeroU64,
};

use crate::bitboard::{
    enums::{Dir, File, Rank},
    masks::Mask,
};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Square(i8);

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (file, rank) = self.algebraic();
        write!(f, "Square::at(File::{:?}, Rank::{:?})", file, rank)
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (fi, ra) = self.algebraic();
        write!(f, "{}{}", fi.as_char(), ra.as_char())
    }
}

impl Square {
    pub const fn as_mask(&self) -> Mask {
        Mask::new(1 << self.0)
    }

    pub const fn new(ix: i8) -> Option<Self> {
        match ix {
            0..=63 => Some(Square(ix as i8)),
            _ => None,
        }
    }

    pub const fn from_mask(mask: Mask) -> Option<Self> {
        if mask.occupied() == 1 {
            mask.first()
        } else {
            None
        }
    }

    pub const fn index(&self) -> i8 {
        self.0
    }

    pub const fn algebraic(&self) -> (File, Rank) {
        (
            File::file(self.index() / 8).unwrap(),
            Rank::rank(self.index() % 8).unwrap(),
        )
    }

    pub const fn at(file: File, rank: Rank) -> Self {
        Square::from_mask(Mask::new(file.as_mask().as_u64() & rank.as_mask().as_u64())).unwrap()
    }

    pub const fn go(&self, dir: Dir) -> Option<Self> {
        let dir = dir.as_offset();
        let file = match self.0 % 8 + dir % 8 {
            f @ 0..=7 => f,
            _ => return None,
        };
        let rank = match self.0 / 8 + dir / 8 {
            r @ 0..=7 => r,
            _ => return None,
        };
        Some(Self(rank * 8 + file))
    }

    pub const fn goes(&self, dirs: &[Dir]) -> Option<Self> {
        let mut n = 0;
        let mut res = Some(*self);

        while n < dirs.len() {
            let Some(sq) = res else {
                return None;
            };
            res = sq.go(dirs[n]);
            n += 1;
        }

        res
    }

    pub fn invariant(&self) {
        assert_eq!(self.as_mask().occupied(), 1);
    }
}

#[test]
fn moves() {
    use Dir::*;
    use File::*;
    use Rank::*;
    let at = Square::at;

    assert_eq!(
        at(A, _8).go(South),
        Some(at(A, _7)),
        "west form a8 should be a7"
    );
    assert_eq!(at(A, _8).go(North), None, "no north from a8");
    assert_eq!(
        at(A, _8).go(East),
        Some(at(B, _8)),
        "east from a8 should be b8"
    );
    assert_eq!(at(A, _8).go(West), None, "no west from a8");
}
