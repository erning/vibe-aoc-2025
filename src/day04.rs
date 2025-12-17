//! Day 4: Printing Department
//!
//! ## Problem Description
//!
//! Part 1: Find rolls of paper (@) that can be accessed by forklifts (fewer than 4 adjacent rolls).
//! Part 2: Find total rolls that can be removed through iterative removal process.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse grid where '@' represents paper rolls and '.' represents empty space.
//!
//! **Part 1 Strategy**:
//! - For each '@' in the grid, count adjacent '@' symbols (8 directions)
//! - Mark accessible if fewer than 4 adjacent rolls
//! - Count total accessible rolls
//!
//! **Part 2 Strategy**:
//! - Iteratively remove accessible rolls
//! - After each removal, recheck adjacent rolls that might now be accessible
//! - Continue until no more rolls can be removed
//!
//! **Complexity**: O(n * m * k) where n,m are grid dimensions and k is iteration count for Part 2.

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn get_adjacent_positions(
    row: usize,
    col: usize,
    max_row: usize,
    max_col: usize,
) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();

    for dr in -1i32..=1 {
        for dc in -1i32..=1 {
            if dr == 0 && dc == 0 {
                continue; // Skip current position
            }

            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            if new_row >= 0
                && new_row < max_row as i32
                && new_col >= 0
                && new_col < max_col as i32
            {
                positions.push((new_row as usize, new_col as usize));
            }
        }
    }

    positions
}

fn count_adjacent_rolls(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let positions =
        get_adjacent_positions(row, col, grid.len(), grid[0].len());
    let mut count = 0;

    for (r, c) in positions {
        if grid[r][c] == '@' {
            count += 1;
        }
    }

    count
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let mut accessible_count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '@' {
                let adjacent_rolls = count_adjacent_rolls(&grid, row, col);
                if adjacent_rolls < 4 {
                    accessible_count += 1;
                }
            }
        }
    }

    accessible_count
}

pub fn part_two(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut total_removed = 0;

    loop {
        // Find all currently removable rolls
        let mut to_remove = Vec::new();
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == '@' {
                    let adjacent_rolls =
                        count_adjacent_rolls(&grid, row, col);
                    if adjacent_rolls < 4 {
                        to_remove.push((row, col));
                    }
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        // Remove them all
        total_removed += to_remove.len();
        for (row, col) in to_remove {
            grid[row][col] = '.';
        }
    }

    total_removed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(4);
        assert_eq!(part_one(&input), 13);
        // Note: Part 2 example gives 43 but real input gives 8690
        // The example logic appears to be different or has different expected behavior
        assert_eq!(part_two(&input), 43);
    }
}
