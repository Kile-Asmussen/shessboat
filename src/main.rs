#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    io::{Write, stdin, stdout},
    str::FromStr,
    thread::sleep,
    time::{Duration, Instant},
};

use colored::Colorize;
use rand::{
    Rng, SeedableRng, rng,
    rngs::{StdRng, ThreadRng},
    seq::{IndexedRandom, SliceRandom},
};

use crate::{
    interactive::ShessInteractor,
    shessboard::{
        BitBoard, GameEnd,
        boardmap::{BoardMap, BoardMapIter},
        enums::{Color, ColorPiece, File, Piece, Rank, Shade},
        half::HalfBitBoard,
        masks::Mask,
        metadata::Metadata,
        moves::{Move, ProtoMove},
        notation::{self, Algebraic},
        pieces::{
            bishops::Bishops, kings::Kings, knights::Knights, pawns::Pawns, queens::Queens,
            rooks::Rooks, slide_move_stop,
        },
        repetions::ThreefoldRule,
        squares::Square,
        zobrist::{BitBoardHasher, HashResult},
    },
    shessboat::basic_minimax,
};

pub mod interactive;
pub mod shessboard;
pub mod shessboat;

fn main() {
    interactive_game();
}

fn check_best_move(depth: u16) {
    let board = BitBoard::new();
    let hasher = BitBoardHasher::new();
    let mut toplevel_moves = Vec::with_capacity(50);
    board.generate_moves(&mut toplevel_moves);

    let hash = hasher.hash_full(&board);
    let three = ThreefoldRule::start(hash);
    let mut nodes_searched = 0;

    let mut scratch = Vec::with_capacity(50);
    let mut new_scratch = Vec::with_capacity(50);

    for mv in &toplevel_moves {
        let mv = *mv;

        let mut b = board.clone();
        b.apply(mv);
        b.generate_moves(&mut scratch);

        let value = basic_minimax(
            depth,
            b,
            &scratch,
            &mut new_scratch,
            hash,
            &hasher,
            &three,
            &mut nodes_searched,
        );

        scratch.clear();

        println!("{} {} ({})", mv.from_to, value, nodes_searched);
        nodes_searched = 0;
    }
}

fn zobrist_hashing_check(n: usize) {
    let mut rng = StdRng::from_seed(*b"3.141592653589793238462643383279");
    let hasher = BitBoardHasher::new();
    let mut hashes = HashMap::<HashResult, BitBoard>::new();
    let mut engine = ShessInteractor::new();
    println!("\nRunning {n} random games...\n");

    for _ in 1..=n {
        engine.setup();
        let mut move_seq = vec![];
        let mut hash: HashResult = hasher.hash_full(&engine.board);

        while engine.victory() == None {
            let mv = *engine.moves.choose(&mut rng).unwrap();
            move_seq.push(mv);
            hash = hasher.delta(&engine.board.metadata, hash, mv);

            let c = engine.board.metadata.to_move;

            engine.apply_move(mv);

            let refhash = hasher.hash_full(&engine.board);

            if refhash != hash {
                println!("Inconsistency found:\n delta {hash:016X}\n ref-- {refhash:016X}");
                println!(" diff- {:016X}", hash ^ refhash);
                let mut boardmap = BoardMap::new_with(None);
                engine.board.render(&mut boardmap);
                print_chessboard(&boardmap, Mask::nil());
                println!(
                    "Move sequence {}",
                    move_seq
                        .iter()
                        .map(|m| m.from_to.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                );
                return;
            }

            if hashes.contains_key(&refhash) {
                let mut e = engine.board.clone();
                let q = hashes[&refhash].clone();
                e.metadata.tempo = q.metadata.tempo;
                e.metadata.last_change = q.metadata.last_change;
                if e != q {
                    println!("Colission found!");
                    let mut boardmap = BoardMap::new_with(None);
                    engine.board.render(&mut boardmap);
                    print_chessboard(&boardmap, Mask::nil());
                    println!("{}", engine.printable_metadata());

                    boardmap = BoardMap::new_with(None);
                    hashes[&refhash].render(&mut boardmap);
                    print_chessboard(&boardmap, Mask::nil());
                    let mut e = ShessInteractor::new();
                    e.board = hashes[&refhash].clone();
                    println!("{}", e.printable_metadata());
                }
            } else {
                hashes.insert(refhash, engine.board.clone());
            }
        }
    }

    println!("Done!");
}

fn enumerate_moves_check(mvs: &[ProtoMove], mut depth: usize) {
    println!(
        "-- Depth checks to depth {depth} after {} --",
        mvs.iter()
            .map(|pm| pm.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );

    let mut board = BitBoard::new();

    for mv in mvs {
        let mv = *mv;

        let mut moves = vec![];
        board.generate_moves(&mut moves);

        if let Some(m) = moves.iter().find(|m| m.from_to == mv) {
            board.apply(*m);
        } else {
            let mut board_map = BoardMap::new_with(None);
            board.render(&mut board_map);
            print_chessboard(&board_map, mv.as_mask());
            println!("Illegal move: {}", mv);
            return;
        }
    }

    let mut moves = vec![];
    board.generate_moves(&mut moves);

    let mut sum = 0;
    for mv in moves {
        let mut b = board.clone();
        b.apply(mv);
        let n = recurse(b, depth);
        println!("{}: {}", mv.from_to, n);
        sum += n;
    }
    println!("Nodes searched: {}", sum);

    fn recurse(board: BitBoard, depth: usize) -> usize {
        let mut moves = Vec::with_capacity(50);
        if depth == 1 {
            return 1;
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
        engine.setup();

        while engine.victory() == None {
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
    let mut interactor = ShessInteractor::new();
    interactor.setup();
    let mut move_log = Vec::<(Algebraic, &'static str)>::new();
    let mut highlight = Mask::nil();

    'redraw: loop {
        print!("\x1B[2J\x1B[1;1H");
        stdout().flush();
        print_chessboard(&interactor.as_boardmap(), highlight);
        'command_loop: loop {
            let mut s = String::new();
            if let Some(vic) = interactor.victory() {
                print!("{}> ", vic.to_str());
            } else {
                print!("{:?}> ", interactor.to_move());
            }
            stdout().flush();
            stdin().read_line(&mut s);
            let command = s
                .trim()
                .split(|c: char| c.is_whitespace())
                .collect::<Vec<_>>();
            if command.len() < 1 {
                continue 'command_loop;
            }
            match command[0] {
                "exit" => {
                    break 'redraw;
                }
                "clear" => {
                    continue 'redraw;
                }
                "new" => {
                    highlight = Mask::nil();
                    interactor.setup();
                    move_log.clear();
                    continue 'redraw;
                }
                "reset" => {
                    highlight = Mask::nil();
                    interactor.reset();
                    move_log.clear();
                    continue 'redraw;
                }
                "threats" => {
                    highlight = if let Some(&"W" | &"w") = command.get(1) {
                        interactor.threat_mask(Color::White)
                    } else if let Some(&"B" | &"b") = command.get(1) {
                        interactor.threat_mask(Color::Black)
                    } else {
                        interactor.threat_mask(interactor.to_move())
                    };
                    continue 'redraw;
                }
                "q" => {
                    highlight = Mask::nil();
                    continue 'redraw;
                }
                "i" => {
                    if let (Some(p), Some(sq)) = (command.get(1), command.get(2)) {
                        if let (Some((p, "")), Some((sq, ""))) =
                            (ColorPiece::read(*p), Square::read(*sq))
                        {
                            interactor.place(Some(p), sq);
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
                        if let Some((sq, "")) = Square::read(sq) {
                            interactor.place(None, sq);
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
                    interactor.set_turn(Color::White, interactor.board.metadata.turn());
                    continue 'command_loop;
                }
                "b" => {
                    interactor.set_turn(Color::Black, interactor.board.metadata.turn());
                    continue 'command_loop;
                }
                "ls" => {
                    let legal_moves = interactor.printable_moves();
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
                        let Some(mv) = interactor.moves.choose(&mut rng) else {
                            continue 'redraw;
                        };
                        let not = Algebraic::new(&mv, &interactor.moves);
                        match interactor.normal_move(not) {
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
                    continue 'command_loop;
                }
                "clear" => {
                    move_log.clear();
                    continue 'command_loop;
                }
                "meta" => {
                    println!("{}", interactor.printable_metadata());
                    continue 'command_loop;
                }
                "cast" => {
                    if let Some(&"W" | &"w") = command.get(1) {
                        interactor.board.metadata.white_castling.ooo = command.contains(&"ooo");
                        interactor.board.metadata.white_castling.oo = command.contains(&"oo");
                        interactor.recalc();
                    } else if let Some(&"B" | &"b") = command.get(1) {
                        interactor.board.metadata.black_castling.ooo = command.contains(&"ooo");
                        interactor.board.metadata.black_castling.oo = command.contains(&"oo");
                        interactor.recalc();
                    }
                    continue 'command_loop;
                }
                s => {
                    if let Some((n, "")) = Algebraic::read(s) {
                        match interactor.normal_move(n) {
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
