#![allow(unused)]

use colored::Colorize;

use crate::bitboard::{
    boardmap::BoardMap,
    enums::{Color, ColorPiece, File, Rank, Shade},
    masks::Mask,
    pieces::{
        bishops::Bishops, kings::Kings, knights::Knights, pawns::Pawns, queens::Queens,
        rooks::Rooks, slide_move_stop,
    },
    squares::Square,
};

pub mod bitboard;

fn main() {
    let mut board = bitboard::BitBoard::new();

    let mut print: BoardMap<Option<ColorPiece>> = BoardMap::new_with(None);
    let mut highlight = BoardMap::default();

    board.metadata.to_move = Color::Black;
    board.black.pawns = Pawns::nil();

    let mut moves = vec![];
    board.generate_moves(&mut moves);

    board.render(&mut print);

    // let mask: Mask = board.white.threats(Color::White, board.black.as_mask());

    // mask.render(&mut highlight);

    print_chessboard(&print, &highlight);

    for m in moves {
        println!("{}", m);
    }
}

#[test]
fn sizeof_option_i32() {
    assert_eq!(std::mem::size_of::<Option<i32>>(), 8);
}

fn print_chessboard(pieces: &BoardMap<Option<ColorPiece>>, highlights: &BoardMap<bool>) {
    let chessboard = [
        Rank::_8,
        Rank::_7,
        Rank::_6,
        Rank::_5,
        Rank::_4,
        Rank::_3,
        Rank::_2,
        Rank::_1,
    ];
    for rank in chessboard {
        for sq in rank.as_mask().iter() {
            let piece = pieces.at(sq);
            let mut fg_color = match piece {
                Some(p) if p.color() == Color::White => colored::Color::TrueColor {
                    r: 0xFF,
                    g: 0xFF,
                    b: 0xFF,
                },
                _ => colored::Color::TrueColor {
                    r: 0x00,
                    g: 0x00,
                    b: 0x00,
                },
            };

            let bg_color = if highlights.at(sq) {
                colored::Color::TrueColor {
                    r: 0xAF,
                    g: 0x7F,
                    b: 0x7F,
                }
            } else {
                if (sq.as_mask() & Shade::Light.as_mask()).any() {
                    colored::Color::TrueColor {
                        r: 0x9F,
                        g: 0x9F,
                        b: 0x9F,
                    }
                } else if (sq.as_mask() & Shade::Dark.as_mask()).any() {
                    colored::Color::TrueColor {
                        r: 0x5F,
                        g: 0x5F,
                        b: 0x5F,
                    }
                } else {
                    panic!()
                }
            };

            let print_char = if let Some(piece) = piece {
                piece.piece().black_unicode()
            } else {
                ' '
            };

            print!(
                "{}",
                format!(" {} ", print_char).color(fg_color).on_color(bg_color)
            )
        }
        println!(" {}", rank.as_rank() + 1);
    }
    for c in "abcdefgh".chars() {
        print!(" {} ", c);
    }
    println!();
}
