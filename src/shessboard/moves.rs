use core::panic::PanicMessage;
use std::{fmt::Display, fs::metadata};

use crate::shessboard::{
    BitBoard, CastlingInfo,
    castling::{CastlingDetails, CastlingRights, CastlingSide},
    enums::{Color, ColorPiece, Dir, File, Piece, Rank},
    half::HalfBitBoard,
    masks::Mask,
    notation::Algebraic,
    pieces::{
        kings::Kings,
        knights::{self, Knights},
        pawns::{self, EnPassant, Pawns},
        rooks::Rooks,
    },
    squares::Square,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ProtoMove {
    pub from: Square,
    pub to: Square,
}

impl ProtoMove {
    pub fn dist(&self) -> i8 {
        self.from.dist(self.to)
    }

    pub fn dir(&self) -> Option<Dir> {
        todo!()
    }

    pub fn as_mask(&self) -> Mask {
        Mask::nil().set(self.from).set(self.to)
    }

    pub fn positive(&self) -> bool {
        self.from.index() < self.to.index()
    }

    pub fn makes_king_checked(
        &self,
        active: Mask,
        king: Kings,
        capture: Option<(Square, Piece)>,
        passive: &HalfBitBoard,
        passive_color: Color,
    ) -> bool {
        passive
            .threats(passive_color, active ^ self.as_mask(), capture)
            .overlap(king.as_mask())
            .any()
    }
}

#[test]
fn rook_captured_piece_hypothetical() {
    let mut board = BitBoard::empty();

    board.black.pawns = Pawns::new(Square::at(File::D, Rank::_4).as_mask());

    board.white.rooks = Rooks::new(Square::at(File::H, Rank::_4).as_mask());

    assert_eq!(
        board
            .white
            .threats(Color::White, board.black.as_mask(), None),
        Mask::visboard([
            0b_00000001,
            0b_00000001,
            0b_00000001,
            0b_00000001,
            0b_00011110, //4
            0b_00000001,
            0b_00000001,
            0b_00000001,
            // abcdefgh
        ])
    );

    assert_eq!(
        board.white.threats(
            Color::White,
            board.black.as_mask(),
            Some((Square::at(File::H, Rank::_4), Piece::Rook))
        ),
        Mask::nil()
    );

    assert_eq!(
        board.white.threats(
            Color::White,
            board.black.as_mask(),
            Some((Square::at(File::D, Rank::_4), Piece::Pawn))
        ),
        Mask::visboard([
            0b_00000001,
            0b_00000001,
            0b_00000001,
            0b_00000001,
            0b_00011110, //4
            0b_00000001,
            0b_00000001,
            0b_00000001,
            // abcdefgh
        ])
    );

    assert_eq!(
        board.white.threats(
            Color::White,
            board.black.as_mask()
                ^ ProtoMove {
                    from: Square::at(File::D, Rank::_4),
                    to: Square::at(File::D, Rank::_3),
                }
                .as_mask(),
            None
        ),
        Mask::visboard([
            0b_00000001,
            0b_00000001,
            0b_00000001,
            0b_00000001,
            0b_11111110, //4
            0b_00000001,
            0b_00000001,
            0b_00000001,
            // abcdefgh
        ])
    );

    board.white.pawns = Pawns::new(Square::at(File::E, Rank::_4).as_mask());

    assert_eq!(
        board
            .white
            .threats(Color::White, board.black.as_mask(), None),
        Mask::visboard([
            0b_00000001,
            0b_00000001,
            0b_00000001,
            0b_00010101,
            0b_00001110, //4
            0b_00000001,
            0b_00000001,
            0b_00000001,
            // abcdefgh
        ])
    );

    assert_eq!(
        board.white.threats(
            Color::White,
            board.black.as_mask(),
            Some((Square::at(File::E, Rank::_4), Piece::Pawn))
        ),
        Mask::visboard([
            0b_00000001,
            0b_00000001,
            0b_00000001,
            0b_00000001,
            0b_00011110, //4
            0b_00000001,
            0b_00000001,
            0b_00000001,
            // abcdefgh
        ])
    );

    assert_eq!(
        board.white.threats(
            Color::White,
            board.black.as_mask()
                ^ ProtoMove {
                    from: Square::at(File::D, Rank::_4),
                    to: Square::at(File::D, Rank::_3),
                }
                .as_mask(),
            Some((Square::at(File::E, Rank::_4), Piece::Pawn))
        ),
        Mask::visboard([
            0b_00000001,
            0b_00000001,
            0b_00000001,
            0b_00000001,
            0b_11111110, //4
            0b_00000001,
            0b_00000001,
            0b_00000001,
            // abcdefgh
        ])
    );
}

#[test]
fn en_passant_into_check() {
    let mut board = BitBoard::empty();

    board.metadata.to_move = Color::Black;
    board.metadata.en_passant = Some(EnPassant {
        to: Square::at(File::E, Rank::_3),
        capture: Square::at(File::E, Rank::_4),
    });

    board.white.pawns = Pawns::new(Square::at(File::E, Rank::_4).as_mask());
    board.black.pawns = Pawns::new(Square::at(File::D, Rank::_4).as_mask());

    board.white.kings = Kings::new(Square::at(File::H, Rank::_8).as_mask());
    board.black.kings = Kings::new(Square::at(File::A, Rank::_4).as_mask());

    board.white.rooks = Rooks::new(Square::at(File::H, Rank::_4).as_mask());

    let en_passant = ProtoMove {
        from: Square::at(File::D, Rank::_4),
        to: Square::at(File::E, Rank::_3),
    };
    let captured_pawn = Some((Square::at(File::E, Rank::_4), Piece::Pawn));

    assert!(en_passant.makes_king_checked(
        board.black.as_mask(),
        board.black.kings,
        captured_pawn,
        &board.white,
        Color::White,
    ));

    let mut moves = vec![];
    board.generate_moves(&mut moves);

    let not = Algebraic::read("dxe3").unwrap();

    assert_eq!(not.find(&moves), vec![]);
}

impl Display for ProtoMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.from, self.to)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(align(8))]
pub struct Move {
    pub color_and_piece: ColorPiece,
    pub from_to: ProtoMove,
    pub castling: Option<ProtoMove>,
    pub capture: Option<(Square, Piece)>,
    pub promotion: Option<Piece>,
}

impl Move {
    pub fn en_passant_square(&self) -> Option<EnPassant> {
        if self.color_and_piece == ColorPiece::WhitePawn {
            if let ((f, Rank::_2), Rank::_4) =
                (self.from_to.from.algebraic(), self.from_to.to.rank())
            {
                return Some(EnPassant {
                    to: Square::at(f, Rank::_3),
                    capture: Square::at(f, Rank::_4),
                });
            } else {
                return None;
            };
        } else if self.color_and_piece == ColorPiece::BlackPawn {
            if let ((f, Rank::_7), Rank::_5) =
                (self.from_to.from.algebraic(), self.from_to.to.rank())
            {
                return Some(EnPassant {
                    to: Square::at(f, Rank::_6),
                    capture: Square::at(f, Rank::_5),
                });
            } else {
                return None;
            };
        } else {
            return None;
        }
    }

    pub fn castling_rights(&self, details: CastlingDetails) -> (CastlingRights, CastlingRights) {
        let (color, piece) = self.color_and_piece.split();

        let mut active = CastlingRights {
            ooo: true,
            oo: true,
        };
        let mut passive = CastlingRights {
            ooo: true,
            oo: true,
        };

        if piece == Piece::King {
            active.ooo = false;
            active.oo = false;
        } else if piece == Piece::Rook {
            if self.from_to.from == Square::at(details.ooo.rook_move.from, color.starting_rank()) {
                active.ooo = false;
            }

            if self.from_to.from == Square::at(details.oo.rook_move.from, color.starting_rank()) {
                active.oo = false;
            }
        }

        if let Some((sq, Piece::Rook)) = self.capture {
            if piece == Piece::Rook {
                let color = color.other();

                if sq == Square::at(details.ooo.rook_move.from, color.starting_rank()) {
                    passive.ooo = false;
                }

                if sq == Square::at(details.oo.rook_move.from, color.starting_rank()) {
                    passive.oo = false;
                }
            }
        }

        (active, passive)
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.color_and_piece.unicode(), self.from_to)?;
        if let Some((sq, p)) = self.capture {
            write!(
                f,
                "\u{00D7}{}{}",
                ColorPiece::new(self.color_and_piece.color().other(), p).unicode(),
                sq
            )?;
        }
        if let Some(p) = self.promotion {
            let p = ColorPiece::new(self.color_and_piece.color(), p);
            write!(f, "-{}", p.unicode())?;
        }
        if let Some(c) = self.castling {
            if self.from_to.to.index() < self.from_to.from.index() {
                write!(f, "-O-O-O")?;
            } else {
                write!(f, "-O-O")?;
            }
        }
        Ok(())
    }
}

#[test]
fn size_fuckery() {
    assert_eq!(std::mem::size_of::<Move>(), std::mem::size_of::<u64>());
    assert_eq!(
        std::mem::size_of::<Option<Move>>(),
        std::mem::size_of::<Move>()
    );
    assert_eq!(std::mem::align_of::<Move>(), 8);
}

impl BitBoard {
    pub fn apply(&mut self, mv: Move) {
        let (color, piece) = mv.color_and_piece.split();

        // update metadata
        self.metadata.to_move = color.other();
        if piece == Piece::Pawn || mv.capture.is_some() {
            self.metadata.last_change = self.metadata.tempo;
        }
        self.metadata.tempo += 1;
        self.metadata.en_passant = mv.en_passant_square();

        // calculate changes to castling rights
        let (cr_active, cr_passive) = mv.castling_rights(self.metadata.castling_details);

        let (active_castling, passive_castling) = self.metadata.castling_rights_mut(color);

        active_castling.update(cr_active);
        passive_castling.update(cr_passive);

        // calculate changes to board
        let (active, passive) = self.color_mut(color);
        if let Some((sq, piece)) = mv.capture {
            *passive.piece_mask_mut(piece) ^= sq.as_mask();
        }
        if let Some(p) = mv.promotion {
            *active.piece_mask_mut(Piece::Pawn) ^= mv.from_to.from.as_mask();
            *active.piece_mask_mut(p) ^= mv.from_to.to.as_mask();
        } else {
            *active.piece_mask_mut(mv.color_and_piece.piece()) ^= mv.from_to.as_mask();
        }
        match mv.castling {
            Some(pm) => *active.piece_mask_mut(Piece::Rook) ^= pm.as_mask(),
            None => {}
        }
    }

    pub fn generate_moves(&self, res: &mut Vec<Move>) {
        let active_mask = self.active().as_mask();
        let passive_mask = self.passive().as_mask();
        let color = self.metadata.to_move;

        self.active().queens.enumerate_legal_moves(
            color,
            active_mask,
            self.passive(),
            passive_mask,
            self.active().kings,
            res,
        );

        self.active().rooks.enumerate_legal_moves(
            color,
            active_mask,
            self.passive(),
            passive_mask,
            self.active().kings,
            res,
        );

        self.active().bishops.enumerate_legal_moves(
            color,
            active_mask,
            self.passive(),
            passive_mask,
            self.active().kings,
            res,
        );

        self.active().knights.enumerate_legal_moves(
            color,
            active_mask,
            self.passive(),
            self.active().kings,
            res,
        );

        self.active().pawns.enumerate_legal_moves(
            color,
            active_mask,
            passive_mask,
            self.passive(),
            self.metadata.en_passant,
            self.active().kings,
            res,
        );

        self.active().kings.enumerate_legal_moves(
            color,
            active_mask,
            passive_mask,
            self.passive(),
            self.metadata.castling_rights(color).0,
            self.metadata.castling_details,
            res,
        );
    }
}
