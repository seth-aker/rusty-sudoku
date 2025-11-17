pub enum Strategy {
    "FULL_HOUSE",
    "NAKED_SINGLE",
    "HIDDEN_SINGLE",
    "LOCKED_CANDIDATE_POINTING",
    "LOCKED_CANDIDATE_CLAIMING",
    "HIDDEN_PAIRS",
    "NAKED_PAIRS",
    "HIDDEN_TRIPLES",
    "NAKED_TRIPLES",
    "HIDDEN_QUADS",
    "NAKED_QUADS",
    "GUESS"
}

pub struct Step {
    rowIndex: u8,
    colIndex: u8,
    value: u8,
    candidateRemoved: boolean,
    type: Option<Strategy>
}
