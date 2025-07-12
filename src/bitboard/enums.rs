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

    pub const fn rank(e: u32) -> Option<Self> {
        use Rank::*;
        if e > 7 {
            None
        } else {
            Some([_1, _2, _3, _4, _5, _6, _7, _8][e as usize])
        }
    }

    pub const fn as_rank(&self) -> u32 {
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

    pub const fn file(e: u32) -> Option<Self> {
        use File::*;
        if e > 7 {
            None
        } else {
            Some([A, B, C, D, E, F, G, H][e as usize])
        }
    }

    pub const fn as_file(&self) -> u32 {
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

#[rustfmt::skip]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u64)]
pub enum Orthogonals {
    North = Mask::board([0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF]).as_u64(),
    East  = Mask::board([0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0]).as_u64(),
    South = Mask::board([0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00]).as_u64(),
    West  = Mask::board([0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F]).as_u64(),
}

impl Orthogonals {
    pub const fn as_mask(&self) -> Mask {
        Mask::new(*self as u64)
    }

    pub const fn cardinal(&self) -> Cardinals {
        match self {
            Orthogonals::North => Cardinals::North,
            Orthogonals::East => Cardinals::East,
            Orthogonals::South => Cardinals::South,
            Orthogonals::West => Cardinals::West,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u64)]
pub enum Diagonals {
    NorthEast = Mask::board([0x80, 0xC0, 0xE0, 0xF0, 0xF8, 0xFC, 0xFE, 0xFF]).as_u64(),
    SouthEast = Mask::board([0xFF, 0xFE, 0xFC, 0xF8, 0xF0, 0xE0, 0xC0, 0x80]).as_u64(),
    SouthWest = Mask::board([0xFF, 0x7F, 0x3F, 0x1F, 0x0F, 0x07, 0x03, 0x01]).as_u64(),
    NorthWest = Mask::board([0x01, 0x03, 0x07, 0x0F, 0x1F, 0x3F, 0x7F, 0xFF]).as_u64(),
}

impl Diagonals {
    pub const fn as_mask(&self) -> Mask {
        Mask::new(*self as u64)
    }

    pub const fn cardinal(&self) -> Cardinals {
        match self {
            Diagonals::NorthEast => Cardinals::NorthEast,
            Diagonals::SouthEast => Cardinals::SouthEast,
            Diagonals::SouthWest => Cardinals::SouthWest,
            Diagonals::NorthWest => Cardinals::NorthWest,
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u64)]
pub enum Cardinals {
    North = Orthogonals::North as u64,
    NorthEast = Diagonals::NorthEast as u64,
    East = Orthogonals::East as u64,
    SouthEast = Diagonals::SouthEast as u64,
    South = Orthogonals::South as u64,
    SouthWest = Diagonals::SouthWest as u64,
    West = Orthogonals::West as u64,
    NorthWest = Diagonals::NorthWest as u64,
}

impl Cardinals {
    pub const fn as_mask(&self) -> Mask {
        Mask::new(*self as u64)
    }
}
