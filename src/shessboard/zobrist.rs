use std::{
    backtrace,
    collections::{HashMap, btree_map::Values},
    io::{BufReader, Read, Write},
    path::Path,
};

use rand::{Fill, SeedableRng};

use crate::shessboard::{
    BitBoard, CastlingInfo, CastlingRights,
    boardmap::BoardMap,
    castling::CastlingSide,
    enums::{Color, ColorPiece, Piece},
    half::HalfBitBoard,
    masks::Mask,
    moves::Move,
    pieces::{Micropawns, pawns::Pawns},
    squares::Square,
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
        if file.read(&mut value_buf)? != 8 {
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
    pub fn hash_square(&self, sq: Square) -> HashResult {
        self.at(sq)
    }

    pub fn hash_mask(&self, m: Mask) -> HashResult {
        let mut res = 0;
        for sq in m.iter() {
            res ^= self.hash_square(sq);
        }
        res
    }
}

#[derive(Default)]
pub struct BitBoardHasher {
    pub black_to_move: HashResult,
    pub en_passant_possible: HashResult,
    pub white: HalfBitBoardHasher,
    pub black: HalfBitBoardHasher,
}

impl BitBoardHasher {
    pub const PI: &[u8; 32] = b"3.141592653589793238462643383279";

    pub fn new() -> Self {
        let mut rng = rand::rngs::StdRng::from_seed(*Self::PI);
        let mut res = Self::default();
        res.fill(&mut rng);
        res
    }

    pub fn hash(&self, board: &BitBoard) -> HashResult {
        self.hash_to_move(board.metadata.to_move)
            ^ self.hash_en_passant(board.metadata.en_passant.is_some())
            ^ self.white.hash(&board.white)
            ^ self.black.hash(&board.black)
    }

    pub fn hash_to_move(&self, turn: Color) -> HashResult {
        if turn == Color::Black {
            self.black_to_move
        } else {
            0
        }
    }

    pub fn hash_en_passant(&self, en_passant: bool) -> HashResult {
        if en_passant {
            self.en_passant_possible
        } else {
            0
        }
    }

    pub fn hash_add_a_move(&self, mut hash: HashResult, mv: Move) -> HashResult {
        let (same, opposite) = match mv.color_and_piece.color() {
            Color::White => (&self.white, &self.black),
            Color::Black => (&self.black, &self.white),
        };

        let mut res = same.hash_piece(mv.color_and_piece.piece(), mv.from_to.from)
            ^ same.hash_piece(mv.color_and_piece.piece(), mv.from_to.to);

        if let Some(pm) = mv.castling {
            if pm.positive() {
                res ^= same.castling.oo
            } else {
                res ^= same.castling.ooo
            }
        }

        res
    }
}

impl Fill for BitBoardHasher {
    fn fill<R: rand::Rng + ?Sized>(&mut self, rng: &mut R) {
        self.black_to_move = rng.random();
        self.en_passant_possible = rng.random();
        self.white.fill(rng);
        self.black.fill(rng);
    }
}

#[derive(Default, Debug)]
pub struct HalfBitBoardHasher {
    pub castling: CastlingInfo<HashResult>,
    pub kings: MaskHasher,
    pub queens: MaskHasher,
    pub rooks: MaskHasher,
    pub bishops: MaskHasher,
    pub knights: MaskHasher,
    pub pawns: MaskHasher,
}

impl HalfBitBoardHasher {
    pub fn hash(&self, hbb: &HalfBitBoard) -> HashResult {
        self.kings.hash_mask(hbb.kings.as_mask())
            ^ self.queens.hash_mask(hbb.queens.as_mask())
            ^ self.rooks.hash_mask(hbb.rooks.as_mask())
            ^ self.bishops.hash_mask(hbb.bishops.as_mask())
            ^ self.knights.hash_mask(hbb.knights.as_mask())
            ^ self.pawns.hash_mask(hbb.pawns.as_mask())
    }

    pub fn hasher_for_piece(&self, piece: Piece) -> &MaskHasher {
        match piece {
            Piece::Pawn => &self.pawns,
            Piece::Knight => &self.knights,
            Piece::Bishop => &self.bishops,
            Piece::Rook => &self.rooks,
            Piece::Queen => &self.queens,
            Piece::King => &self.kings,
        }
    }

    pub fn hash_piece(&self, piece: Piece, square: Square) -> HashResult {
        self.hasher_for_piece(piece).hash_square(square)
    }

    pub fn hash_castle(&self, castling: CastlingRights) -> HashResult {
        (if castling.ooo { self.castling.ooo } else { 0 })
            ^ (if castling.oo { self.castling.oo } else { 0 })
    }
}

impl Fill for HalfBitBoardHasher {
    fn fill<R: rand::Rng + ?Sized>(&mut self, rng: &mut R) {
        self.castling = CastlingInfo {
            ooo: rng.random(),
            oo: rng.random(),
        };
        self.kings.fill(rng);
        self.queens.fill(rng);
        self.rooks.fill(rng);
        self.bishops.fill(rng);
        self.knights.fill(rng);
        self.pawns.fill(rng);
    }
}
