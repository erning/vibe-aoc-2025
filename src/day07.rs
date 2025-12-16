//! Day 7: Laboratories
//!
//! Simulate tachyon beams through a manifold with splitters.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse a 2D grid finding the start position `S` and
//! splitter positions `^`.
//!
//! **Part 1 Strategy**: Simulate beams moving downward from S.
//! - When a beam hits a splitter `^`, it stops and two new beams emit
//!   from left and right
//! - Count total number of splits that occur
//! - Use BFS to process beams level by level
//! - Track which columns have active beams at each row
//! - Complexity: O(rows * cols) worst case
//!
//! **Part 2 Strategy**: Count timelines using many-worlds interpretation.
//! - Each particle takes both paths at every splitter
//! - Track how many particles are in each column at each row
//! - When a particle hits a splitter, it creates 2 particles (left/right)
//! - Count total particles that exit the bottom
//! - Complexity: O(rows * cols) with potential for large numbers

use std::collections::HashSet;

/// Parse input grid and return (grid, start_column)
fn parse_input(input: &str) -> (Vec<Vec<char>>, usize) {
    let grid: Vec<Vec<char>> =
        input.lines().map(|l| l.chars().collect()).collect();

    // Find start position (S)
    let start_col = grid[0]
        .iter()
        .position(|&c| c == 'S')
        .expect("No start position found");

    (grid, start_col)
}

/// Part 1: Count how many times beams are split
pub fn part_one(input: &str) -> u64 {
    let (grid, start_col) = parse_input(input);
    let width = grid[0].len();

    let mut split_count = 0u64;

    // Track active beam columns at current row
    // Start with a single beam from S going down
    let mut active_beams: HashSet<usize> = HashSet::new();
    active_beams.insert(start_col);

    // Process row by row starting from row 1 (below S)
    for row in grid.iter().skip(1) {
        let mut next_beams: HashSet<usize> = HashSet::new();

        for &col in &active_beams {
            let cell = row[col];

            if cell == '^' {
                // Beam hits splitter - count this split
                split_count += 1;

                // Emit beams left and right
                if col > 0 {
                    next_beams.insert(col - 1);
                }
                if col + 1 < width {
                    next_beams.insert(col + 1);
                }
            } else {
                // Beam continues downward
                next_beams.insert(col);
            }
        }

        active_beams = next_beams;
    }

    split_count
}

/// Part 2: Count timelines (many-worlds interpretation)
/// Each particle takes both paths at every splitter
pub fn part_two(input: &str) -> u64 {
    let (grid, start_col) = parse_input(input);
    let width = grid[0].len();

    // Track particle count at each column (can be very large)
    // Use a map of column -> count
    let mut particles: std::collections::HashMap<usize, u64> =
        std::collections::HashMap::new();
    particles.insert(start_col, 1);

    // Process row by row
    for row in grid.iter().skip(1) {
        let mut next_particles: std::collections::HashMap<usize, u64> =
            std::collections::HashMap::new();

        for (&col, &count) in &particles {
            let cell = row[col];

            if cell == '^' {
                // Each particle splits into 2 (one goes left, one goes right)
                // Each timeline continues as 2 separate timelines
                if col > 0 {
                    *next_particles.entry(col - 1).or_insert(0) += count;
                }
                if col + 1 < width {
                    *next_particles.entry(col + 1).or_insert(0) += count;
                }
            } else {
                // Particle continues downward
                *next_particles.entry(col).or_insert(0) += count;
            }
        }

        particles = next_particles;
    }

    // Sum all timelines at the bottom
    particles.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_part1() {
        let input = read_example(7);
        assert_eq!(part_one(&input), 21);
    }

    #[test]
    fn example_part2() {
        let input = read_example(7);
        assert_eq!(part_two(&input), 40);
    }
}
