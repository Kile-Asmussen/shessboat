use std::collections::HashMap;

use crate::shessboard::zobrist::{BitBoardHasher, HashResult};

pub enum ThreefoldRule<'a> {
    Static(HashMap<HashResult, usize>),
    Speculative(HashResult, &'a ThreefoldRule<'a>),
}

impl ThreefoldRule<'static> {
    pub fn empty() -> Self {
        Self::Static(HashMap::new())
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
