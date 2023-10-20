use std::{
    num::NonZeroU8,
    ops::{BitAnd, BitOr, BitXor, Not},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PieceColor {
    White = 0x10, // to move
    Black = 0x20,
}

impl PieceColor {
    pub fn letter(self) -> char {
        use PieceColor::*;
        match self {
            White => 'w',
            Black => 'b',
        }
    }

    pub fn from_letter(c: char) -> Option<Self> {
        use PieceColor::*;
        Some(match c {
            'w' | 'W' => White,
            'b' | 'B' => Black,
            _ => return None,
        })
    }

    fn pawn(self) -> Piece {
        Piece::new(PieceValue::Pawn, self)
    }

    fn knight(self) -> Piece {
        Piece::new(PieceValue::Knight, self)
    }

    fn bishop(self) -> Piece {
        Piece::new(PieceValue::Bishop, self)
    }

    fn rook(self) -> Piece {
        Piece::new(PieceValue::Rook, self)
    }

    fn queen(self) -> Piece {
        Piece::new(PieceValue::Queen, self)
    }

    fn king(self) -> Piece {
        Piece::new(PieceValue::King, self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum PieceValue {
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
}

impl PieceValue {
    pub fn millipawns(self) -> i64 {
        match self {
            PieceValue::King => i64::MAX,
            PieceValue::Queen => 9000,
            PieceValue::Rook => 5000,
            PieceValue::Bishop => 3333,
            PieceValue::Knight => 3000,
            PieceValue::Pawn => 1000,
        }
    }

    pub fn letter(self) -> char {
        match self {
            PieceValue::Pawn => 'P',
            PieceValue::Knight => 'N',
            PieceValue::Bishop => 'B',
            PieceValue::Rook => 'R',
            PieceValue::Queen => 'Q',
            PieceValue::King => 'K',
        }
    }

    fn white(self) -> Piece {
        Piece::new(self, PieceColor::White)
    }

    fn black(self) -> Piece {
        Piece::new(self, PieceColor::Black)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Piece(NonZeroU8);

impl Piece {
    fn new(p: PieceValue, c: PieceColor) -> Self {
        Self(unsafe { NonZeroU8::new_unchecked(p as u8 | c as u8) })
    }

    pub fn letter(self: Self) -> char {
        use PieceColor::*;
        use PieceValue::*;
        match self.decode() {
            (King, White) => 'K',
            (Queen, White) => 'Q',
            (Rook, White) => 'R',
            (Bishop, White) => 'B',
            (Knight, White) => 'N',
            (Pawn, White) => 'P',
            (King, Black) => 'k',
            (Queen, Black) => 'q',
            (Rook, Black) => 'r',
            (Bishop, Black) => 'b',
            (Knight, Black) => 'n',
            (Pawn, Black) => 'p',
        }
    }

    pub fn from_letter(l: char) -> Option<Self> {
        use PieceColor::*;
        use PieceValue::*;
        let (v, c) = match l {
            'K' => (King, White),
            'Q' => (Queen, White),
            'R' => (Rook, White),
            'B' => (Bishop, White),
            'N' => (Knight, White),
            'P' => (Pawn, White),
            'k' => (King, Black),
            'q' => (Queen, Black),
            'r' => (Rook, Black),
            'b' => (Bishop, Black),
            'n' => (Knight, Black),
            'p' => (Pawn, Black),
            _ => return None,
        };
        Some(Self::new(v, c))
    }

    pub fn decode(self) -> (PieceValue, PieceColor) {
        (self.value(), self.color())
    }

    pub fn color(self) -> PieceColor {
        const COLOR_MASK: u8 = PieceColor::White as u8 | PieceColor::Black as u8;
        unsafe { std::mem::transmute(self.0.get() & COLOR_MASK) }
    }

    pub fn value(self) -> PieceValue {
        const VALUE_MASK: u8 = PieceValue::Pawn as u8
            | PieceValue::Knight as u8
            | PieceValue::Bishop as u8
            | PieceValue::Rook as u8
            | PieceValue::Queen as u8
            | PieceValue::King as u8;
        unsafe { std::mem::transmute(self.0.get() & VALUE_MASK) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SquareTint {
    Dark = 0, // a1
    Light = 255,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Square(u8);

impl Square {
    fn index(self) -> usize {
        self.0 as usize
    }

    fn new(i: usize) -> Self {
        if i >= 64 {
            panic!()
        } else {
            Self(i as u8)
        }
    }

    fn coord(self) -> (u8, u8) {
        (self.0 % 8, self.0 / 8)
    }

    pub fn rank(self) -> usize {
        (self.coord().0 + 1) as usize
    }

    pub fn file(self) -> char {
        (self.coord().1 + b'a') as char
    }

    pub fn at(x: u8, y: u8) -> Self {
        if x >= 8 || y >= 8 {
            panic!()
        } else {
            Self(x | (y << 3))
        }
    }

    pub fn algebraic(self) -> &'static str {
        Self::ALGEBRAIC[self.index()]
    }

    pub fn from_algebraic(it: &str) -> Option<Self> {
        Self::ALGEBRAIC
            .iter()
            .position(|ti| it == *ti)
            .map(|s| Square(s as u8))
    }

    pub fn tint(self) -> SquareTint {
        let (x, y) = self.coord();
        if (x + y) % 2 == 0 {
            SquareTint::Dark
        } else {
            SquareTint::Light
        }
    }

    pub fn position(self) -> Position {
        Position(1 << self.index())
    }

    const ALGEBRAIC: [&'static str; 64] = [
        "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", //
        "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", //
        "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", //
        "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4", //
        "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", //
        "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6", //
        "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", //
        "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8", //
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position(u64);

impl Position {
    const EMPTY: Position = Position(0);

    fn any(self) -> bool {
        self != Self::EMPTY
    }

    fn has(self, sq: Square) -> bool {
        (self & sq.position()).any()
    }
}

impl Not for Position {
    type Output = Position;

    fn not(self) -> Self::Output {
        Position(!self.0)
    }
}

impl BitAnd<Position> for Position {
    type Output = Position;

    fn bitand(self, rhs: Position) -> Self::Output {
        Position(self.0 & rhs.0)
    }
}

impl BitOr<Position> for Position {
    type Output = Position;

    fn bitor(self, rhs: Position) -> Self::Output {
        Position(self.0 | rhs.0)
    }
}

impl BitXor<Position> for Position {
    type Output = Position;

    fn bitxor(self, rhs: Position) -> Self::Output {
        Position(self.0 ^ rhs.0)
    }
}

impl IntoIterator for Position {
    type Item = Square;

    type IntoIter = SquareIter;

    fn into_iter(self) -> Self::IntoIter {
        SquareIter(0, self.0)
    }
}

pub struct SquareIter(usize, u64);

impl Iterator for SquareIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 >= 64 {
            Option::None
        } else {
            let res = if self.1 & 0x1 == 1 {
                Some(Square::new(self.0))
            } else {
                None
            };
            self.0 += 1;
            self.1 >>= 1;
            return res;
        }
    }
}
