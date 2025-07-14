use std::ops::Index;

use rand::Fill;

use crate::bitboard::{
    masks::{Mask, SquareIter},
    squares::Square,
};

#[derive(Debug)]
pub struct BoardMap<T>([T; 64]);

impl<T: Sized + Copy> BoardMap<T> {
    pub const fn new(it: [T; 64]) -> Self {
        Self(it)
    }

    pub const fn new_with(it: T) -> Self
    where
        T: Copy,
    {
        Self([it; 64])
    }

    pub const fn at(&self, sq: Square) -> T {
        self.0[sq.index() as usize]
    }

    pub const fn set(&mut self, sq: Square, it: T) -> &mut Self {
        self.0[sq.index() as usize] = it;
        self
    }

    pub const fn iter(&self) -> BoardMapIter<T>
    where
        T: Copy + Sized,
    {
        BoardMapIter(Mask::full().iter(), self)
    }
}

impl BoardMap<Mask> {
    pub fn overlap(&self, mask: Mask) -> Mask {
        mask.iter().map(|sq| self.at(sq)).sum()
    }
}

impl BoardMap<char> {
    pub const fn to_mask(&self, c: char) -> Mask {
        let mut res = Mask::nil();
        let mut it = self.iter();
        while let Some((sq, x)) = it.next() {
            if c == x {
                res.set(sq);
            }
        }
        res
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

impl<'a, T: Sized + Copy> IntoIterator for &'a BoardMap<T> {
    type Item = (Square, T);

    type IntoIter = BoardMapIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct BoardMapIter<'a, T>(SquareIter, &'a BoardMap<T>);

impl<'a, T: Sized + Copy> BoardMapIter<'a, T> {
    pub const fn next(&mut self) -> Option<(Square, T)> {
        let Some(sq) = self.0.next() else {
            return None;
        };

        Some((sq, self.1.at(sq)))
    }
}

impl<'a, T: Sized + Copy> Iterator for BoardMapIter<'a, T> {
    type Item = (Square, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
