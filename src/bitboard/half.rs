use crate::bitboard::{
    boardmap::BoardMap,
    enums::{Color, Piece},
    masks::Mask,
    pieces::{
        bishops::Bishops, kings::Kings, knights::Knights, pawns::Pawns, queens::Queens,
        rooks::Rooks,
    },
};

#[derive(Clone, Debug)]
pub struct HalfBitBoard {
    pub kings: Kings,
    pub queens: Queens,
    pub rooks: Rooks,
    pub bishops: Bishops,
    pub knights: Knights,
    pub pawns: Pawns,
}

impl HalfBitBoard {
    pub fn new(board: &BoardMap<char>, c: Color) -> Self {
        Self {
            kings: Kings::new(board.to_mask(c.letter(Piece::King))),
            queens: Queens::new(board.to_mask(c.letter(Piece::Queen))),
            rooks: Rooks::new(board.to_mask(c.letter(Piece::Rook))),
            bishops: Bishops::new(board.to_mask(c.letter(Piece::Bishop))),
            knights: Knights::new(board.to_mask(c.letter(Piece::Knight))),
            pawns: Pawns::new(board.to_mask(c.letter(Piece::Pawn))),
        }
    }

    pub fn render(&self, board: &mut BoardMap<char>, color: Color) {
        self.kings.render(board, color);
        self.queens.render(board, color);
        self.rooks.render(board, color);
        self.bishops.render(board, color);
        self.knights.render(board, color);
        self.pawns.render(board, color);
    }

    pub fn as_mask(&self) -> Mask {
        self.kings.as_mask()
            | self.queens.as_mask()
            | self.rooks.as_mask()
            | self.bishops.as_mask()
            | self.knights.as_mask()
            | self.pawns.as_mask()
    }
}
