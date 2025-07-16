use std::ops::Index;

use rand::Fill;

use crate::bitboard::{
    enums::{ColorPiece, File, Rank},
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
    pub fn overlay(&self, mask: Mask) -> Mask {
        mask.iter().map(|sq| self.at(sq)).sum()
    }
}

impl BoardMap<bool> {
    pub const fn to_mask(&self) -> Mask {
        let mut res = Mask::nil();
        let mut it = self.iter();
        while let Some((sq, x)) = it.next() {
            if x {
                res = res.set(sq);
            }
        }
        res
    }
}

impl BoardMap<char> {
    pub const fn to_mask(&self, c: char) -> Mask {
        let mut res = Mask::nil();
        let mut it = self.iter();
        while let Some((sq, x)) = it.next() {
            if c == x {
                res = res.set(sq);
            }
        }
        res
    }
}

impl BoardMap<Option<ColorPiece>> {
    pub const fn to_mask(&self, c: ColorPiece) -> Mask {
        let mut res = Mask::nil();
        let mut it = self.iter();
        while let Some((sq, x)) = it.next() {
            if let Some(x) = x {
                if c as u8 == x as u8 {
                    res = res.set(sq);
                }
            }
        }
        res
    }
}

#[test]
fn why_u_no_worky() {
    let mut x = BoardMap::new_with(None);
    x.set(Square::at(File::A, Rank::_1), Some(ColorPiece::WhiteKing));

    assert_eq!(x.to_mask(ColorPiece::WhiteKing), Mask::new(1));
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
