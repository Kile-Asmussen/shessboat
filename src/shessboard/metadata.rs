use crate::shessboard::{
    castling::{CastlingDetails, CastlingRights},
    enums::{Color, Piece},
    pieces::pawns::EnPassant,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Metadata {
    pub to_move: Color,
    pub tempo: u16,
    pub last_change: u16,
    pub white_castling: CastlingRights,
    pub black_castling: CastlingRights,
    pub castling_details: CastlingDetails,
    pub en_passant: Option<EnPassant>,
}

impl Metadata {
    pub fn castling_rights(&self, color: Color) -> (CastlingRights, CastlingRights) {
        match color {
            Color::White => (self.white_castling, self.black_castling),
            Color::Black => (self.black_castling, self.white_castling),
        }
    }

    pub fn castling_rights_mut(
        &mut self,
        color: Color,
    ) -> (&mut CastlingRights, &mut CastlingRights) {
        match color {
            Color::White => (&mut self.white_castling, &mut self.black_castling),
            Color::Black => (&mut self.black_castling, &mut self.white_castling),
        }
    }

    pub fn turn(&self) -> usize {
        (self.tempo / 2 + 1) as usize
    }

    pub fn new() -> Self {
        Self {
            to_move: Color::White,
            tempo: 0,
            last_change: 0,
            white_castling: CastlingRights::new(),
            black_castling: CastlingRights::new(),
            castling_details: CastlingDetails::new(),
            en_passant: None,
        }
    }

    // pub fn new_480(arr: [Piece; 8]) -> Self {
    //     Self {
    //         to_move: Color::White,
    //         tempo: 0,
    //         last_change: 0,
    //         white_castling: CastlingRights::new(),
    //         black_castling: CastlingRights::new(),
    //         castling_details: CastlingDetails::new_480(arr),
    //         en_passant: None,
    //     }
    // }

    // pub fn new_960(arr: [Piece; 8]) -> Self {
    //     Self {
    //         to_move: Color::White,
    //         tempo: 0,
    //         last_change: 0,
    //         white_castling: CastlingRights::new(),
    //         black_castling: CastlingRights::new(),
    //         castling_details: CastlingDetails::new_960(arr),
    //         en_passant: None,
    //     }
    // }

    pub fn empty() -> Metadata {
        Self {
            to_move: Color::White,
            tempo: 0,
            last_change: 0,
            white_castling: CastlingRights {
                ooo: false,
                oo: false,
            },
            black_castling: CastlingRights {
                ooo: false,
                oo: false,
            },
            castling_details: CastlingDetails::new(),
            en_passant: None,
        }
    }
}
