use core::panic::PanicMessage;
use std::fmt::Display;

use crate::bitboard::{
    BitBoard, CastlingInfo,
    castling::CastlingSide,
    enums::{Color, ColorPiece, Dir, Piece, Rank},
    half::HalfBitBoard,
    masks::Mask,
    pieces::{
        kings::Kings,
        knights::{self, Knights},
        pawns::{self, Pawns},
    },
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ProtoMove {
    pub from: Square,
    pub to: Square,
}

impl ProtoMove {
    pub fn dist(&self) -> i8 {
        self.from.dist(self.to)
    }

    pub fn dir(&self) -> Option<Dir> {
        todo!()
    }

    pub fn as_mask(&self) -> Mask {
        Mask::nil().set(self.from).set(self.to)
    }

    pub fn makes_king_checked(
        &self,
        active: Mask,
        king: Kings,
        capture: Option<(Square, Piece)>,
        passive: &HalfBitBoard,
        passive_color: Color,
    ) -> bool {
        passive
            .threats(passive_color, active ^ self.as_mask(), capture)
            .overlap(king.as_mask())
            .any()
    }
}

impl Display for ProtoMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.from, self.to)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub color_and_piece: ColorPiece,
    pub from_to: ProtoMove,
    pub castling: Option<CastlingSide>,
    pub capture: Option<(Square, Piece)>,
    pub promotion: Option<Piece>,
}

impl Move {
    pub fn en_passant_square(&self) -> Option<Square> {
        if self.color_and_piece == ColorPiece::WhitePawn {
            if let ((f, Rank::_2), Rank::_4) =
                (self.from_to.from.algebraic(), self.from_to.to.rank())
            {
                return Some(Square::at(f, Rank::_3));
            } else {
                return None;
            };
        } else if self.color_and_piece == ColorPiece::BlackPawn {
            if let ((f, Rank::_7), Rank::_5) =
                (self.from_to.from.algebraic(), self.from_to.to.rank())
            {
                return Some(Square::at(f, Rank::_6));
            } else {
                return None;
            };
        } else {
            return None;
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.color_and_piece.unicode(), self.from_to)?;
        if let Some((sq, p)) = self.capture {
            write!(
                f,
                "\u{00D7}{}{}",
                ColorPiece::new(self.color_and_piece.color().other(), p).unicode(),
                sq
            )?;
        }
        if let Some(c) = self.castling {
            match c {
                CastlingSide::OOO => write!(f, "O-O-O")?,
                CastlingSide::OO => write!(f, "O-O")?,
            }
        }
        Ok(())
    }
}

#[test]
fn size_fuckery() {
    assert_eq!(std::mem::size_of::<Move>(), 8);
    assert_eq!(std::mem::size_of::<Option<Move>>(), 8);
}

impl BitBoard {
    pub fn apply(&mut self, mv: &Move) {
        let (active, passive) = self.color_mut(mv.color_and_piece.color());
        *active.piece_mask_mut(mv.color_and_piece.piece()) ^= mv.from_to.as_mask();
        if let Some((sq, piece)) = mv.capture {
            *passive.piece_mask_mut(piece) ^= sq.as_mask();
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

        self.active()
            .kings
            .enumerate_legal_moves(color, active_mask, self.passive(), res);
    }
}
