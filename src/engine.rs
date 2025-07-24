use std::collections::HashMap;

use crate::shessboard::{
    BitBoard,
    zobrist::{BitBoardHasher, PositionHashes},
};

pub struct ShessEngine<'a> {
    pub hasher: &'a BitBoardHasher,
    pub hashes: PositionHashes,
    pub board: BitBoard,
}

impl<'a> ShessEngine<'a> {
    pub fn new(hasher: &'a BitBoardHasher) -> Self {
        Self {
            hashes: HashMap::new(),
            hasher,
            board: BitBoard::empty(),
        }
    }
}
