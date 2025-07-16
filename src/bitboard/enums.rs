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
        match self {
            Self::White => Mask::board([
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_11111111,
                0b_11111111,
            ]),
            Self::Black => Mask::board([
                0b_11111111,
                0b_11111111,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
                0b_00000000,
            ]),
        }
    }

    pub const fn other(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub const fn white_letter(&self) -> char {
        match self {
            Piece::Pawn => 'P',
            Piece::Knight => 'N',
            Piece::Bishop => 'B',
            Piece::Rook => 'R',
            Piece::Queen => 'Q',
            Piece::King => 'K',
        }
    }

    pub const fn black_letter(&self) -> char {
        match self {
            Piece::Pawn => 'p',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Rook => 'r',
            Piece::Queen => 'q',
            Piece::King => 'k',
        }
    }

    pub const fn black_unicode(&self) -> char {
        match self {
            Piece::Pawn => '\u{265F}',
            Piece::Knight => '\u{265E}',
            Piece::Bishop => '\u{265D}',
            Piece::Rook => '\u{265C}',
            Piece::Queen => '\u{265B}',
            Piece::King => '\u{265A}',
        }
    }

    pub const fn white_unicode(&self) -> char {
        match self {
            Piece::Pawn => '\u{2659}',
            Piece::Knight => '\u{2658}',
            Piece::Bishop => '\u{2657}',
            Piece::Rook => '\u{2656}',
            Piece::Queen => '\u{2655}',
            Piece::King => '\u{2654}',
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Dir {
    North = 1,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Dir {
    pub const fn as_offset(&self) -> (i8, i8) {
        match self {
            Dir::North => (0, 1),
            Dir::NorthEast => (1, 1),
            Dir::East => (1, 0),
            Dir::SouthEast => (1, -1),
            Dir::South => (0, -1),
            Dir::SouthWest => (-1, -1),
            Dir::West => (-1, 0),
            Dir::NorthWest => (-1, 1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorPiece {
    WhitePawn = 1,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

impl ColorPiece {
    pub const fn new(c: Color, p: Piece) -> Self {
        use Color::*;
        use ColorPiece::*;
        use Piece::*;
        match c {
            White => match p {
                Pawn => WhitePawn,
                Knight => WhiteKnight,
                Bishop => WhiteBishop,
                Rook => WhiteRook,
                Queen => WhiteQueen,
                King => WhiteKing,
            },
            Black => match p {
                Pawn => BlackPawn,
                Knight => BlackKnight,
                Bishop => BlackBishop,
                Rook => BlackRook,
                Queen => BlackQueen,
                King => BlackKing,
            },
        }
    }

    pub const fn color(&self) -> Color {
        self.split().0
    }

    pub const fn piece(&self) -> Piece {
        self.split().1
    }

    pub const fn split(&self) -> (Color, Piece) {
        use Color::*;
        use ColorPiece::*;
        use Piece::*;
        match self {
            WhitePawn => (White, Pawn),
            WhiteKnight => (White, Knight),
            WhiteBishop => (White, Bishop),
            WhiteRook => (White, Rook),
            WhiteQueen => (White, Queen),
            WhiteKing => (White, King),
            BlackPawn => (Black, Pawn),
            BlackKnight => (Black, Knight),
            BlackBishop => (Black, Bishop),
            BlackRook => (Black, Rook),
            BlackQueen => (Black, Queen),
            BlackKing => (Black, King),
        }
    }

    pub const fn letter(&self) -> char {
        use ColorPiece::*;
        match self.color() {
            Color::White => self.piece().white_letter(),
            Color::Black => self.piece().black_letter(),
        }
    }

    pub const fn unicode(&self) -> char {
        use ColorPiece::*;
        match self.color() {
            Color::White => self.piece().white_unicode(),
            Color::Black => self.piece().black_unicode(),
        }
    }
}
