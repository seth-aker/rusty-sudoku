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
    pub row_index: usize,
    pub col_index: usize,
    pub value: usize,
    pub candidate_removed: bool,
    pub step_type: Option<Strategy>,
}
