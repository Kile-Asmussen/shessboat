#![allow(unused)]

use colored::Colorize;

use crate::bitboard::{
    boardmap::BoardMap,
    enums::{File, Rank, Shade},
    masks::Mask,
    pieces::{kings::Kings, knights::Knights},
    squares::Square,
};

pub mod bitboard;

fn main() {
    let board = bitboard::BitBoard::default();

    let mut print = BoardMap::default();
    let mut highlight = BoardMap::default();

    board.render(&mut print);

    let mask = Kings::moves_from(Square::at(File::E, Rank::_5));

    mask.render(&mut highlight);

    print_chessboard(&print, &highlight);
}

#[test]
fn sizeof_option_i32() {
    assert_eq!(std::mem::size_of::<Option<i32>>(), 8);
}

fn print_chessboard(pieces: &BoardMap<char>, highlights: &BoardMap<bool>) {
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
                'A'..='Z' | ' ' => colored::Color::TrueColor {
                    r: 0xFF,
                    g: 0xFF,
                    b: 0xFF,
                },
                'a'..='z' => colored::Color::TrueColor {
                    r: 0x00,
                    g: 0x00,
                    b: 0x00,
                },
                _ => panic!(),
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

            let print_char = match piece {
                'K' | 'k' => '\u{265A}',
                'Q' | 'q' => '\u{265B}',
                'R' | 'r' => '\u{265C}',
                'B' | 'b' => '\u{265D}',
                'N' | 'n' => '\u{265E}',
                'P' | 'p' => '\u{265F}',
                c => {
                    fg_color = colored::Color::TrueColor {
                        r: 0xFF,
                        g: 0,
                        b: 0,
                    };
                    c
                }
            };

            print!(
                "{}",
                format!(" {} ", print_char)
                    .color(fg_color)
                    .on_color(bg_color)
            )
        }
        println!(" {}", rank.as_rank() + 1);
    }
    for c in "abcdefgh".chars() {
        print!(" {} ", c);
    }
    println!();
}
