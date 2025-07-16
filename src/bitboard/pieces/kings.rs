use crate::bitboard::{
    boardmap::BoardMap,
    enums::{Color, ColorPiece, Dir, Piece, Rank},
    half::HalfBitBoard,
    masks::Mask,
    moves::{Move, ProtoMove},
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

    pub fn render(&self, board: &mut BoardMap<Option<ColorPiece>>, color: Color) {
        for sq in self.0.iter() {
            board.set(sq, Some(ColorPiece::new(color, Piece::King)));
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
        return Mask::new(
            x(sq.go(North))
                | x(sq.go(East))
                | x(sq.go(South))
                | x(sq.go(West))
                | x(sq.go(NorthEast))
                | x(sq.go(SouthEast))
                | x(sq.go(SouthWest))
                | x(sq.go(NorthWest)),
        );

        const fn x(sq: Option<Square>) -> u64 {
            let Some(sq) = sq else {
                return 0u64;
            };
            sq.as_mask().as_u64()
        }
    }

    pub fn threats(&self, same: Mask) -> Mask {
        Self::MOVES.overlap(self.as_mask()) & !same
    }

    pub fn enumerate_legal_moves(
        &self,
        color: Color,
        active_mask: Mask,
        passive: &HalfBitBoard,
        res: &mut Vec<Move>,
    ) {
        let color_and_piece = ColorPiece::new(color, Piece::King);

        if !self.as_mask().any() {
            return;
        }

        for from in self.as_mask() {
            let possible = Kings::MOVES.at(from) & !active_mask;

            for to in possible {
                let from_to = ProtoMove { from, to };

                let capture = passive.piece(to).map(|p| (to, p));

                if from_to.makes_king_checked(active_mask, *self, capture, passive, color.other()) {
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
