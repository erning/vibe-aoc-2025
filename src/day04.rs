//! Day 4: Printing Department
//!
//! ## Problem Description
//!
//! Part 1: Count rolls of paper (@) that have fewer than 4 rolls in their
//! 8 adjacent positions (Moore neighborhood).
//!
//! Part 2: Repeatedly remove accessible rolls (those with < 4 adjacent rolls)
//! until no more can be removed. Count total rolls removed.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse grid of characters, identifying positions of rolls.
//!
//! **Part 1 Strategy**:
//! - For each position with a roll (@), count rolls in 8 adjacent positions
//! - If count < 4, mark as accessible
//! - Count total accessible rolls
//!
//! **Part 2 Strategy**:
//! - Repeatedly find and remove accessible rolls
//! - After each removal, recalculate which rolls are accessible
//! - Continue until no more rolls can be accessed
//! - Track and sum the removals at each stage
//!
//! **Complexity**: O(n * m * iterations) where n×m is grid size and iterations
//! is number of removal rounds (typically small).

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn count_adjacent_rolls(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let mut count = 0;
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let r = row as i32 + dr;
            let c = col as i32 + dc;
            if r >= 0
                && r < rows
                && c >= 0
                && c < cols
                && grid[r as usize][c as usize] == '@'
            {
                count += 1;
            }
        }
    }

    count
}

fn get_accessible_rolls(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut accessible = Vec::new();

    for (row, row_data) in grid.iter().enumerate() {
        for (col, &ch) in row_data.iter().enumerate() {
            if ch == '@' {
                let adjacent = count_adjacent_rolls(grid, row, col);
                if adjacent < 4 {
                    accessible.push((row, col));
                }
            }
        }
    }

    accessible
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    get_accessible_rolls(&grid).len()
}

pub fn part_two(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut total_removed = 0;

    loop {
        let accessible = get_accessible_rolls(&grid);
        if accessible.is_empty() {
            break;
        }

        // Remove all accessible rolls
        let removed_count = accessible.len();
        for (row, col) in accessible {
            grid[row][col] = '.';
        }

        total_removed += removed_count;
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
