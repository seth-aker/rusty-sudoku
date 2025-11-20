pub enum Strategy {
    FullHouse,
    NakedSingle,
    HiddenSingle,
    LockedCandidatePointing,
    LockedCandidateClaiming,
    HiddenPairs,
    NakedPairs,
    HiddenTriples,
    NakedTriples,
    HiddenQuads,
    NakedQuads,
    GUESS,
}
pub struct Step {
    row_index: usize,
    col_index: usize,
    value: usize,
    candidate_removed: bool,
    step_type: Option<Strategy>,
}
