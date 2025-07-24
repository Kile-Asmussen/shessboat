use crate::shessboard::{
    boardmap::BoardMap,
    enums::{Color, ColorPiece, Dir, Piece, Shade},
    masks::Mask,
    pieces::{
        bishops::{self, Bishops},
        kings::Kings,
        knights::Knights,
        pawns::Pawns,
        queens::{self, Queens},
        rooks::{self, Rooks},
    },
    squares::Square,
};

#[derive(Clone, Debug, PartialEq, Eq)]
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

    pub fn nil() -> Self {
        Self {
            kings: Kings::nil(),
            queens: Queens::nil(),
            rooks: Rooks::nil(),
            bishops: Bishops::nil(),
            knights: Knights::nil(),
            pawns: Pawns::nil(),
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

    pub const fn as_mask(&self) -> Mask {
        self.kings
            .as_mask()
            .overlay(self.queens.as_mask())
            .overlay(self.rooks.as_mask())
            .overlay(self.bishops.as_mask())
            .overlay(self.knights.as_mask())
            .overlay(self.pawns.as_mask())
    }

    pub const fn piece_mask(&self, piece: Piece) -> Mask {
        match piece {
            Piece::Pawn => self.pawns.as_mask(),
            Piece::Knight => self.knights.as_mask(),
            Piece::Bishop => self.bishops.as_mask(),
            Piece::Rook => self.rooks.as_mask(),
            Piece::Queen => self.queens.as_mask(),
            Piece::King => self.kings.as_mask(),
        }
    }

    pub const fn piece_mask_mut(&mut self, piece: Piece) -> &mut Mask {
        match piece {
            Piece::Pawn => self.pawns.mut_mask(),
            Piece::Knight => self.knights.mut_mask(),
            Piece::Bishop => self.bishops.mut_mask(),
            Piece::Rook => self.rooks.mut_mask(),
            Piece::Queen => self.queens.mut_mask(),
            Piece::King => self.kings.mut_mask(),
        }
    }

    pub const fn delete(&mut self, sq: Square) {
        self.kings = Kings::new(self.kings.as_mask().unset(sq));
        self.queens = Queens::new(self.queens.as_mask().unset(sq));
        self.rooks = Rooks::new(self.rooks.as_mask().unset(sq));
        self.bishops = Bishops::new(self.bishops.as_mask().unset(sq));
        self.knights = Knights::new(self.knights.as_mask().unset(sq));
        self.pawns = Pawns::new(self.pawns.as_mask().unset(sq));
    }

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

    pub const fn threats(
        &self,
        color: Color,
        mut opposite: Mask,
        cap: Option<(Square, Piece)>,
    ) -> Mask {
        let mut blocking = self.as_mask();
        if let Some((sq, _)) = cap {
            blocking = blocking.unset(sq)
        }
        blocking = blocking.overlay(opposite);

        let king = self.kings.threats();
        let queen = self.queens.captured(cap).threats(blocking);
        let rook = self.rooks.captured(cap).threats(blocking);
        let bishop = self.bishops.captured(cap).threats(blocking);
        let knight = self.knights.captured(cap).threats();
        let pawn = self.pawns.captured(cap).threats(color);

        king.overlay(queen)
            .overlay(rook)
            .overlay(bishop)
            .overlay(knight)
            .overlay(pawn)
    }

    pub fn set_piece(&mut self, piece: Option<Piece>, sq: Square) {
        for m in [
            self.queens.mut_mask(),
            self.rooks.mut_mask(),
            self.bishops.mut_mask(),
            self.knights.mut_mask(),
            self.pawns.mut_mask(),
            self.kings.mut_mask(),
        ] {
            *m = m.unset(sq)
        }

        if let Some(p) = piece {
            let x = self.piece_mask_mut(p);
            *x = x.set(sq);
        }
    }

    pub fn has_sufficient_materiel(&self) -> bool {
        let pawns = self.pawns.as_mask().occupied();
        let rooks = self.rooks.as_mask().occupied();
        let queens = self.queens.as_mask().occupied();
        let dark_bishops = self
            .bishops
            .as_mask()
            .overlap(Shade::Dark.as_mask())
            .occupied();
        let light_bishops = self
            .bishops
            .as_mask()
            .overlap(Shade::Light.as_mask())
            .occupied();
        let knights = self.knights.as_mask().occupied();

        pawns > 0
            || rooks > 0
            || queens > 0
            || (light_bishops > 0 && dark_bishops > 0)
            || (knights >= 2)
            || (knights == 1 && (light_bishops + dark_bishops) == 1)
    }
}
