use std::{
    backtrace,
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
    moves::Move,
    pieces::{
        Micropawns,
        pawns::{EnPassant, Pawns},
    },
    squares::Square,
};

pub type PositionHashes = HashMap<HashResult, Micropawns>;

#[derive(Debug, Default)]
struct ReadHashesError;
impl Error for ReadHashesError {}
impl Display for ReadHashesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Error reading hash file")
    }
}

pub type MaskHasher = BoardMap<HashResult>;
pub type HashResult = u128;

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
    pub black_to_move: HashResult,
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
        self.hash_to_move(board.metadata.to_move)
            ^ self.hash_en_passant(board.metadata.en_passant)
            ^ self.white.hash_castle(board.metadata.white_castling)
            ^ self.black.hash_castle(board.metadata.black_castling)
            ^ self.white.hash(&board.white)
            ^ self.black.hash(&board.black)
    }

    pub fn hash_to_move(&self, turn: Color) -> HashResult {
        if turn == Color::Black {
            self.black_to_move //.clone()
        } else {
            0
            // HashResult::default()
        }
    }

    pub fn hash_en_passant(&self, en_passant: Option<EnPassant>) -> HashResult {
        if let Some(EnPassant { to, capture }) = en_passant {
            self.en_passant_file[to.file().as_file() as usize] //.clone()
        } else {
            0
            //HashResult::default()
        }
    }

    pub fn delta_hash_move(&self, board: &BitBoard, mut hash: HashResult, mv: Move) -> HashResult {
        let (color, piece) = mv.color_and_piece.split();

        let (same, opposite) = match color {
            Color::White => (&self.white, &self.black),
            Color::Black => (&self.black, &self.white),
        };

        hash ^= self.black_to_move; //.clone();

        if let Some(p) = mv.promotion {
            hash ^=
                same.hash_piece(Piece::Pawn, mv.from_to.from) ^ same.hash_piece(p, mv.from_to.to)
        } else {
            hash ^= same.hash_piece(piece, mv.from_to.from) ^ same.hash_piece(piece, mv.from_to.to);
        }

        if let Some((sq, p)) = mv.capture {
            hash ^= opposite.hash_piece(p, sq);
        }

        if let Some(cast) = mv.castling {
            hash ^= same.hash_piece(Piece::Rook, cast.from) ^ same.hash_piece(Piece::Rook, cast.to)
        }

        let (mut same_cast, mut opp_cast) = board.metadata.castling_rights(color);
        let (same_new_cast, opp_new_cast) = mv.castling_rights(board.metadata.castling_details);

        hash ^= same.hash_castle(same_cast) ^ opposite.hash_castle(opp_cast);
        same_cast.update(same_new_cast);
        opp_cast.update(opp_new_cast);
        hash ^= same.hash_castle(same_cast) ^ opposite.hash_castle(opp_cast);

        hash ^= self.hash_en_passant(board.metadata.en_passant)
            ^ self.hash_en_passant(mv.en_passant_square());

        hash
    }
}

impl Fill for BitBoardHasher {
    fn fill<R: rand::Rng + ?Sized>(&mut self, rng: &mut R) {
        self.black_to_move = rng.random();
        self.en_passant_file.fill(rng);
        // self.black_to_move = BTreeSetWrapper(BTreeSet::from_iter(["black".to_string()]));
        // let q = "abcdefgh"
        //     .chars()
        //     .map(|c| BTreeSetWrapper(BTreeSet::from_iter([format!("epc {}", c)])))
        //     .collect::<Vec<_>>();
        // self.en_passant_file = std::array::from_fn(|n| q[n].clone());
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
        (if castling.ooo {
            self.castling.ooo //.clone()
        } else {
            0
            //HashResult::default()
        }) ^ (if castling.oo {
            self.castling.oo //.clone()
        } else {
            0
            //HashResult::default()
        })
    }
}

impl Fill for HalfBitBoardHasher {
    fn fill<R: rand::Rng + ?Sized>(&mut self, rng: &mut R) {
        // if self.color == Color::White {
        self.castling = CastlingInfo {
            ooo: rng.random(),
            // ooo: BTreeSetWrapper(BTreeSet::from_iter(["O-O-O".to_string()])),
            oo: rng.random(),
            // oo: BTreeSetWrapper(BTreeSet::from_iter(["O-O".to_string()])),
        };
        // } else {
        //     self.castling = CastlingInfo {
        //         ooo: BTreeSetWrapper(BTreeSet::from_iter(["o-o-o".to_string()])),
        //         oo: BTreeSetWrapper(BTreeSet::from_iter(["o-o".to_string()])),
        //     };
        // }

        self.kings.fill(rng /* self.color, Piece::King */);
        self.queens.fill(rng /* self.color, Piece::Queen */);
        self.rooks.fill(rng /* self.color, Piece::Rook */);
        self.bishops.fill(rng /* self.color, Piece::Bishop */);
        self.knights.fill(rng /* self.color, Piece::Knight */);
        self.pawns.fill(rng /* self.color, Piece::Pawn */);
    }
}

// #[derive(Clone, Debug, PartialEq, Eq, Default)]
// pub struct BTreeSetWrapper(BTreeSet<String>);
// impl BitXor for BTreeSetWrapper {
//     type Output = BTreeSetWrapper;

//     fn bitxor(self, rhs: Self) -> Self::Output {
//         Self(
//             self.0
//                 .symmetric_difference(&rhs.0)
//                 .map(|s| s.clone())
//                 .collect(),
//         )
//     }
// }
// impl BitXorAssign for BTreeSetWrapper {
//     fn bitxor_assign(&mut self, rhs: Self) {
//         *self = self.clone() ^ rhs;
//     }
// }
// impl Display for BTreeSetWrapper {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         Debug::fmt(&self, f)
//     }
// }
// impl UpperHex for BTreeSetWrapper {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         Debug::fmt(&self, f)
//     }
// }
// impl Hash for BTreeSetWrapper {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         for s in &self.0 {
//             s.hash(state)
//         }
//     }
// }
//
// impl BoardMap<BTreeSetWrapper> {
//     pub fn fill<R: ?Sized>(&mut self, _: &mut R, c: Color, p: Piece) {
//         let color_piece = ColorPiece::new(c, p);
//         for sq in Mask::full() {
//             self.set_clone(
//                 sq,
//                 BTreeSetWrapper(BTreeSet::from_iter([format!(
//                     "{}{}",
//                     color_piece.letter(),
//                     sq
//                 )])),
//             );
//         }
//     }
// }
