use crate::shessboard::{
    boardmap::BoardMap,
    enums::{Color, ColorPiece, Dir, Piece},
    masks::Mask,
    pieces::{
        bishops::Bishops, kings::Kings, knights::Knights, pawns::Pawns, queens::Queens,
        rooks::Rooks,
    },
    squares::Square,
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
    pub fn new(board: &BoardMap<Option<ColorPiece>>, c: Color) -> Self {
        Self {
            kings: Kings::new(board.to_mask(ColorPiece::new(c, Piece::King))),
            queens: Queens::new(board.to_mask(ColorPiece::new(c, Piece::Queen))),
            rooks: Rooks::new(board.to_mask(ColorPiece::new(c, Piece::Rook))),
            bishops: Bishops::new(board.to_mask(ColorPiece::new(c, Piece::Bishop))),
            knights: Knights::new(board.to_mask(ColorPiece::new(c, Piece::Knight))),
            pawns: Pawns::new(board.to_mask(ColorPiece::new(c, Piece::Pawn))),
        }
    }

    pub fn render(&self, board: &mut BoardMap<Option<ColorPiece>>, color: Color) {
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

    pub fn piece_mask(&self, piece: Piece) -> Mask {
        match piece {
            Piece::Pawn => self.pawns.as_mask(),
            Piece::Knight => self.knights.as_mask(),
            Piece::Bishop => self.bishops.as_mask(),
            Piece::Rook => self.rooks.as_mask(),
            Piece::Queen => self.queens.as_mask(),
            Piece::King => self.kings.as_mask(),
        }
    }

    pub fn piece_mask_mut(&mut self, piece: Piece) -> &mut Mask {
        match piece {
            Piece::Pawn => self.pawns.mut_mask(),
            Piece::Knight => self.knights.mut_mask(),
            Piece::Bishop => self.bishops.mut_mask(),
            Piece::Rook => self.rooks.mut_mask(),
            Piece::Queen => self.queens.mut_mask(),
            Piece::King => self.kings.mut_mask(),
        }
    }

    pub fn delete(&mut self, sq: Square) {}

    pub fn piece_at(&self, sq: Square) -> Option<Piece> {
        let sq = sq.as_mask();
        if self.kings.as_mask().overlap(sq).any() {
            Some(Piece::King)
        } else if self.queens.as_mask().overlap(sq).any() {
            Some(Piece::Queen)
        } else if self.rooks.as_mask().overlap(sq).any() {
            Some(Piece::Rook)
        } else if self.bishops.as_mask().overlap(sq).any() {
            Some(Piece::Bishop)
        } else if self.knights.as_mask().overlap(sq).any() {
            Some(Piece::Knight)
        } else if self.pawns.as_mask().overlap(sq).any() {
            Some(Piece::Pawn)
        } else {
            None
        }
    }

    pub fn threats(&self, color: Color, opposite: Mask, cap: Option<(Square, Piece)>) -> Mask {
        let same = self.as_mask();

        let king = self.kings.threats(same);
        let queen = self.queens.captured(cap).threats(Rooks::nil(), Bishops::nil(), same, opposite);
        let rook = self.rooks.captured(cap).threats(Queens::nil(), same, opposite);
        let bishop = self.bishops.captured(cap).threats(Queens::nil(), same, opposite);
        let knight = self.knights.captured(cap).threats(same);
        let pawn = self.pawns.captured(cap).threats(color, same);

        king | queen | rook | bishop | knight | pawn
    }
}
