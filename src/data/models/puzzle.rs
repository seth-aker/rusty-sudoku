pub enum DifficultyRating {
    "BEGINNER",
    "EASY",
    "MEDIUM",
    "HARD",
    "IMPOSSIBLE"
}
pub struct PuzzleDifficulty {
    rating: DifficultyRating,
    score: usize
}

pub struct Puzzle {
    id: String,
    cells: [u8, 81],
    candidates: [[u8,9], 81]
    difficulty: PuzzleDifficulty
}
