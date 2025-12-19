//! Day 04: Printing Department
//!
//! ## Problem Description
//!
//! Paper rolls ('@') are arranged on a grid. Forklifts can only access a roll
//! if there are fewer than four adjacent rolls (in 8 directions: up, down, left,
//! right, and 4 diagonals).
//!
//! **Part 1**: Count how many rolls can be accessed immediately (have < 4 adjacent rolls).
//! **Part 2**: Count total rolls that can be removed if after removing accessible rolls,
//! other rolls become accessible, repeating until no more rolls can be removed.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse grid of characters to identify paper roll positions.
//! Part 1: For each paper roll, count adjacent rolls (8 directions).
//! Part 2: Use BFS/queue to iteratively process accessible rolls and update neighbors.
//!
//! **Complexity**: O(R*C) for Part 1, O(R*C*(R*C)) worst case for Part 2.
//!
//! ## Algorithm Explanation
//!
//! Part 1: Simple counting of adjacent '@' symbols for each position.
//! Part 2: Queue-based approach where we process accessible rolls and update the state of neighbors.

use std::collections::VecDeque;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

pub fn part_one(input: &str) -> u32 {
    let grid = parse_input(input);
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();

    let mut count = 0;

    for (r, row) in grid.iter().enumerate().take(rows) {
        for (c, &cell) in row.iter().enumerate().take(cols) {
            if cell == '@' {
                // Count adjacent '@' symbols
                let adjacent_count =
                    count_adjacent_ats(&grid, r, c, rows, cols);
                if adjacent_count < 4 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_adjacent_ats(
    grid: &[Vec<char>],
    r: usize,
    c: usize,
    rows: usize,
    cols: usize,
) -> u32 {
    let mut count = 0;

    // Check 8 adjacent positions (including diagonals)
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue; // Skip the center position
            }

            let new_r = r as i32 + dr;
            let new_c = c as i32 + dc;

            // Check bounds
            if new_r >= 0
                && new_r < rows as i32
                && new_c >= 0
                && new_c < cols as i32
            {
                let adj_r = new_r as usize;
                let adj_c = new_c as usize;

                if grid[adj_r][adj_c] == '@' {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn part_two(input: &str) -> u32 {
    let mut grid = parse_input(input);
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();

    // Create a grid to track how many '@' symbols are adjacent to each position
    let mut adjacent_counts = vec![vec![0; cols]; rows];

    // Calculate initial adjacent counts
    for (r, row) in grid.iter().enumerate().take(rows) {
        for (c, &cell) in row.iter().enumerate().take(cols) {
            if cell == '@' {
                update_adjacent_counts(
                    &mut adjacent_counts,
                    r,
                    c,
                    rows,
                    cols,
                    1,
                );
            }
        }
    }

    // Find all initially accessible rolls (those with < 4 adjacent '@'s)
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' && adjacent_counts[r][c] < 4 {
                queue.push_back((r, c));
            }
        }
    }

    let mut removed_count = 0;

    while let Some((r, c)) = queue.pop_front() {
        // If this position has already been marked as removed, skip it
        if grid[r][c] == '.' {
            continue;
        }

        // Remove the paper roll
        grid[r][c] = '.';
        removed_count += 1;

        // Update adjacent counts for all neighbors
        update_adjacent_counts(&mut adjacent_counts, r, c, rows, cols, -1);

        // Check neighbors to see if they're now accessible
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue; // Skip the center position
                }

                let new_r = r as i32 + dr;
                let new_c = c as i32 + dc;

                // Check bounds
                if new_r >= 0
                    && new_r < rows as i32
                    && new_c >= 0
                    && new_c < cols as i32
                {
                    let adj_r = new_r as usize;
                    let adj_c = new_c as usize;

                    // If this is a paper roll and it's now accessible, add it to the queue
                    if grid[adj_r][adj_c] == '@'
                        && adjacent_counts[adj_r][adj_c] < 4
                    {
                        queue.push_back((adj_r, adj_c));
                    }
                }
            }
        }
    }

    removed_count
}

fn update_adjacent_counts(
    adjacent_counts: &mut [Vec<i32>],
    r: usize,
    c: usize,
    rows: usize,
    cols: usize,
    delta: i32,
) {
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue; // Skip the center position
            }

            let new_r = r as i32 + dr;
            let new_c = c as i32 + dc;

            // Check bounds
            if new_r >= 0
                && new_r < rows as i32
                && new_c >= 0
                && new_c < cols as i32
            {
                let adj_r = new_r as usize;
                let adj_c = new_c as usize;

                adjacent_counts[adj_r][adj_c] += delta;
            }
        }
    }
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
