use crate::shessboard::{
    BitBoard, GameEnd,
    boardmap::BoardMap,
    moves::Move,
    pieces::{Micropawns, P},
    repetions::ThreefoldRule,
    zobrist::{BitBoardHasher, HashResult},
};

mod heuristics;

fn basic_minimax<'a>(
    depth: u16,
    board: BitBoard,
    moves: &[Move],
    scratch: &mut Vec<Move>,
    hash: HashResult,
    hasher: &BitBoardHasher,
    threefold: &ThreefoldRule<'a>,
) -> Micropawns {
    if let Some(ge) = GameEnd::determine(&board, &moves, hash, threefold) {
        return ge.value(board.metadata.to_move);
    }

    if depth == 0 {
        return static_evaluation(board);
    } else {
        let mut max_val = GameEnd::DEFEAT;
        let mut new_scratch = Vec::with_capacity(scratch.capacity());

        for mv in moves {
            let mv = *mv;
            let new_hash = hasher.delta_hash_move(&board, hash, mv);
            let seen = threefold.see(new_hash);
            let mut b = board.clone();
            b.apply(mv);
            b.generate_moves(scratch);
            let val = -basic_minimax(
                depth - 1,
                b,
                &scratch,
                &mut new_scratch,
                new_hash,
                hasher,
                &seen,
            );
            if val > max_val {
                max_val = val
            }
            scratch.clear();
        }

        return max_val;
    }
}

pub fn static_evaluation(board: BitBoard) -> Micropawns {
    board.white.materiel() - board.black.materiel()
}
