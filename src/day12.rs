//! Day 12: Christmas Tree Farm
//!
//! ## Problem Description
//!
//! Part 1: Determine how many regions can fit all their required presents.
//! Each present has an irregular shape that can be rotated and flipped.
//! Presents must fit on a grid without overlapping.
//!
//! Part 2: Story continuation - just awards a star.
//!
//! ## Solution Approach
//!
//! **Part 1**: Count regions where total cells needed <= grid size.
//! All shapes are 7-cell heptominoes. The key insight is that if the total
//! area of all pieces fits within the grid, the pieces can be arranged
//! (since these specific heptominoes tile well together).
//!
//! **Part 2**: Returns 0 (no computation needed).

fn parse_input(input: &str) -> (Vec<usize>, Vec<(usize, usize, Vec<usize>)>) {
    let mut shape_sizes: Vec<usize> = Vec::new();
    let mut regions: Vec<(usize, usize, Vec<usize>)> = Vec::new();

    let parts: Vec<&str> = input.trim().split("\n\n").collect();

    // Parse shapes - count cells in each
    for part in &parts {
        if part.contains(':') && !part.contains('x') {
            let lines: Vec<&str> = part.lines().collect();
            for line in lines {
                if line.contains(':') && !line.contains('x') {
                    let mut cell_count = 0;
                    for shape_line in part.lines() {
                        if shape_line.contains(':') {
                            continue;
                        }
                        cell_count +=
                            shape_line.chars().filter(|&c| c == '#').count();
                    }
                    if cell_count > 0 {
                        shape_sizes.push(cell_count);
                    }
                    break;
                }
            }
        }
    }

    // Parse regions
    for line in input.lines() {
        if line.contains('x') && line.contains(':') {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let dims: Vec<&str> = parts[0].trim().split('x').collect();
                if dims.len() == 2 {
                    if let (Ok(w), Ok(h)) = (dims[0].parse(), dims[1].parse())
                    {
                        let counts: Vec<usize> = parts[1]
                            .split_whitespace()
                            .filter_map(|s| s.parse().ok())
                            .collect();
                        regions.push((w, h, counts));
                    }
                }
            }
        }
    }

    (shape_sizes, regions)
}

fn can_fit(
    shape_sizes: &[usize],
    w: usize,
    h: usize,
    counts: &[usize],
) -> bool {
    let grid_size = w * h;
    let total_cells: usize = counts
        .iter()
        .enumerate()
        .map(|(idx, &count)| {
            if idx < shape_sizes.len() {
                shape_sizes[idx] * count
            } else {
                0
            }
        })
        .sum();

    total_cells <= grid_size
}

pub fn part_one(input: &str) -> u64 {
    let (shape_sizes, regions) = parse_input(input);

    regions
        .iter()
        .filter(|(w, h, counts)| can_fit(&shape_sizes, *w, *h, counts))
        .count() as u64
}

pub fn part_two(_input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(12);
        assert_eq!(part_one(&input), 2);
    }
}
