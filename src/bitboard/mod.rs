pub mod boardmap;
pub mod enums;
pub mod half;
pub mod hash;
pub mod masks;
pub mod moves;
pub mod pieces;
pub mod squares;

use std::{collections::HashSet, hash::Hash, sync::LazyLock};

use enums::Color;

use crate::bitboard::{
    boardmap::BoardMap,
    enums::{File, Piece, Rank},
    half::HalfBitBoard,
    hash::BitBoardHasher,
    masks::Mask,
    pieces::{
        bishops::Bishops, chess_960, kings::Kings, knights::Knights, pawns::Pawns, queens::Queens,
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
    pub fn new() -> Self {
        Self::new_960(518)
    }

    pub fn new_960(n: usize) -> Self {
        let mut arr = chess_960(n);
        let mut board = [' '; 64];

        memcpy(&mut board[0..8], &arr);
        for c in &mut arr {
            *c = c.to_ascii_lowercase();
        }
        memcpy(&mut board[56..64], &arr);
        memset(&mut board[8..16], 'P');
        memset(&mut board[48..56], 'p');

        let mut board = BoardMap::new(board);

        return Self::new_board(&board);

        fn memcpy(dst: &mut [char], src: &[char]) {
            for (d, s) in dst.iter_mut().zip(src.iter()) {
                *d = *s;
            }
        }

        fn memset(dst: &mut [char], c: char) {
            for d in dst {
                *d = c;
            }
        }
    }

    pub fn new_board(board: &BoardMap<char>) -> Self {
        Self {
            metadata: Metadata::default(),
            white: HalfBitBoard::new(board, Color::White),
            black: HalfBitBoard::new(board, Color::Black),
        }
    }

    pub fn render(&self, board: &mut BoardMap<char>) {
        self.white.render(board, Color::White);
        self.black.render(board, Color::Black);
    }

    pub fn as_mask(&self) -> Mask {
        self.white.as_mask() | self.black.as_mask()
    }

    pub fn white(&self) -> &HalfBitBoard {
        &self.white
    }

    pub fn black(&self) -> &HalfBitBoard {
        &self.black
    }

    pub fn active(&self) -> &HalfBitBoard {
        match self.metadata.to_move {
            Color::White => self.white(),
            Color::Black => self.black(),
        }
    }

    pub fn inactive(&self) -> &HalfBitBoard {
        match self.metadata.to_move {
            Color::White => self.black(),
            Color::Black => self.white(),
        }
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

#[derive(Clone, Debug)]
pub struct Metadata {
    hash: u128,
    to_move: Color,
    half_turn: usize,
    change_happened_at: usize,
    white_castling: CastlingRights,
    black_castling: CastlingRights,
    en_passant: Option<Square>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            hash: 0,
            to_move: Color::White,
            half_turn: 1,
            change_happened_at: 0,
            white_castling: CastlingRights::default(),
            black_castling: CastlingRights::default(),
            en_passant: None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CastlingRights {
    ooo: CastlingRight,
    oo: CastlingRight,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CastlingRight {
    Retained = 1,
    Forefeited,
    Claimed,
}

impl CastlingRights {
    pub fn new(ooo: CastlingRight, oo: CastlingRight) -> Self {
        Self { ooo, oo }
    }

    fn ooo(&self) -> CastlingRight {
        self.ooo
    }

    fn oo(&self) -> CastlingRight {
        self.oo
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self {
            ooo: CastlingRight::Retained,
            oo: CastlingRight::Retained,
        }
    }
}
