use crate::bitboard::masks::Mask;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Rooks(Mask);

impl Rooks {
    pub fn materiel(&self) -> Millipawns {
        Millipawns(self.0.occupied() as usize * 5000)
    }
}
