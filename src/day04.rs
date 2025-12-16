//! Day 4: Printing Department
//!
//! Count accessible paper rolls in a warehouse grid. A roll (`@`) is accessible
//! if it has fewer than 4 rolls in its 8 adjacent positions.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse the grid into a 2D boolean array where true = roll.
//!
//! **Part 1 Strategy**: Count rolls with < 4 neighbors.
//! - For each roll, count adjacent rolls in all 8 directions
//! - A roll is accessible if neighbor_count < 4
//! - Complexity: O(rows * cols)
//!
//! **Part 2 Strategy**: Iteratively remove accessible rolls until none remain.
//! - Use a working copy of the grid
//! - Each iteration: find all accessible rolls, remove them, add to total
//! - Repeat until no accessible rolls found
//! - Complexity: O(rolls * rows * cols) worst case, but typically much faster

/// 8 directions: N, NE, E, SE, S, SW, W, NW
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

/// Count the number of adjacent rolls for a position in the grid.
fn count_neighbors(grid: &[Vec<bool>], row: usize, col: usize) -> usize {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let r = row as i32;
    let c = col as i32;

    DIRECTIONS
        .iter()
        .filter(|&&(dr, dc)| {
            let nr = r + dr;
            let nc = c + dc;
            nr >= 0
                && nr < rows
                && nc >= 0
                && nc < cols
                && grid[nr as usize][nc as usize]
        })
        .count()
}

/// Check if a roll at (row, col) is accessible (has < 4 neighbors).
fn is_accessible(grid: &[Vec<bool>], row: usize, col: usize) -> bool {
    grid[row][col] && count_neighbors(grid, row, col) < 4
}

pub fn part_one(input: &str) -> u64 {
    let grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;
    for row in 0..rows {
        for col in 0..cols {
            if is_accessible(&grid, row, col) {
                count += 1;
            }
        }
    }
    count
}

pub fn part_two(input: &str) -> u64 {
    let mut grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();

    let mut total_removed = 0;

    loop {
        // Find all accessible rolls in current state
        let mut to_remove: Vec<(usize, usize)> = Vec::new();
        for row in 0..rows {
            for col in 0..cols {
                if is_accessible(&grid, row, col) {
                    to_remove.push((row, col));
                }
            }
        }

        // If no accessible rolls, we're done
        if to_remove.is_empty() {
            break;
        }

        // Remove all accessible rolls
        for (row, col) in &to_remove {
            grid[*row][*col] = false;
        }

        total_removed += to_remove.len() as u64;
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

    #[test]
    fn test_neighbor_counting() {
        // Simple 3x3 grid with center roll surrounded by 8 rolls
        let grid = vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ];
        // Center has 8 neighbors
        assert_eq!(count_neighbors(&grid, 1, 1), 8);
        // Corner has 3 neighbors
        assert_eq!(count_neighbors(&grid, 0, 0), 3);
        // Edge has 5 neighbors
        assert_eq!(count_neighbors(&grid, 0, 1), 5);
    }
}
