pub type CastlingRights = CastlingInfo<bool>;

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
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
    pub fn any(&self) -> bool {
        self.ooo || self.oo
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CastlingSide {
    OOO = 1,
    OO,
}
