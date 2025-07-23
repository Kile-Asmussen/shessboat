use std::arch::x86_64::CpuidResult;

use crate::shessboard::{
    boardmap::BoardMap,
    castling::{CastlingDetails, CastlingInfo, CastlingRights, CastlingSide},
    enums::{Color, ColorPiece, Dir, File, Piece, Rank},
    half::HalfBitBoard,
    masks::Mask,
    moves::{Move, ProtoMove},
    pieces::{Micropawns, queens::Queens, slide_move_stop},
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
        self.as_mask().occupied() as i64 * 1_000_000_000
    }

    pub const fn as_mask(&self) -> Mask {
        self.0
    }

    pub const fn mut_mask(&mut self) -> &mut Mask {
        &mut self.0
    }

    pub fn render(&self, board: &mut BoardMap<Option<ColorPiece>>, color: Color) {
        for sq in self.as_mask().iter() {
            board.set(sq, Some(ColorPiece::new(color, Piece::King)));
        }
    }

    const MOVES: BoardMap<Mask> = Self::build_move_db();

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

    pub const fn threats(&self) -> Mask {
        Self::MOVES.overlays(self.as_mask())
    }

    pub const CASTLING_OOO: [Mask; 8] = [
        Mask::nil(),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_10000000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_11000000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_01100000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_00110000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_00011000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_00001100u8]),
        Mask::nil(),
    ];

    pub const CASTLING_OO: [Mask; 8] = [
        Mask::nil(),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_10000000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_11000000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_01100000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_00110000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_00011000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_00001100u8]),
        Mask::nil(),
    ];

    pub fn enumerate_legal_moves(
        &self,
        color: Color,
        active_mask: Mask,
        passive: &HalfBitBoard,
        castling: CastlingRights,
        castling_details: CastlingDetails,
        res: &mut Vec<Move>,
    ) {
        let color_and_piece = ColorPiece::new(color, Piece::King);
        let threats = passive.threats(color.other(), active_mask, None);

        if !self.as_mask().any() {
            return;
        }

        for from in self.as_mask() {
            let possible = Kings::MOVES.at(from) & !active_mask & !threats;

            for to in possible {
                let from_to = ProtoMove { from, to };

                let capture = passive.piece_at(to).map(|p| (to, p));

                res.push(Move {
                    color_and_piece,
                    from_to,
                    capture,
                    castling: None,
                    promotion: None,
                });
            }
        }

        // TODO: Fix castling queenside
    }
}
