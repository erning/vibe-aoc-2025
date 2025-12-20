//! Day 7: Laboratories
//!
//! ## Problem Description
//!
//! Tachyon beam splitter simulation in quantum mode.
//!
//! - Part 1: Count distinct splitter positions encountered
//! - Part 2: Count distinct timelines (unique paths through the manifold)
//!
//! Key insight: A beam continues downward until it hits a splitter (^),
//! then creates two branches. Each timeline is a unique combination of
//! left/right choices at each splitter.

use std::collections::HashMap;
use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_one(input: &str) -> u64 {
    let grid = parse_input(input);
    let (start_row, start_col) = find_start(&grid);

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
        r += 1;

        if r < 0 || r >= grid.len() as i32 {
            return;
        }

        let r_usize = r as usize;
        let line = grid[r_usize];

        if c < 0 || c as usize >= line.len() {
            return;
        }

        if visited.contains(&(r, c)) {
            return;
        }
        visited.insert((r, c));

        let ch = line.chars().nth(c as usize).unwrap_or('.');

        if ch == '^' {
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

    let mut memo = HashMap::new();
    count_timelines_memo(&grid, start_row as i32, start_col as i32, &mut memo)
}

fn count_timelines_memo(
    grid: &[&str],
    start_r: i32,
    start_c: i32,
    memo: &mut HashMap<(i32, i32), u64>,
) -> u64 {
    if let Some(&count) = memo.get(&(start_r, start_c)) {
        return count;
    }

    let mut r = start_r;
    let c = start_c;
    let mut local_visited = HashSet::new();

    loop {
        r += 1;

        if r < 0 || r >= grid.len() as i32 {
            memo.insert((start_r, start_c), 1);
            return 1;
        }

        let r_usize = r as usize;
        let line = grid[r_usize];

        if c < 0 || c as usize >= line.len() {
            memo.insert((start_r, start_c), 1);
            return 1;
        }

        if local_visited.contains(&(r, c)) {
            memo.insert((start_r, start_c), 1);
            return 1;
        }
        local_visited.insert((r, c));

        let ch = line.chars().nth(c as usize).unwrap_or('.');

        if ch == '^' {
            let left_count = count_timelines_memo(grid, r, c - 1, memo);
            let right_count = count_timelines_memo(grid, r, c + 1, memo);
            let total = left_count + right_count;
            memo.insert((start_r, start_c), total);
            return total;
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
