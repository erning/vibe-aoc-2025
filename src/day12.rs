//! Day 12: Christmas Tree Farm
//!
//! 2D packing problem with polyominoes.
//!
//! Part 1: Count regions that can fit all their listed presents.
//! Part 2: Free star (returns 0).

use std::collections::HashSet;

type Shape = Vec<(i32, i32)>;
type Region = (usize, usize, Vec<usize>);

fn parse_input(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let blocks: Vec<&str> = input.trim().split("\n\n").collect();

    // Parse shapes - each block before regions is a shape
    let mut shapes: Vec<Shape> = Vec::new();
    let mut regions: Vec<Region> = Vec::new();

    for block in blocks {
        let lines: Vec<&str> = block.lines().collect();

        // Check if this is a region block (contains "x")
        if lines.iter().any(|l| l.contains('x')) {
            // Parse regions
            for line in lines {
                if line.contains('x') {
                    let mut parts = line.split(": ");
                    let dims = parts.next().unwrap();
                    let counts_str = parts.next().unwrap();

                    let dim_parts: Vec<usize> =
                        dims.split('x').map(|s| s.parse().unwrap()).collect();
                    let width = dim_parts[0];
                    let height = dim_parts[1];

                    let counts: Vec<usize> = counts_str
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();

                    regions.push((width, height, counts));
                }
            }
        } else {
            // Parse shape
            let mut shape: Vec<(i32, i32)> = Vec::new();
            let mut row = 0i32;
            for line in lines {
                if line.ends_with(':') {
                    continue; // Skip index line
                }
                for (col, ch) in line.chars().enumerate() {
                    if ch == '#' {
                        shape.push((row, col as i32));
                    }
                }
                row += 1;
            }
            if !shape.is_empty() {
                shapes.push(shape);
            }
        }
    }

    (shapes, regions)
}

fn normalize_shape(shape: &Shape) -> Shape {
    let min_r = shape.iter().map(|(r, _)| *r).min().unwrap_or(0);
    let min_c = shape.iter().map(|(_, c)| *c).min().unwrap_or(0);
    let mut normalized: Shape =
        shape.iter().map(|(r, c)| (r - min_r, c - min_c)).collect();
    normalized.sort();
    normalized
}

fn rotate_90(shape: &Shape) -> Shape {
    // (r, c) -> (c, -r)
    normalize_shape(&shape.iter().map(|(r, c)| (*c, -*r)).collect())
}

fn flip_horizontal(shape: &Shape) -> Shape {
    // (r, c) -> (r, -c)
    normalize_shape(&shape.iter().map(|(r, c)| (*r, -*c)).collect())
}

fn all_orientations(shape: &Shape) -> Vec<Shape> {
    let mut orientations: HashSet<Vec<(i32, i32)>> = HashSet::new();
    let mut current = normalize_shape(shape);

    for _ in 0..4 {
        orientations.insert(current.clone());
        orientations.insert(flip_horizontal(&current));
        current = rotate_90(&current);
    }

    orientations.into_iter().collect()
}

fn can_place(
    grid: &[Vec<bool>],
    shape: &Shape,
    r: i32,
    c: i32,
    height: usize,
    width: usize,
) -> bool {
    for (dr, dc) in shape {
        let nr = r + dr;
        let nc = c + dc;
        if nr < 0 || nr >= height as i32 || nc < 0 || nc >= width as i32 {
            return false;
        }
        if grid[nr as usize][nc as usize] {
            return false;
        }
    }
    true
}

fn place(grid: &mut [Vec<bool>], shape: &Shape, r: i32, c: i32) {
    for (dr, dc) in shape {
        grid[(r + dr) as usize][(c + dc) as usize] = true;
    }
}

fn unplace(grid: &mut [Vec<bool>], shape: &Shape, r: i32, c: i32) {
    for (dr, dc) in shape {
        grid[(r + dr) as usize][(c + dc) as usize] = false;
    }
}

fn solve(
    grid: &mut Vec<Vec<bool>>,
    pieces: &[Vec<Shape>],
    piece_idx: usize,
    height: usize,
    width: usize,
) -> bool {
    if piece_idx >= pieces.len() {
        return true;
    }

    for orientation in &pieces[piece_idx] {
        for r in 0..height as i32 {
            for c in 0..width as i32 {
                if can_place(grid, orientation, r, c, height, width) {
                    place(grid, orientation, r, c);
                    if solve(grid, pieces, piece_idx + 1, height, width) {
                        return true;
                    }
                    unplace(grid, orientation, r, c);
                }
            }
        }
    }

    false
}

fn can_fit_region(
    shapes: &[Shape],
    width: usize,
    height: usize,
    counts: &[usize],
) -> bool {
    // Build list of pieces (with repetition based on counts)
    let mut pieces: Vec<Vec<Shape>> = Vec::new();
    for (shape_idx, &count) in counts.iter().enumerate() {
        let orientations = all_orientations(&shapes[shape_idx]);
        for _ in 0..count {
            pieces.push(orientations.clone());
        }
    }

    if pieces.is_empty() {
        return true;
    }

    // Check total area
    let total_shape_area: usize = counts
        .iter()
        .enumerate()
        .map(|(i, &c)| c * shapes[i].len())
        .sum();
    let region_area = width * height;
    if total_shape_area > region_area {
        return false;
    }

    let mut grid = vec![vec![false; width]; height];
    solve(&mut grid, &pieces, 0, height, width)
}

pub fn part_one(input: &str) -> usize {
    let (shapes, regions) = parse_input(input);
    regions
        .iter()
        .filter(|(width, height, counts)| {
            can_fit_region(&shapes, *width, *height, counts)
        })
        .count()
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
