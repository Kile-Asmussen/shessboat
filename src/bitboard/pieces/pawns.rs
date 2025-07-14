use crate::bitboard::{
    boardmap::BoardMap,
    enums::{Color, Dir, Piece, Rank},
    masks::Mask,
    pieces::Micropawns,
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Pawns(Mask);

impl Pawns {
    pub const fn nil() -> Self {
        Self(Mask::nil())
    }

    pub const fn new(mask: Mask) -> Self {
        Self(mask)
    }

    pub fn materiel(&self) -> Micropawns {
        self.0.occupied() as isize * 1_000_000
    }

    pub const fn as_mask(&self) -> Mask {
        self.0
    }

    pub const fn mut_mask(&mut self) -> &mut Mask {
        &mut self.0
    }

    pub fn render(&self, board: &mut BoardMap<char>, color: Color) {
        let piece = match color {
            Color::White => 'P',
            Color::Black => 'p',
        };

        for sq in self.0.iter() {
            board.set(sq, piece);
        }
    }

    const WHITE_MOVES: BoardMap<Mask> = Self::build_move_db(Dir::North).0;
    const WHITE_THREATS: BoardMap<Mask> = Self::build_move_db(Dir::North).1;

    const BLACK_MOVES: BoardMap<Mask> = Self::build_move_db(Dir::South).0;
    const BLACK_THREATS: BoardMap<Mask> = Self::build_move_db(Dir::South).1;

    const fn build_move_db(dir: Dir) -> (BoardMap<Mask>, BoardMap<Mask>) {
        let mut moves = BoardMap::new_with(Mask::nil());
        let mut threats = BoardMap::new_with(Mask::nil());

        let mut sqiter = Mask::full().iter();

        while let Some(sq) = sqiter.next() {
            let (amove, athreat) = Self::moves_from(sq, dir);
            moves.set(sq, amove);
            threats.set(sq, athreat);
        }

        (moves, threats)
    }

    pub const fn moves_from(sq: Square, dir: Dir) -> (Mask, Mask) {
        if let Dir::North | Dir::South = dir {
        } else {
            panic!();
        }

        let moves = if let (_, Rank::_2 | Rank::_7) = sq.algebraic() {
            Mask::new(x(sq.go(dir)) | x(sq.goes([dir, dir])))
        } else {
            Mask::new(x(sq.go(dir)))
        };

        let threats = Mask::new(x(sq.goes([dir, Dir::East])) | x(sq.goes([dir, Dir::West])));

        return (moves, threats);

        const fn x(sq: Option<Square>) -> u64 {
            let Some(sq) = sq else {
                return 0u64;
            };
            sq.as_mask().as_u64()
        }
    }
}
