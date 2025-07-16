use crate::bitboard::{
    CastlingInfo,
    castling::CastlingSide,
    enums::{Color, Piece, Rank},
    pieces::pawns::Pawns,
    squares::Square,
};

pub struct ProtoMove {
    pub from: Square,
    pub to: Square,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub color: Color,
    pub piece: Piece,
    pub from: Square,
    pub to: Square,
    pub castling: Option<CastlingSide>,
    pub capture: Option<(Square, Piece)>,
    pub en_passant_skipped: bool,
    pub promotion: Option<Piece>,
}

impl Move {
    pub fn is_en_passant_capture(&self) -> bool {
        self.capture.map_or(false, |(sq, p)| {
            self.piece == Piece::Pawn && p == Piece::Pawn && sq != self.to
        })
    }

    pub fn en_passant_square(&self) -> Option<Square> {
        if self.piece != Piece::Pawn {
            return None;
        }

        if self.color == Color::White {
            if let ((f, Rank::_2), (_, Rank::_4)) = (self.from.algebraic(), self.to.algebraic()) {
                return Some(Square::at(f, Rank::_3));
            } else {
                return None;
            };
        } else {
            if let ((f, Rank::_7), (_, Rank::_5)) = (self.from.algebraic(), self.to.algebraic()) {
                return Some(Square::at(f, Rank::_6));
            } else {
                return None;
            };
        }
    }
}

#[test]
fn size_fuckery() {
    println!("{}", std::mem::size_of::<Move>());
}
