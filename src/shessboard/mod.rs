pub mod boardmap;
pub mod castling;
pub mod enums;
pub mod half;
pub mod masks;
pub mod moves;
pub mod notation;
pub mod pieces;
pub mod squares;
pub mod zobrist;

use std::{collections::HashSet, hash::Hash, sync::LazyLock};

use crate::shessboard::{
    boardmap::BoardMap,
    castling::{CastlingInfo, CastlingRights},
    enums::{Color, ColorPiece, File, Piece, Rank},
    half::HalfBitBoard,
    masks::Mask,
    pieces::{
        bishops::Bishops, chess_960, kings::Kings, knights::Knights, pawns::Pawns, queens::Queens,
        rooks::Rooks,
    },
    squares::Square,
    zobrist::BitBoardHasher,
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

    pub fn empty() -> Self {
        Self::new_board(&BoardMap::new_with(None), Metadata::empty())
    }

    pub fn new_960(n: usize) -> Self {
        Self::new_starting_array(chess_960(n))
    }

    pub fn new_starting_array(mut arr: [Piece; 8]) -> Self {
        let mut board = [None; 64];

        board[0..8].copy_from_slice(&arr.map(|p| Some(ColorPiece::new(Color::White, p))));
        board[56..64].copy_from_slice(&arr.map(|p| Some(ColorPiece::new(Color::Black, p))));
        board[8..16].fill(Some(ColorPiece::WhitePawn));
        board[48..56].fill(Some(ColorPiece::BlackPawn));

        let mut board = BoardMap::new(board);
        return Self::new_board(&board, Metadata::new_starting_array(arr));
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

    pub fn only_kings(&self) -> bool {
        self.white.only_king() && self.black.only_king()
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
    pub rook_files: CastlingInfo<File>,
    pub en_passant: Option<(Square, Square)>,
}

impl Metadata {
    pub fn castling_right(&self, color: Color) -> &CastlingRights {
        match color {
            Color::White => &self.white_castling,
            Color::Black => &self.black_castling,
        }
    }

    pub fn castling_right_mut(
        &mut self,
        color: Color,
    ) -> (&mut CastlingRights, &mut CastlingRights) {
        match color {
            Color::White => (&mut self.white_castling, &mut self.black_castling),
            Color::Black => (&mut self.black_castling, &mut self.white_castling),
        }
    }

    pub fn turn(&self) -> usize {
        self.half_turn / 2 + 1
    }

    pub fn turn_clock(&self) -> usize {
        (self.half_turn - self.change_happened_at) / 2 + 1
    }

    pub fn new_starting_array(array: [Piece; 8]) -> Self {
        let mut rook_files = CastlingInfo {
            ooo: File::A,
            oo: File::H,
        };

        let mut king_seen = false;
        for (n, p) in array.iter().enumerate() {
            king_seen |= p == &Piece::King;

            if p == &Piece::Rook {
                if !king_seen {
                    rook_files.ooo = File::file(n as i8).unwrap()
                } else {
                    rook_files.oo = File::file(n as i8).unwrap()
                }
            }
        }

        Self {
            hash: 0,
            to_move: Color::White,
            half_turn: 0,
            change_happened_at: 0,
            white_castling: CastlingRights::new(),
            black_castling: CastlingRights::new(),
            rook_files,
            en_passant: None,
        }
    }

    fn empty() -> Metadata {
        Self {
            hash: 0,
            to_move: Color::White,
            half_turn: 0,
            change_happened_at: 0,
            white_castling: CastlingRights {
                ooo: false,
                oo: false,
            },
            black_castling: CastlingRights {
                ooo: false,
                oo: false,
            },
            rook_files: CastlingInfo {
                ooo: File::A,
                oo: File::H,
            },
            en_passant: None,
        }
    }
}
