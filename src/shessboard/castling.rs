use crate::shessboard::{enums::File, moves::ProtoMove};

pub type CastlingRights = CastlingInfo<bool>;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct CastlingInfo<T> {
    pub ooo: T,
    pub oo: T,
}

impl<T> CastlingInfo<T> {
    fn side(&self, c: CastlingSide) -> &T {
        match c {
            CastlingSide::OOO => &self.ooo,
            CastlingSide::OO => &self.oo,
        }
    }
}

impl CastlingRights {
    pub fn new() -> Self {
        Self {
            ooo: true,
            oo: true,
        }
    }

    pub fn update(&mut self, cr: CastlingRights) {
        self.ooo &= cr.ooo;
        self.oo &= cr.oo;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CastlingSide {
    OOO = 1,
    OO,
}
