use crate::bitboard::{
    colorfault::Colorfault,
    enums::{Color, Dir, Piece},
    masks::Mask,
    movedbs::MoveDb,
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

    const MOVE_DB: MoveDb<Mask> = Self::build_move_db();

    const fn build_move_db() -> MoveDb<Mask> {
        let mut n = 0;
        let mut res = [Mask::new(0); 64];

        while n < 64 {
            res[n] = Self::moves_from(Square::new(n as i8).unwrap());

            n += 1;
        }

        MoveDb::new(res)
    }

    pub const fn moves_from(sq: Square) -> Mask {
        use Dir::*;
        return Mask::new(
            x(sq.go(North))
                | x(sq.go(East))
                | x(sq.go(South))
                | x(sq.go(West))
                | x(sq.goes(&[North, East]))
                | x(sq.goes(&[South, East]))
                | x(sq.goes(&[South, West]))
                | x(sq.goes(&[North, West])),
        );

        const fn x(sq: Option<Square>) -> u64 {
            let Some(sq) = sq else {
                return 0u64;
            };
            sq.as_mask().as_u64()
        }
    }
}
