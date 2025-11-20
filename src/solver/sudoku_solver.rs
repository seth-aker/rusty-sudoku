use core::cell;
use core::iter::Iterator;

use crate::data::models::puzzle::{self, Puzzle};
use crate::data::models::strategies;
const PUZZLE_WIDTH: usize = 9;
const BLOCK_WIDTH: usize = 3;
pub fn solve_puzzle(puzzle: &Puzzle) {
    let original = puzzle.clone();
    let steps: Vec<strategies::Step> = Vec::new();
    fill_puzzle_candidates(puzzle);
}

fn solve(puzzle: &Puzzle, steps: &Vec<strategies::Step>) {
    if(is_puzzle_solved(puzzle)) {
        return true;
    }
    let mut single = findSingle(puzzle);

}
fn is_puzzle_solved(puzzle: &Puzzle) -> bool {
    for row in 0..PUZZLE_WIDTH {
        for col in 0..PUZZLE_WIDTH {
            let cell_value = puzzle.cells[(row * PUZZLE_WIDTH) + col]; 
            if cell_value == 0 || !num_works_in_cell(row, col, cell_value, puzzle.cells) {
                return false
            }
        }
    }
    return true;
}

fn fill_puzzle_candidates(puzzle: &Puzzle) {
    for potential_candidate in 1..PUZZLE_WIDTH {
        for row in 0..PUZZLE_WIDTH {
            for col in 0..PUZZLE_WIDTH {
                let cell_num = row * PUZZLE_WIDTH + col;
                let cell_val = puzzle.cells[cell_num];
                if cell_val != 0 || !num_works_in_cell(row, col, potential_candidate, puzzle) {
                    continue;
                }
                puzzle.candidates[cell_num][potential_candidate] = true;
            }
        }
    }
    return puzzle
}
fn num_works_in_cell(row_index: usize, col_index: usize, potential_num: u8, puzzle: [u8; 81]) -> bool {
    let cell_value = puzzle[row_index * PUZZLE_WIDTH + col_index];
    let block_x = col_index / BLOCK_WIDTH; // 0, 1, 2
    let block_y = row_index / BLOCK_WIDTH; // 0, 1, 2
    let block = get_block(block_x, block_y, puzzle);
    let row = get_row(row_index, puzzle);
    let col = get_col(col_index, puzzle);
    for i in 0..PUZZLE_WIDTH {
        if i != get_cell_pos_in_block(row_index, col_index, puzzle) && block[i] == potential_num {
            return false;
        }
        if(i != col_index && row[i] == potential_num) {
            return false
        }
        if(i != row_index && col[i] == potential_num) {
            return false
        }
    }
    return true
}

fn find_single(puzzle: &Puzzle) {
    for row_index in 0..PUZZLE_WIDTH {
        for col_index in 0..PUZZLE_WIDTH {
            let cell_index = (row_index * PUZZLE_WIDTH) + col_index;
            let cell_value = puzzle.cells[cell_index];
            if cell_value != 0 {
                continue;
            }
            if count_candidates(puzzle.candidates[cell_index]) == 1 { 
                let single_type: strategies::Strategy = if is_full_house(row_index, col_index, puzzle) {strategies::Strategy {FullHouse}} else {strategies::Strategy{NakedSingle}}; 
                let candidate_value = puzzle.candidates[cell_index].iter().position(|candidate| candidate == true);
                return strategies::Step { row_index: row_index, col_index: col_index, value: candidate_value.unwrap(), candidate_removed: false, step_type: single_type }
            }
            let hidden_single = find_hidden_single(row_index, col_index, puzzle);
        }
    }
}
fn is_full_house(row_index: usize, col_index: usize, puzzle: &Puzzle) {
    let row_values = get_row(row_index, puzzle);
    let col_values = get_col(col_index, puzzle);
    let block_x = col_index / BLOCK_WIDTH;
    let block_y = row_index / BLOCK_WIDTH;
    let block_values = get_block(block_x, block_y, puzzle);

    if count_unit(row_values) == 8 || count_unit(col_values) == 8 || count_unit(block_values) == 8  {
        return true;
    }
    return false;
}
fn get_block(block_x: usize, block_y: usize, puzzle: [u8; 81]) -> [u8; 9] {
    let mut block: [u8; 9] = [0; 9];
    for block_index in 0..PUZZLE_WIDTH {
        let row_modifier = (block_index / BLOCK_WIDTH) * PUZZLE_WIDTH; // 0 0 0, 1 1 1, 2 2 2
        let col_modifier = block_index % BLOCK_WIDTH; // 0 1 2, 0 1 2, 0 1 2
        let block_x_offset = block_x * BLOCK_WIDTH;
        let block_y_offset = block_y * BLOCK_WIDTH * PUZZLE_WIDTH;
        let cell_index = row_modifier + col_modifier + block_x_offset + block_y_offset;
        block[block_index] = puzzle[cell_index]
    }
    return block;
}

fn get_row(row_index: usize, puzzle: [u8; 81]) -> [u8; 9] {
    let mut row: [u8; 9] = [0, 9];
    for i in 0..PUZZLE_WIDTH {
        row[i] = puzzle[row_index * PUZZLE_WIDTH + i];
    }
    return row
}

fn get_col(col_index: usize, puzzle: [u8; 81]) -> [u8; 9] {
    let mut col: [u8; 9] = [0; 9];
    for i in 0..PUZZLE_WIDTH {
        col[i] = puzzle[i * PUZZLE_WIDTH + col_index]
    }
}
fn get_cell_pos_in_block(row_index: usize, col_index: usize, puzzle: [u8; 81]) -> usize {
    let block_x = col_index / BLOCK_WIDTH; // 0, 1, 2
    let block_y = row_index / BLOCK_WIDTH; // 0, 1, 2
    let normalized_x = col_index - (block_x * BLOCK_WIDTH);
    let normalized_y = row_index - (block_y * BLOCK_WIDTH); 

    return (normalized_y * BLOCK_WIDTH) + normalized_x;
}
fn count_candidates(cell_candidates: [bool; 9]) {
    let mut count: u8 = 0;
    for candidate in cell_candidates {
        if candidate {
            count = count + 1;
        }
    }
    return count;
}
fn count_unit(unit: [u8; 9]) -> u8 {
    let mut count: u8 = 0;
    for each in unit {
        if each != 0 {
            count = count + 1
        }
    }
    return count
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
