use std::ops::{Add, AddAssign};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Validity {
    #[default]
    ProbablyValid,
    DefinitelyInvalid(&'static str),
}

use Validity::*;

impl Validity {
    pub fn assert(self) {
        if let DefinitelyInvalid(msg) = self {
            panic!("Validation failed: {}", msg);
        }
    }

    pub fn explain(self, msg: &'static str) -> Validity {
        if let DefinitelyInvalid("") = self {
            DefinitelyInvalid(msg)
        } else {
            self
        }
    }
}

impl Add<Validity> for Validity {
    type Output = Validity;

    fn add(self, rhs: Validity) -> Self::Output {
        if let DefinitelyInvalid(s) = self {
            self
        } else {
            rhs
        }
    }
}

impl AddAssign<Validity> for Validity {
    fn add_assign(&mut self, rhs: Validity) {
        *self = *self + rhs;
    }
}

pub trait Validatable {
    fn valid(&self) -> Validity;
}

pub trait Validator<T> {
    fn validate(&self, it: &T) -> Validity;
}

impl Validatable for bool {
    fn valid(&self) -> Validity {
        if *self {
            ProbablyValid
        } else {
            DefinitelyInvalid("")
        }
    }
}
