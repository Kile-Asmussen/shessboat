use crate::bitboard::{
    colorfault::Colorfault,
    enums::{Color, Piece},
    masks::Mask,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Kings(Mask);

impl Colorfault for Kings {
    fn colorfault(c: Color) -> Self {
        Self(c.as_mask() & Piece::King.as_mask())
    }
}

impl Kings {
    pub fn as_mask(&self) -> Mask {
        self.0
    }

    pub fn render(&self, board: &mut [char; 64], color: Color) {
        let piece = match color {
            Color::White => 'K',
            Color::Black => 'k',
        };

        for sq in self.0.iter() {
            board[sq.index() as usize] = piece
        }
    }
}
