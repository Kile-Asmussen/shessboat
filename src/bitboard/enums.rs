use super::masks::Mask;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u64)]
pub enum Color {
    White = Mask::board([0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).as_u64(),
    Black = Mask::board([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF]).as_u64(),
}

impl Color {
    pub const fn as_mask(&self) -> Mask {
        Mask::new(*self as u64)
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u64)]
pub enum Shade {
    Dark =  Mask::board([0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55]).as_u64(),
    Light = Mask::board([0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA]).as_u64(),
}

impl Shade {
    pub const fn as_mask(&self) -> Mask {
        Mask::new(*self as u64)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u64)]
pub enum Rank {
    _8 = Mask::board([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF]).as_u64(),
    _7 = Mask::board([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00]).as_u64(),
    _6 = Mask::board([0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00]).as_u64(),
    _5 = Mask::board([0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00]).as_u64(),
    _4 = Mask::board([0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00]).as_u64(),
    _3 = Mask::board([0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00]).as_u64(),
    _2 = Mask::board([0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).as_u64(),
    _1 = Mask::board([0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).as_u64(),
}

impl Rank {
    pub const fn as_mask(&self) -> Mask {
        Mask::new(*self as u64)
    }

    pub const fn rank(e: i32) -> Option<Self> {
        use Rank::*;
        match e {
            0..=7 => Some([_1, _2, _3, _4, _5, _6, _7, _8][e as usize]),
            _ => None,
        }
    }

    pub const fn as_rank(&self) -> i32 {
        self.as_mask().first().unwrap().index() / 8
    }
}

#[test]
fn rank_as_rank_roundtrip() {
    assert_eq!(Rank::_1, Rank::rank(Rank::_1.as_rank()).unwrap());
    assert_eq!(Rank::_2, Rank::rank(Rank::_2.as_rank()).unwrap());
    assert_eq!(Rank::_3, Rank::rank(Rank::_3.as_rank()).unwrap());
    assert_eq!(Rank::_4, Rank::rank(Rank::_4.as_rank()).unwrap());
    assert_eq!(Rank::_5, Rank::rank(Rank::_5.as_rank()).unwrap());
    assert_eq!(Rank::_6, Rank::rank(Rank::_6.as_rank()).unwrap());
    assert_eq!(Rank::_7, Rank::rank(Rank::_7.as_rank()).unwrap());
    assert_eq!(Rank::_8, Rank::rank(Rank::_8.as_rank()).unwrap());
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u64)]
pub enum File {
    A = Mask::board([0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80]).as_u64(),
    B = Mask::board([0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40]).as_u64(),
    C = Mask::board([0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20]).as_u64(),
    D = Mask::board([0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10]).as_u64(),
    E = Mask::board([0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08]).as_u64(),
    F = Mask::board([0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04]).as_u64(),
    G = Mask::board([0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02]).as_u64(),
    H = Mask::board([0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]).as_u64(),
}

impl File {
    pub const fn as_mask(&self) -> Mask {
        Mask::new(*self as u64)
    }

    pub const fn file(e: i32) -> Option<Self> {
        use File::*;
        match e {
            0..=7 => Some([A, B, C, D, E, F, G, H][e as usize]),
            _ => None,
        }
    }

    pub const fn as_file(&self) -> i32 {
        self.as_mask().first().unwrap().index()
    }
}

#[test]
fn file_as_file_roundtrip() {
    assert_eq!(File::A, File::file(File::A.as_file()).unwrap());
    assert_eq!(File::B, File::file(File::B.as_file()).unwrap());
    assert_eq!(File::C, File::file(File::C.as_file()).unwrap());
    assert_eq!(File::D, File::file(File::D.as_file()).unwrap());
    assert_eq!(File::E, File::file(File::E.as_file()).unwrap());
    assert_eq!(File::F, File::file(File::F.as_file()).unwrap());
    assert_eq!(File::G, File::file(File::G.as_file()).unwrap());
    assert_eq!(File::H, File::file(File::H.as_file()).unwrap());
}

#[rustfmt::skip]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u64)]
pub enum Piece {
    Pawn =   Mask::board([0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00]).as_u64(),
    Knight = Mask::board([0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42]).as_u64(),
    Bishop = Mask::board([0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24]).as_u64(),
    Rook =   Mask::board([0x81, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x81]).as_u64(),
    Queen =  Mask::board([0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10]).as_u64(),
    King =   Mask::board([0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08]).as_u64(),
}

impl Piece {
    pub const fn as_mask(&self) -> Mask {
        Mask::new(*self as u64)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Dir {
    North = 8,
    East = 1,
    South = -8,
    West = -1,
}
