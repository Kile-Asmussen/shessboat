use rand::distr::weighted;

use crate::shessboard::{
    enums::{File, Piece, Rank},
    moves::ProtoMove,
    pieces::kings::Kings,
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct CastlingInfo<T> {
    pub ooo: T,
    pub oo: T,
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

impl SemiProtoMove {
    pub const fn as_move(&self, rank: Rank) -> ProtoMove {
        ProtoMove {
            from: Square::at(self.from, rank),
            to: Square::at(self.to, rank),
        }
    }
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
}
