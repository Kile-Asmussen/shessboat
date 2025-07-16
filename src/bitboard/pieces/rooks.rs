use crate::bitboard::{
    boardmap::BoardMap,
    enums::{Color, ColorPiece, Dir, Piece},
    masks::Mask,
    pieces::Micropawns,
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Rooks(Mask);

impl Rooks {
    pub const fn nil() -> Self {
        Self(Mask::nil())
    }

    pub const fn new(mask: Mask) -> Self {
        Self(mask)
    }

    pub const fn materiel(&self) -> Micropawns {
        self.0.occupied() as i64 * 5_000_000
    }

    pub const fn as_mask(&self) -> Mask {
        self.0
    }

    pub const fn mut_mask(&mut self) -> &mut Mask {
        &mut self.0
    }

    pub fn render(&self, board: &mut BoardMap<Option<ColorPiece>>, color: Color) {
        for sq in self.0.iter() {
            board.set(sq, Some(ColorPiece::new(color, Piece::Rook)));
        }
    }

    pub const NORTH: BoardMap<Mask> = Self::build_move_db(Dir::North);
    pub const EAST: BoardMap<Mask> = Self::build_move_db(Dir::East);
    pub const SOUTH: BoardMap<Mask> = Self::build_move_db(Dir::South);
    pub const WEST: BoardMap<Mask> = Self::build_move_db(Dir::West);

    const fn build_move_db(dir: Dir) -> BoardMap<Mask> {
        let mut sqiter = Mask::full().iter();
        let mut res = BoardMap::<Mask>::new([Mask::nil(); 64]);

        while let Some(sq) = sqiter.next() {
            res.set(sq, Self::moves_from(sq, dir));
        }

        res
    }

    pub const fn moves_from(sq: Square, dir: Dir) -> Mask {
        let mut sq = sq.go(dir);
        let mut res = Mask::nil();
        while let Some(s) = sq {
            sq = s.go(dir);
            res.set(s);
        }
        res
    }
}
