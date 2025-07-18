#![allow(unused)]

use std::{
    io::{Write, stdin, stdout},
    thread::sleep,
    time::Duration,
};

use colored::Colorize;
use rand::{rng, seq::SliceRandom};

use crate::{
    engine::ShessEngine,
    shessboard::{
        algebraic::Notation,
        boardmap::BoardMap,
        enums::{Color, ColorPiece, File, Piece, Rank, Shade},
        masks::Mask,
        moves::ProtoMove,
        pieces::{
            bishops::Bishops, kings::Kings, knights::Knights, pawns::Pawns, queens::Queens,
            rooks::Rooks, slide_move_stop,
        },
        squares::Square,
    },
};

pub mod engine;
pub mod shessboard;

fn main() {
    let mut engine = ShessEngine::new();
    engine.set_position(518);
    let mut move_log = Vec::<(Notation, &'static str)>::new();
    let mut highlight = Mask::nil();

    loop {
        print!("\x1B[2J\x1B[1;1H");
        stdout().flush();
        print_chessboard(&engine.as_boardmap(), highlight);
        loop {
            let mut s = String::new();
            if engine.moves.len() == 0 {
                let res = if engine.board.is_in_check(engine.to_move()) {
                    match engine.to_move() {
                        Color::White => "0–1",
                        Color::Black => "1–0",
                    }
                } else {
                    "½–½"
                };
                print!("{}> ", res);
            } else {
                print!("{:?}> ", engine.to_move());
            }
            stdout().flush();
            stdin().read_line(&mut s);
            let s = s.trim().split(|c: char| c.is_whitespace()).collect::<Vec<_>>();
            if s.len() < 1 {
                continue;
            }
            match s[0] {
                "exit" => {
                    return;
                }
                "clear" => {
                    break;
                }
                "pos" => {
                    if let Some(n) = s.get(1) {
                        if let Ok(n) = n.parse() {
                            engine.set_position(n);
                            move_log.clear();
                            highlight = Mask::nil();
                            break;
                        } else {
                            println!("Invalid number");
                            continue;
                        }
                    } else {
                        engine.set_position(518);
                        move_log.clear();
                        highlight = Mask::nil();
                        break;
                    }
                }
                "reset" => {
                    highlight = Mask::nil();
                    engine.reset();
                    move_log.clear();
                    break;
                }
                "threats" => {
                    highlight = if let Some(&"W" | &"w") = s.get(1) {
                        engine.threat_mask(Color::White)
                    } else if let Some(&"B" | &"b") = s.get(1) {
                        engine.threat_mask(Color::Black)
                    } else {
                        engine.threat_mask(engine.to_move())
                    };
                    break;
                }
                "q" => {
                    highlight = Mask::nil();
                    break;
                }
                "i" => {
                    if let (Some(p), Some(sq)) = (s.get(1), s.get(2)) {
                        if let (Some(p), Some(sq)) =
                            (ColorPiece::from_str(*p), Notation::read_square(*sq))
                        {
                            engine.place(Some(p), sq);
                            break;
                        } else {
                            println!("Format: <piece letter> <square>");
                            continue;
                        }
                    } else {
                        println!("Format: <piece letter> <square>");
                        continue;
                    }
                }
                "d" => {
                    if let Some(sq) = s.get(1) {
                        if let Some(sq) = Notation::read_square(sq) {
                            engine.place(None, sq);
                            break;
                        } else {
                            println!("Format: <square>");
                            continue;
                        }
                    } else {
                        println!("Format: <square>");
                        continue;
                    }
                }
                "ls" => {
                    let legal_moves = engine.printable_moves();
                    if legal_moves.len() == 0 {
                        println!("No legal moves");
                    }
                    for mvs in legal_moves.chunks(8) {
                        for mv in mvs {
                            print!("{}", mv);
                        }
                        println!();
                    }
                    continue;
                }
                "log" => {
                    for (n, mv) in move_log.chunks(2).enumerate() {
                        print!("{}. ", n + 1);
                        for m in mv {
                            print!("{}{} ", m.0, m.1);
                        }
                        println!();
                    }
                }
                "meta" => {
                    println!("{}", engine.printable_metadata());
                    continue;
                }
                "cast" => {
                    if let Some(&"W" | &"w") = s.get(1) {
                        engine.board.metadata.white_castling.ooo = s.contains(&"ooo");
                        engine.board.metadata.white_castling.oo = s.contains(&"oo");
                    } else if let Some(&"B" | &"b") = s.get(1) {
                        engine.board.metadata.black_castling.ooo = s.contains(&"ooo");
                        engine.board.metadata.black_castling.oo = s.contains(&"oo");
                    }
                }
                "mv" => {
                    if let (Some(sq1), Some(sq2)) = (s.get(1), s.get(2)) {
                        if let (Some(sq1), Some(sq2)) =
                            (Notation::read_square(*sq1), Notation::read_square(*sq2))
                        {
                            engine.cheat_move(ProtoMove { from: sq1, to: sq2 });
                            break;
                        } else {
                            println!("Format: <square> <square>");
                            continue;
                        }
                    } else {
                        println!("Format: <square> <square>");
                        continue;
                    }
                }
                s => {
                    if let Some(n) = Notation::read(s) {
                        match engine.normal_move(n) {
                            Ok(ns) => {
                                move_log.push(ns.0);
                                highlight = ns.1.from_to.as_mask();
                                break;
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                                continue;
                            }
                        }
                    } else {
                        println!("Unrecognized command");
                        continue;
                    }
                }
            }
        }
    }
}

fn print_chessboard(pieces: &BoardMap<Option<ColorPiece>>, highlights: Mask) {
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

            let bg_color = if highlights.contains(sq) {
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
