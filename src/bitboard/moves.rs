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
        let mut res = Mask::nil();
        res.set(self.from).set(self.to);
        res
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Move {
    pub color_and_piece: Option<ColorPiece>,
    pub from_to: Option<ProtoMove>,
    pub castling: Option<CastlingSide>,
    pub capture: Option<(Square, Piece)>,
    pub en_passant_last_turn: bool,
    pub promotion: Option<Piece>,
}

impl Move {
    pub fn en_passant_square(&self) -> Option<Square> {
        let Some(from_to) = self.from_to else {
            return None;
        };
        if self.color_and_piece == Some(ColorPiece::WhitePawn) {
            if let ((f, Rank::_2), (_, Rank::_4)) =
                (from_to.from.algebraic(), from_to.to.algebraic())
            {
                return Some(Square::at(f, Rank::_3));
            } else {
                return None;
            };
        } else if self.color_and_piece == Some(ColorPiece::BlackPawn) {
            if let ((f, Rank::_7), (_, Rank::_5)) =
                (from_to.from.algebraic(), from_to.to.algebraic())
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
    fn annotate(&self, res: &mut Move) {
        let Some(from_to) = res.from_to else {
            return;
        };

        if res.color_and_piece.is_none() {
            let color = self.metadata.to_move;
            let piece = self.active().piece(from_to.from);

            if let Some(piece) = self.active().piece(from_to.from) {
                res.color_and_piece = Some(ColorPiece::new(color, piece))
            }
        }

        if let Some(ColorPiece::WhiteKing | ColorPiece::BlackKing) = res.color_and_piece {
            if from_to.dist() == 2 {
                todo!()
            }
        }
    }

    fn apply(&mut self, mv: &Move) -> bool {
        todo!()
    }

    fn generate_moves(&self, res: &mut Vec<Move>) {
        self.generate_knight_moves(res);
    }

    fn generate_knight_moves(&self, res: &mut Vec<Move>) {
        let color_and_piece = Some(ColorPiece::new(self.metadata.to_move, Piece::Knight));

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
                let capture = passive.piece(from);

                res.push(Move {
                    color_and_piece,
                    from_to: Some(ProtoMove { from, to }),
                    castling: None,
                    capture: capture.map(|p| (to, p)),
                    en_passant_last_turn: self.metadata.en_passant.is_some(),
                    promotion: None,
                });
            }
        }
    }
}
