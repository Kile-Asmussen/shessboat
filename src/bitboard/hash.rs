use std::{
    backtrace,
    collections::{HashMap, btree_map::Values},
    io::{BufReader, Read, Write},
    path::Path,
};

use rand::{Fill, SeedableRng};

use crate::bitboard::{
    BitBoard, CastlingRight, CastlingRights, boardmap::BoardMap, enums::Color, half::HalfBitBoard,
    masks::Mask, pieces::Micropawns, squares::Square,
};

pub type PositionHasher = HashMap<HashResult, Micropawns>;

fn save_position_hashes(ph: &PositionHasher, f: &Path) -> std::io::Result<()> {
    let mut file = std::io::BufWriter::new(std::fs::File::create(f)?);
    for (k, v) in ph {
        file.write(&k.to_le_bytes());
        file.write(&v.to_le_bytes());
    }
    Ok(())
}

fn recover_position_hashes(f: &Path) -> std::io::Result<PositionHasher> {
    let file = std::fs::File::open(f)?;
    let filesize = file.metadata()?.len();
    let positions = filesize
        / (std::mem::size_of::<HashResult>() as u64 + std::mem::size_of::<Micropawns>() as u64);
    let mut result = PositionHasher::with_capacity(positions as usize);
    let mut file = BufReader::new(file);

    let mut position_buf = [0u8; 16];
    let mut value_buf = [0u8; 8];

    loop {
        if file.read(&mut position_buf)? != 16 {
            break;
        }
        if file.read(&mut value_buf)? == 8 {
            break;
        }
        let position = HashResult::from_le_bytes(position_buf);
        let value = Micropawns::from_le_bytes(value_buf);
        result.insert(position, value);
    }

    Ok(HashMap::new())
}

pub type MaskHasher = BoardMap<HashResult>;
pub type HashResult = u128;

impl MaskHasher {
    pub fn hash(&self, m: Mask) -> HashResult {
        let mut res = 0;
        for sq in m.iter() {
            res ^= self.at(sq);
        }
        res
    }
}

#[derive(Default)]
pub struct BitBoardHasher {
    black_to_move: HashResult,
    en_passant: HashResult,
    white_castling: (HashResult, HashResult),
    black_castling: (HashResult, HashResult),

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

    pub fn hash(&self, board: &BitBoard) -> HashResult {
        self.hash_to_move(board.metadata.to_move)
            ^ self.hash_half(&board.white)
            ^ !self.hash_half(&board.black)
            ^ self.hash_en_passant(board.metadata.en_passant.is_some())
            ^ self.hash_castle(board.metadata.white_castling, Color::White)
            ^ self.hash_castle(board.metadata.black_castling, Color::Black)
    }

    pub fn hash_to_move(&self, turn: Color) -> HashResult {
        if turn == Color::Black {
            self.black_to_move
        } else {
            0
        }
    }

    pub fn hash_en_passant(&self, en_passant: bool) -> HashResult {
        if en_passant { self.en_passant } else { 0 }
    }

    pub fn hash_half(&self, board: &HalfBitBoard) -> HashResult {
        self.kings.hash(board.kings().as_mask())
            ^ self.queens.hash(board.queens().as_mask())
            ^ self.rooks.hash(board.rooks().as_mask())
            ^ self.bishops.hash(board.bishops().as_mask())
            ^ self.knights.hash(board.knights().as_mask())
            ^ self.pawns.hash(board.pawns().as_mask())
    }

    pub fn hash_castle(&self, castling: CastlingRights, side: Color) -> HashResult {
        let vals = match side {
            Color::White => self.white_castling,
            Color::Black => self.black_castling,
        };

        (if castling.ooo() == CastlingRight::Retained {
            vals.0
        } else {
            0
        }) ^ (if castling.oo() == CastlingRight::Retained {
            vals.1
        } else {
            0
        })
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
