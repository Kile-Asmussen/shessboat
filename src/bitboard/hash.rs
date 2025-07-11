use std::collections::HashMap;

use rand::{Fill, SeedableRng};

use crate::bitboard::{BitBoard, HalfBitBoard, enums::Color, masks::Mask, pieces::Micropawns};

pub struct MaskHasher([u128; 64]);

impl Default for MaskHasher {
    fn default() -> Self {
        Self([0; 64])
    }
}

impl MaskHasher {
    pub fn hash(&self, m: Mask) -> u128 {
        let mut res = 0;
        for s in m.iter() {
            res ^= self.0[s.index() as usize]
        }
        res
    }
}

impl Fill for MaskHasher {
    fn fill<R: rand::Rng + ?Sized>(&mut self, rng: &mut R) {
        self.0.fill(rng);
    }
}

#[derive(Default)]
pub struct BitBoardHasher {
    black_to_move: u128,
    en_passant: u128,

    kings: MaskHasher,
    queens: MaskHasher,
    rooks: MaskHasher,
    bishops: MaskHasher,
    knights: MaskHasher,
    pawns: MaskHasher,
}

impl BitBoardHasher {
    pub const PI: &[u8; 32] = b"3.141592653589793238462643383279";

    pub fn new(&mut self) -> Self {
        let mut rng = rand::rngs::StdRng::from_seed(*Self::PI);
        let mut res = Self::default();
        res.fill(&mut rng);
        res
    }

    pub fn hash(&self, board: &BitBoard) -> u128 {
        (if board.metadata.turn == Color::White {
            0
        } else {
            self.black_to_move
        }) ^ self.hash_half(&board.white)
            ^ !self.hash_half(&board.black)
            ^ (if let Some(_) = board.metadata.en_passant {
                self.en_passant
            } else {
                0
            })
    }

    pub fn hash_half(&self, board: &HalfBitBoard) -> u128 {
        self.kings.hash(board.kings.as_mask())
            ^ self.queens.hash(board.queens.as_mask())
            ^ self.rooks.hash(board.rooks.as_mask())
            ^ self.bishops.hash(board.bishops.as_mask())
            ^ self.knights.hash(board.knights.as_mask())
            ^ self.pawns.hash(board.pawns.as_mask())
    }
}

impl Fill for BitBoardHasher {
    fn fill<R: rand::Rng + ?Sized>(&mut self, rng: &mut R) {
        self.black_to_move = rng.random();
        self.en_passant = rng.random();
        self.kings.fill(rng);
        self.queens.fill(rng);
        self.rooks.fill(rng);
        self.bishops.fill(rng);
        self.knights.fill(rng);
        self.pawns.fill(rng);
    }
}
