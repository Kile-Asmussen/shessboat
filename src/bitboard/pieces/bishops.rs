use crate::bitboard::{
    boardmap::BoardMap,
    enums::{Color, ColorPiece, Dir, Piece},
    half::HalfBitBoard,
    masks::Mask,
    moves::{Move, ProtoMove},
    pieces::{
        Micropawns,
        kings::Kings,
        queens::{self, Queens},
        slide_move_stop,
    },
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Bishops(Mask);

impl Bishops {
    pub const fn nil() -> Self {
        Self(Mask::nil())
    }

    pub const fn new(mask: Mask) -> Self {
        Self(mask)
    }

    pub const fn materiel(&self) -> Micropawns {
        self.0.occupied() as i64 * 3_333_333
    }

    pub const fn as_mask(&self) -> Mask {
        self.0
    }

    pub const fn mut_mask(&mut self) -> &mut Mask {
        &mut self.0
    }

    pub const fn captured(&self, cap: Option<(Square, Piece)>) -> Self {
        if let Some((sq, Piece::Bishop)) = cap {
            Self(self.as_mask().unset(sq))
        } else {
            *self
        }
    }

    pub fn render(&self, board: &mut BoardMap<Option<ColorPiece>>, color: Color) {
        for sq in self.0.iter() {
            board.set(sq, Some(ColorPiece::new(color, Piece::Bishop)));
        }
    }

    pub fn threats(&self, queens: Queens, same: Mask, opposite: Mask) -> Mask {
        let mask = self.as_mask() | queens.as_mask();
        Queens::directional_threats(mask, &Queens::NORTHWEST, true, same, opposite)
            | Queens::directional_threats(mask, &Queens::NORTHEAST, true, same, opposite)
            | Queens::directional_threats(mask, &Queens::SOUTHEAST, false, same, opposite)
            | Queens::directional_threats(mask, &Queens::SOUTHWEST, false, same, opposite)
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
        let color_and_piece = ColorPiece::new(color, Piece::Bishop);

        if !self.as_mask().any() {
            return;
        }

        for from in self.as_mask() {
            let possible =
                slide_move_stop(true, Queens::NORTHWEST.at(from), active_mask, passive_mask)
                    | slide_move_stop(true, Queens::NORTHEAST.at(from), active_mask, passive_mask)
                    | slide_move_stop(false, Queens::SOUTHEAST.at(from), active_mask, passive_mask)
                    | slide_move_stop(false, Queens::SOUTHWEST.at(from), active_mask, passive_mask);

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
