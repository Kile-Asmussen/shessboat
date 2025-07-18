use std::arch::x86_64::CpuidResult;

use crate::shessboard::{
    boardmap::BoardMap,
    castling::{CastlingInfo, CastlingRights, CastlingSide},
    enums::{Color, ColorPiece, Dir, File, Piece, Rank},
    half::HalfBitBoard,
    masks::Mask,
    moves::{Move, ProtoMove},
    pieces::{Micropawns, queens::Queens, slide_move_stop},
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Kings(Mask);

impl Kings {
    pub const fn nil() -> Self {
        Self(Mask::nil())
    }

    pub const fn new(mask: Mask) -> Self {
        Self(mask)
    }

    pub const fn materiel(&self) -> Micropawns {
        self.as_mask().occupied() as i64 * 1_000_000_000
    }

    pub const fn as_mask(&self) -> Mask {
        self.0
    }

    pub const fn mut_mask(&mut self) -> &mut Mask {
        &mut self.0
    }

    pub fn render(&self, board: &mut BoardMap<Option<ColorPiece>>, color: Color) {
        for sq in self.as_mask().iter() {
            board.set(sq, Some(ColorPiece::new(color, Piece::King)));
        }
    }

    const MOVES: BoardMap<Mask> = Self::build_move_db();

    const fn build_move_db() -> BoardMap<Mask> {
        let mut n = 0;
        let mut res = [Mask::new(0); 64];

        while n < 64 {
            res[n] = Self::moves_from(Square::new(n as i8).unwrap());

            n += 1;
        }

        BoardMap::new(res)
    }

    pub const fn moves_from(sq: Square) -> Mask {
        use Dir::*;
        return Mask::new(
            x(sq.go(North))
                | x(sq.go(East))
                | x(sq.go(South))
                | x(sq.go(West))
                | x(sq.go(NorthEast))
                | x(sq.go(SouthEast))
                | x(sq.go(SouthWest))
                | x(sq.go(NorthWest)),
        );

        const fn x(sq: Option<Square>) -> u64 {
            let Some(sq) = sq else {
                return 0u64;
            };
            sq.as_mask().as_u64()
        }
    }

    pub fn threats(&self, same: Mask) -> Mask {
        Self::MOVES.overlay(self.as_mask()) & !same
    }

    pub const CASTLING_OOO: [Mask; 8] = [
        Mask::nil(),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_10000000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_11000000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_01100000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_00110000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_00011000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_00001100u8]),
        Mask::nil(),
    ];

    pub const CASTLING_OO: [Mask; 8] = [
        Mask::nil(),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_10000000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_11000000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_01100000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_00110000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_00011000u8]),
        Mask::board([0, 0, 0, 0, 0, 0, 0, 0b_00001100u8]),
        Mask::nil(),
    ];

    pub fn enumerate_legal_moves(
        &self,
        color: Color,
        active_mask: Mask,
        passive: &HalfBitBoard,
        castling: CastlingRights,
        rook_files: CastlingInfo<File>,
        res: &mut Vec<Move>,
    ) {
        let color_and_piece = ColorPiece::new(color, Piece::King);

        if !self.as_mask().any() {
            return;
        }

        if let Some(from) = self.as_mask().first() {
            let possible = Kings::MOVES.at(from)
                & !active_mask
                & !passive.threats(color.other(), active_mask, None);

            for to in possible {
                let from_to = ProtoMove { from, to };

                let capture = passive.piece_at(to).map(|p| (to, p));

                res.push(Move {
                    color_and_piece,
                    from_to,
                    capture,
                    castling: None,
                    promotion: None,
                });
            }

            if castling.ooo {
                castling_move(
                    color,
                    from,
                    true,
                    &Self::CASTLING_OOO,
                    &Queens::EAST,
                    active_mask,
                    passive,
                    rook_files.ooo,
                    res,
                );
            }

            if castling.oo {
                castling_move(
                    color,
                    from,
                    false,
                    &Self::CASTLING_OO,
                    &Queens::WEST,
                    active_mask,
                    passive,
                    rook_files.oo,
                    res,
                );
            }
        }

        fn castling_move(
            color: Color,
            from: Square,
            west: bool,
            king_masks: &[Mask; 8],
            rook_slide_masks: &BoardMap<Mask>,
            active_mask: Mask,
            passive: &HalfBitBoard,
            rook_file: File,
            res: &mut Vec<Move>,
        ) {
            let mut king_move = king_masks[from.file().as_file() as usize];

            if color == Color::Black {
                king_move = king_move.mirror()
            }

            let king_from_to = ProtoMove {
                from,
                to: (if west {
                    king_move.first()
                } else {
                    king_move.last()
                })
                .unwrap(),
            };

            let rook_from_to = ProtoMove {
                from: Square::at(rook_file, color.starting_rank()),
                to: king_from_to.to.go(if west { Dir::East } else { Dir::West }).unwrap(),
            };

            let rook_move = slide_move_stop(
                true,
                rook_slide_masks.at(rook_from_to.from),
                from.as_mask(),
                Mask::nil(),
            );

            if (rook_move & (active_mask | passive.as_mask())).any() {
                return;
            }

            if (passive.threats(color.other(), active_mask, None) & king_move).any() {
                return;
            }

            if king_from_to.makes_king_checked(
                active_mask ^ rook_from_to.as_mask(),
                Kings::new(king_from_to.to.as_mask()),
                None,
                passive,
                color.other(),
            ) {
                return;
            }

            res.push(Move {
                color_and_piece: ColorPiece::new(color, Piece::King),
                from_to: king_from_to,
                castling: Some(CastlingSide::OOO),
                capture: None,
                promotion: None,
            })
        }
    }
}
