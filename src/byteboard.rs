use crate::{
    board::{Board, Validity},
    elements::Piece,
    fen::XFen,
    moves::GeneralMove,
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
        todo!()
    }
}
