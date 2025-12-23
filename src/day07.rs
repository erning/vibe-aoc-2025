//! Day 7: Laboratories
//!
//! Part 1: Count how many times tachyon beams are split when traveling through
//!         a manifold. Beams start at S, go down, and split at ^ into left/right.
//! Part 2: Count how many unique timelines (end positions) a quantum particle
//!         reaches through many-worlds interpretation of splitting.
//!
//! ## Solution Approach
//!
//! **Input Format**: A 2D grid with:
//! - `S` - starting position of tachyon beam
//! - `^` - splitter that sends beams left and right
//! - `.` - empty space beams pass through
//!
//! **Part 1 Strategy**:
//! - Use BFS to simulate all beams moving downward
//! - Track which splitters have been hit (de-duplicate)
//! - When a beam hits a splitter, create two new beams going from adjacent
//!   columns in the next row
//! - Count unique splitters that were hit
//!
//! **Part 2 Strategy**:
//! - Use dynamic programming to count ways to reach each cell
//! - For each cell, propagate the count to cells below
//! - Splitters split the count to left and right adjacent cells below
//! - Count unique end positions (cells in bottom row with count > 0)
//!
//! **Complexity**: O(h * w) where h is height and w is width of grid.

use std::collections::{HashSet, VecDeque};

/// Position in the grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

/// Parse input into a grid and find the source position
fn parse_input(input: &str) -> (Vec<Vec<char>>, Pos) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    // Find source position
    let mut source = Pos { row: 0, col: 0 };
    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == 'S' {
                source = Pos { row, col };
                break;
            }
        }
        if source.row == row {
            break;
        }
    }

    (grid, source)
}

/// Count how many times the beam is split (unique splitters hit)
pub fn part_one(input: &str) -> usize {
    let (grid, source) = parse_input(input);
    if grid.is_empty() {
        return 0;
    }

    let height = grid.len();

    // Track which splitters have been hit
    let mut hit_splitters: HashSet<Pos> = HashSet::new();

    // Track visited beam states to avoid infinite loops
    let mut visited: HashSet<Pos> = HashSet::new();

    // BFS queue for beam positions
    let mut queue: VecDeque<Pos> = VecDeque::new();
    queue.push_back(source);

    while let Some(pos) = queue.pop_front() {
        let Pos { row, col } = pos;

        // Move down to next row
        let next_row = row + 1;
        if next_row >= height {
            continue; // Exited grid
        }

        // Check what's in the cell below at the same column
        if next_row < grid.len() {
            let row_len = grid[next_row].len();
            if col >= row_len {
                continue; // Column out of bounds for this row
            }

            let ch = grid[next_row][col];
            if ch == '^' {
                // Hit a splitter
                let splitter_pos = Pos { row: next_row, col };
                if hit_splitters.insert(splitter_pos) {
                    // Create two new beams going to adjacent columns in the SAME row
                    // They will then continue down from there
                    if col > 0 {
                        let left_pos = Pos {
                            row: next_row,
                            col: col - 1,
                        };
                        if visited.insert(left_pos) {
                            queue.push_back(left_pos);
                        }
                    }
                    if col + 1 < row_len {
                        let right_pos = Pos {
                            row: next_row,
                            col: col + 1,
                        };
                        if visited.insert(right_pos) {
                            queue.push_back(right_pos);
                        }
                    }
                }
            } else {
                // Continue down from this column
                let next_pos = Pos { row: next_row, col };
                if visited.insert(next_pos) {
                    queue.push_back(next_pos);
                }
            }
        }
    }

    hit_splitters.len()
}

/// Count unique timelines (end positions) for quantum particle
pub fn part_two(input: &str) -> u128 {
    let (grid, source) = parse_input(input);
    if grid.is_empty() {
        return 0;
    }

    let height = grid.len();
    let width = grid.iter().map(|row| row.len()).max().unwrap_or(0);

    // Count number of ways to reach each cell in the current row
    // ways[col] = number of timelines that reach this column at current row
    let mut ways: Vec<u128> = vec![0; width];
    ways[source.col] = 1;

    // Process each row from top to bottom
    for row in 0..height {
        let mut next_ways: Vec<u128> = vec![0; width];

        for col in 0..width {
            if ways[col] == 0 {
                continue;
            }

            let current_ways = ways[col];
            let next_row = row + 1;

            if next_row >= height {
                // Exit at bottom - add to next_ways to count this timeline
                next_ways[col] += current_ways;
                continue;
            }

            if col >= grid[next_row].len() {
                // Column out of bounds, beam exits
                next_ways[col] += current_ways;
                continue;
            }

            let ch = grid[next_row][col];
            if ch == '^' {
                // Hit a splitter - timeline splits to both paths
                if col > 0 {
                    next_ways[col - 1] += current_ways;
                }
                if col + 1 < width {
                    next_ways[col + 1] += current_ways;
                }
            } else {
                // No splitter - continue down
                next_ways[col] += current_ways;
            }
        }

        ways = next_ways;
    }

    // Total timelines = sum of all ways at the bottom row
    ways.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn test_part_one() {
        let input = read_example(7);
        assert_eq!(part_one(&input), 21);
    }

    #[test]
    fn test_part_two() {
        let input = read_example(7);
        assert_eq!(part_two(&input), 40);
    }
}
