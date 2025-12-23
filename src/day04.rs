//! Day 4: Printing Department
//!
//! ## Problem Description
//!
//! The grid contains rolls of paper marked with `@`. A forklift can access a
//! roll if there are fewer than four rolls in the eight adjacent positions.
//!
//! **Part 1:** Count how many rolls can be accessed.
//!
//! **Part 2:** Iteratively remove accessible rolls. Once a roll is removed,
//! other rolls might become accessible. Continue until no more rolls can be
//! accessed. Count the total number removed.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse the grid into a 2D vector of characters.
//!
//! **Part 1**: For each cell containing `@`, count adjacent `@` cells (8 directions).
//! If count < 4, it's accessible. Sum all accessible rolls.
//!
//! **Part 2**: Iteratively find and remove all accessible rolls:
//! - Create a mutable grid
//! - Loop until no more rolls are removed:
//!   - Find all accessible rolls in current state
//!   - Mark them for removal
//!   - Remove them and increment count
//! - Return total removed

/// Parse input into a 2D grid
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.trim().lines().map(|l| l.chars().collect()).collect()
}

/// Count adjacent rolls (8 directions)
fn count_adjacent(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = row as i64 + dr;
            let nc = col as i64 + dc;
            if nr >= 0 && nr < rows as i64 && nc >= 0 && nc < cols as i64
                && grid[nr as usize][nc as usize] == '@'
            {
                count += 1;
            }
        }
    }

    count
}

/// Find all accessible rolls in the grid
fn find_accessible(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut accessible = Vec::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '@' {
                let adj = count_adjacent(grid, row, col);
                if adj < 4 {
                    accessible.push((row, col));
                }
            }
        }
    }
    accessible
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let accessible = find_accessible(&grid);
    accessible.len()
}

pub fn part_two(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut total_removed = 0;

    loop {
        let accessible = find_accessible(&grid);
        if accessible.is_empty() {
            break;
        }
        // Remove all accessible rolls
        for (row, col) in &accessible {
            grid[*row][*col] = '.';
        }
        total_removed += accessible.len();
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
