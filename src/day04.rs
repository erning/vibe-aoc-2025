//! Day 4: Printing Department
//!
//! ## Puzzle Overview
//!
//! The Elves need help optimizing forklift work by identifying which rolls of paper
//! can be accessed. A roll can be accessed if there are fewer than 4 rolls of paper
//! in the 8 adjacent positions (horizontal, vertical, and diagonal).
//!
//! ## Part 1 Strategy
//!
//! Count the number of rolls that can be accessed by forklifts in one step.
//! A roll is accessible if it has fewer than 4 adjacent @ symbols.
//!
//! **Algorithm**:
//! 1. Parse input into 2D grid of characters
//! 2. For each position containing '@', count adjacent '@' symbols
//! 3. Count positions with < 4 adjacent '@' symbols
//! 4. Return the count
//!
//! **Complexity**: O(n*m) where n,m are grid dimensions
//!
//! ## Part 2 Strategy
//!
//! After removing accessible rolls, new rolls may become accessible. This process
//! repeats until no more rolls can be removed. Count the total number of rolls
//! removed through this cascading process.
//!
//! **Algorithm**:
//! 1. Parse input into mutable 2D grid
//! 2. Loop until no more rolls can be removed:
//!    - Find all accessible rolls in current state
//!    - Remove them from the grid
//!    - Count removed rolls
//! 3. Return total removed count
//!
//! **Complexity**: O(k*n*m) where k is number of removal rounds

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

/// Count adjacent '@' symbols for a given position
fn count_adjacent(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }

            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if new_row >= 0
                && new_row < rows as isize
                && new_col >= 0
                && new_col < cols as isize
                && grid[new_row as usize][new_col as usize] == '@'
            {
                count += 1;
            }
        }
    }

    count
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut accessible_count = 0;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == '@' {
                let adjacent_count = count_adjacent(&grid, row, col);
                if adjacent_count < 4 {
                    accessible_count += 1;
                }
            }
        }
    }

    accessible_count
}

pub fn part_two(input: &str) -> usize {
    let mut grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_removed = 0;

    loop {
        let mut to_remove = Vec::new();

        // Find all accessible rolls in current state
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == '@' {
                    let adjacent_count = count_adjacent(&grid, row, col);
                    if adjacent_count < 4 {
                        to_remove.push((row, col));
                    }
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        // Remove the accessible rolls
        for (row, col) in &to_remove {
            grid[*row][*col] = '.';
        }

        total_removed += to_remove.len();
    }

    total_removed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_part_one() {
        let input = read_example(4);
        assert_eq!(part_one(&input), 13);
    }

    #[test]
    fn example_part_two() {
        let input = read_example(4);
        assert_eq!(part_two(&input), 43);
    }
}
