use rand::prelude::*;

use crate::bitboard::BitHalfBoard;
use crate::board::{CastlingRights, Side};
use crate::moves::Tempi;
use crate::pieces::{Color, Piece};
use crate::squares::{Position, Square};

pub struct ZobristHasher {
    board: BitHalfBoard<[u128; 64], u128>,
    en_passant: [u128; 64],
    white_to_move: u128,
}

pub trait Zobristic {
    fn zobrist(&self, zh: &ZobristHasher) -> u128;
}

impl ZobristHasher {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            board: BitHalfBoard {
                kings: Self::array(&mut rng),
                queens: Self::array(&mut rng),
                rooks: Self::array(&mut rng),
                bishops: Self::array(&mut rng),
                knights: Self::array(&mut rng),
                pawns: Self::array(&mut rng),
                castling_rights: CastlingRights {
                    queen_side: rng.next_u128(),
                    king_side: rng.next_u128(),
                },
            },
            en_passant: Self::array(&mut rng),
            white_to_move: rng.next_u128(),
        }
    }

    fn array(trng: &mut ThreadRng) -> [u128; 64] {
        let mut array = [0; 64];
        for i in &mut array {
            *i = trng.next_u128();
        }
        return array;
    }

    pub fn hash_bithalfboard(&self, half_board: &BitHalfBoard<Position, bool>) -> u128 {
        self.hash_position(Piece::King, half_board.kings)
            ^ self.hash_position(Piece::Queen, half_board.queens)
            ^ self.hash_position(Piece::Rook, half_board.rooks)
            ^ self.hash_position(Piece::Bishop, half_board.bishops)
            ^ self.hash_position(Piece::Knight, half_board.knights)
            ^ self.hash_position(Piece::Pawn, half_board.pawns)
            ^ (half_board.castling_rights.king_side as u128 * self.board.castling_rights.king_side)
            ^ (half_board.castling_rights.queen_side as u128
                * self.board.castling_rights.queen_side)
    }

    pub fn hash_position(&self, piece: Piece, pos: Position) -> u128 {
        let mut res = 0;
        for sq in pos {
            res ^= self.hash_square(piece, sq);
        }
        return res;
    }

    pub fn hash_square(&self, piece: Piece, sq: Square) -> u128 {
        let array = self.board.get(piece);
        return array[sq.index()];
    }

    pub fn hash_enpassant_square(&self, eps: Option<Square>) -> u128 {
        if let Some(sq) = eps {
            self.en_passant[sq.index()]
        } else {
            0
        }
    }

    pub fn hash_to_move(&self, c: Color) -> u128 {
        self.hash_with_color(c, self.white_to_move)
    }

    pub fn hash_tempi(&self, t: Tempi) -> u128 {
        self.hash_with_color(t.to_move(), self.white_to_move)
    }

    pub fn hash_with_color(&self, c: Color, h: u128) -> u128 {
        if c == Color::White {
            h
        } else {
            !h
        }
    }

    pub fn hash_castling_right(&self, s: Side) -> u128 {
        match s {
            Side::Queens => self.board.castling_rights.queen_side,
            Side::Kings => self.board.castling_rights.king_side,
        }
    }
}

trait ThreadRngExtensions {
    fn next_u128(&mut self) -> u128;
}

impl ThreadRngExtensions for ThreadRng {
    fn next_u128(&mut self) -> u128 {
        self.next_u64() as u128 | ((self.next_u64() as u128) << 64)
    }
}
