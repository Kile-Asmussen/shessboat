use std::u64;

use crate::bitboard::{
    colorfault::Colorfault,
    enums::{Color, Dir, File, Piece, Rank},
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

    pub const MOVE_DB: MoveDb<Mask> = MoveDb::new(Self::knight_moves());

    const fn knight_moves() -> [Mask; 64] {
        let mut n = 0i32;
        let mut res = [Mask::new(0); 64];

        while n < 64 {
            res[n as usize] = Self::moves_from(Square::new(n).unwrap());

            n += 1;
        }

        res
    }

    pub const fn moves_from(sq: Square) -> Mask {
        use Dir::*;
        return Mask::new(
            Self::x(sq.goes(&[North, North, East]))
                | Self::x(sq.goes(&[North, North, West]))
                | Self::x(sq.goes(&[East, East, South]))
                | Self::x(sq.goes(&[East, East, North]))
                | Self::x(sq.goes(&[South, South, West]))
                | Self::x(sq.goes(&[South, South, East]))
                | Self::x(sq.goes(&[West, West, North]))
                | Self::x(sq.goes(&[West, West, South])),
        );
    }

    const fn x(sq: Option<Square>) -> u64 {
        let Some(sq) = sq else {
            return 0u64;
        };
        sq.as_mask().as_u64()
    }
}

impl Colorfault for Knights {
    fn colorfault(c: Color) -> Self {
        Self(Piece::Knight.as_mask() & c.as_mask())
    }
}

#[test]
fn move_db() {
    use Dir::*;
    use File::*;
    use Rank::*;
    let at = Square::at;
    let x = Knights::x;

    assert_eq!(at(A, _8).goes(&[South, South, East]), Some(at(B, _6)));

    for sq in Mask::new(u64::MAX).iter() {
        assert!(
            Knights::moves_from(sq).any(),
            "No knight moves from {:?}",
            sq
        );
    }

    assert!(Knights::moves_from(at(D, _4)).contains(at(B, _3))); 

    let move_db = Knights::MOVE_DB.at(Square::at(D, _4));

    let at = |f, r| Square::at(f, r).as_mask();

    let can_move_to = at(B, _3)
        | at(B, _5)
        | at(C, _2)
        | at(C, _6)
        | at(E, _2)
        | at(E, _6)
        | at(F, _3)
        | at(F, _5);

    assert_eq!(move_db, can_move_to);

    let move_db = Knights::MOVE_DB.at(Square::at(A, _1));
    let can_move_to = at(B, _3) | at(C, _2);
    assert_eq!(move_db, can_move_to);
}
