//! Day 7: Laboratories
//!
//! ## Problem Description
//!
//! Tachyon beam simulation through a manifold with splitters.
//! - Part 1: Count how many times the beam splits (encounters `^`)
//! - Part 2: With quantum splitting (all paths), count distinct end timelines

use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_one(input: &str) -> u64 {
    let grid = parse_input(input);

    // Find starting position (marked 'S')
    let mut start_row = 0;
    let mut start_col = 0;
    let mut found = false;

    for (r, line) in grid.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start_row = r;
                start_col = c;
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    // Count splitters directly below S in the same column
    let mut splits = 0;
    let mut row = start_row;
    let col = start_col;

    loop {
        row += 1; // Move downward

        if row >= grid.len() {
            break; // Exit manifold
        }

        let line = grid[row];
        if col >= line.len() {
            break;
        }

        let ch = line.chars().nth(col).unwrap_or('.');

        if ch == '^' {
            splits += 1;
            // Continue counting (don't break)
        }
    }

    splits
}

pub fn part_two(input: &str) -> u64 {
    let grid = parse_input(input);

    // Find starting position (marked 'S')
    let mut start_row = 0;
    let mut start_col = 0;
    let mut found = false;

    for (r, line) in grid.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start_row = r;
                start_col = c;
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    // With quantum splitting, a particle takes both paths at each splitter
    // Track all possible end positions
    let mut final_positions = HashSet::new();

    // BFS/DFS through all possible paths
    let mut queue = vec![(
        start_row as i32,
        start_col as i32,
        0, // Direction: 0=down, 1=left, 2=right
    )];
    let mut visited = HashSet::new();

    while let Some((mut r, mut c, dir)) = queue.pop() {
        let state = (r, c, dir);
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state);

        loop {
            // Move in current direction
            match dir {
                0 => r += 1, // Down
                1 => c -= 1, // Left
                2 => c += 1, // Right
                _ => break,
            }

            // Check bounds - if we exit, this is a final position
            if r < 0 || r >= grid.len() as i32 || c < 0 {
                final_positions.insert((r, c));
                break;
            }

            let r_usize = r as usize;
            if r_usize >= grid.len() {
                final_positions.insert((r, c));
                break;
            }

            let line = grid[r_usize];
            if c as usize >= line.len() {
                final_positions.insert((r, c));
                break;
            }

            let ch = line.chars().nth(c as usize).unwrap_or('.');

            if ch == '^' {
                // Split: add both left and right paths to queue
                queue.push((r, c, 1)); // Left
                queue.push((r, c, 2)); // Right
                break; // This path splits, so we move to processing the new paths
            }
            // Continue in same direction for '.' or 'S'
        }
    }

    final_positions.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_part_one() {
        let input = read_example(7);
        assert_eq!(part_one(&input), 8);
    }

    #[test]
    fn example_part_two() {
        let input = read_example(7);
        assert_eq!(part_two(&input), 16);
    }
}
