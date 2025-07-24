use crate::shessboard::{
    boardmap::BoardMap,
    pieces::{Micropawns, P},
};

const OPENING_INCENTIVE: (BoardMap<Micropawns>, BoardMap<Micropawns>) = {
    BoardMap::board_and_mirror(&[
        [0; 8],
        [0; 8],
        [0; 8],
        [0; 8],
        [0; 8],
        [0; 8],
        [0; 8],
        [0, -1 * P, -1 * P, -1 * P, 0, -1 * P, -1 * P, 0],
    ])
};

const PAWN_POSITION: (BoardMap<Micropawns>, BoardMap<Micropawns>) = {
    BoardMap::board_and_mirror(&[
        [0; 8],
        [10 * P / 10; 8],
        [5 * P / 10; 8],
        [3 * P / 10; 8],
        [2 * P / 10; 8],
        [1 * P / 10; 8],
        [0; 8],
        [0; 8],
    ])
};

const KING_SAFETY_INCENTIVE: (BoardMap<Micropawns>, BoardMap<Micropawns>) = {
    BoardMap::board_and_mirror(&[
        [0; 8], [0; 8], [0; 8], [0; 8], [0; 8], [0; 8], [0; 8], [0; 8],
    ])
};
