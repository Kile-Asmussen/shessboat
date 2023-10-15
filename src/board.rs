use crate::{
    moves::Tempi,
    pieces::{Color, Piece},
    squares::{Position, Square},
    zobrist::{ZobristHasher, Zobristic},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Queens,
    Kings,
}

impl Side {
    fn piece(self) -> Piece {
        match self {
            Side::Queens => Piece::Queen,
            Side::Kings => Piece::King,
        }
    }
}

pub trait Board: Zobristic + Default {
    fn standard() -> Self;
    fn find(&self, c: Color, p: Piece) -> Position;
    fn at(&self, n: Square) -> Option<(Color, Piece)>;
    fn valid(&self) -> bool;
    fn tempi(&self) -> Tempi;
    fn en_passant_square(&self) -> Option<Square>;
    fn castling_possible(&self, c: Color, s: Side) -> bool;

    fn set_position(&mut self, p: Option<(Color, Piece)>, ps: Position);
    fn set_square(&mut self, p: Option<(Color, Piece)>, sq: Square);
    fn set_castling_rook(&mut self, c: Color, s: Side, sq: Option<Square>);
    fn set_en_passant(&mut self, sq: Option<Square>);
    fn set_tempi(&mut self, t: Tempi);
}

pub trait BoardExtensions: Board {
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
                if self.castling_possible(col, side) {
                    castling.push(side.piece().fen(col));
                }
            }
        }
        if castling.is_empty() {
            castling.push('-');
        }

        return format!(
            "{board} {to_move} {cr} {eps} {hmc} {turn}",
            board = ranks,
            to_move = self.tempi().to_move().fen(),
            cr = castling,
            eps = self.en_passant_square().map(Square::alg).unwrap_or("-"),
            hmc = self.tempi().fifty_move_clock(),
            turn = self.tempi().turn()
        );
    }

    fn en_passant_pawn(&self) -> Option<Square> {
        let sq = self.en_passant_square()?;
        let (f, r) = sq.file_and_rank();
        Some(Square::from_file_and_rank(
            f,
            if self.tempi().to_move() == Color::White {
                r - 1
            } else {
                r + 1
            },
        ))
    }
}

impl<B: Board> BoardExtensions for B {}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct CastlingRights<Right> {
    pub queen_side: Right,
    pub king_side: Right,
}

impl<Right> CastlingRights<Right>
where
    Right: Copy + Clone,
{
    pub fn get(&self, side: Side) -> Right {
        match side {
            Side::Queens => self.queen_side,
            Side::Kings => self.king_side,
        }
    }
}

impl<Right> CastlingRights<Right> {
    pub fn get_mut(&mut self, side: Side) -> &mut Right {
        match side {
            Side::Queens => &mut self.queen_side,
            Side::Kings => &mut self.king_side,
        }
    }
}

impl Zobristic for CastlingRights<Option<Square>> {
    fn zobrist(&self, zh: &ZobristHasher) -> u128 {
        (self.king_side.is_some() as u128 * zh.hash_castling_right(Side::Kings))
            ^ (self.queen_side.is_some() as u128 * zh.hash_castling_right(Side::Queens))
    }
}
