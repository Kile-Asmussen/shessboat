use crate::bitboard::{
    boardmap::BoardMap,
    enums::{Color, Dir, Piece},
    masks::Mask,
    pieces::Micropawns,
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Bishops(Mask);

impl Bishops {
    pub const fn nil() -> Self {
        Self(Mask::nil())
    }

    pub const fn new(mask: Mask) -> Self {
        Self(mask)
    }

    pub const fn materiel(&self) -> Micropawns {
        self.0.occupied() as i64 * 3_333_333
    }

    pub const fn as_mask(&self) -> Mask {
        self.0
    }

    pub const fn mut_mask(&mut self) -> &mut Mask {
        &mut self.0
    }

    pub fn render(&self, board: &mut BoardMap<char>, color: Color) {
        let piece = match color {
            Color::White => 'B',
            Color::Black => 'b',
        };

        for sq in self.0.iter() {
            board.set(sq, piece);
        }
    }

    pub const NORTHEAST: BoardMap<Mask> = Self::build_move_db([Dir::North, Dir::East]);
    pub const SOUTHEAST: BoardMap<Mask> = Self::build_move_db([Dir::South, Dir::East]);
    pub const SOUTHWEST: BoardMap<Mask> = Self::build_move_db([Dir::South, Dir::West]);
    pub const NORTHWEST: BoardMap<Mask> = Self::build_move_db([Dir::North, Dir::West]);

    const fn build_move_db(dir: [Dir; 2]) -> BoardMap<Mask> {
        let mut sqiter = Mask::full().iter();
        let mut res = BoardMap::<Mask>::new([Mask::nil(); 64]);

        while let Some(sq) = sqiter.next() {
            res.set(sq, Self::moves_from(sq, dir));
        }

        res
    }

    pub const fn moves_from(sq: Square, dir: [Dir; 2]) -> Mask {
        let mut sq = sq.goes(dir);
        let mut res = Mask::nil();
        while let Some(s) = sq {
            sq = s.goes(dir);
            res.set(s);
        }
        res
    }
}
