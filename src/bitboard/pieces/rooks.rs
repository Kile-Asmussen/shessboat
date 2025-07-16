use crate::bitboard::{
    boardmap::BoardMap,
    enums::{Color, ColorPiece, Dir, Piece},
    half::HalfBitBoard,
    masks::Mask,
    moves::{Move, ProtoMove},
    pieces::{Micropawns, kings::Kings, queens::Queens, slide_move_stop},
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Rooks(Mask);

impl Rooks {
    pub const fn nil() -> Self {
        Self(Mask::nil())
    }

    pub const fn new(mask: Mask) -> Self {
        Self(mask)
    }

    pub const fn materiel(&self) -> Micropawns {
        self.0.occupied() as i64 * 5_000_000
    }

    pub const fn as_mask(&self) -> Mask {
        self.0
    }

    pub const fn captured(&self, cap: Option<(Square, Piece)>) -> Self {
        if let Some((sq, Piece::Rook)) = cap {
            Self(self.as_mask().unset(sq))
        } else {
            *self
        }
    }

    pub fn render(&self, board: &mut BoardMap<Option<ColorPiece>>, color: Color) {
        for sq in self.0.iter() {
            board.set(sq, Some(ColorPiece::new(color, Piece::Rook)));
        }
    }

    pub fn threats(&self, queens: Queens, same: Mask, opposite: Mask) -> Mask {
        let mask = self.as_mask() | queens.as_mask();
        Queens::directional_threats(mask, &Queens::NORTH, true, same, opposite)
            | Queens::directional_threats(mask, &Queens::EAST, true, same, opposite)
            | Queens::directional_threats(mask, &Queens::SOUTH, false, same, opposite)
            | Queens::directional_threats(mask, &Queens::WEST, false, same, opposite)
    }

    pub fn enumerate_legal_moves(
        &self,
        color: Color,
        active_mask: Mask,
        passive: &HalfBitBoard,
        passive_mask: Mask,
        kings: Kings,
        res: &mut Vec<Move>,
    ) {
        let color_and_piece = ColorPiece::new(color, Piece::Rook);

        if !self.as_mask().any() {
            return;
        }

        for from in self.as_mask() {
            let possible = slide_move_stop(true, Queens::NORTH.at(from), active_mask, passive_mask)
                | slide_move_stop(true, Queens::EAST.at(from), active_mask, passive_mask)
                | slide_move_stop(false, Queens::SOUTH.at(from), active_mask, passive_mask)
                | slide_move_stop(false, Queens::WEST.at(from), active_mask, passive_mask);

            for to in possible {
                let from_to = ProtoMove { from, to };

                let capture = passive.piece(to).map(|p| (to, p));

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
