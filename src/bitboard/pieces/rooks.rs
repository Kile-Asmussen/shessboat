use crate::bitboard::{
    boardmap::BoardMap,
    colorfault::Colorfault,
    enums::{Color, Piece},
    masks::Mask,
    pieces::Micropawns,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Rooks(Mask);

impl Rooks {
    pub const fn materiel(&self) -> Micropawns {
        self.0.occupied() as usize * 5_000_000
    }

    pub const fn as_mask(&self) -> Mask {
        self.0
    }

    pub fn render(&self, board: &mut BoardMap<char>, color: Color) {
        let piece = match color {
            Color::White => 'R',
            Color::Black => 'r',
        };

        for sq in self.0.iter() {
            board.set(sq, piece);
        }
    }

    pub fn moves_from(sq: Square, dir: Dir) -> Mask {
        let mut sq = Some(sq);
        let mut res = Mask::nil();
        while sq.is_some() {
            sq = sq.go(dir);
        }
    }
}

impl Colorfault for Rooks {
    fn colorfault(c: Color) -> Self {
        Self(Piece::Rook.as_mask() & c.as_mask())
    }
}
