use std::{
    fmt::{Debug, Display},
    num::NonZeroI8,
};

use crate::shessboard::{
    enums::{Dir, File, Rank},
    masks::Mask,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Square(NonZeroI8);

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (file, rank) = self.algebraic();
        write!(f, "Square::at(File::{file:?}, Rank::{rank:?})")
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
        Mask::new(1 << self.index())
    }

    pub const fn new(ix: i8) -> Option<Self> {
        match ix {
            0..=63 => Some(Square(unsafe { NonZeroI8::new_unchecked(ix + 1) })),
            _ => None,
        }
    }

    pub const fn read<'a>(chars: &mut core::str::Chars<'a>) -> Option<Self> {
        let Some(f) = chars.next() else {
            return None;
        };

        let Some(r) = chars.next() else {
            return None;
        };

        Some(Square::at(File::from_char(f)?, Rank::from_char(r)));
    }

    pub const fn from_mask(mask: Mask) -> Option<Self> {
        if mask.occupied() == 1 {
            mask.first()
        } else {
            None
        }
    }

    pub const fn index(&self) -> i8 {
        self.0.get() - 1
    }

    pub const fn algebraic(&self) -> (File, Rank) {
        (self.file(), self.rank())
    }

    pub const fn rank(&self) -> Rank {
        Rank::rank(self.index() / 8).unwrap()
    }

    pub const fn file(&self) -> File {
        File::file(self.index() % 8).unwrap()
    }

    pub const fn at(file: File, rank: Rank) -> Self {
        Self::new(rank.as_rank() * 8 + file.as_file()).unwrap()
    }

    pub const fn dist(&self, other: Square) -> i8 {
        let (f, r) = self.algebraic();
        let (of, or) = other.algebraic();
        (f.as_file() - of.as_file()).abs() + (r.as_rank() - or.as_rank()).abs()
    }

    pub const fn go(&self, dir: Dir) -> Option<Self> {
        let val = self.index();
        let (f, r) = dir.as_offset();
        let file = match val % 8 + f {
            f @ 0..=7 => f,
            _ => return None,
        };
        let rank = match val / 8 + r {
            r @ 0..=7 => r,
            _ => return None,
        };
        Some(Self::new(rank * 8 + file).unwrap())
    }

    pub const fn goes<const N: usize>(&self, dirs: [Dir; N]) -> Option<Self> {
        let mut n = 0;
        let mut res = Some(*self);

        while n < N {
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
fn why_u_no_worky() {
    use File::*;
    use Rank::*;
    let at = Square::at;
    assert_eq!(at(A, _1).index(), 0);
    assert_eq!(at(A, _8).index(), 56);
    assert_eq!(at(B, _7).index(), 49);

    assert_eq!(at(C, _5).algebraic(), (C, _5));
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

    assert_eq!(
        at(A, _8).go(SouthEast),
        Some(at(B, _7)),
        "southeast from a8 is b7"
    )
}
