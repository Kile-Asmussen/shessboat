use crate::bitboard::{
    boardmap::BoardMap,
    colorfault::Colorfault,
    enums::{Color, Piece},
    masks::Mask,
    pieces::Micropawns,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Bishops(Mask);

impl Bishops {
    pub fn materiel(&self) -> Micropawns {
        self.0.occupied() as usize * 3_333_333
    }

    pub fn as_mask(&self) -> Mask {
        self.0
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
}

impl Colorfault for Bishops {
    fn colorfault(c: Color) -> Self {
        Self(c.as_mask() & Piece::Bishop.as_mask())
    }
}
