use crate::{
    bitboard::CastlingRights,
    board::{Board, Side},
    pieces::{self, Color, Piece},
    squares::{AllSquares, Position, Square},
};

struct ByteBoard {
    board: [u8; 64],
    pub tempo: u32,
    pub last_advance: u32,
}

impl ByteBoard {
    const PAWN: u8 = 0x1;
    const KNIGHT: u8 = 0x2;
    const BISHOP: u8 = 0x3;
    const ROOK: u8 = 0x4;
    const QUEEN: u8 = 0x5;
    const KING: u8 = 0x6;

    const PIECE_MASK: u8 = 0xF;

    const WHITE: u8 = 0x10;
    const BLACK: u8 = 0x20;
    const COLOR_MASK: u8 = 0x30;

    const SPECIAL: u8 = 0x40;

    fn decode_byte(b: u8) -> Option<(Color, Piece)> {
        if b & !Self::SPECIAL == 0 {
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
                _ => panioc!("Invalid byte"),
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
}

impl Board for ByteBoard {
    fn standard() -> Self {
        let R = Self::WHITE | Self::ROOK | Self::SPECIAL;
        let N = Self::WHITE | Self::KNIGHT;
        let B = Self::WHITE | Self::BISHOP;
        let K = Self::WHITE | Self::KING;
        let Q = Self::WHITE | Self::QUEEN;
        let P = Self::WHITE | Self::PAWN;
        let r = Self::WHITE | Self::ROOK | Self::SPECIAL;
        let n = Self::WHITE | Self::KNIGHT;
        let b = Self::WHITE | Self::BISHOP;
        let k = Self::WHITE | Self::KING;
        let q = Self::WHITE | Self::QUEEN;
        let p = Self::WHITE | Self::PAWN;
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
        }
    }

    pub fn get(&self, c: Color, p: Piece) -> Position {
        let mut res = Position::empty();

        for sq in AllSquares {
            if self.at(sq) == Some((c, p)) {
                res = res.with(sq.position());
            }
        }

        return res;
    }

    pub fn at(&self, n: Square) -> Option<(Color, Piece)> {
        Self::decode_byte(self.board[n.index()])
    }

    pub fn valid(&self) -> bool {
        todo!()
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

    pub fn tempo_clock(&self) -> usize {
        self.last_advance as usize
    }

    pub fn en_passant_square(&self) -> Option<Square> {
        for sq in AllSquares {
            if self.board[sq.index()] == Self::SPECIAL {
                return Some(sq);
            }
        }
        None
    }

    pub fn castling_possible(&self, c: Color, s: Side) -> bool {
        todo!()
    }
}
