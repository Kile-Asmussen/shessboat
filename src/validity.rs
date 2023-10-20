#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Validity {
    #[default]
    ProbablyValid,
    DefinitelyInvalid,
}

impl Validity {
    fn assert(self) {
        assert_eq!(self, Validity::ProbablyValid);
    }
}

impl From<bool> for Validity {
    fn from(value: bool) -> Self {
        use Validity::*;
        if value {
            ProbablyValid
        } else {
            DefinitelyInvalid
        }
    }
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
