use std::fmt::Binary;

use crate::shessboard::{
    boardmap::BoardMap,
    enums::{Color, ColorPiece, Dir, Piece},
    half::HalfBitBoard,
    masks::Mask,
    moves::{Move, ProtoMove},
    pieces::{
        Micropawns,
        bishops::{self, Bishops},
        kings::Kings,
        rooks::Rooks,
        slide_move_stop,
    },
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Queens(Mask);

impl Queens {
    pub const fn nil() -> Self {
        Self(Mask::nil())
    }

    pub const fn new(mask: Mask) -> Self {
        Self(mask)
    }

    pub const fn materiel(&self) -> Micropawns {
        self.as_mask().occupied() as i64 * 9_000_000
    }

    pub const fn as_mask(&self) -> Mask {
        self.0
    }

    pub const fn mut_mask(&mut self) -> &mut Mask {
        &mut self.0
    }

    pub const fn captured(&self, cap: Option<(Square, Piece)>) -> Self {
        if let Some((sq, Piece::Queen)) = cap {
            Self(self.as_mask().unset(sq))
        } else {
            *self
        }
    }

    pub fn render(&self, board: &mut BoardMap<Option<ColorPiece>>, color: Color) {
        for sq in self.as_mask().iter() {
            board.set(sq, Some(ColorPiece::new(color, Piece::Queen)));
        }
    }

    pub const NORTH: BoardMap<Mask> = Self::build_move_db(Dir::North);
    pub const EAST: BoardMap<Mask> = Self::build_move_db(Dir::East);
    pub const SOUTH: BoardMap<Mask> = Self::build_move_db(Dir::South);
    pub const WEST: BoardMap<Mask> = Self::build_move_db(Dir::West);

    pub const NORTHEAST: BoardMap<Mask> = Self::build_move_db(Dir::NorthEast);
    pub const SOUTHEAST: BoardMap<Mask> = Self::build_move_db(Dir::SouthEast);
    pub const SOUTHWEST: BoardMap<Mask> = Self::build_move_db(Dir::SouthWest);
    pub const NORTHWEST: BoardMap<Mask> = Self::build_move_db(Dir::NorthWest);

    const fn build_move_db(dir: Dir) -> BoardMap<Mask> {
        let mut sqiter = Mask::full().iter();
        let mut res = BoardMap::<Mask>::new([Mask::nil(); 64]);

        while let Some(sq) = sqiter.next() {
            res.set(sq, Queens::moves_from(sq, dir));
        }

        res
    }

    pub const fn moves_from(sq: Square, dir: Dir) -> Mask {
        let mut sq = sq.go(dir);
        let mut res = Mask::nil();
        while let Some(s) = sq {
            sq = s.go(dir);
            res = res.set(s);
        }
        res
    }

    pub const fn threats(&self, pieces: Mask) -> Mask {
        let mask = self.as_mask();
        self.directional_threats(&Self::NORTHWEST, true, pieces)
            .overlay(self.directional_threats(&Self::NORTH, true, pieces))
            .overlay(self.directional_threats(&Self::NORTHEAST, true, pieces))
            .overlay(self.directional_threats(&Self::EAST, true, pieces))
            .overlay(self.directional_threats(&Self::SOUTHEAST, false, pieces))
            .overlay(self.directional_threats(&Self::SOUTH, false, pieces))
            .overlay(self.directional_threats(&Self::SOUTHWEST, false, pieces))
            .overlay(self.directional_threats(&Self::WEST, false, pieces))
    }

    pub const fn directional_threats(
        &self,
        move_masks: &BoardMap<Mask>,
        positive: bool,
        blockers: Mask,
    ) -> Mask {
        let mut res = Mask::nil();
        let mut iter = self.as_mask().iter();
        while let Some(sq) = iter.next() {
            let move_mask = move_masks.at(sq);
            res = res.overlay(slide_move_stop(positive, move_mask, Mask::nil(), blockers));
        }
        res
    }

    pub fn enumerate_legal_moves(
        &self,
        color: Color,
        active_mask: Mask,
        passive: &HalfBitBoard,
        passive_mask: Mask,
        kings: Kings,
        res: &mut Vec<Move>,
    ) {
        let color_and_piece = ColorPiece::new(color, Piece::Queen);

        if !self.as_mask().any() {
            return;
        }

        for from in self.as_mask() {
            let possible =
                slide_move_stop(true, Queens::NORTHWEST.at(from), active_mask, passive_mask)
                    | slide_move_stop(true, Queens::NORTH.at(from), active_mask, passive_mask)
                    | slide_move_stop(true, Queens::NORTHEAST.at(from), active_mask, passive_mask)
                    | slide_move_stop(true, Queens::EAST.at(from), active_mask, passive_mask)
                    | slide_move_stop(false, Queens::SOUTHEAST.at(from), active_mask, passive_mask)
                    | slide_move_stop(false, Queens::SOUTH.at(from), active_mask, passive_mask)
                    | slide_move_stop(false, Queens::SOUTHWEST.at(from), active_mask, passive_mask)
                    | slide_move_stop(false, Queens::WEST.at(from), active_mask, passive_mask);

            for to in possible {
                let from_to = ProtoMove { from, to };

                let capture = passive.piece_at(to).map(|p| (to, p));

                if from_to.makes_king_checked(active_mask, kings, capture, passive, color.other()) {
                    continue;
                }

                res.push(Move {
                    color_and_piece,
                    from_to,
                    capture,
                    castling: None,
                    promotion: None,
                });
            }
        }
    }
}
