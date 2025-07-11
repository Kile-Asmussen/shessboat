pub mod bishops;
pub mod kings;
pub mod knights;
pub mod pawns;
pub mod queens;
pub mod rooks;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Millipawns(pub usize);
