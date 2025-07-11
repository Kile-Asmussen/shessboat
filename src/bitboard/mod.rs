mod colorfault;
mod enums;
mod hash;
mod masks;
mod pieces;
mod squares;

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

struct BitBoard {
    metadata: Metadata,
    white: HalfBitBoard,
    black: HalfBitBoard,
}

impl BitBoard {
    fn render(&self, board: &mut [char; 64], highlights: &mut [bool; 64]) {
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

struct HalfBitBoard {
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
    fn render(&self, board: &mut [char; 64], color: Color) {}
}

struct Metadata {
    hash: Option<u128>,
    turn: Color,
    most_recent_move: Option<(Square, Square)>,
    en_passant: Option<Square>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            hash: None,
            turn: Color::White,
            most_recent_move: None,
            en_passant: None,
        }
    }
}
