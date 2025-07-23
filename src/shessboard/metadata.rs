use crate::shessboard::{
    castling::{CastlingDetails, CastlingRights},
    enums::{Color, Piece},
    pieces::pawns::EnPassant,
};

#[derive(Clone, Debug)]
pub struct Metadata {
    pub to_move: Color,
    pub half_turn: u16,
    pub change_happened_at: u16,
    pub white_castling: CastlingRights,
    pub black_castling: CastlingRights,
    pub castling_details: CastlingDetails,
    pub en_passant: Option<EnPassant>,
}

impl Metadata {
    pub fn castling_right(&self, color: Color) -> &CastlingRights {
        match color {
            Color::White => &self.white_castling,
            Color::Black => &self.black_castling,
        }
    }

    pub fn castling_right_mut(
        &mut self,
        color: Color,
    ) -> (&mut CastlingRights, &mut CastlingRights) {
        match color {
            Color::White => (&mut self.white_castling, &mut self.black_castling),
            Color::Black => (&mut self.black_castling, &mut self.white_castling),
        }
    }

    pub fn turn(&self) -> usize {
        (self.half_turn / 2 + 1) as usize
    }

    pub fn turn_clock(&self) -> usize {
        (self.half_turn - self.change_happened_at) as usize / 2 + 1
    }

    pub fn new() -> Self {
        Self {
            to_move: Color::White,
            half_turn: 0,
            change_happened_at: 0,
            white_castling: CastlingRights::new(),
            black_castling: CastlingRights::new(),
            castling_details: CastlingDetails::new(),
            en_passant: None,
        }
    }

    pub fn new_480(arr: [Piece; 8]) -> Self {
        Self {
            to_move: Color::White,
            half_turn: 0,
            change_happened_at: 0,
            white_castling: CastlingRights::new(),
            black_castling: CastlingRights::new(),
            castling_details: CastlingDetails::new_480(arr),
            en_passant: None,
        }
    }

    pub fn new_960(arr: [Piece; 8]) -> Self {
        Self {
            to_move: Color::White,
            half_turn: 0,
            change_happened_at: 0,
            white_castling: CastlingRights::new(),
            black_castling: CastlingRights::new(),
            castling_details: CastlingDetails::new_960(arr),
            en_passant: None,
        }
    }

    pub fn empty() -> Metadata {
        Self {
            to_move: Color::White,
            half_turn: 0,
            change_happened_at: 0,
            white_castling: CastlingRights {
                ooo: false,
                oo: false,
            },
            black_castling: CastlingRights {
                ooo: false,
                oo: false,
            },
            castling_details: CastlingDetails::empty(),
            en_passant: None,
        }
    }
}
