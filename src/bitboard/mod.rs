pub mod boardmap;
pub mod castling;
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
    castling::{CastlingInfo, CastlingRight, CastlingRights},
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
    pub metadata: Metadata,
    pub white: HalfBitBoard,
    pub black: HalfBitBoard,
}

impl BitBoard {
    pub fn new() -> Self {
        Self::new_960(518)
    }

    pub fn new_960(n: usize) -> Self {
        Self::new_starting_array(chess_960(n))
    }

    pub fn new_starting_array(mut arr: [char; 8]) -> Self {
        let mut board = [' '; 64];

        board[0..8].copy_from_slice(&arr);
        arr = arr.map(|c| c.to_ascii_lowercase());
        board[56..64].copy_from_slice(&arr);
        board[8..16].fill('P');
        board[48..56].fill('p');

        let mut board = BoardMap::new(board);

        return Self::new_board(&board);
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

    pub fn active(&self) -> &HalfBitBoard {
        match self.metadata.to_move {
            Color::White => &self.white,
            Color::Black => &self.black,
        }
    }

    pub fn passive(&self) -> &HalfBitBoard {
        match self.metadata.to_move {
            Color::White => &self.black,
            Color::Black => &self.white,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Metadata {
    pub hash: u128,
    pub to_move: Color,
    pub half_turn: usize,
    pub change_happened_at: usize,
    pub white_castling: CastlingRights,
    pub black_castling: CastlingRights,
    pub en_passant: Option<Square>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            hash: 0,
            to_move: Color::White,
            half_turn: 1,
            change_happened_at: 0,
            white_castling: CastlingInfo::default(),
            black_castling: CastlingInfo::default(),
            en_passant: None,
        }
    }
}
