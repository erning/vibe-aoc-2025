//! Day 4: Printing Department
//!
//! Part 1: Count paper rolls (@) with fewer than 4 adjacent paper rolls.
//! Part 2: Iteratively remove accessible rolls until no more can be removed.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Reads the grid as a vector of strings (rows).
//!
//! **Part 1 Strategy**:
//! - Iterate through each cell in the grid
//! - For each paper roll (@), count its 8 neighbors (horizontal, vertical, diagonal)
//! - A roll is accessible if it has < 4 neighboring rolls
//! - Count all accessible rolls
//!
//! **Part 2 Strategy**:
//! - Use BFS-style iterative removal
//! - Repeatedly find all accessible rolls (same criteria as Part 1)
//! - Remove them all at once and continue until no more rolls are accessible
//! - Keep track of total removed rolls
//!
//! **Complexity**: O(h * w) for each iteration where h=height, w=width.
//! In worst case, Part 2 runs O(n) iterations where n is number of rolls,
//! giving O(n * h * w) overall.

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

/// Count the number of adjacent paper rolls at position (r, c)
fn count_adjacent(grid: &[Vec<char>], r: usize, c: usize) -> usize {
    let h = grid.len();
    let w = grid[0].len();
    let mut count = 0;

    // Check all 8 directions
    for dr in -1i32..=1 {
        for dc in -1i32..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < h as i32 && nc >= 0 && nc < w as i32
                && grid[nr as usize][nc as usize] == '@'
            {
                count += 1;
            }
        }
    }
    count
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let h = grid.len();
    let w = grid[0].len();
    let mut count = 0;

    for r in 0..h {
        for c in 0..w {
            if grid[r][c] == '@' && count_adjacent(&grid, r, c) < 4 {
                count += 1;
            }
        }
    }
    count
}

pub fn part_two(input: &str) -> usize {
    let mut grid = parse_input(input);
    let h = grid.len();
    let w = grid[0].len();
    let mut total_removed = 0;

    loop {
        // Find all accessible rolls
        let mut to_remove: Vec<(usize, usize)> = Vec::new();
        for r in 0..h {
            for c in 0..w {
                if grid[r][c] == '@' && count_adjacent(&grid, r, c) < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        // Remove all accessible rolls
        for (r, c) in to_remove {
            grid[r][c] = '.';
            total_removed += 1;
        }
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
