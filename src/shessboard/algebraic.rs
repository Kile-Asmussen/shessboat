use std::fmt::Display;

use crate::shessboard::{
    enums::{File, Piece, Rank},
    squares::Square,
};

#[derive(Clone, Copy, Debug)]
struct Notation {
    piece: Piece,
    origin_rank: Option<Rank>,
    origin_file: Option<File>,
    destination: Square,
    capture: bool,
}

impl Notation {
    pub fn read(s: &str) -> Self {
        todo!()
    }
}

impl Display for Notation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
