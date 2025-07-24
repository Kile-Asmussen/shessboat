use std::{
    fmt::{Debug, Display},
    num::NonZeroI8,
};

use crate::shessboard::{
    enums::{Dir, File, Rank},
    masks::Mask,
    moves::ProtoMove,
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

    pub fn read(s: &str) -> Option<(Self, &str)> {
        let mut cs = s.chars();
        Some((
            Self::at(File::from_char(cs.next()?)?, Rank::from_char(cs.next()?)?),
            cs.as_str(),
        ))
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

#[allow(non_upper_case_globals)]
impl Square {
    pub const a1: Square = Square::at(File::A, Rank::_1);
    pub const b1: Square = Square::at(File::B, Rank::_1);
    pub const c1: Square = Square::at(File::C, Rank::_1);
    pub const d1: Square = Square::at(File::D, Rank::_1);
    pub const e1: Square = Square::at(File::E, Rank::_1);
    pub const f1: Square = Square::at(File::F, Rank::_1);
    pub const g1: Square = Square::at(File::G, Rank::_1);
    pub const h1: Square = Square::at(File::H, Rank::_1);

    pub const a2: Square = Square::at(File::A, Rank::_2);
    pub const b2: Square = Square::at(File::B, Rank::_2);
    pub const c2: Square = Square::at(File::C, Rank::_2);
    pub const d2: Square = Square::at(File::D, Rank::_2);
    pub const e2: Square = Square::at(File::E, Rank::_2);
    pub const f2: Square = Square::at(File::F, Rank::_2);
    pub const g2: Square = Square::at(File::G, Rank::_2);
    pub const h2: Square = Square::at(File::H, Rank::_2);

    pub const a3: Square = Square::at(File::A, Rank::_3);
    pub const b3: Square = Square::at(File::B, Rank::_3);
    pub const c3: Square = Square::at(File::C, Rank::_3);
    pub const d3: Square = Square::at(File::D, Rank::_3);
    pub const e3: Square = Square::at(File::E, Rank::_3);
    pub const f3: Square = Square::at(File::F, Rank::_3);
    pub const g3: Square = Square::at(File::G, Rank::_3);
    pub const h3: Square = Square::at(File::H, Rank::_3);

    pub const a4: Square = Square::at(File::A, Rank::_4);
    pub const b4: Square = Square::at(File::B, Rank::_4);
    pub const c4: Square = Square::at(File::C, Rank::_4);
    pub const d4: Square = Square::at(File::D, Rank::_4);
    pub const e4: Square = Square::at(File::E, Rank::_4);
    pub const f4: Square = Square::at(File::F, Rank::_4);
    pub const g4: Square = Square::at(File::G, Rank::_4);
    pub const h4: Square = Square::at(File::H, Rank::_4);

    pub const a5: Square = Square::at(File::A, Rank::_5);
    pub const b5: Square = Square::at(File::B, Rank::_5);
    pub const c5: Square = Square::at(File::C, Rank::_5);
    pub const d5: Square = Square::at(File::D, Rank::_5);
    pub const e5: Square = Square::at(File::E, Rank::_5);
    pub const f5: Square = Square::at(File::F, Rank::_5);
    pub const g5: Square = Square::at(File::G, Rank::_5);
    pub const h5: Square = Square::at(File::H, Rank::_5);

    pub const a6: Square = Square::at(File::A, Rank::_6);
    pub const b6: Square = Square::at(File::B, Rank::_6);
    pub const c6: Square = Square::at(File::C, Rank::_6);
    pub const d6: Square = Square::at(File::D, Rank::_6);
    pub const e6: Square = Square::at(File::E, Rank::_6);
    pub const f6: Square = Square::at(File::F, Rank::_6);
    pub const g6: Square = Square::at(File::G, Rank::_6);
    pub const h6: Square = Square::at(File::H, Rank::_6);

    pub const a7: Square = Square::at(File::A, Rank::_7);
    pub const b7: Square = Square::at(File::B, Rank::_7);
    pub const c7: Square = Square::at(File::C, Rank::_7);
    pub const d7: Square = Square::at(File::D, Rank::_7);
    pub const e7: Square = Square::at(File::E, Rank::_7);
    pub const f7: Square = Square::at(File::F, Rank::_7);
    pub const g7: Square = Square::at(File::G, Rank::_7);
    pub const h7: Square = Square::at(File::H, Rank::_7);

    pub const a8: Square = Square::at(File::A, Rank::_8);
    pub const b8: Square = Square::at(File::B, Rank::_8);
    pub const c8: Square = Square::at(File::C, Rank::_8);
    pub const d8: Square = Square::at(File::D, Rank::_8);
    pub const e8: Square = Square::at(File::E, Rank::_8);
    pub const f8: Square = Square::at(File::F, Rank::_8);
    pub const g8: Square = Square::at(File::G, Rank::_8);
    pub const h8: Square = Square::at(File::H, Rank::_8);

    pub const fn to(self, other: Square) -> ProtoMove {
        ProtoMove {
            from: self,
            to: other,
        }
    }
}
