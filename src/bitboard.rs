use crate::{
    board::{Board, Side},
    pieces::{Color, Piece},
    squares::{Position, Square},
};

pub struct BitBoard {
    pub black: BitHalfBoard<Position, bool>,
    pub white: BitHalfBoard<Position, bool>,
    pub en_passant_square: Option<Square>,
    pub tempo: u32,
    pub last_advance: u32,
}

pub struct BitHalfBoard<Board, Right> {
    pub kings: Board,
    pub queens: Board,
    pub rooks: Board,
    pub bishops: Board,
    pub knights: Board,
    pub pawns: Board,
    pub castling_rights: CastlingRights<Right>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights<Right> {
    pub queen_side: Right,
    pub king_side: Right,
}

impl Board for BitBoard {
    pub fn standard() -> Self {
        let castling_rights = CastlingRights {
            queen_side: true,
            king_side: true,
        };
        Self {
            black: BitHalfBoard {
                kings: Square::from_file_and_rank('e', 8).position(),
                queens: Square::from_file_and_rank('d', 8).position(),
                rooks: Square::from_file_and_rank('a', 8)
                    .position()
                    .with(Square::from_file_and_rank('h', 8).position()),
                bishops: Square::from_file_and_rank('c', 8)
                    .position()
                    .with(Square::from_file_and_rank('f', 8).position()),
                knights: Square::from_file_and_rank('b', 8)
                    .position()
                    .with(Square::from_file_and_rank('g', 8).position()),
                pawns: Position::rank(7),
                castling_rights,
            },
            white: BitHalfBoard {
                kings: Square::from_file_and_rank('e', 1).position(),
                queens: Square::from_file_and_rank('d', 1).position(),
                rooks: Square::from_file_and_rank('a', 1)
                    .position()
                    .with(Square::from_file_and_rank('h', 1).position()),
                bishops: Square::from_file_and_rank('c', 1)
                    .position()
                    .with(Square::from_file_and_rank('f', 1).position()),
                knights: Square::from_file_and_rank('b', 1)
                    .position()
                    .with(Square::from_file_and_rank('g', 1).position()),
                pawns: Position::rank(2),
                castling_rights,
            },
            en_passant_square: None,
            tempo: 0,
            last_advance: 0,
        }
    }

    pub fn get(&self, c: Color, p: Piece) -> Position {
        match c {
            Color::White => *self.white.get(p),
            Color::Black => *self.black.get(p),
        }
    }

    pub fn at(&self, n: Square) -> Option<(Color, Piece)> {
        self.white
            .at(n)
            .map(|p| (Color::White, p))
            .or_else(|| self.black.at(n).map(|p| (Color::Black, p)))
    }

    pub fn valid(&self) -> bool {
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

    pub fn to_move(&self) -> Color {
        if self.tempo & 1 == 0 {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn turn(&self) -> usize {
        (self.tempo / 2 + 1) as usize
    }

    pub fn en_passant_square(&self) -> Option<Square> {
        self.en_passant_square
    }

    pub fn tempo_clock(&self) -> usize {
        self.last_advance as usize
    }

    pub fn castling_possible(&self, c: Color, s: Side) -> bool {
        match (c, s) {
            (Color::White, Side::Queens) => self.white.castling_rights.queen_side,
            (Color::White, Side::Kings) => self.white.castling_rights.king_side,
            (Color::Black, Side::Queens) => self.black.castling_rights.queen_side,
            (Color::Black, Side::Kings) => self.black.castling_rights.king_side,
        }
    }
}

impl BitBoard {
    fn en_passant_square_valid(&self) -> bool {
        if let Some(sq) = self.en_passant_square {
            let (f, r) = sq.file_and_rank();
            if self.to_move() == Color::White {
                self.at(Square::from_file_and_rank(f, r + 1)) == Some((Color::White, Piece::Pawn))
            } else {
                self.at(Square::from_file_and_rank(f, r - 1)) == Some((Color::Black, Piece::Pawn))
            }
        } else {
            true
        }
    }
}

impl BitHalfBoard<Position, bool> {
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
        let rights = self.castling_rights.king_side as u32 + self.castling_rights.queen_side as u32;

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

    pub fn at(&self, sq: Square) -> Option<Piece> {
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
}

impl CastlingRights<bool> {
    pub fn fen(&self, c: Color) -> &'static str {
        match (c, self.king_side, self.queen_side) {
            (Color::White, true, true) => "KQ",
            (Color::White, true, false) => "K",
            (Color::White, false, true) => "Q",
            (Color::White, false, false) => "",
            (Color::Black, true, true) => "kq",
            (Color::Black, true, false) => "k",
            (Color::Black, false, true) => "q",
            (Color::Black, false, false) => "",
        }
    }
}
