//! Day 4: Printing Department
//!
//! ## Problem Description
//!
//! Part 1: Count rolls of paper (@) that have fewer than 4 adjacent
//! rolls (in 8 directions).
//!
//! Part 2: Repeatedly remove accessible rolls (with <4 neighbors) until
//! no more can be removed. Count total removed.
//!
//! ## Solution Approach
//!
//! **Part 1**: For each @ in the grid, count its @ neighbors in all 8
//! directions. If count < 4, it's accessible.
//!
//! **Part 2**: Simulate removal process. In each iteration, find all
//! accessible rolls, mark them for removal, then remove them. Repeat
//! until no more accessible rolls exist.
//!
//! **Complexity**: O(n*m*k) where n,m are grid dimensions and k is the
//! number of iterations (usually small).

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn count_neighbors(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut count = 0;
    for (dr, dc) in directions {
        let nr = row as isize + dr;
        let nc = col as isize + dc;
        if nr >= 0
            && nr < rows
            && nc >= 0
            && nc < cols
            && grid[nr as usize][nc as usize] == '@'
        {
            count += 1;
        }
    }
    count
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let mut count = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == '@' && count_neighbors(&grid, r, c) < 4 {
                count += 1;
            }
        }
    }

    count
}

pub fn part_two(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut total_removed = 0;

    loop {
        // Find all accessible rolls
        let mut to_remove = Vec::new();
        for (r, row) in grid.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if cell == '@' && count_neighbors(&grid, r, c) < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        // Remove them
        for (r, c) in &to_remove {
            grid[*r][*c] = '.';
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
