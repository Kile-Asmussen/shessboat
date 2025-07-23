use crate::shessboard::{
    enums::{File, Piece},
    moves::ProtoMove,
    pieces::kings::Kings,
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CastlingSide {
    OOO = 1,
    OO,
}

pub type CastlingRights = CastlingInfo<bool>;

impl CastlingRights {
    pub fn new() -> Self {
        Self {
            ooo: true,
            oo: true,
        }
    }

    pub fn update(&mut self, cr: CastlingRights) {
        self.ooo &= cr.ooo;
        self.oo &= cr.oo;
    }
}

pub type CastlingMasks = CastlingInfo<CastlingMask>;

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct CastlingMask {
    rook: u8,
    king: u8,
}

impl CastlingMasks {
    pub fn new_480(arr: &[Piece; 8]) -> Self {
        let mut west_rook = 0usize;
        let mut east_rook = 0usize;
        let mut king = 0usize;

        for (i, p) in arr.iter().enumerate() {
            if *p == Piece::Rook {
                if king == 0 {
                    west_rook = i;
                } else {
                    east_rook = i;
                }
            } else if *p == Piece::King {
                king = i;
            }
        }

        let mut res = CastlingMasks {
            ooo: CastlingMask { rook: 0, king: 0 },
            oo: CastlingMask { rook: 0, king: 0 },
        };

        let king = Square::new(king as i8).unwrap();

        todo!()
    }
}
