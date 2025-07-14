use core::panic;
use std::collections::BTreeSet;

use crate::bitboard::masks::Mask;

pub mod bishops;
pub mod kings;
pub mod knights;
pub mod pawns;
pub mod queens;
pub mod rooks;

pub type Micropawns = isize;

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

pub fn chess_960(mut frcn: usize) -> [char; 8] {
    let mut res = ['.'; 8];
    frcn %= 960;

    let bl = frcn % 4 * 2 + 1;
    res[bl] = 'B';
    frcn /= 4;

    let bd = frcn % 4 * 2;
    res[bd] = 'B';
    frcn /= 4;

    let mut q = frcn % 6;
    skip_over(q, 'Q', &mut res);

    frcn /= 6;
    let (mut n1, mut n2) = match frcn {
        x @ 0..=3 => (0, x),
        x @ 4..=6 => (1, x - 3),
        x @ 7..=8 => (2, x - 5),
        9 => (3, 3),
        _ => panic!(),
    };

    skip_over(n1, 'N', &mut res);
    skip_over(n2, 'N', &mut res);
    skip_over(0, 'R', &mut res);
    skip_over(0, 'K', &mut res);
    skip_over(0, 'R', &mut res);

    return res;

    fn skip_over(mut n: usize, c: char, res: &mut [char; 8]) {
        for x in &mut res[..] {
            if *x == '.' && n == 0 {
                *x = c;
                break;
            } else if *x == '.' {
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

    fn s(a: [char; 8]) -> String {
        a.iter().collect()
    }
}
