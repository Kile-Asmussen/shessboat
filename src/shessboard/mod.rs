pub mod boardmap;
pub mod castling;
pub mod enums;
pub mod forced_draws;
pub mod half;
pub mod masks;
pub mod metadata;
pub mod moves;
pub mod notation;
pub mod pieces;
pub mod squares;
pub mod zobrist;

use std::{collections::HashSet, hash::Hash, sync::LazyLock};

use crate::shessboard::{
    boardmap::BoardMap,
    castling::{CastlingDetail, CastlingDetails, CastlingInfo, CastlingRights},
    enums::{Color, ColorPiece, File, Piece, Rank},
    forced_draws::{LastChange, ThreefoldRule},
    half::HalfBitBoard,
    masks::Mask,
    metadata::Metadata,
    moves::Move,
    pieces::{
        Millipawns, P,
        bishops::Bishops,
        chess_960,
        kings::Kings,
        knights::Knights,
        pawns::{EnPassant, Pawns},
        queens::Queens,
        rooks::Rooks,
    },
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

impl BitBoard {
    pub fn apply(&mut self, mv: Move) {
        let (color, piece) = mv.color_and_piece.split();

        // update metadata
        self.metadata.to_move = color.other();
        self.metadata.tempo += 1;
        self.metadata.en_passant = mv.en_passant_square();

        // calculate changes to castling rights
        let details = self.metadata.castling_details;
        let (cr_active, cr_passive) = mv.castling_rights(self.metadata.castling_details);

        let (active_castling, passive_castling) = self.metadata.castling_rights_mut(color);

        active_castling.update(cr_active);
        passive_castling.update(cr_passive);

        // calculate changes to board
        let (active, passive) = self.color_mut(color);

        if let Some((sq, piece)) = mv.capture {
            *passive.piece_mask_mut(piece) ^= sq.as_mask();
        }

        if let Some(p) = mv.promotion {
            *active.piece_mask_mut(Piece::Pawn) ^= mv.from_to.from.as_mask();
            *active.piece_mask_mut(p) ^= mv.from_to.to.as_mask();
        } else if let Some(castling::CastlingSide::OOO) = mv.castling {
            *active.piece_mask_mut(Piece::Rook) ^= details
                .ooo
                .rook_move
                .as_move(color.starting_rank())
                .as_mask();
            *active.piece_mask_mut(Piece::King) ^= details
                .ooo
                .king_move
                .as_move(color.starting_rank())
                .as_mask()
        } else if let Some(castling::CastlingSide::OO) = mv.castling {
            *active.piece_mask_mut(Piece::Rook) ^= details
                .oo
                .rook_move
                .as_move(color.starting_rank())
                .as_mask();
            *active.piece_mask_mut(Piece::King) ^= details
                .oo
                .king_move
                .as_move(color.starting_rank())
                .as_mask()
        } else {
            *active.piece_mask_mut(mv.color_and_piece.piece()) ^= mv.from_to.as_mask();
        }
    }

    pub fn undo(&mut self, mv: Move) {
        let (color, piece) = mv.color_and_piece.split();

        // update metadata
        self.metadata.to_move = color;
        self.metadata.tempo -= 1;
        self.metadata.en_passant = if let Some((capture, Piece::Pawn)) = mv.capture {
            if piece == Piece::Pawn && mv.from_to.to != capture {
                Some(EnPassant {
                    to: mv.from_to.to,
                    capture,
                })
            } else {
                None
            }
        } else {
            None
        };

        // calculate changes to castling rights
        let details = self.metadata.castling_details;
        let (cr_active, cr_passive) = mv.castling_rights(self.metadata.castling_details);

        let (active_castling, passive_castling) = self.metadata.castling_rights_mut(color);

        active_castling.downdate(cr_active);
        passive_castling.downdate(cr_passive);

        // calculate changes to board
        let (active, passive) = self.color_mut(color);

        if let Some((sq, piece)) = mv.capture {
            *passive.piece_mask_mut(piece) ^= sq.as_mask();
        }

        if let Some(p) = mv.promotion {
            *active.piece_mask_mut(Piece::Pawn) ^= mv.from_to.from.as_mask();
            *active.piece_mask_mut(p) ^= mv.from_to.to.as_mask();
        } else if let Some(castling::CastlingSide::OOO) = mv.castling {
            *active.piece_mask_mut(Piece::Rook) ^= details
                .ooo
                .rook_move
                .as_move(color.starting_rank())
                .as_mask();
            *active.piece_mask_mut(Piece::King) ^= details
                .ooo
                .king_move
                .as_move(color.starting_rank())
                .as_mask()
        } else if let Some(castling::CastlingSide::OO) = mv.castling {
            *active.piece_mask_mut(Piece::Rook) ^= details
                .oo
                .rook_move
                .as_move(color.starting_rank())
                .as_mask();
            *active.piece_mask_mut(Piece::King) ^= details
                .oo
                .king_move
                .as_move(color.starting_rank())
                .as_mask()
        } else {
            *active.piece_mask_mut(mv.color_and_piece.piece()) ^= mv.from_to.as_mask();
        }
    }

    pub fn generate_moves(&self, res: &mut Vec<Move>) {
        let active_mask = self.active().as_mask();
        let passive_mask = self.passive().as_mask();
        let color = self.metadata.to_move;

        self.active().queens.enumerate_legal_moves(
            color,
            active_mask,
            self.passive(),
            passive_mask,
            self.active().kings,
            res,
        );

        self.active().rooks.enumerate_legal_moves(
            color,
            active_mask,
            self.passive(),
            passive_mask,
            self.active().kings,
            res,
        );

        self.active().bishops.enumerate_legal_moves(
            color,
            active_mask,
            self.passive(),
            passive_mask,
            self.active().kings,
            res,
        );

        self.active().knights.enumerate_legal_moves(
            color,
            active_mask,
            self.passive(),
            self.active().kings,
            res,
        );

        self.active().pawns.enumerate_legal_moves(
            color,
            active_mask,
            passive_mask,
            self.passive(),
            self.metadata.en_passant,
            self.active().kings,
            res,
        );

        self.active().kings.enumerate_legal_moves(
            color,
            active_mask,
            passive_mask,
            self.passive(),
            self.metadata.castling_rights(color).0,
            self.metadata.castling_details,
            res,
        );
    }
}
