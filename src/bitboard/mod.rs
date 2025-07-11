mod colorfault;
pub mod enums;
pub mod hash;
pub mod masks;
pub mod pieces;
pub mod squares;

use std::{collections::HashSet, hash::Hash, sync::LazyLock};

use enums::Color;

use crate::bitboard::{
    colorfault::Colorfault,
    hash::BitBoardHasher,
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
    fn render(&self, board: &mut [char; 64], color: Color) {
        self.kings.render(board, color);
        self.queens.render(board, color);
        self.rooks.render(board, color);
        self.bishops.render(board, color);
        self.knights.render(board, color);
        self.pawns.render(board, color);
    }
}

#[derive(Clone, Debug)]
pub struct Metadata {
    hash: Option<u128>,
    turn: Color,
    white_castling: CastlingRights,
    black_castling: CastlingRights,
    most_recent_move: Option<(Square, Square)>,
    en_passant: Option<Square>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            hash: None,
            turn: Color::White,
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
