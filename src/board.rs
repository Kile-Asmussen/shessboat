use crate::fen::XFen;

pub trait Board {
    fn new(fen: &XFen) -> Self;

    fn valid(&self, mv: Move) -> Validity;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Validity {
    DefinitelyInvalid,
    ProbablyValid,
}
