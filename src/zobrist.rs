use rand::prelude::*;

use crate::bitboard::{BitBoard, BitHalfBoard, CastlingRights};
use crate::moves::{CastlingMove, Move, PawnMove, PromotionMove, StandardMove};
use crate::pieces::{Color, Piece};
use crate::squares::{Position, Square};

pub struct ZobristHasher {
    board: BitHalfBoard<[u128; 64], u128>,
    en_passant: [u128; 64],
    black_to_move: u128,
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
            black_to_move: rng.next_u128(),
        }
    }

    fn array(trng: &mut ThreadRng) -> [u128; 64] {
        let mut array = [0; 64];
        for i in &mut array {
            *i = trng.next_u128();
        }
        return array;
    }

    pub fn hash_board(&self, board: &BitBoard) -> u128 {
        self.hash_turn(board.to_move())
            ^ Self::hash_color(Color::White, self.hash_halfboard(&board.white))
            ^ Self::hash_color(Color::Black, self.hash_halfboard(&board.black))
            ^ self.hash_enpassent(board.en_passant_square)
    }

    pub fn hash_halfboard(&self, half_board: &BitHalfBoard<Position, bool>) -> u128 {
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

    fn hash_position(&self, piece: Piece, pos: Position) -> u128 {
        let mut res = 0;
        let array = self.board.get(piece);
        let mut pos = pos.mask;
        for h in array {
            if pos & 1 == 1 {
                res ^= h;
            }
            pos >>= 1;
        }
        return res;
    }

    fn hash_square(&self, piece: Piece, sq: Square) -> u128 {
        let mut res = 0;
        let array = self.board.get(piece);
        return array[sq.index()];
    }

    fn hash_move(&self, r#move: Move, eps_was: Option<Square>) -> u128 {
        match r#move {
            Move::Castling(castl) => self.hash_castling(castl, eps_was),
            Move::Pawn(pawn) => self.hash_pawn_move(pawn, eps_was),
            Move::Standard(std) => self.hash_standard_move(std, eps_was),
            Move::Promotion(prommy) => self.hash_promotion(prommy, eps_was),
        }
    }

    fn hash_standard_move(&self, std: StandardMove, eps: Option<Square>) -> u128 {
        let mut res = Self::hash_color(
            std.color,
            self.hash_square(std.piece, std.from) ^ self.hash_square(std.piece, std.to),
        );

        if let Some(capture) = std.capture {
            res ^= Self::hash_color(std.color.opposite(), self.hash_square(capture, std.to));
        }

        res ^= self.hash_turn(std.color.opposite());

        return res;
    }

    fn hash_castling(&self, castl: CastlingMove, eps_was: Option<Square>) -> u128 {
        let mut res = Self::hash_color(
            castl.color,
            self.hash_square(Piece::King, castl.king_from)
                ^ self.hash_square(Piece::King, castl.king_to),
        );
        res ^= Self::hash_color(
            castl.color,
            self.hash_square(Piece::Rook, castl.rook_from)
                ^ self.hash_square(Piece::Rook, castl.rook_to),
        );
        res ^= self.hash_enpassent(eps_was);
        res ^= self.hash_turn(castl.color.opposite());
        return res;
    }

    fn hash_pawn_move(&self, pawn: PawnMove, eps_was: Option<Square>) -> u128 {
        let mut res = Self::hash_color(
            pawn.color,
            self.hash_square(Piece::Pawn, pawn.from) ^ self.hash_square(Piece::Pawn, pawn.to),
        );

        if let Some((capture, sq)) = pawn.capture {
            res ^= Self::hash_color(pawn.color.opposite(), self.hash_square(capture, sq));
        }

        res ^= self.hash_enpassent(eps_was);
        res ^= self.hash_enpassent(pawn.exposes_en_passent());
        res ^= self.hash_turn(pawn.color.opposite());

        return res;
    }

    fn hash_promotion(&self, prommy: PromotionMove, eps_was: Option<Square>) -> u128 {
        let mut res = Self::hash_color(prommy.color, self.hash_square(Piece::Pawn, prommy.from));
        res ^= Self::hash_color(prommy.color, self.hash_square(prommy.into, prommy.to));

        res ^= self.hash_enpassent(eps_was);
        res ^= self.hash_turn(prommy.color.opposite());
        return res;
    }

    fn hash_enpassent(&self, eps: Option<Square>) -> u128 {
        if let Some(sq) = eps {
            self.en_passant[sq.index()]
        } else {
            0
        }
    }

    fn hash_turn(&self, c: Color) -> u128 {
        if c == Color::White {
            0
        } else {
            self.black_to_move
        }
    }

    fn hash_color(c: Color, h: u128) -> u128 {
        if c == Color::White {
            h
        } else {
            !h
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
