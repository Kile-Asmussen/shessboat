use crate::shessboard::{
    BitBoard, Metadata,
    algebraic::Notation,
    boardmap::BoardMap,
    castling::{CastlingInfo, CastlingRights},
    enums::{Color, ColorPiece},
    masks::Mask,
    moves::{Move, ProtoMove},
    squares::Square,
};

pub struct ShessInteractor {
    pub board: BitBoard,
    pub moves: Vec<Move>,
}

impl ShessInteractor {
    pub fn new() -> Self {
        Self {
            board: BitBoard::empty(),
            moves: Vec::with_capacity(50),
        }
    }

    pub fn to_move(&self) -> Color {
        self.board.metadata.to_move
    }

    pub fn set_position(&mut self, n: usize) {
        self.board = BitBoard::new_960(n);
        self.moves.clear();
        self.board.generate_moves(&mut self.moves);
    }

    pub fn reset(&mut self) {
        self.board = BitBoard::empty();
        self.board.metadata.black_castling = CastlingRights {
            ooo: false,
            oo: false,
        };
        self.board.metadata.white_castling = CastlingRights {
            ooo: false,
            oo: false,
        };
        self.moves.clear();
    }

    pub fn place(&mut self, p: Option<ColorPiece>, sq: Square) {
        self.board.set_piece(p, sq);
        self.moves.clear();
        self.board.generate_moves(&mut self.moves);
    }

    pub fn legal_move_mask(&self, sq: Square) -> Mask {
        self.moves
            .iter()
            .filter(|m| m.from_to.from == sq)
            .map(|m| m.from_to.to.as_mask())
            .sum()
    }

    pub fn threat_mask(&self, c: Color) -> Mask {
        self.board
            .color(c)
            .threats(c, self.board.color(c.other()).as_mask(), None)
    }

    pub fn apply_move(&mut self, m: &Move) {
        self.board.apply(m);
        self.moves.clear();
        self.board.generate_moves(&mut self.moves);
    }

    pub fn normal_move(
        &mut self,
        n: Notation,
    ) -> Result<((Notation, &'static str), Move), &'static str> {
        let v = n.find(&self.moves);
        let mv = if v.len() == 0 {
            return Err("No such legal move");
        } else if v.len() > 1 {
            return Err("Ambiguous move");
        } else {
            v[0].clone()
        };

        self.board.apply(&mv);
        self.moves.clear();
        self.board.generate_moves(&mut self.moves);
        if self.board.is_in_check(self.to_move()) {
            if self.moves.len() != 0 {
                Ok(((n, "+"), mv))
            } else {
                Ok(((n, "#"), mv))
            }
        } else {
            Ok(((n, ""), mv))
        }
    }

    pub fn set_turn(&mut self, c: Color, n: usize) {
        self.board.metadata.to_move = c;
        self.board.metadata.half_turn = (n - 1) * 2 + if c == Color::Black { 1 } else { 0 };
        self.board.metadata.change_happened_at = self.board.metadata.half_turn;
    }

    pub fn printable_metadata(&self) -> String {
        let metadata = &self.board.metadata;
        let to_move = metadata.to_move;
        let (wooo, woo) = castles(metadata.white_castling);
        let (booo, boo) = castles(metadata.black_castling);
        let epc = metadata
            .en_passant
            .map(|x| format!("{}", x.0))
            .unwrap_or("n/a".to_string());
        let turn = metadata.turn();
        let clock = metadata.turn_clock();
        let (left, right) = (
            metadata.rook_files.ooo.as_char(),
            metadata.rook_files.oo.as_char(),
        );
        let (wooo, woo) = castles(metadata.white_castling);
        let (booo, boo) = castles(metadata.black_castling);

        return format!(
            "Turn: {turn} ({clock}), {to_move:?} to move
White castling righs: {left} {wooo} K {woo} {right}
Black castling rights: {left} {booo} K {boo} {right}
En passant square: {epc}",
        );

        fn castles(r: CastlingRights) -> (&'static str, &'static str) {
            (
                if r.ooo { "O-O-O" } else { "-" },
                if r.oo { "O-O" } else { "-" },
            )
        }
    }

    pub fn printable_moves(&self) -> Vec<String> {
        self.moves
            .iter()
            .map(|m| format!("{:<12}", m.to_string()))
            .collect::<Vec<_>>()
    }

    pub fn as_boardmap(&self) -> BoardMap<Option<ColorPiece>> {
        let mut res = BoardMap::new_with(None);

        self.board.render(&mut res);

        res
    }
}
