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
    scratch: &mut Vec<Move>,
    hash: HashResult,
    hasher: &BitBoardHasher,
    threefold: &ThreefoldRule<'a>,
) -> Micropawns {
    board.generate_moves(&mut scratch);


    if let Some(ge) = GameEnd::determine(&board, &scratch) {
        return ge.value(board.metadata.to_move);
    }

    if threefold.count(hash ^ hasher.hash_to_move(board.metadata.to_move)) == 3 {
        return 0;
    }

    if depth == 0 {
        board.white.materiel() - board.black.materiel()
    } else {
        let mut new_scratch = Vec::with_capacity(scratch.capacity());
        board.generate_moves(scratch);


        if 

        let mut max_val = GameEnd::DEFEAT;

        for mv in scratch {
            let mv = *mv;
            let mv_hash = hasher.delta_hash_move(&board, hash, mv);
            let seen = threefold.see(mv_hash);
            let mut b = board.clone();
            b.apply(mv);
            let val = -basic_minimax(depth - 1, b, &mut new_scratch, mv_hash, hasher, &seen);
            if val > max_val {
                max_val = val
            }
            new_scratch.clear();
        }

        return max_val;
    }
}
