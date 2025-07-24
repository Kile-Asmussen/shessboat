use std::ops::Index;

use rand::Fill;

use crate::shessboard::{
    enums::{ColorPiece, File, Rank},
    masks::{Mask, SquareIter},
    pieces::Micropawns,
    squares::Square,
};

#[derive(Debug)]
pub struct BoardMap<T>([T; 64]);

impl<T: Sized + Copy> BoardMap<T> {
    pub const fn new(it: [T; 64]) -> Self {
        Self(it)
    }

    pub const fn new_with(it: T) -> Self {
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
        T: Sized,
    {
        BoardMapIter(Mask::full().iter(), self)
    }
}

impl<T: Sized + Clone> BoardMap<T> {
    pub fn new_with_clone(it: T) -> Self {
        Self(std::array::from_fn(|_| it.clone()))
    }

    pub fn at_clone(&self, sq: Square) -> T {
        self.0[sq.index() as usize].clone()
    }

    pub fn set_clone(&mut self, sq: Square, it: T) -> &mut Self {
        self.0[sq.index() as usize] = it.clone();
        self
    }
}

impl<T: Default> BoardMap<T> {
    pub fn reset(&mut self) {
        for x in &mut self.0 {
            *x = Default::default()
        }
    }
}

impl BoardMap<Mask> {
    pub const fn overlays(&self, mask: Mask) -> Mask {
        let mut res = Mask::nil();
        let mut iter = mask.iter();
        while let Some(sq) = iter.next() {
            res = res.overlay(self.at(sq));
        }
        res
    }

    pub const fn overlaps(&self, mask: Mask) -> Mask {
        let mut res = Mask::nil();
        let mut iter = mask.iter();
        while let Some(sq) = iter.next() {
            res = res.overlap(self.at(sq));
        }
        res
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

impl BoardMap<Micropawns> {
    pub const fn sum_mask(&self, m: Mask) -> Micropawns {
        let mut res = 0;
        let mut it = m.iter();
        while let Some(sq) = it.next() {
            res += self.at(sq)
        }
        res
    }

    pub const fn board(b: &[[Micropawns; 8]; 8]) -> Self {
        let mut res = [0; 64];
        let mut it = Mask::full().iter();

        while let Some(sq) = it.next() {
            res[sq.index() as usize] =
                b[7 - sq.rank().as_rank() as usize][sq.file().as_file() as usize];
        }

        Self::new(res)
    }

    pub const fn board_and_mirror(b: &[[Micropawns; 8]; 8]) -> (Self, Self) {
        let mut b = *b;
        let white = Self::board(&b);
        let b = [b[7], b[6], b[5], b[4], b[3], b[2], b[1], b[0]];
        let black = Self::board(&b);
        (white, black)
    }
}

#[test]
fn board_setup() {
    let board = BoardMap::board(&[
        [5, 0, 0, 0, 0, 0, 0, 6],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 4, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 2, 0, 0, 3],
    ]);

    assert_eq!(board.at(Square::a1), 1);
    assert_eq!(board.at(Square::a2), 0);
    assert_eq!(board.at(Square::e1), 2);
    assert_eq!(board.at(Square::h1), 3);
    assert_eq!(board.at(Square::d4), 4);
    assert_eq!(board.at(Square::a8), 5);
    assert_eq!(board.at(Square::h8), 6);
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
