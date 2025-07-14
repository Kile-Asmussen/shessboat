use crate::bitboard::{
    enums::{Color, Piece},
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    piece: Piece,
    from: Square,
    to: Square,
    capture: Option<(Square, Piece)>,
    en_passant: Option<Square>,
    promotion: Option<Piece>,
}
