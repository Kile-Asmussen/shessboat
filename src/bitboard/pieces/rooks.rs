use crate::bitboard::{
    colorfault::Colorfault,
    enums::{Color, Piece},
    masks::Mask,
    movedbs::MoveDb,
    pieces::Micropawns,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Rooks(Mask);

impl Rooks {
    pub fn materiel(&self) -> Micropawns {
        Micropawns(self.0.occupied() as usize * 5_000_000)
    }

    pub fn as_mask(&self) -> Mask {
        self.0
    }

    pub fn render(&self, board: &mut [char; 64], color: Color) {
        let piece = match color {
            Color::White => 'R',
            Color::Black => 'r',
        };

        for sq in self.0.iter() {
            board[sq.index() as usize] = piece
        }
    }
}

impl Colorfault for Rooks {
    fn colorfault(c: Color) -> Self {
        Self(Piece::Rook.as_mask() & c.as_mask())
    }
}
