use crate::data::models::puzzle::Puzzle;
use crate::data::models::strategies;
const ROW_LENGTH: usize = 9;
const BLOCK_WIDTH: usize = 3;
fn solvePuzzle(puzzle: &Puzzle) {
    let original = puzzle.clone();
    let steps: Vec<strategies::Step> = Vec::new();
}
fn fillPuzzleCandidates(puzzle: [u8; 81]) {
    for potential_candidate in 1..ROW_LENGTH {
        for row in 0..ROW_LENGTH {
            for col in 0..ROW_LENGTH {
                let cell_num = row * ROW_LENGTH + col;
                let cell = puzzle[cell_num];
            }
        }
    }
}
fn numWorksInCell(row_index: usize, col_index: usize, potential_num: usize, puzzle: [u8; 81]) {
    let cell = puzzle[row_index * ROW_LENGTH + col_index];
    let block_x = col_index / BLOCK_WIDTH; // 0, 1, 2
    let block_y = row_index / BLOCK_WIDTH; // 0, 1, 2
    get_block(block_x, block_y, puzzle);
}

fn get_block(block_x: usize, block_y: usize, puzzle: [u8; 81]) -> [u8; 9] {
    let mut block: [u8; 9] = [0; 9];
    for block_index in 0..ROW_LENGTH {
        let row_modifier = (block_index / BLOCK_WIDTH) * ROW_LENGTH; // 0 0 0, 1 1 1, 2 2 2
        let col_modifier = block_index % BLOCK_WIDTH; // 0 1 2, 0 1 2, 0 1 2
        let block_x_offset = block_x * BLOCK_WIDTH;
        let block_y_offset = block_y * BLOCK_WIDTH * ROW_LENGTH;
        let puzzle_index = row_modifier + col_modifier + block_x_offset + block_y_offset;
        block[block_index] = puzzle[puzzle_index]
    }
    return block;
}

// 00 01 02 | 03 04 05 | 06 07 08
// 09 10 11 | 12 13 14 | 15 16 17
// 18 19 20 | 21 22 23 | 24 25 26
// -----------------------------
// 27 28 29 | 30 31 32 | 33 34 35
// 36 37 38 | 39 40 41 | 42 43 44
// 45 46 47 | 48 49 50 | 51 52 53
// ------------------------------
// 54 55 56 | 57 58 59 | 60 61 62
// 63 64 65 | 66 67 68 | 69 70 71
// 72 73 74 | 75 76 77 | 78 79 80
