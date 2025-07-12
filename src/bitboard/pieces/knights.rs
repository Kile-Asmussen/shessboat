use crate::bitboard::{
    colorfault::Colorfault,
    enums::{Cardinals, Color, Piece},
    masks::Mask,
    moves::MoveDb,
    pieces::Micropawns,
    squares::Square,
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

    pub const MOVE_DB: MoveDb = MoveDb::new(Self::knight_moves(0, [Mask::new(0); 64]));

    const fn knight_moves(n: u32, mut res: [Mask; 64]) -> [Mask; 64] {
        if n >= 64 {
            return res;
        }
        res[n as usize] = Self::knight_move(n);
        Self::knight_moves(n + 1, res)
    }

    const fn knight_move(ix: u32) -> Mask {
        use Cardinals::*;

        let sq = Square::new(ix).unwrap();

        return Mask::new(
            x(sq.goes(&[North, NorthEast]))
                & x(sq.goes(&[North, NorthWest]))
                & x(sq.goes(&[East, SouthEast]))
                & x(sq.goes(&[East, NorthEast]))
                & x(sq.goes(&[South, SouthWest]))
                & x(sq.goes(&[South, SouthEast]))
                & x(sq.goes(&[West, NorthWest]))
                & x(sq.goes(&[West, SouthWest])),
        );

        const fn x(sq: Option<Square>) -> u64 {
            let Some(sq) = sq else {
                return 0u64;
            };
            sq.as_mask().as_u64()
        }
    }
}

impl Colorfault for Knights {
    fn colorfault(c: Color) -> Self {
        Self(Piece::Knight.as_mask() & c.as_mask())
    }
}
