use std::arch::x86_64::CpuidResult;

use crate::shessboard::{
    boardmap::BoardMap,
    castling::{CastlingDetail, CastlingDetails, CastlingInfo, CastlingRights, CastlingSide},
    enums::{Color, ColorPiece, Dir, File, Piece, Rank},
    half::HalfBitBoard,
    masks::Mask,
    moves::{Move, ProtoMove},
    pieces::{Millipawns, queens::Queens, slide_move_stop},
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

    pub fn enumerate_legal_moves(
        &self,
        color: Color,
        active_mask: Mask,
        passive_mask: Mask,
        passive: &HalfBitBoard,
        castling: CastlingRights,
        castling_details: CastlingDetails,
        res: &mut Vec<Move>,
    ) {
        let color_and_piece = ColorPiece::new(color, Piece::King);

        let threats = passive.threats(color.other(), active_mask ^ self.as_mask(), None);

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

        let unking = (passive_mask | active_mask) & !self.as_mask();

        let threats = if castling.ooo || castling.oo {
            passive.threats(color.other(), active_mask, None)
        } else {
            Mask::nil()
        };

        if castling.ooo {
            Self::castling_move(
                self.as_mask(),
                color,
                castling_details.ooo,
                CastlingSide::OOO,
                threats,
                unking,
                res,
            )
        }

        if castling.oo {
            Self::castling_move(
                self.as_mask(),
                color,
                castling_details.oo,
                CastlingSide::OO,
                threats,
                unking,
                res,
            )
        }
    }

    // TODO: fix
    fn castling_move(
        mut king: Mask,
        color: Color,
        detail: CastlingDetail,
        castling: CastlingSide,
        threats: Mask,
        unking: Mask,
        res: &mut Vec<Move>,
    ) {
        let rank = color.starting_rank();
        king |= Mask::new_rank(rank, detail.king_mask);
        let rook = Mask::new_rank(rank, detail.rook_mask);
        if !(king & threats).any() && !(king & unking).any() && !(rook & unking).any() {
            res.push(Move {
                color_and_piece: ColorPiece::new(color, Piece::King),
                from_to: ProtoMove {
                    from: detail.king_move.as_move(rank).from,
                    to: detail.rook_move.as_move(rank).from,
                },
                castling: Some(castling),
                capture: None,
                promotion: None,
            })
        }
    }
}
