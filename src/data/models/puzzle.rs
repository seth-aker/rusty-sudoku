#[derive(Clone, Copy)]

pub enum DifficultyRating {
    BEGINNER,
    EASY,
    MEDIUM,
    HARD,
    IMPOSSIBLE,
}
#[derive(Clone, Copy)]
pub struct PuzzleDifficulty {
    pub rating: DifficultyRating,
    pub score: usize,
}
#[derive(Clone)]
pub struct Puzzle {
    pub id: String,
    pub cells: [u8; 81],
    pub candidates: [[u8; 9]; 81],
    pub difficulty: PuzzleDifficulty,
}
