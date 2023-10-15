use crate::{
    pieces::{Color, Piece},
    squares::{Position, Square},
    zobrist::ZobristHasher,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Queens,
    Kings,
}

impl Side {
    fn piece(self) -> Piece {
        match self {
            Side::Queens => Piece::King,
            Side::Kings => Piece::Queen,
        }
    }
}

pub trait Board {
    fn standard() -> Self;
    fn zobrist(&self, zh: &ZobristHasher) -> u128;
    fn get(&self, c: Color, p: Piece) -> Position;
    fn at(&self, n: Square) -> Option<(Color, Piece)>;
    fn valid(&self) -> bool;
    fn to_move(&self) -> Color;
    fn en_passant_square(&self) -> Option<Square>;
    fn turn(&self) -> usize;
    fn tempo_clock(&self) -> usize;
    fn castling_possible(&self, c: Color, s: Side) -> bool;

    fn fen(&self) -> String {
        const SPACES: [&'static str; 9] = ["", "1", "2", "3", "4", "5", "6", "7", "8"];

        let ranks = (1u32..=8)
            .rev()
            .map(|rank| {
                let mut res = String::new();
                let mut space = 0;
                for file in 'a'..='h' {
                    if let Some((c, p)) = self.at(Square::from_file_and_rank(file, rank)) {
                        res.push_str(SPACES[space]);
                        space = 0;
                        res.push(p.fen(c));
                    } else {
                        space += 1;
                    }
                }
                res.push_str(SPACES[space]);
                res
            })
            .collect::<Vec<String>>()
            .join("/");

        let mut castling = String::new();
        for col in [Color::White, Color::Black] {
            for side in [Side::Kings, Side::Queens] {
                castling.push(side.piece().fen(col));
            }
        }
        if castling.is_empty() {
            castling.push('-');
        }

        return format!(
            "{board} {to_move} {cr} {eps} {hmc} {turn}",
            board = ranks,
            to_move = self.to_move().fen(),
            cr = castling,
            eps = self.en_passant_square().map(Square::alg).unwrap_or("-"),
            hmc = self.tempo_clock(),
            turn = self.turn()
        );
    }

    fn en_passant_pawn(&self) -> Option<Square> {
        let sq = self.en_passant_square()?;
        let (f, r) = sq.file_and_rank();
        Some(Square::from_file_and_rank(
            f,
            if self.to_move() == Color::White {
                r - 1
            } else {
                r + 1
            },
        ))
    }
}
