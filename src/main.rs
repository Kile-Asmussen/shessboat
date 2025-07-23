#![allow(unused)]

use std::{
    io::{Write, stdin, stdout},
    str::FromStr,
    thread::sleep,
    time::{Duration, Instant},
};

use colored::Colorize;
use rand::{
    Rng, rng,
    rngs::ThreadRng,
    seq::{IndexedRandom, SliceRandom},
};

use crate::{
    interactive::ShessInteractor,
    shessboard::{
        BitBoard, Metadata,
        boardmap::{BoardMap, BoardMapIter},
        enums::{Color, ColorPiece, File, Piece, Rank, Shade},
        half::HalfBitBoard,
        masks::Mask,
        moves::{Move, ProtoMove},
        notation::{self, Algebraic},
        pieces::{
            bishops::Bishops, kings::Kings, knights::Knights, pawns::Pawns, queens::Queens,
            rooks::Rooks, slide_move_stop,
        },
        squares::Square,
    },
};

pub mod engine;
pub mod interactive;
pub mod shessboard;

fn main() {
    random_games_move_enumeration_benchmark(10_000);
}

fn enumerate_moves_check(mut depth: usize) {
    println!("-- Depth checks to depth {depth} --");

    for i in 1..=depth {
        println!("Depth {i}: {}", recurse(BitBoard::new(), i));
    }

    fn recurse(board: BitBoard, depth: usize) -> usize {
        let mut moves = Vec::with_capacity(50);
        if depth <= 1 {
            board.generate_moves(&mut moves);
            return moves.len();
        } else {
            board.generate_moves(&mut moves);
            let mut sum = 0;
            for mv in moves {
                let mut b = board.clone();
                b.apply(mv);
                sum += recurse(b, depth - 1);
            }
            return sum;
        }
    }
}

fn random_games_move_enumeration_benchmark(n: usize) {
    let mut rng = ThreadRng::default();
    let mut engine = ShessInteractor::new();
    let mut longboard = BoardMap::<Option<ColorPiece>>::new_with(None);
    let mut longcolor = Color::White;
    let mut longsearch = Duration::ZERO;
    let mut longmoves = vec![];
    let mut moves = vec![];
    println!("\n  Running {n} random games...\n");

    for _ in 1..=n {
        engine.set_position(rng.random_range(1..=960));

        while engine.moves.len() > 0
            && !engine.board.only_kings()
            && engine.board.metadata.turn() < 1000
        {
            let mv = engine.moves.choose(&mut rng).unwrap().clone();
            let now = Instant::now();
            engine.apply_move(mv);
            let delta = now.elapsed();
            if delta > longsearch {
                longsearch = delta;
                longboard.reset();
                engine.board.render(&mut longboard);
                longcolor = engine.to_move().other();
                longmoves = engine.moves.clone();
            }
            moves.push((delta, engine.moves.len()));
        }
    }

    println!(
        "  Longest move enumeration took {:.3} us with total of {} legal moves for {:?}.",
        longsearch.as_nanos() as f64 / 1000.0,
        longmoves.len(),
        longcolor
    );
    print_chessboard(&longboard, Mask::nil());
    for mv in longmoves.chunks(10) {
        print!("  ");
        for m in mv {
            print!("{:<8}", Algebraic::new(m, &longmoves).to_string());
        }
        println!();
    }

    let avg_duration = moves.iter().map(|(d, _)| d).sum::<Duration>() / moves.len() as u32;

    println!(
        "\n  Average move enumeration duration was {:.3} us.",
        avg_duration.as_nanos() as f64 / 1000.0
    );

    println!()
}

fn interactive_game() {
    let mut rng = ThreadRng::default();
    let mut engine = ShessInteractor::new();
    engine.set_position(518);
    let mut move_log = Vec::<(Algebraic, &'static str)>::new();
    let mut highlight = Mask::nil();

    'redraw: loop {
        print!("\x1B[2J\x1B[1;1H");
        stdout().flush();
        print_chessboard(&engine.as_boardmap(), highlight);
        'command_loop: loop {
            let mut s = String::new();
            if engine.moves.len() == 0 || engine.board.only_kings() {
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
            let command = s
                .trim()
                .split(|c: char| c.is_whitespace())
                .collect::<Vec<_>>();
            if command.len() < 1 {
                continue;
            }
            match command[0] {
                "exit" => {
                    break 'redraw;
                }
                "clear" => {
                    continue 'redraw;
                }
                "pos" => {
                    if let Some(n) = command.get(1) {
                        if let Ok(n) = n.parse() {
                            engine.set_position(n);
                            move_log.clear();
                            highlight = Mask::nil();
                            continue 'redraw;
                        } else {
                            println!("Invalid number");
                            continue 'command_loop;
                        }
                    } else {
                        engine.set_position(518);
                        move_log.clear();
                        highlight = Mask::nil();
                        continue 'redraw;
                    }
                }
                "reset" => {
                    highlight = Mask::nil();
                    engine.reset();
                    move_log.clear();
                    continue 'redraw;
                }
                "threats" => {
                    highlight = if let Some(&"W" | &"w") = command.get(1) {
                        engine.threat_mask(Color::White)
                    } else if let Some(&"B" | &"b") = command.get(1) {
                        engine.threat_mask(Color::Black)
                    } else {
                        engine.threat_mask(engine.to_move())
                    };
                    continue 'redraw;
                }
                "q" => {
                    highlight = Mask::nil();
                    continue 'redraw;
                }
                "i" => {
                    if let (Some(p), Some(sq)) = (command.get(1), command.get(2)) {
                        if let (Some(p), Some(sq)) =
                            (ColorPiece::from_str(*p), Algebraic::read_square(*sq))
                        {
                            engine.place(Some(p), sq);
                            continue 'redraw;
                        } else {
                            println!("Format: <piece letter> <square>");
                            continue 'command_loop;
                        }
                    } else {
                        println!("Format: <piece letter> <square>");
                        continue 'command_loop;
                    }
                }
                "d" => {
                    if let Some(sq) = command.get(1) {
                        if let Some(sq) = Algebraic::read_square(sq) {
                            engine.place(None, sq);
                            continue 'redraw;
                        } else {
                            println!("Format: <square>");
                            continue 'command_loop;
                        }
                    } else {
                        println!("Format: <square>");
                        continue 'command_loop;
                    }
                }
                "w" => {
                    engine.set_turn(Color::White, engine.board.metadata.turn());
                }
                "b" => {
                    engine.set_turn(Color::Black, engine.board.metadata.turn());
                }
                "ls" => {
                    let legal_moves = engine.printable_moves();
                    if legal_moves.len() == 0 {
                        println!("No legal moves");
                    }
                    for mvs in legal_moves.chunks(8) {
                        for mv in mvs {
                            print!("{} ", mv);
                        }
                        println!();
                    }
                    continue 'command_loop;
                }
                "r" => {
                    let n = if let Some(n) = command.get(1) {
                        (*n).parse().unwrap_or(1)
                    } else {
                        1
                    };
                    for _ in 1..=n {
                        let Some(mv) = engine.moves.choose(&mut rng) else {
                            continue 'redraw;
                        };
                        let not = Algebraic::new(&mv, &engine.moves);
                        match engine.normal_move(not) {
                            Ok(ns) => {
                                move_log.push(ns.0);
                            }
                            Err(e) => {}
                        }
                    }
                    continue 'redraw;
                }
                "log" => {
                    if let Some(&"clear") = command.get(1) {
                        move_log.clear();
                    } else {
                        for (n, mv) in move_log.chunks(2).enumerate() {
                            print!("{}. ", n + 1);
                            for m in mv {
                                print!("{}{} ", m.0, m.1);
                            }
                            println!();
                        }
                    }
                }
                "meta" => {
                    println!("{}", engine.printable_metadata());
                    continue 'command_loop;
                }
                "cast" => {
                    if let Some(&"W" | &"w") = command.get(1) {
                        engine.board.metadata.white_castling.ooo = command.contains(&"ooo");
                        engine.board.metadata.white_castling.oo = command.contains(&"oo");
                    } else if let Some(&"B" | &"b") = command.get(1) {
                        engine.board.metadata.black_castling.ooo = command.contains(&"ooo");
                        engine.board.metadata.black_castling.oo = command.contains(&"oo");
                    }
                }
                s => {
                    if let Some(n) = Algebraic::read(s) {
                        match engine.normal_move(n) {
                            Ok(ns) => {
                                move_log.push(ns.0);
                                highlight = ns.1.from_to.as_mask();
                                continue 'redraw;
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                                continue 'command_loop;
                            }
                        }
                    } else {
                        println!("Unrecognized command");
                        continue 'command_loop;
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
        print!("  ");
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
                format!(" {} ", print_char)
                    .color(fg_color)
                    .on_color(bg_color)
            )
        }
        println!(" {}", rank.as_rank() + 1);
    }
    print!("  ");
    for c in "abcdefgh".chars() {
        print!(" {} ", c);
    }
    println!();
}
