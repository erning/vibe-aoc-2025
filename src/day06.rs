//! Day 6: Trash Compactor
//!
//! ## Puzzle Overview
//!
//! The Elves need help solving cephalopod math homework involving formatted worksheets
//! where numbers are arranged vertically in columns. Problems are separated by
//! space-only columns, with operators at the bottom.
//!
//! ## Part 1 Strategy
//!
//! Parse a formatted math worksheet where numbers are arranged vertically.
//! Read problems left-to-right and sum all problem results.
//!
//! **Algorithm**:
//! 1. Parse input as 2D grid preserving exact spacing
//! 2. Find problem boundaries (columns that are entirely spaces)
//! 3. Extract numbers from vertical digit sequences in each problem
//! 4. Apply the operator (+ or *) at the bottom of each problem
//! 5. Sum all problem results for grand total
//!
//! **Complexity**: O(n*m) where n is grid height, m is grid width
//!
//! ## Part 2 Strategy
//!
//! Same parsing logic but read problems right-to-left instead of left-to-right.
//! This changes which digits form which numbers in each problem.
//!
//! **Algorithm**:
//! 1. Same parsing as Part 1
//! 2. Process problems in reverse order (right-to-left)
//! 3. Apply same operation and summation logic
//!
//! **Complexity**: Same as Part 1

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

/// Find columns that are entirely spaces (problem boundaries)
fn find_problem_boundaries(grid: &[Vec<char>]) -> Vec<usize> {
    if grid.is_empty() {
        return Vec::new();
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut boundaries = Vec::new();

    for col in 0..cols {
        let mut is_space_column = true;
        for row in 0..rows {
            if grid[row][col] != ' ' {
                is_space_column = false;
                break;
            }
        }
        if is_space_column {
            boundaries.push(col);
        }
    }

    boundaries
}

/// Solve a single problem and return its result
fn solve_problem(
    grid: &[Vec<char>],
    start_col: usize,
    end_col: usize,
    read_right_to_left: bool,
) -> u64 {
    let rows = grid.len();
    let operator_row = rows - 1;

    // Find the operator by looking at the bottom row in the problem area
    let mut operator = ' ';
    for col in start_col..=end_col {
        if grid[operator_row][col] == '+' || grid[operator_row][col] == '*' {
            operator = grid[operator_row][col];
            break;
        }
    }

    // Extract numbers based on reading direction
    let mut numbers = Vec::new();

    if read_right_to_left {
        // Part 2: Read vertically from right to left
        for col in (start_col..=end_col).rev() {
            let mut digits = String::new();
            for row in 0..operator_row {
                if grid[row][col].is_ascii_digit() {
                    digits.push(grid[row][col]);
                }
            }
            if !digits.is_empty() {
                if let Ok(num) = digits.parse::<u64>() {
                    numbers.push(num);
                }
            }
        }
    } else {
        // Part 1: Read horizontally from left to right
        for row in 0..operator_row {
            let mut digits = String::new();
            for col in start_col..=end_col {
                if grid[row][col].is_ascii_digit() {
                    digits.push(grid[row][col]);
                } else if !digits.is_empty() {
                    break;
                }
            }
            if !digits.is_empty() {
                if let Ok(num) = digits.parse::<u64>() {
                    numbers.push(num);
                }
            }
        }
    }

    // Apply the operator
    match operator {
        '+' => numbers.iter().sum(),
        '*' => numbers.iter().product(),
        _ => 0,
    }
}

pub fn part_one(input: &str) -> u64 {
    let grid = parse_input(input);
    let boundaries = find_problem_boundaries(&grid);

    let mut total = 0u64;
    let mut start_col = 0;

    for &boundary in &boundaries {
        if boundary > start_col {
            let end_col = boundary - 1;
            total += solve_problem(&grid, start_col, end_col, false);
            start_col = boundary + 1;
        }
    }

    // Handle the last problem
    if start_col < grid[0].len() {
        total += solve_problem(&grid, start_col, grid[0].len() - 1, false);
    }

    total
}

pub fn part_two(input: &str) -> u64 {
    let grid = parse_input(input);
    let boundaries = find_problem_boundaries(&grid);

    let mut total = 0u64;
    let mut problem_ranges = Vec::new();

    // Collect all problem ranges in order
    let mut start_col = 0;
    for &boundary in &boundaries {
        if boundary > start_col {
            let end_col = boundary - 1;
            problem_ranges.push((start_col, end_col));
            start_col = boundary + 1;
        }
    }

    // Handle the last problem
    if start_col < grid[0].len() {
        problem_ranges.push((start_col, grid[0].len() - 1));
    }

    // Process problems in reverse order (right-to-left)
    for &(start_col, end_col) in problem_ranges.iter().rev() {
        total += solve_problem(&grid, start_col, end_col, true);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_part_one() {
        let input = read_example(6);
        assert_eq!(part_one(&input), 4277556);
    }

    #[test]
    fn example_part_two() {
        let input = read_example(6);
        assert_eq!(part_two(&input), 3263827);
    }
}
