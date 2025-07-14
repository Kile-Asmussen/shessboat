pub mod boardmap;
pub mod enums;
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

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
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

impl HalfBitBoard {
    pub fn new(board: &BoardMap<char>, c: Color) -> Self {
        Self {
            kings: Kings::new(board.to_mask(c.letter(Piece::King))),
            queens: Queens::new(board.to_mask(c.letter(Piece::Queen))),
            rooks: Rooks::new(board.to_mask(c.letter(Piece::Rook))),
            bishops: Bishops::new(board.to_mask(c.letter(Piece::Bishop))),
            knights: Knights::new(board.to_mask(c.letter(Piece::Knight))),
            pawns: Pawns::new(board.to_mask(c.letter(Piece::Pawn))),
        }
    }

    pub fn render(&self, board: &mut BoardMap<char>, color: Color) {
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
    ooo: bool,
    oo: bool,
}

impl CastlingRights {
    pub fn new(ooo: bool, oo: bool) -> Self {
        Self { ooo, oo }
    }

    fn ooo(&self) -> bool {
        self.ooo
    }

    fn oo(&self) -> bool {
        self.oo
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self {
            ooo: true,
            oo: true,
        }
    }
}
