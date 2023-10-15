use crate::{
    pieces::{Color, Piece},
    squares::Square,
    zobrist::{ZobristHasher, Zobristic},
};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Tempi {
    tempo: u32,
    last_advance: u32,
}

impl Tempi {
    pub fn to_move(self) -> Color {
        if self.tempo & 1 == 0 {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn turn(self) -> usize {
        (self.tempo / 2 + 1) as usize
    }

    pub fn fifty_move_clock(&self) -> usize {
        self.last_advance as usize
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move {
    Castling(CastlingMove),
    Pawn(PawnMove),
    Promotion(PromotionMove),
    Standard(StandardMove),
}

impl Zobristic for Move {
    fn zobrist(&self, zh: &ZobristHasher) -> u128 {
        match self {
            Move::Castling(c) => c.zobrist(zh),
            Move::Pawn(p) => p.zobrist(zh),
            Move::Promotion(p) => p.zobrist(zh),
            Move::Standard(s) => s.zobrist(zh),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StandardMove {
    pub color: Color,
    pub piece: Piece,
    pub from: Square,
    pub to: Square,
    pub capture: Option<Piece>,
}

impl Zobristic for StandardMove {
    fn zobrist(&self, zh: &ZobristHasher) -> u128 {
        let mut res = zh.hash_with_color(
            self.color,
            zh.hash_square(self.piece, self.from) ^ zh.hash_square(self.piece, self.to),
        );
        if let Some(capture) = self.capture {
            res ^= zh.hash_with_color(self.color.opposite(), zh.hash_square(capture, self.to));
        }
        res ^= zh.hash_to_move(self.color.opposite());
        return res;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PawnMove {
    pub color: Color,
    pub from: Square,
    pub to: Square,
    pub capture: Option<(Piece, Square)>,
}

impl Zobristic for PawnMove {
    fn zobrist(&self, zh: &ZobristHasher) -> u128 {
        let mut res = zh.hash_with_color(
            self.color,
            zh.hash_square(Piece::Pawn, self.from) ^ zh.hash_square(Piece::Pawn, self.to),
        );
        if let Some((capture, sq)) = self.capture {
            res ^= zh.hash_with_color(self.color.opposite(), zh.hash_square(capture, sq));
        }
        res ^= zh.hash_enpassant_square(self.exposes_en_passant());
        res ^= zh.hash_to_move(self.color.opposite());
        return res;
    }
}

impl PawnMove {
    pub fn exposes_en_passant(&self) -> Option<Square> {
        let (f1, r1) = self.from.file_and_rank();
        let (f2, r2) = self.to.file_and_rank();

        if f1 == f2 && r1.abs_diff(r2) == 2 {
            Some(Square::from_file_and_rank(f1, (r1 + r2) / 2))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PromotionMove {
    pub color: Color,
    pub from: Square,
    pub to: Square,
    pub into: Piece,
}

impl Zobristic for PromotionMove {
    fn zobrist(&self, zh: &ZobristHasher) -> u128 {
        let mut res = zh.hash_with_color(self.color, zh.hash_square(Piece::Pawn, self.from));
        res ^= zh.hash_with_color(self.color, zh.hash_square(self.into, self.to));
        res ^= zh.hash_to_move(self.color.opposite());
        return res;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CastlingMove {
    pub color: Color,
    pub king_from: Square,
    pub king_to: Square,
    pub rook_from: Square,
    pub rook_to: Square,
}

impl Zobristic for CastlingMove {
    fn zobrist(&self, zh: &ZobristHasher) -> u128 {
        let mut res = zh.hash_with_color(
            self.color,
            zh.hash_square(Piece::King, self.king_from) ^ zh.hash_square(Piece::King, self.king_to),
        );
        res ^= zh.hash_with_color(
            self.color,
            zh.hash_square(Piece::Rook, self.rook_from) ^ zh.hash_square(Piece::Rook, self.rook_to),
        );
        res ^= zh.hash_to_move(self.color.opposite());
        return res;
    }
}
