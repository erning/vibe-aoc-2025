//! Day 7: Laboratories
//!
//! ## Puzzle Overview
//!
//! The Elves need help fixing a teleporter with a tachyon manifold. Tachyon beams
//! enter at 'S' and move downward. When they encounter splitters ('^'), they
//! create new beams to the left and right.
//!
//! ## Part 1 Strategy
//!
//! Count how many times beams are split as they travel through the manifold.
//! Beams start at 'S' and move downward. When a beam hits a splitter ('^'),
//! it stops and creates new beams to the immediate left and right of the splitter.
//!
//! **Algorithm**:
//! 1. Parse input into 2D grid
//! 2. Find starting position 'S'
//! 3. Simulate beam movement using a queue of beam positions
//! 4. For each beam, move downward until hitting splitter or grid boundary
//! 5. When hitting splitter, count split and add new beams to queue
//! 6. Continue until no more beams
//!
//! **Complexity**: O(n*m*k) where n,m are grid dimensions, k is number of beams
//!
//! ## Part 2 Strategy
//!
//! Count the total number of different timelines/paths a single tachyon particle
//! could end up on. This is a quantum many-worlds interpretation where each
//! splitter creates multiple timelines.
//!
//! **Algorithm**:
//! 1. Parse input into 2D grid
//! 2. Find starting position 'S'
//! 3. Use DFS/BFS to explore all possible paths through the manifold
//! 4. Count unique ending positions/paths
//! 5. Return the total number of possible timelines
//!
//! **Complexity**: O(n*m) for exploring all paths

use std::collections::{HashMap, HashSet, VecDeque};

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

/// Find the starting position 'S'
fn find_start(grid: &[Vec<char>]) -> (usize, usize) {
    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == 'S' {
                return (row, col);
            }
        }
    }
    (0, 0) // Default, should never happen
}

/// Simulate beam splitting and count total splits
fn simulate_beams(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let start_pos = find_start(grid);

    // Queue of beam positions (row, col) - beams start just below S
    let mut beam_queue = VecDeque::new();
    beam_queue.push_back((start_pos.0 + 1, start_pos.1));

    let mut total_splits = 0;
    let mut visited_beams = HashSet::new();

    while let Some((row, col)) = beam_queue.pop_front() {
        // Skip if already processed this beam position
        if !visited_beams.insert((row, col)) {
            continue;
        }

        // Move beam downward from current position
        let current_col = col;
        let mut current_row = row;

        loop {
            // Check if we've reached the bottom
            if current_row >= rows {
                break; // Beam exits the grid
            }

            match grid[current_row][current_col] {
                '.' => {
                    current_row += 1; // Continue moving down
                    continue;
                }
                '^' => {
                    // Beam hits a splitter - count the split!
                    total_splits += 1;

                    // Create new beams to left and right of splitter
                    if current_col > 0 {
                        beam_queue.push_back((current_row, current_col - 1));
                    }
                    if current_col + 1 < cols {
                        beam_queue.push_back((current_row, current_col + 1));
                    }
                    break; // Original beam stops here
                }
                _ => break, // Unknown character, stop beam
            }
        }
    }

    total_splits
}

/// Count all possible paths through the manifold using recursion with memoization
fn count_timelines(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let start_pos = find_start(grid);

    // Use memoization to avoid recomputing the same subproblems
    let mut memo = HashMap::new();

    // Count all possible paths starting from S position
    count_paths_from_position_with_memo(
        grid,
        start_pos.0,
        start_pos.1,
        rows,
        cols,
        &mut memo,
    )
}

/// Recursive function with memoization to count paths from a given position
fn count_paths_from_position_with_memo(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    // Check bounds
    if row >= rows {
        // Reached bottom of grid - this is a valid ending
        return 1;
    }
    if col >= cols {
        // Reached right edge - this is a valid ending
        return 1;
    }

    // Check memoization
    let key = (row, col);
    if let Some(&result) = memo.get(&key) {
        return result;
    }

    let result = match grid[row][col] {
        'S' => {
            // Starting position - move down
            count_paths_from_position_with_memo(
                grid,
                row + 1,
                col,
                rows,
                cols,
                memo,
            )
        }
        '.' => {
            // Empty space - continue moving down
            count_paths_from_position_with_memo(
                grid,
                row + 1,
                col,
                rows,
                cols,
                memo,
            )
        }
        '^' => {
            // Splitter - particle splits into left and right paths
            let mut total_paths = 0;

            // Left path
            if col > 0 {
                total_paths += count_paths_from_position_with_memo(
                    grid,
                    row,
                    col - 1,
                    rows,
                    cols,
                    memo,
                );
            }

            // Right path
            if col + 1 < cols {
                total_paths += count_paths_from_position_with_memo(
                    grid,
                    row,
                    col + 1,
                    rows,
                    cols,
                    memo,
                );
            }

            total_paths
        }
        _ => {
            // Unknown character - stop here (this is a valid ending)
            1
        }
    };

    // Store result in memo
    memo.insert(key, result);
    result
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    simulate_beams(&grid)
}

pub fn part_two(input: &str) -> usize {
    let grid = parse_input(input);
    count_timelines(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_part_one() {
        let input = read_example(7);
        assert_eq!(part_one(&input), 21);
    }

    #[test]
    fn example_part_two() {
        let input = read_example(7);
        assert_eq!(part_two(&input), 40);
    }
}
