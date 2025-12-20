//! Day 7: Laboratories
//!
//! ## Problem Description
//!
//! Part 1: Count how many times a tachyon beam splits when passing through
//! a manifold. Beams move downward and split into left/right beams at `^`.
//! Multiple beams can merge into the same position.
//!
//! Part 2: Count the number of unique timelines when a single quantum
//! tachyon particle takes both paths at each splitter (many-worlds
//! interpretation).
//!
//! ## Solution Approach
//!
//! **Part 1**: Simulate classical beam splitting with a queue of active
//! beams. Each beam moves down until hitting a splitter, which creates two
//! new beams. Count total splits.
//!
//! **Part 2**: Use recursion with memoization to count unique paths. Each
//! splitter creates a branching point where the particle explores both
//! left and right paths independently.
//!
//! **Complexity**: Part 1 is O(n*m) where n,m are grid dimensions.
//! Part 2 is O(n*m*2^s) worst case where s is splitters, but memoization
//! reduces this significantly in practice.

use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

struct Manifold {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    start: Position,
}

fn parse_input(input: &str) -> Manifold {
    let grid: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    // Find starting position 'S'
    let mut start = Position { row: 0, col: 0 };
    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start = Position { row: r, col: c };
                break;
            }
        }
    }

    Manifold {
        grid,
        rows,
        cols,
        start,
    }
}

pub fn part_one(input: &str) -> i64 {
    let manifold = parse_input(input);
    let mut split_count = 0;

    // Track which splitters have been hit to avoid double counting and infinite loops
    let mut hit_splitters: HashMap<Position, bool> = HashMap::new();

    // Queue of active beams (each beam is at a position)
    let mut queue: VecDeque<Position> = VecDeque::new();
    queue.push_back(manifold.start);

    while let Some(pos) = queue.pop_front() {
        // Move beam downward until hitting a splitter or exiting
        let mut current = pos;

        loop {
            // Move down one row
            current.row += 1;

            // Check if exited the manifold
            if current.row >= manifold.rows {
                break;
            }

            let ch = manifold.grid[current.row][current.col];

            // If hit a splitter, create two new beams
            if ch == '^' {
                let splitter_pos = current;

                // Only process this splitter if we haven't seen it before
                if hit_splitters.contains_key(&splitter_pos) {
                    break; // Already processed, skip
                }

                hit_splitters.insert(splitter_pos, true);
                split_count += 1;

                // Left beam
                if current.col > 0 {
                    queue.push_back(Position {
                        row: current.row,
                        col: current.col - 1,
                    });
                }

                // Right beam
                if current.col + 1 < manifold.cols {
                    queue.push_back(Position {
                        row: current.row,
                        col: current.col + 1,
                    });
                }

                break;
            }
        }
    }

    split_count
}

pub fn part_two(input: &str) -> i64 {
    let manifold = parse_input(input);

    // Precompute: for each column, find the next splitter row (or None if exits)
    let mut next_splitter: Vec<Vec<Option<usize>>> =
        vec![vec![None; manifold.cols]; manifold.rows];

    // Build next_splitter table bottom-up
    for col in 0..manifold.cols {
        let mut next = None;
        for row in (0..manifold.rows).rev() {
            if manifold.grid[row][col] == '^' {
                next = Some(row);
            }
            next_splitter[row][col] = next;
        }
    }

    // Collect all splitters
    let mut splitters: Vec<Position> = Vec::new();
    for row in 0..manifold.rows {
        for col in 0..manifold.cols {
            if manifold.grid[row][col] == '^' {
                splitters.push(Position { row, col });
            }
        }
    }

    // Sort splitters bottom to top for DP
    splitters.sort_by(|a, b| b.row.cmp(&a.row));

    // Cache: splitter position -> number of paths from that splitter
    let mut cache: HashMap<Position, i64> = HashMap::new();

    // Compute paths for each splitter (bottom-up)
    for &splitter in &splitters {
        let mut total = 0;

        // Left path
        if splitter.col > 0 {
            let left_col = splitter.col - 1;
            // Check for next splitter below this position
            if splitter.row + 1 < manifold.rows {
                if let Some(next_row) =
                    next_splitter[splitter.row + 1][left_col]
                {
                    let next_pos = Position {
                        row: next_row,
                        col: left_col,
                    };
                    total += cache.get(&next_pos).unwrap_or(&0);
                } else {
                    total += 1; // Exits
                }
            } else {
                total += 1; // Exits immediately
            }
        }

        // Right path
        if splitter.col + 1 < manifold.cols {
            let right_col = splitter.col + 1;
            // Check for next splitter below this position
            if splitter.row + 1 < manifold.rows {
                if let Some(next_row) =
                    next_splitter[splitter.row + 1][right_col]
                {
                    let next_pos = Position {
                        row: next_row,
                        col: right_col,
                    };
                    total += cache.get(&next_pos).unwrap_or(&0);
                } else {
                    total += 1; // Exits
                }
            } else {
                total += 1; // Exits immediately
            }
        }

        cache.insert(splitter, total);
    }

    // Find result from start position
    let start_col = manifold.start.col;
    if manifold.start.row + 1 < manifold.rows {
        if let Some(first_row) =
            next_splitter[manifold.start.row + 1][start_col]
        {
            let first_splitter = Position {
                row: first_row,
                col: start_col,
            };
            *cache.get(&first_splitter).unwrap_or(&0)
        } else {
            1 // No splitters, just exits
        }
    } else {
        1 // Exits immediately
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(7);
        assert_eq!(part_one(&input), 21);
        assert_eq!(part_two(&input), 40);
    }
}
