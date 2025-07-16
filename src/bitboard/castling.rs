pub type CastlingRights = CastlingInfo<CastlingRight>;

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct CastlingInfo<T> {
    pub ooo: T,
    pub oo: T,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CastlingRight {
    Retained = 1,
    Forefeited,
    Claimed,
}

impl<T> CastlingInfo<T> {
    fn side(&self, c: CastlingSide) -> &T {
        match c {
            CastlingSide::OOO => &self.ooo,
            CastlingSide::OO => &self.oo,
        }
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self {
            ooo: CastlingRight::Retained,
            oo: CastlingRight::Retained,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CastlingSide {
    OOO = 1,
    OO,
}
