use crate::bitboard::{
    colorfault::Colorfault,
    enums::{Color, Piece},
    masks::Mask,
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Kings(Square);

impl Colorfault for Kings {
    fn colorfault(c: Color) -> Self {
        Self(Square::from_mask(c.as_mask() & Piece::King.as_mask()).unwrap())
    }
}

impl Kings {
    pub fn as_mask(&self) -> Mask {
        self.0.as_mask()
    }

    pub fn render(&self, board: &mut [char; 64], color: Color) {
        let piece = match color {
            Color::White => 'K',
            Color::Black => 'k',
        };

        for sq in self.0.as_mask().iter() {
            board[sq.index() as usize] = piece
        }
    }
}
