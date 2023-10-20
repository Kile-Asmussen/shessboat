use std::{default, fmt::Display, iter::Sum};

use crate::{
    castling::CastlingRights,
    elements::{Piece, PieceColor, Square},
};

fn parse_rank(rank: &str) -> Result<[Option<Piece>; 8], &str> {
    const RANK_CHARS: [char; 20] = [
        '1', '2', '3', '4', '5', '6', '7', '8', //
        'p', 'n', 'b', 'r', 'q', 'k', //
        'P', 'N', 'B', 'R', 'Q', 'K', //
    ];

    if rank.chars().all(|c| RANK_CHARS.contains(&c)) {
        return Err("rank contains illegal pieces");
    }

    if rank.chars().map(squares_width).sum::<usize>() != 8 {
        return Err("rank is not 8 files");
    }

    let mut res = [None; 8];
    let mut i = 0usize;

    for c in rank.chars() {
        if let p @ Some(_) = Piece::from_letter(c) {
            res[i] = p;
            i += 1;
        } else {
            i += squares_width(c);
        }
    }

    return Ok(res);

    fn squares_width(c: char) -> usize {
        match c {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            _ => 1,
        }
    }
}

fn parse_board(board: &str) -> Result<[[Option<Piece>; 8]; 8], &str> {
    let ranks = board.split('/').collect::<Vec<&str>>();

    if ranks.len() != 8 {
        return Err("board is not 8 ranks");
    }

    let mut res: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

    let mut i = 0usize;
    for rank in ranks {
        res[i] = parse_rank(rank)?
    }

    return Ok(res);
}

fn parse_to_move(turn: &str) -> Result<PieceColor, &str> {
    let Some(c) = turn.chars().next() else {
        return Err("turn marker wrong length");
    };

    PieceColor::from_letter(c).ok_or("turn marker contains invalid characters")
}

fn parse_castling(castling: &str) -> Result<Vec<Square>, &str> {
    const CASTLING_CHARS: [char; 21] = [
        'K', 'Q', 'k', 'q', '-', //
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', //
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', //
    ];

    if castling.chars().all(|c| CASTLING_CHARS.contains(&c)) {
        return Err("castling rights contains illegal pieces");
    }

    let 1..=4 = castling.chars().count() else {
        return Err("castling rights has incorrect number of characters");
    };

    let mut res = Vec::new();

    for c in castling.chars() {
        let c = match c {
            'K' => 'H',
            'k' => 'h',
            'Q' => 'A',
            'q' => 'a',
            c => c,
        };
        let r = if c.is_ascii_uppercase() { b'1' } else { b'8' };
        let f = c.to_ascii_lowercase() as u8;
        let fr = [f, r];
        let alg = std::str::from_utf8(&fr[..]).expect("invalid utf composed from parts");
        let sq = Square::from_algebraic(alg).expect("invalid square composed from parts");
        res.push(sq);
    }

    Ok(res)
}

fn parse_en_passant_square(eps: &str) -> Result<Option<Square>, &str> {
    if eps == "-" {
        Ok(None)
    } else if let res @ Some(_) = Square::from_algebraic(eps) {
        Ok(res)
    } else {
        Err("invalid en passant square")
    }
}

fn parse_turn_counter(int: &str) -> Result<usize, &str> {
    int.parse::<usize>().map_err(|_| "invalid turn number")
}

fn parse_tempo_clock(int: &str) -> Result<usize, &str> {
    int.parse::<usize>()
        .map_err(|_| "invalid tempo clock number")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XFen {
    pub board: [[Option<Piece>; 8]; 8],
    pub to_move: PieceColor,
    pub castling: Vec<Square>,
    pub eps_square: Option<Square>,
    pub turn: usize,
    pub tempo_clock: usize,
}

impl Default for XFen {
    fn default() -> Self {
        Self {
            board: Default::default(),
            to_move: PieceColor::White,
            castling: Default::default(),
            eps_square: Default::default(),
            turn: Default::default(),
            tempo_clock: Default::default(),
        }
    }
}

fn parse_xfen(fen: &str) -> Result<XFen, &str> {
    let chunks = fen.split_ascii_whitespace().collect::<Vec<&str>>();

    let [board, to_move, castling, eps, turn, clock] = chunks[..] else {
        return Err("FEN string must have six parts");
    };

    Ok(XFen {
        board: parse_board(board)?,
        to_move: parse_to_move(to_move)?,
        castling: parse_castling(castling)?,
        eps_square: parse_en_passant_square(eps)?,
        turn: parse_turn_counter(turn)?,
        tempo_clock: parse_tempo_clock(clock)?,
    })
}

impl ToString for XFen {
    fn to_string(&self) -> String {
        let mut res = String::new();

        let mut first = true;

        for rank in self.board {
            let mut rlee = 0;

            if first {
                first = false;
            } else {
                res.push('/')
            }

            for square in rank {
                if let Some(p) = square {
                    if rlee != 0 {
                        res.push((b'0' + rlee) as char);
                        rlee = 0;
                    }
                    res.push(p.letter())
                } else {
                    rlee += 1;
                }
            }
            if rlee != 0 {
                res.push((b'0' + rlee) as char);
                rlee = 0;
            }
        }

        res.push(' ');

        res.push(self.to_move.letter());

        res.push(' ');

        if self
            .castling
            .iter()
            .all(|sq| sq.file() == 'a' || sq.file() == 'h')
        {
            for sq in &self.castling {
                let p = if sq.file() == 'a' { 'q' } else { 'k' };
                res.push(if sq.rank() == 1 {
                    p.to_ascii_uppercase()
                } else {
                    p
                });
            }
        } else {
            for sq in &self.castling {
                res.push(if sq.rank() == 1 {
                    sq.file().to_ascii_uppercase()
                } else {
                    sq.file()
                });
            }
        }

        res.push(' ');

        res.push_str(self.eps_square.map_or("-", Square::algebraic));

        res.push(' ');

        res.push_str(&self.turn.to_string());

        res.push(' ');

        res.push_str(&self.tempo_clock.to_string());

        return res;
    }
}
