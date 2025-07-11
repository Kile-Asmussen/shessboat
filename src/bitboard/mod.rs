mod colorfault;
mod enums;
mod kings;
mod masks;
mod pieces;
mod squares;

use enums::Color;

use crate::bitboard::{
    pieces::{
        bishops::Bishops, kings::Kings, knights::Knights, pawns::Pawns, queens::Queens,
        rooks::Rooks,
    },
    squares::Square,
};

struct BitBoard {
    metadata: Metadata,
    white: HalfBitBoard,
    black: HalfBitBoard,
}

struct HalfBitBoard {
    kings: Kings,
    queens: Queens,
    rooks: Rooks,
    bishops: Bishops,
    knights: Knights,
    pawns: Pawns,
}

struct Metadata {
    hash: u64,
    turn: Color,
    en_passant: Option<Square>,
}
