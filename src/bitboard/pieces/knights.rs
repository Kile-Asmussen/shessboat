use crate::bitboard::{
    colorfault::Colorfault,
    enums::{Color, Piece},
    masks::Mask,
    pieces::Micropawns,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Knights(Mask);

impl Knights {
    pub fn materiel(&self) -> Micropawns {
        Micropawns(self.0.occupied() as usize * 3_250_000)
    }

    pub fn as_mask(&self) -> Mask {
        self.0
    }

    pub fn render(&self, board: &mut [char; 64], color: Color) {
        let piece = match color {
            Color::White => 'N',
            Color::Black => 'n',
        };

        for sq in self.0.iter() {
            board[sq.index() as usize] = piece
        }
    }
}

impl Colorfault for Knights {
    fn colorfault(c: Color) -> Self {
        Self(Piece::Knight.as_mask() & c.as_mask())
    }
}
