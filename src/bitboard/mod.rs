mod colorfault;
pub mod enums;
pub mod hash;
pub mod masks;
pub mod movedbs;
pub mod pieces;
pub mod squares;

use std::{collections::HashSet, hash::Hash, sync::LazyLock};

use enums::Color;

use crate::bitboard::{
    colorfault::Colorfault,
    hash::BitBoardHasher,
    masks::Mask,
    pieces::{
        bishops::Bishops, kings::Kings, knights::Knights, pawns::Pawns, queens::Queens,
        rooks::Rooks,
    },
    squares::Square,
};

#[derive(Clone, Debug)]
pub struct BitBoard {
    metadata: Metadata,
    white: HalfBitBoard,
    black: HalfBitBoard,
}

impl BitBoard {
    pub fn render(&self, board: &mut [char; 64], highlights: &mut [bool; 64]) {
        if let Some((f, t)) = self.metadata.most_recent_move {
            highlights[f.index() as usize] = true;
            highlights[t.index() as usize] = true;
        }

        self.white.render(board, Color::White);
        self.black.render(board, Color::Black);
    }

    pub fn as_mask(&self) -> Mask {
        self.white.as_mask() | self.black.as_mask()
    }

    pub fn overlap(&self) -> Mask {
        self.white.overlap() & self.black.overlap()
    }

    pub fn invariant(&self) {
        self.white.invariant();
        self.black.invariant();
        assert!(!self.overlap().any());
    }
}

impl Default for BitBoard {
    fn default() -> Self {
        Self {
            metadata: Default::default(),
            white: Colorfault::colorfault(Color::White),
            black: Colorfault::colorfault(Color::Black),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HalfBitBoard {
    kings: Kings,
    queens: Queens,
    rooks: Rooks,
    bishops: Bishops,
    knights: Knights,
    pawns: Pawns,
}

impl Colorfault for HalfBitBoard {
    fn colorfault(c: Color) -> Self {
        Self {
            kings: Colorfault::colorfault(c),
            queens: Colorfault::colorfault(c),
            rooks: Colorfault::colorfault(c),
            bishops: Colorfault::colorfault(c),
            knights: Colorfault::colorfault(c),
            pawns: Colorfault::colorfault(c),
        }
    }
}

impl HalfBitBoard {
    pub fn render(&self, board: &mut [char; 64], color: Color) {
        self.kings.render(board, color);
        self.queens.render(board, color);
        self.rooks.render(board, color);
        self.bishops.render(board, color);
        self.knights.render(board, color);
        self.pawns.render(board, color);
    }

    pub fn as_mask(&self) -> Mask {
        self.kings.as_mask()
            | self.queens.as_mask()
            | self.rooks.as_mask()
            | self.bishops.as_mask()
            | self.knights.as_mask()
            | self.pawns.as_mask()
    }

    pub fn overlap(&self) -> Mask {
        self.kings.as_mask()
            & self.queens.as_mask()
            & self.rooks.as_mask()
            & self.bishops.as_mask()
            & self.knights.as_mask()
            & self.pawns.as_mask()
    }

    pub fn invariant(&self) {
        assert!(!self.overlap().any());
    }
}

#[derive(Clone, Debug)]
pub struct Metadata {
    hash: Option<u128>,
    to_move: Color,
    half_turn: usize,
    change_happened_at: usize,
    white_castling: CastlingRights,
    black_castling: CastlingRights,
    most_recent_move: Option<(Square, Square)>,
    en_passant: Option<Square>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            hash: None,
            to_move: Color::White,
            half_turn: 1,
            change_happened_at: 0,
            white_castling: CastlingRights::default(),
            black_castling: CastlingRights::default(),
            most_recent_move: None,
            en_passant: None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CastlingRights {
    long: bool,
    short: bool,
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self {
            long: true,
            short: true,
        }
    }
}
