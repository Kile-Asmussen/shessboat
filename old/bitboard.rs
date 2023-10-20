use crate::{
    board::{Board, CastlingRights, Side},
    byteboard::ByteBoard,
    moves::Tempi,
    pieces::{Color, Piece},
    squares::{Position, Square},
    zobrist::{ZobristHasher, Zobristic},
};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct BitBoard {
    pub black: BitHalfBoard<Position, Option<Square>>,
    pub white: BitHalfBoard<Position, Option<Square>>,
    pub en_passant_square: Option<Square>,
    pub tempi: Tempi,
}

impl Board for BitBoard {
    fn standard() -> Self {
        Self {
            black: BitHalfBoard {
                kings: Square::from_alg("e8").position(),
                queens: Square::from_alg("d8").position(),
                rooks: Square::from_alg("a8")
                    .position()
                    .with(Square::from_alg("h8").position()),
                bishops: Square::from_alg("c8")
                    .position()
                    .with(Square::from_alg("f8").position()),
                knights: Square::from_alg("b8")
                    .position()
                    .with(Square::from_alg("g8").position()),
                pawns: Position::rank(7),
                castling_rights: CastlingRights {
                    queen_side: Some(Square::from_alg("a8")),
                    king_side: Some(Square::from_alg("h8")),
                },
            },
            white: BitHalfBoard {
                kings: Square::from_alg("e1").position(),
                queens: Square::from_alg("d1").position(),
                rooks: Square::from_alg("a1")
                    .position()
                    .with(Square::from_alg("h1").position()),
                bishops: Square::from_alg("c1")
                    .position()
                    .with(Square::from_alg("f1").position()),
                knights: Square::from_alg("b1")
                    .position()
                    .with(Square::from_alg("g1").position()),
                pawns: Position::rank(2),
                castling_rights: CastlingRights {
                    queen_side: Some(Square::from_alg("a1")),
                    king_side: Some(Square::from_alg("h1")),
                },
            },
            en_passant_square: None,
            tempi: Default::default(),
        }
    }

    fn tempi(&self) -> Tempi {
        self.tempi
    }

    fn find(&self, c: Color, p: Piece) -> Position {
        match c {
            Color::White => *self.white.get(p),
            Color::Black => *self.black.get(p),
        }
    }

    fn at(&self, n: Square) -> Option<(Color, Piece)> {
        self.white
            .at(n)
            .map(|p| (Color::White, p))
            .or_else(|| self.black.at(n).map(|p| (Color::Black, p)))
    }

    fn valid(&self) -> bool {
        self.white.valid(Color::White)
            && self.black.valid(Color::Black)
            && (self.white.num_squares() + self.black.num_squares())
                == self
                    .white
                    .position()
                    .with(self.black.position())
                    .num_squares()
            && self.en_passant_square_valid()
    }

    fn en_passant_square(&self) -> Option<Square> {
        self.en_passant_square
    }

    fn castling_possible(&self, c: Color, s: Side) -> bool {
        match c {
            Color::White => self.white.castling_rights,
            Color::Black => self.black.castling_rights,
        }
        .get(s)
        .is_some()
    }

    fn set_position(&mut self, p: Option<(Color, Piece)>, ps: Position) {
        self.white.set_blank(ps);
        self.black.set_blank(ps);
        if let Some((c, p)) = p {
            let board = match c {
                Color::White => &mut self.white,
                Color::Black => &mut self.black,
            }
            .get_mut(p);
            *board = board.with(ps);
        }
    }

    fn set_square(&mut self, p: Option<(Color, Piece)>, sq: Square) {
        self.set_position(p, sq.position())
    }

    fn set_castling_rook(&mut self, c: Color, s: Side, sq: Option<Square>) {
        if let Some(sq) = sq {
            if self.at(sq) != Some((c, Piece::Rook)) {
                panic!("Not a valid castling rook")
            }
        }
        *(match c {
            Color::White => self.white.castling_rights,
            Color::Black => self.black.castling_rights,
        }
        .get_mut(s)) = sq;
    }

    fn set_en_passant(&mut self, sq: Option<Square>) {
        self.en_passant_square = sq;
    }

    fn set_tempi(&mut self, t: Tempi) {
        self.tempi = t;
    }
}

impl Zobristic for BitBoard {
    fn zobrist(&self, zh: &ZobristHasher) -> u128 {
        zh.hash_tempi(self.tempi())
            ^ zh.hash_with_color(Color::White, self.white.zobrist(zh))
            ^ zh.hash_with_color(Color::Black, self.black.zobrist(zh))
            ^ zh.hash_enpassant_square(self.en_passant_square())
    }
}

impl BitBoard {
    fn en_passant_square_valid(&self) -> bool {
        if let Some(sq) = self.en_passant_square {
            let (f, r) = sq.file_and_rank();
            if self.tempi().to_move() == Color::White {
                self.at(Square::from_file_and_rank(f, r + 1)) == Some((Color::White, Piece::Pawn))
            } else {
                self.at(Square::from_file_and_rank(f, r - 1)) == Some((Color::Black, Piece::Pawn))
            }
        } else {
            true
        }
    }

    pub fn as_byteboard(&self) -> ByteBoard {
        let mut res: ByteBoard = Default::default();

        res.set_tempi(self.tempi());

        for (c, h) in [(Color::White, &self.white), (Color::Black, &self.black)] {
            for s in [Side::Queens, Side::Kings] {
                res.set_castling_rook(c, s, h.castling_rights.get(s))
            }

            use Piece::*;

            for p in [Pawn, Knight, Bishop, Rook, Queen, King] {
                res.set_position(Some((c, p)), *h.get(p))
            }
        }

        res.set_en_passant(self.en_passant_square());

        return res;
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct BitHalfBoard<Board, Right> {
    pub kings: Board,
    pub queens: Board,
    pub rooks: Board,
    pub bishops: Board,
    pub knights: Board,
    pub pawns: Board,
    pub castling_rights: CastlingRights<Right>,
}

impl BitHalfBoard<Position, Option<Square>> {
    fn num_squares(&self) -> u32 {
        self.kings.num_squares()
            + self.queens.num_squares()
            + self.rooks.num_squares()
            + self.bishops.num_squares()
            + self.knights.num_squares()
            + self.pawns.num_squares()
    }

    fn position(&self) -> Position {
        self.kings
            .with(self.queens)
            .with(self.rooks)
            .with(self.bishops)
            .with(self.knights)
            .with(self.pawns)
    }

    fn valid(&self, c: Color) -> bool {
        let rights = self.castling_rights.king_side.is_some() as u32
            + self.castling_rights.queen_side.is_some() as u32;

        if rights != 0 {
            rights <= self.rooks.num_squares()
                && if c == Color::White {
                    self.kings.overlap(Position::rank(1)).populated()
                } else {
                    self.kings.overlap(Position::rank(8)).populated()
                }
        } else {
            true
        }
    }

    fn at(&self, sq: Square) -> Option<Piece> {
        Some(if self.kings.contains(sq) {
            Piece::King
        } else if self.queens.contains(sq) {
            Piece::Queen
        } else if self.rooks.contains(sq) {
            Piece::Rook
        } else if self.bishops.contains(sq) {
            Piece::Bishop
        } else if self.knights.contains(sq) {
            Piece::Knight
        } else if self.pawns.contains(sq) {
            Piece::Pawn
        } else {
            return None;
        })
    }

    fn set_blank(&mut self, ps: Position) {
        self.kings = self.kings.without(ps);
        self.queens = self.queens.without(ps);
        self.rooks = self.rooks.without(ps);
        self.bishops = self.bishops.without(ps);
        self.knights = self.knights.without(ps);
        self.pawns = self.pawns.without(ps);
    }
}

impl<Right> Zobristic for BitHalfBoard<Position, Right>
where
    CastlingRights<Right>: Zobristic,
{
    fn zobrist(&self, zh: &ZobristHasher) -> u128 {
        zh.hash_position(Piece::King, self.kings)
            ^ zh.hash_position(Piece::Queen, self.queens)
            ^ zh.hash_position(Piece::Rook, self.rooks)
            ^ zh.hash_position(Piece::Bishop, self.bishops)
            ^ zh.hash_position(Piece::Knight, self.knights)
            ^ zh.hash_position(Piece::Pawn, self.pawns)
            ^ self.castling_rights.zobrist(zh)
    }
}

impl<Board, Right> BitHalfBoard<Board, Right> {
    pub fn get(&self, p: Piece) -> &Board {
        match p {
            Piece::King => &self.kings,
            Piece::Queen => &self.queens,
            Piece::Rook => &self.rooks,
            Piece::Bishop => &self.bishops,
            Piece::Knight => &self.kings,
            Piece::Pawn => &self.pawns,
        }
    }

    fn get_mut(&mut self, p: Piece) -> &mut Board {
        match p {
            Piece::King => &mut self.kings,
            Piece::Queen => &mut self.queens,
            Piece::Rook => &mut self.rooks,
            Piece::Bishop => &mut self.bishops,
            Piece::Knight => &mut self.kings,
            Piece::Pawn => &mut self.pawns,
        }
    }
}
