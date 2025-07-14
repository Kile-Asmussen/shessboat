use std::ops::Index;

use rand::Fill;

use crate::bitboard::{masks::Mask, squares::Square};

#[derive(Debug)]
pub struct BoardMap<T>([T; 64]);

impl<T: Sized + Copy> BoardMap<T> {
    pub const fn new(it: [T; 64]) -> Self {
        Self(it)
    }

    pub const fn at(&self, sq: Square) -> T {
        self.0[sq.index() as usize]
    }

    pub const fn set(&mut self, sq: Square, it: T) -> &mut Self {
        self.0[sq.index() as usize] = it;
        self
    }
}

impl BoardMap<Mask> {
    pub fn overlap(&self, mask: Mask) -> Mask {
        mask.iter().map(|sq| self.at(sq)).sum()
    }
}

impl<T: Sized + Copy> IntoIterator for BoardMap<T> {
    type Item = T;

    type IntoIter = <[T; 64] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Fill for BoardMap<T>
where
    [T]: Fill,
{
    fn fill<R: rand::Rng + ?Sized>(&mut self, rng: &mut R) {
        for i in 0..64 {
            Fill::fill(&mut self.0, rng);
        }
    }
}

impl<T: Default> Default for BoardMap<T> {
    fn default() -> Self {
        Self(std::array::from_fn(|_| T::default()))
    }
}
