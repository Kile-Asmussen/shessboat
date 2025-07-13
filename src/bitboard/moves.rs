use crate::bitboard::{
    enums::{Color, Piece},
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    from: Square,
    to: Square,
}

impl Move {
    pub const fn new(from: Square, to: Square) -> Self {
        Self { from, to }
    }

    pub const fn from(&self) -> Square {
        self.from
    }

    pub const fn to(&self) -> Square {
        self.to
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ValidMove {
    color: Color,
    piece: Piece,
    moved: Move,
    en_passant: Option<Square>,
    captured: Option<Capture>,
    castled: Option<Move>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Capture {
    piece: Piece,
    at: Square,
}

impl ValidMove {
    fn from(&self) -> Square {
        self.moved.from()
    }

    fn to(&self) -> Square {
        self.moved.to()
    }

    pub fn en_passant(&self) -> Option<Square> {
        self.en_passant
    }
}
