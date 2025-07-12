use std::collections::{HashMap, btree_map::Values};

use rand::{Fill, SeedableRng};

use crate::bitboard::{
    BitBoard, CastlingRights, HalfBitBoard, enums::Color, masks::Mask, pieces::Micropawns,
    squares::Square,
};

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
    white_castling: (u128, u128),
    black_castling: (u128, u128),

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
        self.hash_to_move(board.metadata.to_move)
            ^ self.hash_half(&board.white)
            ^ !self.hash_half(&board.black)
            ^ self.hash_en_passant(board.metadata.en_passant)
            ^ self.hash_castle(board.metadata.white_castling, Color::White)
            ^ self.hash_castle(board.metadata.black_castling, Color::Black)
    }

    pub fn hash_to_move(&self, turn: Color) -> u128 {
        if turn == Color::Black {
            self.black_to_move
        } else {
            0
        }
    }

    pub fn hash_en_passant(&self, passant: Option<Square>) -> u128 {
        if let Some(_) = passant {
            self.en_passant
        } else {
            0
        }
    }

    pub fn hash_half(&self, board: &HalfBitBoard) -> u128 {
        self.kings.hash(board.kings.as_mask())
            ^ self.queens.hash(board.queens.as_mask())
            ^ self.rooks.hash(board.rooks.as_mask())
            ^ self.bishops.hash(board.bishops.as_mask())
            ^ self.knights.hash(board.knights.as_mask())
            ^ self.pawns.hash(board.pawns.as_mask())
    }

    pub fn hash_castle(&self, castling: CastlingRights, side: Color) -> u128 {
        let values = match side {
            Color::White => self.white_castling,
            Color::Black => self.black_castling,
        };

        (if castling.long { values.0 } else { 0 }) ^ (if castling.short { values.1 } else { 0 })
    }
}

impl Fill for BitBoardHasher {
    fn fill<R: rand::Rng + ?Sized>(&mut self, rng: &mut R) {
        self.black_to_move = rng.random();
        self.en_passant = rng.random();
        self.white_castling = (rng.random(), rng.random());
        self.black_castling = (rng.random(), rng.random());
        self.kings.fill(rng);
        self.queens.fill(rng);
        self.rooks.fill(rng);
        self.bishops.fill(rng);
        self.knights.fill(rng);
        self.pawns.fill(rng);
    }
}
