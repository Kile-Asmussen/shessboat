use crate::bitboard::{colorfault::Colorfault, enums::Piece, masks::Mask};

use super::enums::Color;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Kings(Mask);

impl Colorfault for Kings {
    fn colorfault(c: Color) -> Self {
        Self(c.as_mask() & Piece::King.as_mask())
    }
}
