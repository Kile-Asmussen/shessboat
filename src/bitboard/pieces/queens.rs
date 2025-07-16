use crate::bitboard::{
    boardmap::BoardMap,
    enums::{Color, ColorPiece, Dir, Piece},
    masks::Mask,
    pieces::{Micropawns, slide_move_stop},
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

    pub fn threats(&self, same: Mask, opposite: Mask) -> Mask {
        Self::directional_threats(self.as_mask(), positive, same, opposite)
    }

    pub fn directional_threats(
        pieces: Mask,
        move_masks: &BoardMap<Mask>,
        positive: bool,
        same: Mask,
        opposite: Mask,
    ) -> Mask {
        let mut res = Mask::nil();
        for sq in self.as_mask() {
            let move_mask = move_masks.at(sq);
            res |= slide_move_stop(positive, move_mask, same, opposite);
        }
        res
    }
}
