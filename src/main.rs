#![allow(unused)]

use colored::Colorize;

use crate::bitboard::{
    enums::{Rank, Shade},
    masks::Mask,
};

pub mod bitboard;

fn main() {
    let board = bitboard::BitBoard::default();

    let mut print = [' '; 64];
    let mut highlight = [false; 64];

    board.render(&mut print, &mut highlight);

    print_chessboard(&print, &highlight);
}

fn print_chessboard(pieces: &[char; 64], highlights: &[bool; 64]) {
    let chessboard = [
        Rank::_8.as_mask(),
        Rank::_7.as_mask(),
        Rank::_6.as_mask(),
        Rank::_5.as_mask(),
        Rank::_4.as_mask(),
        Rank::_3.as_mask(),
        Rank::_2.as_mask(),
        Rank::_1.as_mask(),
    ];
    for rank in chessboard {
        for sq in rank.iter() {
            let piece = pieces[sq.index() as usize];
            let fg_color = match piece {
                '\u{2654}'..='\u{2659}' => colored::Color::White,
                '\u{255A}'..='\u{265F}' => colored::Color::Black,
                _ => colored::Color::Red,
            };

            let bg_color = if highlights[sq.index() as usize] {
                colored::Color::Red
            } else {
                if (sq.as_mask() & Shade::Light.as_mask()).any() {
                    colored::Color::BrightGreen
                } else if (sq.as_mask() & Shade::Dark.as_mask()).any() {
                    colored::Color::Green
                } else {
                    colored::Color::Yellow
                }
            };

            let print_char = match piece {
                c @ '\u{2654}'..='\u{2659}' => unsafe { char::from_u32_unchecked(c as u32 + 6) },
                c => c,
            };

            print!(
                "{}",
                print_char.to_string().color(fg_color).on_color(bg_color)
            )
        }
        println!();
    }
}
