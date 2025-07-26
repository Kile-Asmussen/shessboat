use std::hash::Hash;

mod heuristics;

use crate::shessboard::{
    BitBoard, GameEnd,
    boardmap::BoardMap,
    forced_draws::{LastChange, ThreefoldRule},
    moves::Move,
    pieces::{Millipawns, P},
    zobrist::{BitBoardHasher, HashResult},
};

trait Minimax {
    fn order_moves(&mut self, moves: &mut Vec<Move>);
    fn memorize(&mut self, hash: HashResult, value: Millipawns);
    fn seen_before(&mut self, hash: HashResult) -> Option<Millipawns>;
    fn static_evaluation(&mut self, board: &BitBoard) -> Millipawns;
}
