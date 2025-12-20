//! Day 7: Laboratories
//!
//! ## Problem Description
//!
//! Tachyon beam splitter simulation - count split events in quantum mode.
//!
//! Key insight: when a downward beam hits ^ at (r,c), it creates two new beams:
//! - Left beam continues downward from (r, c-1)
//! - Right beam continues downward from (r, c+1)
//!
//! The simulation explores all possible paths and counts unique splitting events.

use std::collections::{HashSet, VecDeque};

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_one(input: &str) -> u64 {
    let grid = parse_input(input);
    let (start_row, start_col) = find_start(&grid);

    // Count unique splitter positions that are encountered
    let mut splitters_hit = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start_row as i32, start_col as i32));
    let mut visited_start = HashSet::new();

    while let Some((mut r, c)) = queue.pop_front() {
        if visited_start.contains(&(r, c)) {
            continue;
        }
        visited_start.insert((r, c));

        // Trace downward from (r, c)
        loop {
            // Move down
            r += 1;

            // Check bounds
            if r < 0 || r >= grid.len() as i32 {
                break;
            }

            let r_usize = r as usize;
            let line = grid[r_usize];

            if c < 0 || c as usize >= line.len() {
                break;
            }

            let ch = line.chars().nth(c as usize).unwrap_or('.');

            if ch == '^' {
                // Record this splitter if we haven't hit it before
                if !splitters_hit.contains(&(r, c)) {
                    splitters_hit.insert((r, c));
                }

                // Queue left and right paths
                queue.push_back((r, c - 1));
                queue.push_back((r, c + 1));
                break;
            }
        }
    }

    splitters_hit.len() as u64
}

pub fn part_two(input: &str) -> u64 {
    let grid = parse_input(input);
    let (start_row, start_col) = find_start(&grid);

    let mut visited = HashSet::new();
    let mut end_positions = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start_row as i32, start_col as i32));

    while let Some((mut r, c)) = queue.pop_front() {
        if visited.contains(&(r, c)) {
            continue;
        }

        // Trace downward from (r, c)
        loop {
            // Move down
            r += 1;

            // Check bounds
            if r < 0 || r >= grid.len() as i32 {
                end_positions.insert((r, c));
                break;
            }

            let r_usize = r as usize;
            let line = grid[r_usize];

            if c < 0 || c as usize >= line.len() {
                end_positions.insert((r, c));
                break;
            }

            if visited.contains(&(r, c)) {
                break;
            }
            visited.insert((r, c));

            let ch = line.chars().nth(c as usize).unwrap_or('.');

            if ch == '^' {
                // Queue left and right paths
                queue.push_back((r, c - 1));
                queue.push_back((r, c + 1));
                break;
            }
        }
    }

    end_positions.len() as u64
}

fn find_start(grid: &[&str]) -> (usize, usize) {
    for (r, line) in grid.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == 'S' {
                return (r, c);
            }
        }
    }
    (0, 0)
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
