use crate::shessboard::{
    BitBoard, GameEnd,
    boardmap::BoardMap,
    moves::Move,
    pieces::{Micropawns, P},
    repetions::ThreefoldRule,
    zobrist::{BitBoardHasher, HashResult},
};

mod heuristics;

pub fn basic_minimax<'a>(
    depth: u16,
    board: BitBoard,
    moves: &[Move],
    scratch: &mut Vec<Move>,
    hash: HashResult,
    hasher: &BitBoardHasher,
    threefold: &ThreefoldRule<'a>,
    nodes_searched: &mut usize,
) -> Micropawns {
    *nodes_searched += 1;

    if let Some(ge) = GameEnd::determine(&board, &moves, hash, threefold) {
        return ge.value(board.metadata.to_move);
    }

    if depth == 0 {
        return static_evaluation(board);
    } else {
        return search_best(
            depth,
            board,
            moves,
            scratch,
            hash,
            hasher,
            threefold,
            nodes_searched,
        );
    }
}

fn search_best<'a>(
    depth: u16,
    board: BitBoard,
    moves: &[Move],
    scratch: &mut Vec<Move>,
    hash: u64,
    hasher: &BitBoardHasher,
    threefold: &ThreefoldRule<'a>,
    nodes_searched: &mut usize,
) -> i64 {
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
            nodes_searched,
        );

        new_scratch.clear();
        if val > max_val {
            max_val = val
        }
        scratch.clear();
    }
    max_val
}

pub fn static_evaluation(board: BitBoard) -> Micropawns {
    board.white.materiel() - board.black.materiel()
}
