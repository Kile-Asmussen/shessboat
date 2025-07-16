use std::fmt::Binary;

use crate::bitboard::{
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
        self.0.occupied() as i64 * 9_000_000
    }

    pub const fn as_mask(&self) -> Mask {
        self.0
    }

    pub fn render(&self, board: &mut BoardMap<Option<ColorPiece>>, color: Color) {
        for sq in self.0.iter() {
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

    pub fn threats(&self, rooks: Rooks, bishops: Bishops, same: Mask, opposite: Mask) -> Mask {
        let bmask = self.as_mask() | bishops.as_mask();
        let rmask = self.as_mask() | rooks.as_mask();
        Self::directional_threats(bmask, &Self::NORTHWEST, true, same, opposite)
            | Self::directional_threats(rmask, &Self::NORTH, true, same, opposite)
            | Self::directional_threats(bmask, &Self::NORTHEAST, true, same, opposite)
            | Self::directional_threats(rmask, &Self::EAST, true, same, opposite)
            | Self::directional_threats(bmask, &Self::SOUTHEAST, false, same, opposite)
            | Self::directional_threats(rmask, &Self::SOUTH, false, same, opposite)
            | Self::directional_threats(bmask, &Self::SOUTHWEST, false, same, opposite)
            | Self::directional_threats(rmask, &Self::WEST, false, same, opposite)
    }

    pub fn directional_threats(
        pieces: Mask,
        move_masks: &BoardMap<Mask>,
        positive: bool,
        same: Mask,
        opposite: Mask,
    ) -> Mask {
        let mut res = Mask::nil();
        for sq in pieces {
            let move_mask = move_masks.at(sq);
            res |= slide_move_stop(positive, move_mask, same, opposite);
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

                let capture = passive.piece(to).map(|p| (to, p));

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
