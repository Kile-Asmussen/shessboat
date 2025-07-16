use crate::bitboard::{
    boardmap::BoardMap,
    enums::{Color, Dir, Piece, Rank},
    masks::Mask,
    pieces::Micropawns,
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Kings(Mask);

impl Kings {
    pub const fn nil() -> Self {
        Self(Mask::nil())
    }

    pub const fn new(mask: Mask) -> Self {
        Self(mask)
    }

    pub const fn materiel(&self) -> Micropawns {
        self.0.occupied() as i64 * 1_000_000_000
    }

    pub const fn as_mask(&self) -> Mask {
        self.0
    }

    pub const fn mut_mask(&mut self) -> &mut Mask {
        &mut self.0
    }

    pub fn render(&self, board: &mut BoardMap<char>, color: Color) {
        let piece = match color {
            Color::White => 'K',
            Color::Black => 'k',
        };

        for sq in self.0.iter() {
            board.set(sq, piece);
        }
    }

    const MOVES: BoardMap<Mask> = Self::build_move_db();

    const CASTLING: [u8; 8] = [
        0b_01100000,
        0b_10110000,
        0b_11011000,
        0b_01101100,
        0b_00110110,
        0b_00011011,
        0b_00001101,
        0b_00000110,
    ];

    const fn build_move_db() -> BoardMap<Mask> {
        let mut n = 0;
        let mut res = [Mask::new(0); 64];

        while n < 64 {
            res[n] = Self::moves_from(Square::new(n as i8).unwrap());

            n += 1;
        }

        BoardMap::new(res)
    }

    pub const fn moves_from(sq: Square) -> Mask {
        use Dir::*;
        let x = Self::x;
        return Mask::new(
            x(sq.go(North))
                | x(sq.go(East))
                | x(sq.go(South))
                | x(sq.go(West))
                | x(sq.goes([North, East]))
                | x(sq.goes([South, East]))
                | x(sq.goes([South, West]))
                | x(sq.goes([North, West])),
        );
    }

    const fn x(sq: Option<Square>) -> u64 {
        let Some(sq) = sq else {
            return 0u64;
        };
        sq.as_mask().as_u64()
    }
}
