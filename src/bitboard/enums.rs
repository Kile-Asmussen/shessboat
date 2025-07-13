use crate::bitboard::squares::Square;

use super::masks::Mask;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Color {
    White = 1,
    Black,
}

impl Color {
    pub const fn as_mask(&self) -> Mask {
        use Color::*;
        Mask::board(match self {
            White => [
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_11111111,
                0b_11111111,
            ],
            Black => [
                0b_11111111,
                0b_11111111,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
            ],
        })
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Shade {
    Dark = 1,
    Light,
}

impl Shade {
    pub const fn as_mask(&self) -> Mask {
        use Shade::*;
        Mask::board(match self {
            Dark => [
                0b_01010101,
                0b_10101010,
                0b_01010101,
                0b_10101010,
                0b_01010101,
                0b_10101010,
                0b_01010101,
                0b_10101010,
            ],
            Self::Light => [
                0b_10101010,
                0b_01010101,
                0b_10101010,
                0b_01010101,
                0b_10101010,
                0b_01010101,
                0b_10101010,
                0b_01010101,
            ],
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Rank {
    _1 = 1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
}

impl Rank {
    pub const fn as_mask(&self) -> Mask {
        use Rank::*;
        Mask::board(match self {
            _1 => [0, 0, 0, 0, 0, 0, 0, !0],
            _2 => [0, 0, 0, 0, 0, 0, !0, 0],
            _3 => [0, 0, 0, 0, 0, !0, 0, 0],
            _4 => [0, 0, 0, 0, !0, 0, 0, 0],
            _5 => [0, 0, 0, !0, 0, 0, 0, 0],
            _6 => [0, 0, !0, 0, 0, 0, 0, 0],
            _7 => [0, !0, 0, 0, 0, 0, 0, 0],
            _8 => [!0, 0, 0, 0, 0, 0, 0, 0],
        })
    }

    pub const fn rank(e: i8) -> Option<Self> {
        use Rank::*;
        match e {
            0..=7 => Some([_1, _2, _3, _4, _5, _6, _7, _8][e as usize]),
            _ => None,
        }
    }

    pub const fn as_rank(&self) -> i8 {
        *self as u8 as i8 - 1
    }

    pub const fn as_char(&self) -> char {
        match self {
            Rank::_1 => '1',
            Rank::_2 => '2',
            Rank::_3 => '3',
            Rank::_4 => '4',
            Rank::_5 => '5',
            Rank::_6 => '6',
            Rank::_7 => '7',
            Rank::_8 => '8',
        }
    }
}

#[test]
fn rank_roundtrip() {
    assert_eq!(
        Rank::_2.as_mask().first(),
        Some(Square::at(File::A, Rank::_2))
    );
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum File {
    A = 1,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub const fn as_mask(&self) -> Mask {
        use File::*;
        Mask::board(match self {
            A => [1 << 7; 8],
            B => [1 << 6; 8],
            C => [1 << 5; 8],
            D => [1 << 4; 8],
            E => [1 << 3; 8],
            F => [1 << 2; 8],
            G => [1 << 1; 8],
            H => [1 << 0; 8],
        })
    }

    pub const fn file(e: i8) -> Option<Self> {
        use File::*;
        match e {
            0..=7 => Some([A, B, C, D, E, F, G, H][e as usize]),
            _ => None,
        }
    }

    pub const fn as_file(&self) -> i8 {
        *self as u8 as i8 - 1
    }

    pub const fn as_char(&self) -> char {
        use File::*;
        match self {
            A => 'a',
            B => 'b',
            C => 'c',
            D => 'd',
            E => 'e',
            F => 'f',
            G => 'g',
            H => 'h',
        }
    }
}

#[test]
fn file_mask_roundtrip() {
    assert_eq!(
        File::C.as_mask().first(),
        Some(Square::at(File::C, Rank::_1))
    );
}

#[rustfmt::skip]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Piece {
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
}

impl Piece {
    #[rustfmt::skip]
    pub const fn as_mask(&self) -> Mask {
        use Piece::*;
        Mask::board(match self {
            Pawn => [
                0b_00000000,
                0b_11111111,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_11111111,
                0b_00000000,
            ],
            Knight => [
                0b_01000010,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_01000010,
            ],
            Bishop => [
                0b_00100100,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00100100,
            ],
            Rook => [
                0b_10000001,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_10000001,
            ],
            Queen => [
                0b_00010000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00010000,
            ],
            King => [
                0b_00001000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00001000,
            ],
        })
    }

    pub const fn letter(&self) -> char {
        match self {
            Piece::Pawn => 'P',
            Piece::Knight => 'N',
            Piece::Bishop => 'B',
            Piece::Rook => 'R',
            Piece::Queen => 'Q',
            Piece::King => 'K',
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Dir {
    North = 1,
    East,
    South,
    West,
}

impl Dir {
    pub const fn as_offset(&self) -> i8 {
        match self {
            Dir::North => 8,
            Dir::East => 1,
            Dir::South => -8,
            Dir::West => -1,
        }
    }
}
