use std::{default, fmt::Display, str::FromStr};

use crate::shessboard::{
    castling::{CastlingInfo, CastlingSide},
    enums::{File, Piece, Rank},
    moves::Move,
    pieces,
    squares::Square,
};

mod fen;
mod uci;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Algebraic {
    Castling(CastlingSide),
    Normal(Normal),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Normal {
    pub piece: Piece,
    pub origin_rank: Option<Rank>,
    pub origin_file: Option<File>,
    pub destination: Square,
    pub promotion: Option<Piece>,
    pub capture: bool,
}

impl Algebraic {
    pub fn new(mv: &Move, legal_moves: &[Move]) -> Self {
        match mv.castling {
            Some(pm) => {
                if pm.positive() {
                    return Self::Castling(CastlingSide::OO);
                } else {
                    return Self::Castling(CastlingSide::OOO);
                }
            }
            None => {}
        }

        let mut res = Normal {
            piece: mv.color_and_piece.piece(),
            origin_rank: None,
            origin_file: None,
            promotion: mv.promotion,
            destination: mv.from_to.to,
            capture: mv.capture.is_some(),
        };

        if legal_moves
            .iter()
            .filter(|mv| Self::Normal(res).matches(mv))
            .count()
            > 1
        {
            res.origin_file = Some(mv.from_to.from.file());
        }

        if legal_moves
            .iter()
            .filter(|mv| Self::Normal(res).matches(mv))
            .count()
            > 1
        {
            res.origin_file = None;
            res.origin_rank = Some(mv.from_to.from.rank());
        }

        if legal_moves
            .iter()
            .filter(|mv| Self::Normal(res).matches(mv))
            .count()
            > 1
        {
            res.origin_file = Some(mv.from_to.from.file());
            res.origin_rank = Some(mv.from_to.from.rank());
        }

        Self::Normal(res)
    }

    pub fn find(self, mv: &[Move]) -> Vec<Move> {
        mv.into_iter()
            .filter_map(|m| if self.matches(m) { Some(*m) } else { None })
            .collect::<Vec<_>>()
    }

    pub fn matches(self, mv: &Move) -> bool {
        match self {
            Algebraic::Castling(castling_side) => {
                if let Some(pm) = mv.castling {
                    if pm.positive() {
                        castling_side == CastlingSide::OOO
                    } else {
                        castling_side == CastlingSide::OO
                    }
                } else {
                    false
                }
            }
            Algebraic::Normal(Normal {
                piece,
                origin_rank,
                origin_file,
                destination,
                promotion,
                capture,
            }) => {
                mv.color_and_piece.piece() == piece
                    && origin_file.unwrap_or(mv.from_to.from.file()) == mv.from_to.from.file()
                    && origin_rank.unwrap_or(mv.from_to.from.rank()) == mv.from_to.from.rank()
                    && destination == mv.from_to.to
                    && promotion == mv.promotion
                    && capture == mv.capture.is_some()
            }
        }
    }

    pub fn read(s: &str) -> Option<(Self, &str)> {
        if let Some((n, s)) = Self::read_pawn_move(s) {
            Some((Self::Normal(n), s))
        } else if let Some((n, s)) = Self::read_piece_move(s) {
            Some((Self::Normal(n), s))
        } else if let Some((c, s)) = Self::read_castling(s) {
            Some((Self::Castling(c), s))
        } else {
            None
        }
    }

    pub fn read_pawn_move(s: &str) -> Option<(Normal, &str)> {
        let (origin_file, s) = try_to(s, Self::read_pawn_capture_preamble);
        let (destination, s) = Square::read(s)?;
        let (promotion, s) = try_to(s, Self::read_pawn_promotion);
        Some((
            Normal {
                piece: Piece::Pawn,
                origin_rank: None,
                origin_file,
                destination,
                promotion,
                capture: origin_file.is_some(),
            },
            s,
        ))
    }

    pub fn read_pawn_capture_preamble(s: &str) -> Option<(File, &str)> {
        let (f, s) = File::read(s)?;
        let s = skip_char('x', s)?.1;
        Some((f, s))
    }

    pub fn read_pawn_promotion(s: &str) -> Option<(Piece, &str)> {
        let s = skip_char('=', s)?.1;
        let (p, s) = Piece::read(s, false)?;
        Some((p, s))
    }

    pub fn read_piece_move(s: &str) -> Option<(Normal, &str)> {
        let (piece, s) = Piece::read(s, false)?;
        if piece == Piece::Pawn {
            return None;
        }
        let checkpoint = s;
        let (mut origin_file, s) = try_to(s, File::read);
        let (mut origin_rank, s) = try_to(s, Rank::read);

        let (mut cap_dest, mut s) = try_to(s, Self::read_piece_destination);

        let ((capture, destination), s) = if let Some(cap_dest) = cap_dest {
            (cap_dest, s)
        } else {
            origin_file = None;
            origin_rank = None;
            Self::read_piece_destination(checkpoint)?
        };

        Some((
            Normal {
                piece,
                origin_rank,
                origin_file,
                destination,
                promotion: None,
                capture,
            },
            s,
        ))
    }

    pub fn read_castling(s: &str) -> Option<(CastlingSide, &str)> {
        let (o1, s) = try_to(s, skip_o);
        let o1 = o1.is_some();
        let (_, s) = try_to(s, skip_dash);
        let (o2, s) = try_to(s, skip_o);
        let o2 = o2.is_some();
        let (_, s) = try_to(s, skip_dash);
        let (o3, s) = try_to(s, skip_o);
        let o3 = o3.is_some();

        return Some((
            match o1 as i32 + o2 as i32 + o3 as i32 {
                2 => CastlingSide::OO,
                3 => CastlingSide::OOO,
                _ => return None,
            },
            s,
        ));

        fn skip_o(s: &str) -> Option<(char, &str)> {
            skip_any_char(&['o', 'O', '0'], s)
        }

        fn skip_dash(s: &str) -> Option<((), &str)> {
            skip_char('-', s)
        }
    }

    pub fn read_piece_destination(s: &str) -> Option<((bool, Square), &str)> {
        let (capture, s) = try_to(s, |s| skip_char('x', s));
        let capture = capture.is_some();
        let (destination, s) = Square::read(s)?;
        Some(((capture, destination), s))
    }
}

impl Display for Algebraic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Algebraic::Castling(CastlingSide::OOO) => write!(f, "O-O-O")?,
            Algebraic::Castling(CastlingSide::OO) => write!(f, "O-O")?,
            Algebraic::Normal(Normal {
                piece,
                origin_rank,
                origin_file,
                destination,
                capture,
                promotion,
            }) => {
                if piece != &Piece::Pawn {
                    write!(f, "{}", piece.white_letter())?;
                }
                if let Some(file) = origin_file {
                    write!(f, "{}", file.as_char())?;
                }
                if let Some(rank) = origin_rank {
                    write!(f, "{}", rank.as_char())?;
                }
                if *capture {
                    write!(f, "x")?;
                }
                write!(f, "{}", destination)?;
                if let Some(p) = promotion {
                    write!(f, "={}", p.white_letter())?;
                }
            }
        }

        Ok(())
    }
}

impl FromStr for Algebraic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, "")) = Self::read(s) {
            Ok(a)
        } else {
            Err(())
        }
    }
}

fn try_to<F, T>(s: &str, mut f: F) -> (Option<T>, &str)
where
    F: FnMut(&str) -> Option<(T, &str)>,
{
    if let Some((t, s)) = f(s) {
        (Some(t), s)
    } else {
        (None, s)
    }
}

fn skip_char(c: char, s: &str) -> Option<((), &str)> {
    let mut cs = s.chars();
    if c == cs.next()? {
        Some(((), cs.as_str()))
    } else {
        None
    }
}

fn skip_any_char<'a>(a: &[char], s: &'a str) -> Option<(char, &'a str)> {
    let mut cs = s.chars();
    let c = cs.next()?;
    if a.contains(&c) {
        Some((c, cs.as_str()))
    } else {
        None
    }
}

fn many<F, T>(mut s: &str, mut f: F) -> (Vec<T>, &str)
where
    F: FnMut(&str) -> Option<(T, &str)>,
{
    let mut res = vec![];
    while let Some((t, ss)) = f(s) {
        res.push(t);
        s = ss;
    }
    (res, s)
}

fn some<F, T>(s: &str, f: F) -> Option<(Vec<T>, &str)>
where
    F: FnMut(&str) -> Option<(T, &str)>,
{
    let (res, s) = many(s, f);
    if res.len() == 0 { None } else { Some((res, s)) }
}
