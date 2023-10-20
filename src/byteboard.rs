use crate::{
    board::{Board, Validity},
    elements::Piece,
    fen::XFen,
    moves::{CastlingMove, EnPassantCapture, GeneralMove, PawnPromotion, StandardMove},
};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Byteboard([Option<Piece>; 64]);

impl Default for Byteboard {
    fn default() -> Self {
        Self([None; 64])
    }
}

impl Board for Byteboard {
    fn new(fen: &XFen) -> Self {
        let mut res: Byteboard = Default::default();
        let mut i = 0;

        for rank in fen.board.iter().rev() {
            for square in rank {
                res.0[i] = *square;
                i += 1;
            }
        }

        res
    }

    fn valid_move(&self, mv: GeneralMove) -> Validity {
        match mv {
            GeneralMove::Castling(c) => self.valid_castling(c),
            GeneralMove::EnPassant(e) => self.valid_enpassant(e),
            GeneralMove::Promotion(p) => self.valid_promotion(p),
            GeneralMove::Standard(s) => self.valid_standard_move(s),
        }
    }
}

impl Byteboard {
    fn valid_castling(&self, mv: CastlingMove) -> Validity {
        Validity::ProbablyValid
    }

    fn valid_enpassant(&self, mv: EnPassantCapture) -> Validity {
        Validity::ProbablyValid
    }

    fn valid_promotion(&self, mv: PawnPromotion) -> Validity {
        Validity::ProbablyValid
    }

    fn valid_standard_move(&self, mv: StandardMove) -> Validity {
        Validity::ProbablyValid
    }
}
