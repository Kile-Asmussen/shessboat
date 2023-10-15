use crate::bitboard::BitBoard;

pub mod bitboard;
pub mod board;
pub mod byteboard;
pub mod moves;
pub mod pieces;
pub mod squares;
pub mod zobrist;

fn main() {
    println!("{}", BitBoard::standard().fen());
}
