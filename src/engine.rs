use crate::shessboard::{enums::{Color, ColorPiece}, moves::{Move, ProtoMove}, squares::Square, BitBoard};

pub struct ShessEngine {
    pub board: BitBoard,
    pub moves: Vec<Move>,
}

impl ShessEngine {
    pub fn new() -> Self {
        Self {
            board: BitBoard::new(),
            moves: vec![],
        }
    }

    pub fn reset(&mut self) {
        self.board = BitBoard::empty()
        self.board.generate_moves(&mut self.moves);
    }

    pub fn place(&mut self, p: Option<ColorPiece>, sq: Square) {
        self.board.set_piece(p, sq);
        self.moves.clear();
        self.board.generate_moves(&mut self.moves);
    }

    pub fn force_move(&mut self, m: ProtoMove) {
        let c = if let Some(p) = self.board.white.piece_at(m.from) {
            ColorPiece::new(Color::White, p)
        } else if let  Some(p) = self.board.black.piece_at(m.from) {
            ColorPiece::new(Color::Black, p)
        } else {
            return;
        };

        self.board.set_piece(None, m.to);        

        self.board.apply(&Move {
            from_to: m,
            castling: None,
            capture: None,
            color_and_piece: c,
            promotion: None,
        });

        self.board.generate_moves(&mut self.moves);
    }

    pub fn print_moves(&self) {
        let mut v =
            self.moves.iter().map(|m| format!("{}", m)).collect::<Vec<_>>();
        let n = v.iter().map(|s| s.len()).max().unwrap_or(0) + 1;

        for s in &mut v {
            *s = format!("{s:<n$}");
        }

        for ss in v.chunks(10) {
            for s in ss {
                print!("{}", s);
            }
            println!();
        }
    }
}
