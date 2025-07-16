use crate::bitboard::{
    BitBoard, CastlingInfo,
    castling::CastlingSide,
    enums::{Color, ColorPiece, Dir, Piece, Rank},
    masks::Mask,
    pieces::{
        knights::{self, Knights},
        pawns::Pawns,
    },
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ProtoMove {
    pub from: Square,
    pub to: Square,
}

impl ProtoMove {
    pub fn dist(&self) -> i8 {
        self.from.dist(self.to)
    }

    pub fn dir(&self) -> Option<Dir> {
        todo!()
    }

    pub fn as_mask(&self) -> Mask {
        Mask::nil().set(self.from).set(self.to)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub color_and_piece: ColorPiece,
    pub from_to: ProtoMove,
    pub castling: Option<CastlingSide>,
    pub capture: Option<(Square, Piece)>,
    pub en_passant_last_turn: bool,
    pub promotion: Option<Piece>,
}

impl Move {
    pub fn en_passant_square(&self) -> Option<Square> {
        if self.color_and_piece == ColorPiece::WhitePawn {
            if let ((f, Rank::_2), (_, Rank::_4)) =
                (self.from_to.from.algebraic(), self.from_to.to.algebraic())
            {
                return Some(Square::at(f, Rank::_3));
            } else {
                return None;
            };
        } else if self.color_and_piece == ColorPiece::BlackPawn {
            if let ((f, Rank::_7), (_, Rank::_5)) =
                (self.from_to.from.algebraic(), self.from_to.to.algebraic())
            {
                return Some(Square::at(f, Rank::_6));
            } else {
                return None;
            };
        } else {
            return None;
        }
    }
}

#[test]
fn size_fuckery() {
    assert_eq!(std::mem::size_of::<Move>(), 8);
    assert_eq!(std::mem::size_of::<Option<Move>>(), 8);
}

impl BitBoard {
    pub fn apply(&mut self, mv: &Move) -> bool {
        todo!()
    }

    pub fn generate_moves(&self, res: &mut Vec<Move>) {
        self.generate_knight_moves(res);
    }

    pub fn generate_knight_moves(&self, res: &mut Vec<Move>) {
        let color_and_piece = ColorPiece::new(self.metadata.to_move, Piece::Knight);

        let active = self.active();
        let passive = self.passive();

        let excluded = active.as_mask();

        let knights = active.knights.as_mask();

        if !knights.any() {
            return;
        }

        for from in knights {
            let possible = Knights::MOVES.at(from) & !excluded;

            for to in possible {
                let from_to = ProtoMove { from, to };
                let capture = passive.piece(from);

                res.push(Move {
                    color_and_piece,
                    from_to,
                    castling: None,
                    capture: capture.map(|p| (to, p)),
                    en_passant_last_turn: self.metadata.en_passant.is_some(),
                    promotion: None,
                });
            }
        }
    }
}
