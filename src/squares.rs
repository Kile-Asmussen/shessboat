#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square(u8);

impl Square {
    pub fn from_index(n: usize) -> Square {
        if let 0..=63 = n {
            Self(n as u8)
        } else {
            panic!("Not a valid square number")
        }
    }

    pub fn from_coord(f: usize, r: usize) -> Square {
        if let (0..=7, 0..=7) = (f, r) {
            Self((r * 8 + f) as u8)
        } else {
            panic!("Not a valid coordinate")
        }
    }

    pub fn from_file_and_rank(file: char, rank: u32) -> Square {
        if let (1..=8, 'a'..='h') = (rank, file) {
            Self::from_coord(file as usize - 'a' as usize, (rank - 1) as usize)
        } else {
            panic!("Not a valid rank and file")
        }
    }

    pub fn index(self) -> usize {
        self.0 as usize
    }

    pub fn coord(self) -> (usize, usize) {
        (self.0 as usize % 8, self.0 as usize / 8)
    }

    const FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    pub fn file_and_rank(self) -> (char, u32) {
        (Self::FILES[(self.0 % 8) as usize], (self.0 / 8) as u32 + 1)
    }

    pub fn position(self) -> Position {
        Position::new(1 << self.0)
    }

    pub fn tint(self) -> Tint {
        let (f, r) = self.coord();
        if (r + f) & 1 == 0 {
            Tint::Dark
        } else {
            Tint::Light
        }
    }

    const ALGEBRAIC: [&'static str; 64] = [
        "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", //
        "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", //
        "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", //
        "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4", //
        "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", //
        "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6", //
        "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", //
        "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8", //
    ];

    pub fn alg(self) -> &'static str {
        Self::ALGEBRAIC[self.index()]
    }

    pub fn from_alg(a: &str) -> Self {
        Self::parse_alg(a).unwrap_or_else(|| panic!("Not a valid algebraic notation"))
    }

    pub fn parse_alg(a: &str) -> Option<Self> {
        Self::ALGEBRAIC
            .iter()
            .position(|r| r == a)
            .map(Square::from_index)
    }
}

pub enum Tint {
    Dark,
    Light,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub mask: u64,
}

impl Position {
    pub fn empty() -> Self {
        Self::new(u64::MIN)
    }

    pub fn full() -> Self {
        Self::new(u64::MAX)
    }

    pub fn new(mask: u64) -> Self {
        Self { mask }
    }

    pub fn rank(rank: usize) -> Self {
        if let 1..=8 = rank {
            Self::new(0xFF << ((rank - 1) * 8))
        } else {
            panic!("Not a valid rank")
        }
    }

    pub fn file(file: char) -> Self {
        if let 'a'..='h' = file {
            Self::new(0x0101010101010101 << (file as u32 - 'a' as u32))
        } else {
            panic!("Not a valid file");
        }
    }

    pub fn contains(self, sq: Square) -> bool {
        self.mask & sq.position().mask != 0
    }

    pub fn num_squares(self) -> u32 {
        self.mask.count_ones()
    }

    pub fn overlap(self, other: Self) -> Self {
        Self::new(self.mask & other.mask)
    }

    pub fn with(self, other: Self) -> Self {
        Self::new(self.mask | other.mask)
    }

    pub fn complement(self) -> Self {
        Self::new(!self.mask)
    }

    pub fn without(self, other: Self) -> Self {
        self.overlap(other.complement())
    }

    pub fn populated(self) -> bool {
        self.mask != 0
    }
}

impl IntoIterator for Position {
    type Item = Square;

    type IntoIter = PositionSquaresIterator;

    fn into_iter(self) -> Self::IntoIter {
        PositionSquaresIterator {
            all_squares: AllSquares.into_iter(),
            position: self,
        }
    }
}

pub struct PositionSquaresIterator {
    all_squares: AllSquaresIterator,
    position: Position,
}

impl Iterator for PositionSquaresIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(sq) = self.all_squares.next() {
            if self.position.contains(sq) {
                self.position = self.position.without(sq.position());
                return Some(sq);
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.position.num_squares() as usize;
        (n, Some(n))
    }
}

#[derive(Clone, Copy)]
pub struct AllSquares;

impl IntoIterator for AllSquares {
    type Item = Square;

    type IntoIter = AllSquaresIterator;

    fn into_iter(self) -> Self::IntoIter {
        AllSquaresIterator(0)
    }
}

pub struct AllSquaresIterator(usize);

impl Iterator for AllSquaresIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 >= 64 {
            None
        } else {
            let res = Some(Square::from_index(self.0));
            self.0 += 1;
            res
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (64 - self.0, Some(64 - self.0))
    }
}
