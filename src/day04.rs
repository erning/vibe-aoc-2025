//! Day 4: Printing Department
//!
//! ## Problem Description
//!
//! Part 1: Count how many rolls of paper (@) have fewer than 4 neighboring rolls
//! in the 8 adjacent positions (diagonals included).
//!
//! Part 2: Simulate the process where accessible rolls are removed, potentially
//! making more rolls accessible. Count the total number of rolls removed.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Read the grid into a 2D array of characters.
//!
//! **Part 1**: For each @ cell, count the number of @ in its 8 neighbors.
//! If fewer than 4, it's accessible.
//!
//! **Part 2**: Iteratively find and remove accessible @ cells until no more
//! can be removed. Each iteration may make new cells accessible.
//!
//! ## Complexity Analysis
//!
//! Let R be rows and C be columns.
//! - Time Complexity: O(R * C * iterations) where iterations depend on the input
//! - Space Complexity: O(R * C) for the grid

use std::collections::HashSet;

/// Parse the input into a grid of characters
pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().chars().collect())
        .collect()
}

/// Count the number of '@' neighbors around a position
fn count_neighbors(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue; // Skip the cell itself
            }

            let nr = row as i32 + dr;
            let nc = col as i32 + dc;

            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                if grid[nr as usize][nc as usize] == '@' {
                    count += 1;
                }
            }
        }
    }

    count
}

/// Find all positions that are accessible (fewer than 4 neighbors)
fn find_accessible(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut accessible = Vec::new();

    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == '@' {
                let neighbors = count_neighbors(grid, r, c);
                if neighbors < 4 {
                    accessible.push((r, c));
                }
            }
        }
    }

    accessible
}

/// Part 1: Count initially accessible rolls
pub fn part_one(input: &str) -> impl std::fmt::Display {
    let grid = parse_input(input);
    let accessible = find_accessible(&grid);
    accessible.len()
}

/// Part 2: Count total rolls that can be removed iteratively
pub fn part_two(input: &str) -> impl std::fmt::Display {
    let mut grid = parse_input(input);
    let mut total_removed = 0;
    let mut removed_positions = HashSet::new();

    loop {
        let accessible = find_accessible(&grid);

        if accessible.is_empty() {
            break;
        }

        for (r, c) in accessible {
            // Check if we haven't already removed this position
            if !removed_positions.contains(&(r, c)) {
                grid[r][c] = '.'; // Mark as removed
                removed_positions.insert((r, c));
                total_removed += 1;
            }
        }
    }

    total_removed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_neighbors() {
        let grid = vec![
            vec!['.', '.', '@', '.'],
            vec!['@', '@', '@', '@'],
            vec!['.', '.', '@', '.'],
        ];

        assert_eq!(count_neighbors(&grid, 0, 2), 4); // Center top @ has 4 neighbors
        assert_eq!(count_neighbors(&grid, 1, 1), 6); // Center @ has 6 neighbors
    }

    #[test]
    fn example() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@.@@@@.@@
.@@@@@@@.
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@@.@@@.";

        assert_eq!(part_one(input).to_string(), "13");
        assert_eq!(part_two(input).to_string(), "43");
    }
}