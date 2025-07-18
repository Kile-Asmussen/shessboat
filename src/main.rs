#![allow(unused)]

use std::{io::stdin, thread::sleep, time::Duration};

use colored::Colorize;
use rand::{rng, seq::SliceRandom};

use crate::shessboard::{
    algebraic::Notation,
    boardmap::BoardMap,
    enums::{Color, ColorPiece, File, Rank, Shade},
    masks::Mask,
    pieces::{
        bishops::Bishops, kings::Kings, knights::Knights, pawns::Pawns, queens::Queens,
        rooks::Rooks, slide_move_stop,
    },
    squares::Square,
};

pub mod engine;
pub mod shessboard;

fn main() {
    let mut board = shessboard::BitBoard::new();
    let mut move_log = vec![];
    let mut rng = rng();
    let mut last_move = Mask::nil();
    let mut winner = None;

    loop {
        let mut print = BoardMap::new_with(None);
        let mut highlight = BoardMap::new_with(false);
        board.render(&mut print);
        last_move.render(&mut highlight);
        print_chessboard(&print, &highlight);

        let mut moves = vec![];
        board.generate_moves(&mut moves);

        if moves.len() == 0 {
            if (board.passive().threats(board.metadata.to_move, board.active().as_mask(), None)
                & board.active().kings.as_mask())
            .any()
            {
                winner = Some(board.metadata.to_move);
                break;
            } else {
                break;
            }
        }

        // sleep(Duration::from_millis(500));

        &mut moves[..].shuffle(&mut rng);

        let mv = moves.first().unwrap().clone();
        last_move = mv.from_to.as_mask();

        let notated = Notation::disambiguate(mv, &moves);
        println!("> {}", notated);

        board.apply(&mv);
        move_log.push((notated, if board.is_in_check() { "+" } else { "" }));
    }

    if winner.is_some() {
        move_log.last_mut().unwrap().1 = "#";
    }

    for (turn, moves) in move_log.chunks(2).enumerate() {
        print!("{}. ", turn + 1);
        for (m, x) in moves {
            print!("{}{} ", m, x);
        }
        println!();
    }

    match winner {
        Some(Color::White) => println!("1–0"),
        Some(Color::Black) => println!("0–1"),
        None => println!("½–½"),
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
