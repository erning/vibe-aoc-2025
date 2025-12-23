//! Day 12: Christmas Tree Farm
//!
//! Optimized packing algorithm using bitmask grid representation.

use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Shape {
    cells: Vec<(i32, i32)>,
}

fn parse_input(input: &str) -> (Vec<Vec<Shape>>, Vec<Region>) {
    let mut shapes_input = Vec::new();
    let mut lines: Vec<String> = Vec::new();

    for line in input.lines() {
        if line.contains(':') {
            if !lines.is_empty() {
                shapes_input.push(lines.clone());
                lines.clear();
            }
        } else if !line.is_empty() {
            lines.push(line.to_string());
        }
    }
    if !lines.is_empty() {
        shapes_input.push(lines);
    }

    let shapes: Vec<Vec<Shape>> = shapes_input.iter().map(|lines| {
        let mut cells = Vec::new();
        for (r, line) in lines.iter().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                if ch == '#' {
                    cells.push((r as i32, c as i32));
                }
            }
        }
        let base = normalize(cells);
        get_transformations(&base)
    }).collect();

    let regions: Vec<Region> = input.lines()
        .filter(|line| line.contains('x') && line.contains(':'))
        .map(|line| {
            let parts: Vec<&str> = line.split(&[':', ' '][..]).collect();
            let dims: Vec<usize> = parts[0].split('x').map(|s| s.parse().unwrap()).collect();
            let required: Vec<usize> = parts[1..].iter()
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            Region { width: dims[0], height: dims[1], required }
        })
        .collect();

    (shapes, regions)
}

#[derive(Clone)]
struct Region {
    width: usize,
    height: usize,
    required: Vec<usize>,
}

fn normalize(mut cells: Vec<(i32, i32)>) -> Shape {
    let min_r = cells.iter().map(|&(r, _)| r).min().unwrap();
    let min_c = cells.iter().map(|&(_, c)| c).min().unwrap();
    cells = cells.into_iter().map(|(r, c)| (r - min_r, c - min_c)).collect();
    cells.sort_unstable();
    cells.dedup();
    Shape { cells }
}

fn get_transformations(shape: &Shape) -> Vec<Shape> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for flip in [1i32, -1] {
        for rot in 0..4 {
            let mut cells = Vec::with_capacity(shape.cells.len());
            for &(r, c) in &shape.cells {
                let (r, c) = (r, c * flip);
                let (r, c) = match rot {
                    0 => (r, c),
                    1 => (-c, r),
                    2 => (-r, -c),
                    3 => (c, -r),
                    _ => unreachable!(),
                };
                cells.push((r, c));
            }
            let normalized = normalize(cells);
            let key: &[(i32, i32)] = &normalized.cells;
            if seen.insert(key.to_vec()) {
                result.push(normalized);
            }
        }
    }
    result
}

fn can_fit(shapes: &[Vec<Shape>], region: &Region) -> bool {
    let total_cells: usize = region.required.iter()
        .enumerate()
        .map(|(i, &c)| c * shapes[i][0].cells.len())
        .sum();
    if total_cells > region.width * region.height {
        return false;
    }

    let mut items: Vec<usize> = Vec::new();
    for (i, &c) in region.required.iter().enumerate() {
        items.extend(std::iter::repeat(i).take(c));
    }
    items.sort_unstable_by_key(|&i| std::cmp::Reverse(shapes[i][0].cells.len()));

    let mut grid = vec![0u64; region.height * region.width];
    solve_fast(&mut grid, shapes, &items, 0, region.width, region.height)
}

fn solve_fast(
    grid: &mut [u64],
    shapes: &[Vec<Shape>],
    items: &[usize],
    idx: usize,
    width: usize,
    height: usize,
) -> bool {
    if idx >= items.len() {
        return true;
    }

    let shape_idx = items[idx];

    // Precompute all valid positions for this shape
    let mut placements = Vec::new();

    for shape in &shapes[shape_idx] {
        for pos_r in 0..height {
            for pos_c in 0..width {
                if can_place_fast(grid, shape, pos_r, pos_c, width, height) {
                    placements.push((shape, pos_r, pos_c));
                }
            }
        }
    }

    // Try each placement
    for (shape, pos_r, pos_c) in placements {
        place_fast(grid, shape, pos_r, pos_c, width);

        if solve_fast(grid, shapes, items, idx + 1, width, height) {
            return true;
        }

        unplace_fast(grid, shape, pos_r, pos_c, width);
    }

    false
}

fn can_place_fast(
    grid: &[u64],
    shape: &Shape,
    pos_r: usize,
    pos_c: usize,
    width: usize,
    height: usize,
) -> bool {
    shape.cells.iter().all(|&(dr, dc)| {
        let nr = pos_r as i32 + dr;
        let nc = pos_c as i32 + dc;
        nr >= 0 && nc >= 0 && (nr as usize) < height && (nc as usize) < width
            && grid[nr as usize * width + nc as usize] == 0
    })
}

fn place_fast(grid: &mut [u64], shape: &Shape, pos_r: usize, pos_c: usize, width: usize) {
    for &(dr, dc) in &shape.cells {
        let nr = pos_r as usize + dr as usize;
        let nc = pos_c as usize + dc as usize;
        grid[nr * width + nc] = 1;
    }
}

fn unplace_fast(grid: &mut [u64], shape: &Shape, pos_r: usize, pos_c: usize, width: usize) {
    for &(dr, dc) in &shape.cells {
        let nr = pos_r as usize + dr as usize;
        let nc = pos_c as usize + dc as usize;
        grid[nr * width + nc] = 0;
    }
}

pub fn part_one(input: &str) -> usize {
    let (shapes, regions) = parse_input(input);
    regions.iter().filter(|r| can_fit(&shapes, r)).count()
}

pub fn part_two(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn part1_example() {
        let input = read_example(12);
        assert_eq!(part_one(&input), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part_two(""), 0);
    }
}
