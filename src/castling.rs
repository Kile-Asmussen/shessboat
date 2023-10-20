#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights<Right> {
    pub white: HalfCastlingRights<Right>,
    pub black: HalfCastlingRights<Right>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HalfCastlingRights<Right> {
    pub queen_side: Right,
    pub king_side: Right,
}
