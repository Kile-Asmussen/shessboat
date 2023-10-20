use std::ops::Add;

use crate::{fen::XFen, moves::GeneralMove};

pub trait Board {
    fn new(fen: &XFen) -> Self;

    fn valid_move(&self, mv: GeneralMove) -> Validity;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Validity {
    #[default]
    ProbablyValid,
    DefinitelyInvalid,
}

impl Add<Validity> for Validity {
    type Output = Validity;

    fn add(self, rhs: Validity) -> Self::Output {
        if self == Self::DefinitelyInvalid || rhs == Self::DefinitelyInvalid {
            Self::DefinitelyInvalid
        } else {
            Self::ProbablyValid
        }
    }
}
