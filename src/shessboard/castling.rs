use rand::distr::weighted;

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

pub type CastlingDetails = CastlingInfo<CastlingDetail>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CastlingDetail {
    pub rook_mask: u8,
    pub king_mask: u8,
    pub rook_move: SemiProtoMove,
    pub king_move: SemiProtoMove,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SemiProtoMove {
    pub from: File,
    pub to: File,
}

impl CastlingDetails {
    pub fn new() -> Self {
        Self {
            ooo: CastlingDetail {
                rook_mask: 0b_00001110,
                //            hgfedcba
                king_mask: 0b_00001100,
                rook_move: SemiProtoMove {
                    from: File::A,
                    to: File::D,
                },
                king_move: SemiProtoMove {
                    from: File::E,
                    to: File::C,
                },
            },
            oo: CastlingDetail {
                rook_mask: 0b_01100000,
                //            hgfedcba
                king_mask: 0b_01100000,
                rook_move: SemiProtoMove {
                    from: File::H,
                    to: File::F,
                },
                king_move: SemiProtoMove {
                    from: File::E,
                    to: File::G,
                },
            },
        }
    }

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

        let west_rook = Square::new(west_rook as i8).unwrap();
        let east_rook = Square::new(east_rook as i8).unwrap();
        let king = Square::new(king as i8).unwrap();

        todo!()
    }
}
