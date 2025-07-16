use std::u64;

use crate::bitboard::{
    boardmap::BoardMap,
    enums::{Color, ColorPiece, Dir, File, Piece, Rank},
    half::HalfBitBoard,
    masks::Mask,
    moves::{Move, ProtoMove},
    pieces::{Micropawns, kings::Kings},
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
        self.as_mask().occupied() as i64 * 3_250_000
    }

    pub const fn as_mask(&self) -> Mask {
        self.0
    }

    pub const fn mut_mask(&mut self) -> &mut Mask {
        &mut self.0
    }

    pub const fn captured(&self, cap: Option<(Square, Piece)>) -> Self {
        if let Some((sq, Piece::Knight)) = cap {
            Self(self.as_mask().unset(sq))
        } else {
            *self
        }
    }

    pub fn render(&self, board: &mut BoardMap<Option<ColorPiece>>, color: Color) {
        for sq in self.as_mask().iter() {
            board.set(sq, Some(ColorPiece::new(color, Piece::Knight)));
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

    const fn moves_from(sq: Square) -> Mask {
        use Dir::*;
        return Mask::new(
            x(sq.goes([North, NorthEast]))
                | x(sq.goes([North, NorthWest]))
                | x(sq.goes([East, SouthEast]))
                | x(sq.goes([East, NorthEast]))
                | x(sq.goes([South, SouthWest]))
                | x(sq.goes([South, SouthEast]))
                | x(sq.goes([West, NorthWest]))
                | x(sq.goes([West, SouthWest])),
        );

        const fn x(sq: Option<Square>) -> u64 {
            let Some(sq) = sq else {
                return 0u64;
            };
            sq.as_mask().as_u64()
        }
    }

    pub fn threats(&self, same: Mask) -> Mask {
        Self::MOVES.overlay(self.as_mask()) & !same
    }

    pub fn enumerate_legal_moves(
        &self,
        color: Color,
        active_mask: Mask,
        passive: &HalfBitBoard,
        kings: Kings,
        res: &mut Vec<Move>,
    ) {
        let color_and_piece = ColorPiece::new(color, Piece::Knight);

        if !self.as_mask().any() {
            return;
        }

        for from in self.as_mask() {
            let possible = Knights::MOVES.at(from) & !active_mask;

            for to in possible {
                let from_to = ProtoMove { from, to };

                let capture = passive.piece_at(to).map(|p| (to, p));

                if from_to.makes_king_checked(active_mask, kings, capture, passive, color.other()) {
                    continue;
                }

                res.push(Move {
                    color_and_piece,
                    from_to,
                    capture,
                    castling: None,
                    promotion: None,
                });
            }
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
