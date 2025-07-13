use crate::bitboard::{masks::Mask, squares::Square};

pub struct MoveDb<T>([T; 64]);

impl<T: Sized + Copy> MoveDb<T> {
    pub const fn new(it: [T; 64]) -> Self {
        Self(it)
    }

    pub const fn at(&self, sq: Square) -> T {
        self.0[sq.index() as usize]
    }
}

impl<T: Sized + Copy> IntoIterator for MoveDb<T> {
    type Item = T;

    type IntoIter = <[T; 64] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
