use crate::shessboard::{
    boardmap::BoardMap,
    enums::{Color, ColorPiece, Dir, Piece},
    half::HalfBitBoard,
    masks::Mask,
    moves::{Move, ProtoMove},
    pieces::{
        Millipawns,
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

    pub const fn materiel(&self) -> Millipawns {
        self.as_mask().occupied() as i64 * 3_333
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
        for sq in self.as_mask().iter() {
            board.set(sq, Some(ColorPiece::new(color, Piece::Bishop)));
        }
    }

    pub const fn threats(&self, blockers: Mask) -> Mask {
        let this = Queens::new(self.as_mask());
        this.directional_threats(&Queens::NORTHWEST, true, blockers)
            .overlay(this.directional_threats(&Queens::NORTHEAST, true, blockers))
            .overlay(this.directional_threats(&Queens::SOUTHEAST, false, blockers))
            .overlay(this.directional_threats(&Queens::SOUTHWEST, false, blockers))
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
        let prev_epc = self

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
