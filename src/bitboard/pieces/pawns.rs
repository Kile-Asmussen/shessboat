use crate::bitboard::{
    boardmap::BoardMap,
    enums::{Color, ColorPiece, Dir, Piece, Rank},
    half::HalfBitBoard,
    masks::Mask,
    moves::{Move, ProtoMove},
    pieces::{Micropawns, kings::Kings, queens::Queens},
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
        self.0.occupied() as i64 * 1_000_000
    }

    pub const fn as_mask(&self) -> Mask {
        self.0
    }

    pub const fn captured(&self, sq: Square) -> Self {
        Self(self.as_mask().unset(sq))
    }

    pub fn render(&self, board: &mut BoardMap<Option<ColorPiece>>, color: Color) {
        for sq in self.0.iter() {
            board.set(sq, Some(ColorPiece::new(color, Piece::Pawn)));
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

        let moves = if let Rank::_2 | Rank::_7 = sq.rank() {
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

    pub fn threats(&self, color: Color, same: Mask) -> Mask {
        let threat_masks = match color {
            Color::White => &Self::WHITE_THREATS,
            Color::Black => &Self::BLACK_THREATS,
        };
        threat_masks.overlap(self.as_mask())
    }

    pub const fn promotion_rank(color: Color) -> Rank {
        match color {
            Color::White => Rank::_8,
            Color::Black => Rank::_1,
        }
    }

    pub fn enumerate_legal_moves(
        &self,
        color: Color,
        active_mask: Mask,
        passive_mask: Mask,
        passive: &HalfBitBoard,
        en_passant: Option<(Square, Square)>,
        kings: Kings,
        res: &mut Vec<Move>,
    ) {
        let color_and_piece = ColorPiece::new(color, Piece::Pawn);

        if !self.as_mask().any() {
            return;
        }

        #[allow(non_snake_case)]
        let (MOVES, THREATS) = match color {
            Color::White => (&Self::WHITE_MOVES, &Self::WHITE_THREATS),
            Color::Black => (&Self::WHITE_MOVES, &Self::WHITE_THREATS),
        };

        for from in self.as_mask() {
            let possible_moves = MOVES.at(from) & !active_mask;

            for to in possible_moves {
                let from_to = ProtoMove { from, to };
                if from_to.makes_king_checked(active_mask, kings, None, passive, color.other()) {
                    continue;
                }

                promotions(
                    res,
                    Move {
                        color_and_piece,
                        from_to,
                        castling: None,
                        capture: None,
                        promotion: None,
                    },
                );
            }

            let possible_attacks = THREATS.at(from) & passive_mask;

            for to in possible_attacks {
                let from_to = ProtoMove { from, to };

                let Some(capture) = passive.piece(to) else {
                    continue;
                };
                let capture = Some((to, capture));

                if from_to.makes_king_checked(active_mask, kings, capture, passive, color.other()) {
                    continue;
                }

                promotions(
                    res,
                    Move {
                        color_and_piece,
                        from_to,
                        castling: None,
                        capture,
                        promotion: None,
                    },
                );
            }

            if let Some((to, pawn)) = en_passant {
                'out: {
                    if possible_attacks.contains(to) {
                        let from_to = ProtoMove { from, to };

                        let capture = Some((pawn, Piece::Pawn));

                        if from_to.makes_king_checked(
                            active_mask,
                            kings,
                            capture,
                            passive,
                            color.other(),
                        ) {
                            break 'out;
                        }

                        res.push(Move {
                            color_and_piece,
                            from_to,
                            castling: None,
                            capture,
                            promotion: None,
                        })
                    }
                }
            }
        }

        fn promotions(res: &mut Vec<Move>, mut mv: Move) {
            if mv.from_to.to.rank() == Pawns::promotion_rank(mv.color_and_piece.color()) {
                for piece in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                    mv.promotion = Some(piece);
                    res.push(mv);
                }
            } else {
                res.push(mv);
            }
        }
    }
}
