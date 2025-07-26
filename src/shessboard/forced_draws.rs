use std::collections::HashMap;

use crate::shessboard::{
    enums::ColorPiece,
    moves::Move,
    zobrist::{BitBoardHasher, HashResult},
};

#[derive(Clone, Copy)]
pub enum LastChange<'a> {
    Static(u16),
    Speculative(u16, &'a LastChange<'a>),
}

impl LastChange<'static> {
    pub fn start() -> Self {
        Self::Static(0)
    }
}

impl<'a> LastChange<'a> {
    pub fn see(&'a self, tempo: u16, mv: Move) -> Self {
        if mv.capture.is_some()
            || mv.color_and_piece == ColorPiece::BlackPawn
            || mv.color_and_piece == ColorPiece::WhitePawn
        {
            Self::Speculative(tempo, self)
        } else {
            *self
        }
    }

    pub fn tempo(&self) -> u16 {
        match self {
            LastChange::Static(n) => *n,
            LastChange::Speculative(n, _) => *n,
        }
    }

    pub fn collapse(&'a self) -> LastChange<'static> {
        LastChange::Static(self.tempo())
    }
}

pub enum ThreefoldRule<'a> {
    Static(HashMap<HashResult, usize>),
    Speculative(HashResult, &'a ThreefoldRule<'a>),
}

impl ThreefoldRule<'static> {
    pub fn empty() -> Self {
        Self::Static(HashMap::new())
    }

    pub fn start(hash: HashResult) -> Self {
        Self::Static(HashMap::from_iter([(hash & BitBoardHasher::HASH_BITS, 1)]))
    }

    pub fn from_iter<I>(it: I) -> Self
    where
        I: IntoIterator<Item = HashResult>,
    {
        let mut res = HashMap::new();
        for mut h in it {
            h &= BitBoardHasher::HASH_BITS;
            *res.entry(h).or_insert(0) += 1;
        }
        Self::Static(res)
    }
}

impl<'a> ThreefoldRule<'a> {
    pub fn see(&'a self, hash: HashResult) -> Self {
        Self::Speculative(hash & BitBoardHasher::HASH_BITS, self)
    }

    pub fn collapse(&self) -> ThreefoldRule<'static> {
        return ThreefoldRule::Static(recurse(self));

        fn recurse<'a>(tfr: &ThreefoldRule<'a>) -> HashMap<HashResult, usize> {
            match tfr {
                ThreefoldRule::Static(hash_map) => hash_map.clone(),
                ThreefoldRule::Speculative(hash, threefold_rule) => {
                    let mut res = recurse(*threefold_rule);
                    *res.entry(*hash).or_insert(0) += 1;
                    res
                }
            }
        }
    }

    pub fn count(&self, hash: HashResult) -> usize {
        match self {
            ThreefoldRule::Static(hash_map) => *hash_map.get(&hash).unwrap_or(&0),
            ThreefoldRule::Speculative(hash2, threefold_rule) => {
                threefold_rule.count(hash) + if hash == *hash2 { 1 } else { 0 }
            }
        }
    }
}
