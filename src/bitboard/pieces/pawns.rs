use crate::bitboard::{
    boardmap::BoardMap,
    colorfault::Colorfault,
    enums::{Color, Piece},
    masks::Mask,
    pieces::Micropawns,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Pawns(Mask);

impl Pawns {
    pub fn materiel(&self) -> Micropawns {
        self.0.occupied() as usize * 1_000_000
    }

    pub fn as_mask(&self) -> Mask {
        self.0
    }

    pub fn render(&self, board: &mut BoardMap<char>, color: Color) {
        let piece = match color {
            Color::White => 'P',
            Color::Black => 'p',
        };

        for sq in self.0.iter() {
            board.set(sq, piece);
        }
    }
}

impl Colorfault for Pawns {
    fn colorfault(c: Color) -> Self {
        Self(Piece::Pawn.as_mask() & c.as_mask())
    }
}
