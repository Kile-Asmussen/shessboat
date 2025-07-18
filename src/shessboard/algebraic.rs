use std::{default, fmt::Display, str::FromStr};

use regex::Regex;

use crate::shessboard::{
    castling::CastlingSide,
    enums::{File, Piece, Rank},
    moves::Move,
    pieces,
    squares::Square,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Notation {
    Castling(CastlingSide),
    Normal(Normal),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Normal {
    piece: Piece,
    origin_rank: Option<Rank>,
    origin_file: Option<File>,
    destination: Square,
    capture: bool,
}

impl Notation {
    pub fn disambiguate(mv: Move, legal_moves: &[Move]) -> Self {
        match mv.castling {
            Some(c) => return Self::Castling(c),
            None => {}
        }

        let mut res = Normal {
            piece: mv.color_and_piece.piece(),
            origin_rank: None,
            origin_file: None,
            destination: mv.from_to.to,
            capture: mv.capture.is_some(),
        };

        if legal_moves.iter().filter(|mv| Self::Normal(res).matches(mv)).count() > 1 {
            res.origin_file = Some(mv.from_to.from.file());
        }

        if legal_moves.iter().filter(|mv| Self::Normal(res).matches(mv)).count() > 1 {
            res.origin_file = None;
            res.origin_rank = Some(mv.from_to.from.rank());
        }

        if legal_moves.iter().filter(|mv| Self::Normal(res).matches(mv)).count() > 1 {
            res.origin_file = Some(mv.from_to.from.file());
            res.origin_rank = Some(mv.from_to.from.rank());
        }

        Self::Normal(res)
    }

    pub fn matches(self, mv: &Move) -> bool {
        match self {
            Notation::Castling(castling_side) => Some(castling_side) == mv.castling,
            Notation::Normal(Normal {
                piece,
                origin_rank,
                origin_file,
                destination,
                capture,
            }) => {
                mv.color_and_piece.piece() == piece
                    && origin_file.unwrap_or(mv.from_to.from.file()) == mv.from_to.from.file()
                    && origin_rank.unwrap_or(mv.from_to.from.rank()) == mv.from_to.from.rank()
                    && destination == mv.from_to.to
                    && capture == mv.capture.is_some()
            }
        }
    }

    pub fn read(s: &str) -> Option<Self> {
        let pawn_move = Regex::new(r"\A([abcdefgh][12345678])").ok()?;
        if let Some(pawn_move) = pawn_move.captures(s) {
            return Some(Notation::Normal(Normal {
                piece: Piece::Pawn,
                origin_rank: None,
                origin_file: None,
                destination: Self::read_square(pawn_move.get(1)?.as_str())?,
                capture: false,
            }));
        }

        let pawn_capture = Regex::new(r"\A([abcdefgh])x([abcdefgh][12345678])").ok()?;
        if let Some(pawn_capture) = pawn_capture.captures(s) {
            return Some(Notation::Normal(Normal {
                piece: Piece::Pawn,
                origin_rank: None,
                origin_file: Some(Self::read_file(pawn_capture.get(1)?.as_str())?),
                destination: Self::read_square(pawn_capture.get(2)?.as_str())?,
                capture: true,
            }));
        }

        let piece_move =
            Regex::new(r"\A([BNKQR])([abcdefgh]?)([12345678]?)([abcdefgh][12345678])").ok()?;
        if let Some(piece_move) = piece_move.captures(s) {
            return Some(Notation::Normal(Normal {
                piece: Self::read_piece(piece_move.get(1)?.as_str())?,
                origin_rank: Self::read_rank(piece_move.get(2)?.as_str()),
                origin_file: Self::read_file(piece_move.get(3)?.as_str()),
                destination: Self::read_square(piece_move.get(4)?.as_str())?,
                capture: false,
            }));
        }

        let piece_capture =
            Regex::new(r"\A([BNKQR])([abcdefgh]?)([12345678]?)x([abcdefgh][12345678])").ok()?;
        if let Some(piece_capture) = piece_capture.captures(s) {
            return Some(Notation::Normal(Normal {
                piece: Self::read_piece(piece_capture.get(1)?.as_str())?,
                origin_rank: Self::read_rank(piece_capture.get(2)?.as_str()),
                origin_file: Self::read_file(piece_capture.get(3)?.as_str()),
                destination: Self::read_square(piece_capture.get(4)?.as_str())?,
                capture: true,
            }));
        }

        let castling = Regex::new(r"\AO-O-O|\AO-O").ok()?;
        if let Some(castling) = castling.find(s) {
            return Some(Notation::Castling(match castling.as_str() {
                "O-O-O" => CastlingSide::OOO,
                "O-O" => CastlingSide::OO,
                _ => return None,
            }));
        }

        return None;
    }

    pub fn read_square(s: &str) -> Option<Square> {
        let mut cs = s.chars();
        Some(Square::at(
            File::from_char(cs.next()?)?,
            Rank::from_char(cs.next()?)?,
        ))
    }

    pub fn read_file(s: &str) -> Option<File> {
        let mut cs = s.chars();
        File::from_char(cs.next()?)
    }

    pub fn read_rank(s: &str) -> Option<Rank> {
        let mut cs = s.chars();
        Rank::from_char(cs.next()?)
    }

    pub fn read_piece(s: &str) -> Option<Piece> {
        let mut cs = s.chars();
        Some(match cs.next()? {
            'K' => Piece::King,
            'Q' => Piece::Queen,
            'R' => Piece::Rook,
            'B' => Piece::Bishop,
            'N' => Piece::Knight,
            _ => return None,
        })
    }
}

impl Display for Notation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Notation::Castling(CastlingSide::OOO) => write!(f, "O-O-O")?,
            Notation::Castling(CastlingSide::OO) => write!(f, "O-O")?,
            Notation::Normal(Normal {
                piece,
                origin_rank,
                origin_file,
                destination,
                capture,
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
            }
        }

        Ok(())
    }
}

impl FromStr for Notation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Notation::read(s).ok_or(())
    }
}
