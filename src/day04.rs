//! Day 4: Printing Department
//!
//! Grid puzzle about paper rolls and forklift access.
//!
//! Part 1: Count rolls with fewer than 4 adjacent rolls.
//! Part 2: Repeatedly remove accessible rolls, count total removed.

use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

fn count_adjacent(grid: &[Vec<bool>], row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for dr in -1i32..=1 {
        for dc in -1i32..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;
            if nr >= 0
                && nr < rows as i32
                && nc >= 0
                && nc < cols as i32
                && grid[nr as usize][nc as usize]
            {
                count += 1;
            }
        }
    }
    count
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] && count_adjacent(&grid, r, c) < 4 {
                count += 1;
            }
        }
    }

    count
}

pub fn part_two(input: &str) -> usize {
    let mut grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_removed = 0;

    loop {
        let mut to_remove: HashSet<(usize, usize)> = HashSet::new();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] && count_adjacent(&grid, r, c) < 4 {
                    to_remove.insert((r, c));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (r, c) in &to_remove {
            grid[*r][*c] = false;
        }
        total_removed += to_remove.len();
    }

    total_removed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(4);
        assert_eq!(part_one(&input), 13);
        assert_eq!(part_two(&input), 43);
    }
}
