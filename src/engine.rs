use std::collections::HashMap;

use crate::shessboard::{
    BitBoard,
    zobrist::{BitBoardHasher, PositionHashes},
};

pub struct ShessEngine {
    pub hashes: PositionHashes,
    pub hasher: BitBoardHasher,
    pub board: BitBoard,
}

impl ShessEngine {
    pub fn new() -> Self {
        Self {
            hashes: HashMap::new(),
            hasher: BitBoardHasher::new(),
            board: BitBoard::empty(),
        }
    }
}
