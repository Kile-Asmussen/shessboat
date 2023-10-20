use crate::{board::Board, elements::PieceColor, fen::XFen};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Tempo {
    tempo: usize,
    last_advance: usize
};

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
    fn new(fen: &XFen) -> Self {
        let tempo = fen.turn * 2
            + if fen.to_move == PieceColor::White {
                0
            } else {
                1
            };
        Tempo { tempo, last_advance: tempo - fen.tempo_clock }
    }
}
