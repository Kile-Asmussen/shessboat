use crate::{
    moves::GeneralMove,
    sfen::SFen,
    validity::{Validatable, Validator},
};

pub trait Board: Validator<GeneralMove> + Validatable {
    fn new(fen: &SFen) -> Self;
}
