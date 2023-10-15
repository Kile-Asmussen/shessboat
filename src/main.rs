use crate::{
    bitboard::BitBoard,
    board::{Board, BoardExtensions},
    byteboard::ByteBoard,
};

pub mod bitboard;
pub mod board;
pub mod byteboard;
pub mod moves;
pub mod pieces;
pub mod squares;
pub mod zobrist;

fn main() {
    println!("{}", BitBoard::standard().fen());
    println!("{}", BitBoard::default().fen());
    println!("{}", ByteBoard::standard().fen());
    println!("{}", ByteBoard::default().fen());
}
