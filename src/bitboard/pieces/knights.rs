use std::u64;

use crate::bitboard::{
    boardmap::BoardMap,
    enums::{Color, Dir, File, Piece, Rank},
    masks::Mask,
    pieces::Micropawns,
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Knights(Mask);

impl Knights {
    pub const fn nil() -> Self {
        Self(Mask::nil())
    }

    pub const fn new(mask: Mask) -> Self {
        Self(mask)
    }

    pub const fn materiel(&self) -> Micropawns {
        self.0.occupied() as i64 * 3_250_000
    }

    pub const fn as_mask(&self) -> Mask {
        self.0
    }

    pub const fn mut_mask(&mut self) -> &mut Mask {
        &mut self.0
    }

    pub fn render(&self, board: &mut BoardMap<char>, color: Color) {
        let piece = match color {
            Color::White => 'N',
            Color::Black => 'n',
        };

        for sq in self.0.iter() {
            board.set(sq, piece);
        }
    }

    pub const MOVES: BoardMap<Mask> = Self::build_move_db();

    const fn build_move_db() -> BoardMap<Mask> {
        let mut sqiter = Mask::full().iter();
        let mut res = BoardMap::<Mask>::new([Mask::nil(); 64]);

        while let Some(sq) = sqiter.next() {
            res.set(sq, Self::moves_from(sq));
        }

        res
    }

    pub const fn moves_from(sq: Square) -> Mask {
        use Dir::*;
        return Mask::new(
            x(sq.goes([North, North, East]))
                | x(sq.goes([North, North, West]))
                | x(sq.goes([East, East, South]))
                | x(sq.goes([East, East, North]))
                | x(sq.goes([South, South, West]))
                | x(sq.goes([South, South, East]))
                | x(sq.goes([West, West, North]))
                | x(sq.goes([West, West, South])),
        );

        const fn x(sq: Option<Square>) -> u64 {
            let Some(sq) = sq else {
                return 0u64;
            };
            sq.as_mask().as_u64()
        }
    }
}

#[test]
fn knights_move_db() {
    use Dir::*;
    use File::*;
    use Rank::*;
    let at = Square::at;

    assert_eq!(at(A, _8).goes([South, South, East]), Some(at(B, _6)));

    for sq in Mask::new(u64::MAX).iter() {
        assert!(
            Knights::moves_from(sq).any(),
            "No knight moves from {:?}",
            sq
        );
    }

    assert!(Knights::moves_from(at(D, _4)).contains(at(B, _3)));

    let move_db = Knights::MOVES.at(Square::at(D, _4));

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

    let move_db = Knights::MOVES.at(Square::at(A, _1));
    let can_move_to = at(B, _3) | at(C, _2);
    assert_eq!(move_db, can_move_to);
}
