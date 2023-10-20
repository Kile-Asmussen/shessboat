use crate::{
    bitboard::BitBoard,
    board::{Board, CastlingRights, Side},
    moves::Tempi,
    pieces::{Color, Piece},
    squares::{AllSquares, Position, Square},
    zobrist::{ZobristHasher, Zobristic},
};

#[derive(Debug, PartialEq, Eq)]
pub struct ByteBoard {
    board: [u8; 64],
    white_castle: CastlingRights<Option<Square>>,
    black_castle: CastlingRights<Option<Square>>,
    tempi: Tempi,
}

impl ByteBoard {
    const PAWN: u8 = 0x1;
    const KNIGHT: u8 = 0x2;
    const BISHOP: u8 = 0x3;
    const ROOK: u8 = 0x4;
    const QUEEN: u8 = 0x5;
    const KING: u8 = 0x6;

    const PIECE_MASK: u8 = 0x7;

    const WHITE: u8 = 0x8;
    const BLACK: u8 = 0x10;
    const COLOR_MASK: u8 = Self::WHITE | Self::BLACK;

    const EPS: u8 = 0xFF;

    fn decode_byte(b: u8) -> Option<(Color, Piece)> {
        if b == 0 || b == Self::EPS {
            return None;
        }

        Some((
            match b & Self::COLOR_MASK {
                Self::WHITE => Color::White,
                Self::BLACK => Color::Black,
                _ => panic!("Invalid byte"),
            },
            match b & Self::PIECE_MASK {
                Self::PAWN => Piece::Pawn,
                Self::KNIGHT => Piece::Knight,
                Self::BISHOP => Piece::Bishop,
                Self::ROOK => Piece::Rook,
                Self::QUEEN => Piece::Queen,
                Self::KING => Piece::King,
                _ => panic!("Invalid byte"),
            },
        ))
    }

    fn encode_byte(o: Option<(Color, Piece)>) -> u8 {
        if let Some((c, p)) = o {
            (match c {
                Color::White => Self::WHITE,
                Color::Black => Self::BLACK,
            }) | match p {
                Piece::King => Self::KING,
                Piece::Queen => Self::QUEEN,
                Piece::Rook => Self::ROOK,
                Piece::Bishop => Self::BISHOP,
                Piece::Knight => Self::KNIGHT,
                Piece::Pawn => Self::PAWN,
            }
        } else {
            0
        }
    }

    pub fn as_bitboard(&self) -> BitBoard {
        let mut res: BitBoard = Default::default();

        res.set_tempi(self.tempi());

        for sq in AllSquares {
            res.set_square(self.at(sq), sq)
        }

        for (c, cr) in [
            (Color::White, self.white_castle),
            (Color::Black, self.black_castle),
        ] {
            for s in [Side::Queens, Side::Kings] {
                res.set_castling_rook(c, s, cr.get(s))
            }
        }

        res.set_en_passant(self.en_passant_square());

        return res;
    }
}

impl Default for ByteBoard {
    fn default() -> Self {
        Self {
            board: [Default::default(); 64],
            tempi: Default::default(),
            black_castle: Default::default(),
            white_castle: Default::default(),
        }
    }
}

impl Board for ByteBoard {
    #[allow(non_snake_case)]
    fn standard() -> Self {
        let R = Self::WHITE | Self::ROOK;
        let N = Self::WHITE | Self::KNIGHT;
        let B = Self::WHITE | Self::BISHOP;
        let K = Self::WHITE | Self::KING;
        let Q = Self::WHITE | Self::QUEEN;
        let P = Self::WHITE | Self::PAWN;
        let r = Self::BLACK | Self::ROOK;
        let n = Self::BLACK | Self::KNIGHT;
        let b = Self::BLACK | Self::BISHOP;
        let k = Self::BLACK | Self::KING;
        let q = Self::BLACK | Self::QUEEN;
        let p = Self::BLACK | Self::PAWN;
        Self {
            board: [
                R, N, B, Q, K, B, N, R, //
                P, P, P, P, P, P, P, P, //
                0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, //
                p, p, p, p, p, p, p, p, //
                r, n, b, q, k, b, n, r, //
            ],
            tempi: Default::default(),
            white_castle: CastlingRights {
                queen_side: Some(Square::from_alg("a1")),
                king_side: Some(Square::from_alg("h1")),
            },
            black_castle: CastlingRights {
                queen_side: Some(Square::from_alg("a8")),
                king_side: Some(Square::from_alg("h8")),
            },
        }
    }

    fn find(&self, c: Color, p: Piece) -> Position {
        let mut res = Position::empty();

        for sq in AllSquares {
            if self.at(sq) == Some((c, p)) {
                res = res.with(sq.position());
            }
        }

        return res;
    }

    fn at(&self, n: Square) -> Option<(Color, Piece)> {
        Self::decode_byte(self.board[n.index()])
    }

    fn valid(&self) -> bool {
        todo!()
    }

    fn en_passant_square(&self) -> Option<Square> {
        AllSquares
            .into_iter()
            .find(|sq| self.board[sq.index()] == Self::EPS)
    }

    fn castling_possible(&self, c: Color, s: Side) -> bool {
        match c {
            Color::White => self.white_castle.get(s).is_some(),
            Color::Black => self.black_castle.get(s).is_some(),
        }
    }

    fn tempi(&self) -> Tempi {
        self.tempi
    }

    fn set_square(&mut self, p: Option<(Color, Piece)>, sq: Square) {
        self.board[sq.index()] = Self::encode_byte(p);
    }

    fn set_castling_rook(&mut self, c: Color, s: Side, sq: Option<Square>) {
        if let Some(sq) = sq {
            if self.at(sq) != Some((c, Piece::Rook)) {
                panic!("Not a valid castling rook")
            }
        }
        *(match c {
            Color::White => self.white_castle,
            Color::Black => self.black_castle,
        }
        .get_mut(s)) = sq;
    }

    fn set_en_passant(&mut self, sq: Option<Square>) {
        if let Some(sq) = self.en_passant_square() {
            self.board[sq.index()] = 0;
        }

        if let Some(sq) = sq {
            if self.at(sq) != None {
                panic!("Not a valid EPS square")
            }
            self.board[sq.index()] = Self::EPS;
        }
    }

    fn set_position(&mut self, p: Option<(Color, Piece)>, ps: Position) {
        for sq in ps {
            self.set_square(p, sq)
        }
    }

    fn set_tempi(&mut self, t: Tempi) {
        self.tempi = t;
    }
}

impl Zobristic for ByteBoard {
    fn zobrist(&self, zh: &ZobristHasher) -> u128 {
        let mut res =
            zh.hash_tempi(self.tempi()) ^ zh.hash_enpassant_square(self.en_passant_square());

        for c in [Color::White, Color::Black] {
            for s in [Side::Queens, Side::Kings] {
                if self.castling_possible(c, s) {
                    res ^= zh.hash_with_color(c, zh.hash_castling_right(s));
                }
            }
        }

        for sq in AllSquares {
            if let Some((c, p)) = self.at(sq) {
                res ^= zh.hash_with_color(c, zh.hash_square(p, sq));
            }
        }

        return res;
    }
}
