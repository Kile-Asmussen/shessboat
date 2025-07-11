use crate::bitboard::enums::Color;

pub trait Colorfault {
    fn colorfault(c: Color) -> Self;
}
