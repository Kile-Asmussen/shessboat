use std::{
    collections::{BTreeSet, HashMap, HashSet, btree_map::Values},
    error::{self, Error},
    fmt::{Debug, Display, UpperHex},
    hash::Hash,
    io::{self, BufReader, Read, Write},
    mem::size_of,
    ops::{BitXor, BitXorAssign},
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
    metadata::Metadata,
    moves::Move,
    pieces::{
        Millipawns,
        pawns::{EnPassant, Pawns},
    },
    squares::Square,
};

pub type PositionHashes = HashMap<HashResult, Millipawns>;

#[derive(Debug, Default)]
struct ReadHashesError;
impl Error for ReadHashesError {}
impl Display for ReadHashesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Error reading hash file")
    }
}

pub type MaskHasher = BoardMap<HashResult>;
pub type HashResult = u64;

impl MaskHasher {
    pub fn hash_square(&self, sq: Square) -> HashResult {
        self.at_clone(sq)
    }

    pub fn hash_mask(&self, m: Mask) -> HashResult {
        let mut res = 0; // HashResult::default();
        for sq in m.iter() {
            res ^= self.at_clone(sq);
        }
        res
    }
}

#[derive(Default)]
pub struct BitBoardHasher {
    pub en_passant_file: [HashResult; 8],
    pub white: HalfBitBoardHasher,
    pub black: HalfBitBoardHasher,
}

impl BitBoardHasher {
    pub const PI: &[u8; 32] = b"3.141592653589793238462643383279";

    pub fn new() -> Self {
        let mut rng = rand::rngs::StdRng::from_seed(*Self::PI);
        let mut res = Self::default();
        // res.black.color = Color::Black;
        res.fill(&mut rng);
        res
    }

    pub fn hash_full(&self, board: &BitBoard) -> HashResult {
        Self::hash_to_move(board.metadata.to_move)
            ^ self.hash_en_passant(board.metadata.en_passant)
            ^ self.white.hash_castle(board.metadata.white_castling)
            ^ self.black.hash_castle(board.metadata.black_castling)
            ^ self.white.hash(&board.white)
            ^ self.black.hash(&board.black)
    }

    pub fn hash_en_passant(&self, en_passant: Option<EnPassant>) -> HashResult {
        if let Some(EnPassant { to, capture }) = en_passant {
            self.en_passant_file[to.file().as_file() as usize] //.clone()
        } else {
            0
        }
    }

    pub const BLACK_TO_MOVE: HashResult = 1 << 63;
    pub const HASH_BITS: HashResult = !Self::BLACK_TO_MOVE;

    pub fn hash_to_move(color: Color) -> HashResult {
        if color == Color::Black {
            Self::BLACK_TO_MOVE
        } else {
            0
        }
    }

    pub fn delta(&self, metadata: &Metadata, mut hash: HashResult, mv: Move) -> HashResult {
        let (color, piece) = mv.color_and_piece.split();

        let (same, opposite) = match color {
            Color::White => (&self.white, &self.black),
            Color::Black => (&self.black, &self.white),
        };

        hash &= Self::HASH_BITS;
        hash ^= Self::hash_to_move(color.other());

        if let Some(p) = mv.promotion {
            hash ^=
                same.hash_piece(Piece::Pawn, mv.from_to.from) ^ same.hash_piece(p, mv.from_to.to)
        } else {
            hash ^= same.hash_piece(piece, mv.from_to.from) ^ same.hash_piece(piece, mv.from_to.to);
        }

        if let Some((sq, p)) = mv.capture {
            hash ^= opposite.hash_piece(p, sq);
        }

        if let Some(cs) = mv.castling {
            let cast = metadata
                .castling_details
                .select(cs)
                .rook_move
                .as_move(color.starting_rank());
            hash ^= same.hash_piece(Piece::Rook, cast.from) ^ same.hash_piece(Piece::Rook, cast.to)
        }

        let (mut same_cast, mut opp_cast) = metadata.castling_rights(color);
        let (same_new_cast, opp_new_cast) = mv.castling_rights(metadata.castling_details);

        hash ^= same.hash_castle(same_cast) ^ opposite.hash_castle(opp_cast);
        same_cast.update(same_new_cast);
        opp_cast.update(opp_new_cast);
        hash ^= same.hash_castle(same_cast) ^ opposite.hash_castle(opp_cast);

        hash ^= self.hash_en_passant(metadata.en_passant)
            ^ self.hash_en_passant(mv.en_passant_square());
        hash
    }
}

impl Fill for BitBoardHasher {
    fn fill<R: rand::Rng + ?Sized>(&mut self, rng: &mut R) {
        self.en_passant_file.fill(rng);
        self.en_passant_file = self.en_passant_file.map(|x| x & Self::HASH_BITS);
        self.white.fill(rng);
        self.black.fill(rng);
    }
}

#[derive(Default, Debug)]
pub struct HalfBitBoardHasher {
    // pub color: Color,
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
            ooo: rng.random::<HashResult>() & BitBoardHasher::HASH_BITS,
            oo: rng.random::<HashResult>() & BitBoardHasher::HASH_BITS,
        };

        self.kings.fill(rng);
        self.queens.fill(rng);
        self.rooks.fill(rng);
        self.bishops.fill(rng);
        self.knights.fill(rng);
        self.pawns.fill(rng);
    }
}
