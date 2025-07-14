use crate::bitboard::{
    CastlingRights,
    enums::{Color, Piece},
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    piece: Piece,
    from: Square,
    to: Square,
    castling: Option<Castling>,
    capture: Option<(Square, Piece)>,
    en_passant: Option<Square>,
    promotion: Option<Piece>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Castling {
    OOO = 1,
    OO,
}

#[test]
fn size_fuckery() {
    println!("{}", std::mem::size_of::<Move>());
}
