//! Day 7: Laboratories
//!
//! Tachyon beam splitting through a manifold.
//!
//! Part 1: Count total split events (unique splitter positions hit by any beam).
//! Part 2: Count total timelines (each split doubles the timeline count).

use std::collections::{HashSet, VecDeque};

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let grid: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let mut start = (0, 0);
    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start = (r, c);
            }
        }
    }
    (grid, start)
}

pub fn part_one(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    let height = grid.len();
    let width = grid[0].len();

    // BFS: track all active beams and count unique splitters hit
    let mut beams: VecDeque<(usize, usize)> = VecDeque::new();
    beams.push_back(start);

    let mut split_count = 0;
    let mut visited_splitters: HashSet<(usize, usize)> = HashSet::new();

    while let Some((r, c)) = beams.pop_front() {
        // Move down until we hit a splitter or exit
        let mut row = r + 1;
        while row < height {
            if grid[row][c] == '^' {
                if visited_splitters.insert((row, c)) {
                    split_count += 1;
                    // Add left and right beams
                    if c > 0 {
                        beams.push_back((row, c - 1));
                    }
                    if c + 1 < width {
                        beams.push_back((row, c + 1));
                    }
                }
                break;
            }
            row += 1;
        }
    }

    split_count
}

pub fn part_two(input: &str) -> u64 {
    let (grid, start) = parse_input(input);
    let width = grid[0].len();

    // Process by rows: track how many timelines are in each column
    // When a timeline hits a splitter, it contributes to left and right columns
    let mut column_timelines: Vec<u64> = vec![0; width];
    column_timelines[start.1] = 1;

    for row in grid.iter().skip(start.0 + 1) {
        let mut new_timelines: Vec<u64> = vec![0; width];

        for (col, &timelines) in column_timelines.iter().enumerate() {
            if timelines > 0 {
                if row[col] == '^' {
                    if col > 0 {
                        new_timelines[col - 1] += timelines;
                    }
                    if col + 1 < width {
                        new_timelines[col + 1] += timelines;
                    }
                } else {
                    new_timelines[col] += timelines;
                }
            }
        }

        column_timelines = new_timelines;
    }

    column_timelines.iter().sum()
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
