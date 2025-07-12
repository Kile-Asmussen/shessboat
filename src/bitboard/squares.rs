use std::{fmt::Debug, num::NonZeroU64};

use crate::bitboard::{
    enums::{Cardinals, File, Orthogonals, Rank},
    masks::Mask,
};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Square(NonZeroU64);

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Square::new({})", self.0.trailing_zeros())
    }
}

impl Square {
    pub const fn as_mask(&self) -> Mask {
        Mask::new(self.0.get())
    }

    pub const fn new(ix: u32) -> Option<Self> {
        let Some(n) = 1u64.checked_shl(ix) else {
            return None;
        };
        let Some(n) = NonZeroU64::new(n) else {
            return None;
        };
        Some(Square(n))
    }

    pub const fn from_mask(mask: Mask) -> Option<Self> {
        if mask.occupied() == 1 {
            mask.first()
        } else {
            None
        }
    }

    pub const fn index(&self) -> u32 {
        self.0.trailing_zeros()
    }

    pub const fn rank_and_file(&self) -> (Rank, File) {
        (
            Rank::rank(self.index() % 8).unwrap(),
            File::file(self.index() / 8).unwrap(),
        )
    }

    pub const fn go(&self, dir: Cardinals) -> Option<Self> {
        let (rank, file) = self.rank_and_file();
        let (rank, file) = (rank.as_rank(), file.as_file());
        let (rank, file) = (rank as i32, file as i32);

        #[rustfmt::skip]
        let (rank, file) = match dir {
            Cardinals::North     => (rank + 1, file),
            Cardinals::NorthEast => (rank + 1, file + 1),
            Cardinals::East      => (rank,     file + 1),
            Cardinals::SouthEast => (rank - 1, file + 1),
            Cardinals::South     => (rank - 1, file),
            Cardinals::SouthWest => (rank - 1, file - 1),
            Cardinals::West      => (rank,     file - 1),
            Cardinals::NorthWest => (rank + 1, file - 1),
        };

        if rank < 0 || file < 0 {
            return None;
        }

        let (rank, file) = (rank as u32, file as u32);

        let (Some(rank), Some(file)) = (Rank::rank(rank), File::file(file)) else {
            return None;
        };

        Square::from_mask(Mask::new(rank.as_mask().as_u64() & file.as_mask().as_u64()))
    }

    pub const fn goes(&self, dirs: &[Cardinals]) -> Option<Self> {
        return so_it_goes(*self, 0, dirs);

        const fn so_it_goes(it: Square, n: usize, dirs: &[Cardinals]) -> Option<Square> {
            if n >= dirs.len() {
                return Some(it);
            }

            let dir = dirs[n];

            let Some(res) = it.go(dir) else {
                return None;
            };
            so_it_goes(res, n + 1, dirs)
        }
    }

    pub fn invariant(&self) {
        assert_eq!(self.as_mask().occupied(), 1);
    }
}
