use crate::bitboard::{colorfault::Colorfault, masks::Mask, pieces::Millipawns};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Knights(Mask);

impl Knights {
    pub fn materiel(&self) -> Millipawns {
        Millipawns(self.0.occupied() as usize * 3250)
    }
}
