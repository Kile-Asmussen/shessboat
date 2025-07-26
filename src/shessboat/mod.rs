use std::hash::Hash;

use crate::shessboard::{
    BitBoard, GameEnd,
    boardmap::BoardMap,
    moves::Move,
    pieces::{Millipawns, P},
    repetions::ThreefoldRule,
    zobrist::{BitBoardHasher, HashResult},
};

// trait Minimaxer {
//     fn hasher(&self) -> &BitBoardHasher;
//     fn memo_zobrist(&mut self, hash: HashResult, value: Millipawns);

//     fn static_evaluation(&mut self, board: &BitBoard) -> Millipawns;

//     fn minimax(&mut self, depth: u16, board: &BitBoard, hash: HashResult, moves: &[Move]) -> (Millipawns, Option<Move>) {
//         let mut scratch = Vec::with_capacity(moves.len());
//         let hash = self.hasher().hash_full(board);

//         let mut max_val = GameEnd::DEFEAT;
//         let mut max_move = None;

//         for mv in moves {
//             let mv = *mv;
//             let mut b = board.clone();
//             b.apply(mv);
//             self.hasher().delta(board, hash, mv)
//             b.generate_moves(&mut scratch);

//             let val = self.minimax(depth, &b, hash, &scratch);

//             if val > max_val {
//                 max_val = max_val;
//                 max_move = Some(mv);
//             }

//             scratch.clear();
//         }

//         (max_val, max_move)
//     }
// }

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
) -> Millipawns {
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
        let new_hash = hasher.delta(&board.metadata, hash, mv);
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

pub fn static_evaluation(board: BitBoard) -> Millipawns {
    board.white.materiel() - board.black.materiel()
}
