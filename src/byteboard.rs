use std::ops::{Index, IndexMut};

use crate::{
    board::Board,
    elements::{Piece, Square},
    moves::{CastlingMove, MoveValidator, PawnMove, PieceMove},
    sfen::SFen,
    validity::{Validatable, Validator, Validity},
};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Byteboard([Option<Piece>; 64]);

impl Index<Square> for Byteboard {
    type Output = Option<Piece>;

    fn index(&self, index: Square) -> &Self::Output {
        &self.0[index.index()]
    }
}

impl IndexMut<Square> for Byteboard {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self.0[index.index()]
    }
}

impl Default for Byteboard {
    fn default() -> Self {
        Self([None; 64])
    }
}

impl Board for Byteboard {
    fn new(fen: &SFen) -> Self {
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
}

impl Validatable for Byteboard {
    fn valid(&self) -> Validity {
        Default::default()
    }
}

impl MoveValidator for Byteboard {}

impl Validator<PawnMove> for Byteboard {
    fn validate(&self, it: &PawnMove) -> Validity {
        todo!()
    }
}

impl Validator<PieceMove> for Byteboard {
    fn validate(&self, it: &PieceMove) -> Validity {
        todo!()
    }
}

impl Validator<CastlingMove> for Byteboard {
    fn validate(&self, it: &CastlingMove) -> Validity {
        todo!()
    }
}
