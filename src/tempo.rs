use crate::{
    board::Board,
    elements::PieceColor,
    moves::{ChessMove, GeneralMove, PawnMove},
    sfen::SFen,
    validity::{Validatable, Validator, Validity},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Tempo {
    tempo: usize,
    last_advance: usize,
}

impl Tempo {
    pub fn turn(self) -> usize {
        self.tempo / 2
    }

    fn next(&mut self) {
        self.tempo += 1;
    }

    fn has_advanced(&mut self) {
        self.last_advance = self.tempo;
    }

    pub fn tempi_since_advance(self) -> usize {
        self.last_advance - self.tempo
    }

    pub fn to_move(self) -> PieceColor {
        use PieceColor::*;
        if self.tempo & 1 == 0 {
            White
        } else {
            Black
        }
    }
}

impl Board for Tempo {
    fn new(fen: &SFen) -> Self {
        let tempo = fen.turn * 2
            + if fen.to_move == PieceColor::White {
                0
            } else {
                1
            };
        Tempo {
            tempo,
            last_advance: tempo - fen.tempo_clock,
        }
    }
}

impl Validatable for Tempo {
    fn valid(&self) -> Validity {
        (self.tempi_since_advance() >= 151)
            .valid()
            .explain("Forced draw due to lack of progress")
    }
}

impl Validator<GeneralMove> for Tempo {
    fn validate(&self, it: &GeneralMove) -> Validity {
        (it.color() != self.to_move())
            .valid()
            .explain(match it.color() {
                PieceColor::White => "Not white to move",
                PieceColor::Black => "Not black to move",
            })
    }
}
