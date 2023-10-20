use crate::{fen::XFen, moves::GeneralMove, validity::Validity};

pub trait Board {
    fn new(fen: &XFen) -> Self;

    fn valid_move(&self, mv: GeneralMove) -> Validity;
}
