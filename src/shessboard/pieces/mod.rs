use core::panic;
use std::{collections::BTreeSet, sync::atomic::AtomicUsize};

use crate::shessboard::{
    enums::{ColorPiece, Piece},
    masks::Mask,
};

pub mod bishops;
pub mod kings;
pub mod knights;
pub mod pawns;
pub mod queens;
pub mod rooks;

pub type Micropawns = i64;

pub const fn slide_move_stop(
    positive: bool,
    move_mask: Mask,
    same_color: Mask,
    opposite_color: Mask,
) -> Mask {
    let mut move_mask = move_mask.as_u64();
    let mut same_color = same_color.as_u64();
    let mut opposite_color = opposite_color.as_u64();

    if !positive {
        move_mask = move_mask.reverse_bits();
        same_color = same_color.reverse_bits();
        opposite_color = opposite_color.reverse_bits();
    }

    let same_color_on_move_mask = move_mask & same_color;
    let opposite_color_on_move_mask = move_mask & opposite_color;

    let allowed_by_same_color =
        move_mask & ((same_color_on_move_mask.wrapping_sub(1)) & !same_color_on_move_mask);
    let allowed_by_opposite_color =
        move_mask & ((opposite_color_on_move_mask.wrapping_sub(1)) ^ opposite_color_on_move_mask);
    let allowed = allowed_by_opposite_color & allowed_by_same_color;

    Mask::new(if positive {
        allowed
    } else {
        allowed.reverse_bits()
    })
}

pub fn chess_960(mut frcn: usize) -> [Piece; 8] {
    let mut res = [None; 8];
    frcn %= 960;

    let bl = frcn % 4 * 2 + 1;
    res[bl] = Some(Piece::Bishop);
    frcn /= 4;

    let bd = frcn % 4 * 2;
    res[bd] = Some(Piece::Bishop);
    frcn /= 4;

    let mut q = frcn % 6;
    skip_over(q, Piece::Queen, &mut res);

    frcn /= 6;
    let (mut n1, mut n2) = match frcn {
        x @ 0..=3 => (0, x),
        x @ 4..=6 => (1, x - 3),
        x @ 7..=8 => (2, x - 5),
        9 => (3, 3),
        _ => panic!(),
    };

    skip_over(n1, Piece::Knight, &mut res);
    skip_over(n2, Piece::Knight, &mut res);
    skip_over(0, Piece::Rook, &mut res);
    skip_over(0, Piece::King, &mut res);
    skip_over(0, Piece::Rook, &mut res);

    return res.map(|x| x.unwrap());

    fn skip_over(mut n: usize, c: Piece, res: &mut [Option<Piece>; 8]) {
        for x in &mut res[..] {
            if x.is_none() && n == 0 {
                *x = Some(c);
                break;
            } else if x.is_none() {
                n -= 1;
            }
        }
    }
}

#[test]
fn chess960_known_positions() {
    assert_eq!(s(chess_960(960)), "BBQNNRKR");
    assert_eq!(s(chess_960(1)), "BQNBNRKR");
    assert_eq!(s(chess_960(518)), "RNBQKBNR");

    fn s(a: [Piece; 8]) -> String {
        a.iter().map(|p| p.white_letter()).collect()
    }
}
