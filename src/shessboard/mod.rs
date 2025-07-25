pub mod boardmap;
pub mod castling;
pub mod enums;
pub mod half;
pub mod masks;
pub mod metadata;
pub mod moves;
pub mod notation;
pub mod pieces;
pub mod repetions;
pub mod squares;
pub mod zobrist;

use std::{collections::HashSet, hash::Hash, sync::LazyLock};

use crate::shessboard::{
    boardmap::BoardMap,
    castling::{CastlingDetail, CastlingDetails, CastlingInfo, CastlingRights},
    enums::{Color, ColorPiece, File, Piece, Rank},
    half::HalfBitBoard,
    masks::Mask,
    metadata::Metadata,
    moves::Move,
    pieces::{
        Micropawns, P, bishops::Bishops, chess_960, kings::Kings, knights::Knights, pawns::Pawns,
        queens::Queens, rooks::Rooks,
    },
    repetions::ThreefoldRule,
    squares::Square,
    zobrist::{BitBoardHasher, HashResult},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitBoard {
    pub metadata: Metadata,
    pub white: HalfBitBoard,
    pub black: HalfBitBoard,
}

#[test]
fn bitboard_size() {
    dbg!(
        std::mem::size_of::<BitBoard>(),
        std::mem::size_of::<HalfBitBoard>(),
        std::mem::size_of::<HalfBitBoard>() * 2,
        std::mem::size_of::<Metadata>(),
        std::mem::size_of::<BoardMap<Option<ColorPiece>>>()
    );
}

impl BitBoard {
    pub fn new() -> Self {
        use Piece::*;
        Self::new_starting_array(
            [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook],
            Metadata::new(),
        )
    }

    pub fn empty() -> Self {
        Self::new_board(&BoardMap::new_with(None), Metadata::empty())
    }

    // pub fn new_480(n: usize) -> Self {
    //     let arr = chess_960(n);
    //     Self::new_starting_array(arr, Metadata::new_480(arr))
    // }

    // pub fn new_960(n: usize) -> Self {
    //     let arr = chess_960(n);
    //     Self::new_starting_array(arr, Metadata::new_960(arr))
    // }

    pub fn new_starting_array(arr: [Piece; 8], metadata: Metadata) -> Self {
        let mut board = [None; 64];

        board[0..8].copy_from_slice(&arr.map(|p| Some(ColorPiece::new(Color::White, p))));
        board[56..64].copy_from_slice(&arr.map(|p| Some(ColorPiece::new(Color::Black, p))));
        board[8..16].fill(Some(ColorPiece::WhitePawn));
        board[48..56].fill(Some(ColorPiece::BlackPawn));

        let mut board = BoardMap::new(board);
        return Self::new_board(&board, metadata);
    }

    pub fn new_board(board: &BoardMap<Option<ColorPiece>>, metadata: Metadata) -> Self {
        Self {
            metadata,
            white: HalfBitBoard::new(board, Color::White),
            black: HalfBitBoard::new(board, Color::Black),
        }
    }

    pub fn render(&self, board: &mut BoardMap<Option<ColorPiece>>) {
        self.white.render(board, Color::White);
        self.black.render(board, Color::Black);
    }

    pub fn as_mask(&self) -> Mask {
        self.white.as_mask() | self.black.as_mask()
    }

    pub fn color(&self, color: Color) -> &HalfBitBoard {
        match color {
            Color::White => &self.white,
            Color::Black => &self.black,
        }
    }

    pub fn color_mut(&mut self, color: Color) -> (&mut HalfBitBoard, &mut HalfBitBoard) {
        match color {
            Color::White => (&mut self.white, &mut self.black),
            Color::Black => (&mut self.black, &mut self.white),
        }
    }

    pub fn set_piece(&mut self, c: Option<ColorPiece>, sq: Square) {
        if let Some(c) = c {
            let (b, _) = self.color_mut(c.color());
            b.set_piece(Some(c.piece()), sq);
        } else {
            self.white.set_piece(None, sq);
            self.black.set_piece(None, sq);
        }
    }

    pub fn overwrite(&mut self, board: &BoardMap<Option<ColorPiece>>) {
        for (sq, p) in board {
            self.set_piece(p, sq);
        }
    }

    pub fn active(&self) -> &HalfBitBoard {
        self.color(self.metadata.to_move)
    }

    pub fn passive(&self) -> &HalfBitBoard {
        match self.metadata.to_move {
            Color::White => &self.black,
            Color::Black => &self.white,
        }
    }

    pub fn is_in_check(&self, c: Color) -> bool {
        (self.color(c).kings.as_mask()
            & self.color(c.other()).threats(
                self.metadata.to_move.other(),
                self.active().as_mask(),
                None,
            ))
        .any()
    }

    pub fn sufficient_checkmating_materiel(&self) -> bool {
        self.white.has_sufficient_materiel() || self.black.has_sufficient_materiel()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GameEnd {
    WhiteWins = 1,
    BlackWins = 2,
    Draw = 3,
}

impl GameEnd {
    pub const fn value(&self, c: Color) -> Micropawns {
        if let GameEnd::Draw = *self {
            0
        } else if let (GameEnd::WhiteWins, Color::White) = (*self, c) {
            Self::VICTORY
        } else if let (GameEnd::BlackWins, Color::Black) = (*self, c) {
            Self::VICTORY
        } else {
            Self::DEFEAT
        }
    }

    pub const VICTORY: Micropawns = 1_000_000 * P;
    pub const DEFEAT: Micropawns = -Self::VICTORY;

    pub const fn from_color(c: Color) -> Self {
        match c {
            Color::White => Self::WhiteWins,
            Color::Black => Self::BlackWins,
        }
    }

    pub const fn to_str(&self) -> &'static str {
        match self {
            Self::WhiteWins => "0–1",
            Self::BlackWins => "1–0",
            Self::Draw => "½–½",
        }
    }

    pub fn determine<'a>(
        board: &BitBoard,
        moves: &[Move],
        hash: HashResult,
        three: &'a ThreefoldRule<'a>,
    ) -> Option<Self> {
        if board.metadata.tempo - board.metadata.last_change >= 150 {
            Some(Self::Draw)
        } else if moves.len() == 0 {
            if board.is_in_check(board.metadata.to_move) {
                Some(Self::from_color(board.metadata.to_move.other()))
            } else {
                Some(Self::Draw)
            }
        } else if !board.sufficient_checkmating_materiel() {
            Some(Self::Draw)
        } else if three.count(hash) >= 3 {
            Some(Self::Draw)
        } else {
            None
        }
    }
}
