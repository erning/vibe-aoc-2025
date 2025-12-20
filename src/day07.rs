//! Day 7: Laboratories
//!
//! ## Problem Description
//!
//! Tachyon beam splitter simulation in quantum mode.
//!
//! - Part 1: Count total number of beam splits (every encounter with ^)
//! - Part 2: Count distinct timelines (unique paths through the manifold)
//!
//! Key insight: A beam continues downward until it hits a splitter (^),
//! then creates two branches. In quantum mode, each choice creates a
//! different timeline.

use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_one(input: &str) -> u64 {
    let grid = parse_input(input);
    let (start_row, start_col) = find_start(&grid);

    // Count distinct splitter positions encountered
    let mut splitter_positions = HashSet::new();
    let mut visited = HashSet::new();
    count_splitters(
        &grid,
        start_row as i32,
        start_col as i32,
        &mut visited,
        &mut splitter_positions,
    );

    splitter_positions.len() as u64
}

fn count_splitters(
    grid: &[&str],
    start_r: i32,
    start_c: i32,
    visited: &mut HashSet<(i32, i32)>,
    splitter_positions: &mut HashSet<(i32, i32)>,
) {
    let mut r = start_r;
    let c = start_c;

    loop {
        // Move down
        r += 1;

        // Check bounds
        if r < 0 || r >= grid.len() as i32 {
            return;
        }

        let r_usize = r as usize;
        let line = grid[r_usize];

        if c < 0 || c as usize >= line.len() {
            return;
        }

        // Check visited to prevent infinite loops
        if visited.contains(&(r, c)) {
            return;
        }
        visited.insert((r, c));

        let ch = line.chars().nth(c as usize).unwrap_or('.');

        if ch == '^' {
            // Found a splitter - record it and branch
            splitter_positions.insert((r, c));
            count_splitters(grid, r, c - 1, visited, splitter_positions);
            count_splitters(grid, r, c + 1, visited, splitter_positions);
            return;
        }
    }
}

pub fn part_two(input: &str) -> u64 {
    let grid = parse_input(input);
    let (start_row, start_col) = find_start(&grid);

    let mut timeline_count = 0u64;
    let mut visited = HashSet::new();
    count_timelines(
        &grid,
        start_row as i32,
        start_col as i32,
        &mut visited,
        &mut timeline_count,
    );
    timeline_count
}

fn count_timelines(
    grid: &[&str],
    start_r: i32,
    start_c: i32,
    visited: &mut HashSet<(i32, i32)>,
    timeline_count: &mut u64,
) {
    let mut r = start_r;
    let c = start_c;
    let mut local_visited = HashSet::new();

    loop {
        r += 1;

        if r < 0 || r >= grid.len() as i32 {
            *timeline_count += 1;
            return;
        }

        let r_usize = r as usize;
        let line = grid[r_usize];

        if c < 0 || c as usize >= line.len() {
            *timeline_count += 1;
            return;
        }

        if local_visited.contains(&(r, c)) {
            *timeline_count += 1;
            return;
        }
        local_visited.insert((r, c));

        let ch = line.chars().nth(c as usize).unwrap_or('.');

        if ch == '^' {
            count_timelines(grid, r, c - 1, visited, timeline_count);
            count_timelines(grid, r, c + 1, visited, timeline_count);
            return;
        }
    }
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
        assert_eq!(part_one(&input), 21);
    }

    #[test]
    fn example_part_two() {
        let input = read_example(7);
        assert_eq!(part_two(&input), 40);
    }
}
