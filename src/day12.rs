//! Day 12: Christmas Tree Farm
//!
//! ## Problem Description
//!
//! 2D shape packing: fit oddly-shaped presents into rectangular regions.
//! - Count how many regions can successfully fit all required presents
//! - Part 1 & 2: Similar logic (Part 2 just fluff/star message)

#[derive(Clone, Debug)]
struct Shape {
    cells: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Shape {
    fn new(grid: &[&str]) -> Self {
        let mut cells = Vec::new();
        let mut width = 0;
        let mut height = grid.len();

        for (r, row) in grid.iter().enumerate() {
            width = width.max(row.len());
            for (c, ch) in row.chars().enumerate() {
                if ch == '#' {
                    cells.push((r, c));
                }
            }
        }

        // Normalize to top-left corner
        if !cells.is_empty() {
            let min_r =
                cells.iter().map(|(r, _)| r).min().copied().unwrap_or(0);
            let min_c =
                cells.iter().map(|(_, c)| c).min().copied().unwrap_or(0);

            cells =
                cells.iter().map(|(r, c)| (r - min_r, c - min_c)).collect();

            width =
                cells.iter().map(|(_, c)| c).max().copied().unwrap_or(0) + 1;
            height =
                cells.iter().map(|(r, _)| r).max().copied().unwrap_or(0) + 1;
        }

        Shape {
            cells,
            width,
            height,
        }
    }

    fn rotate_90(&self) -> Shape {
        let mut cells = Vec::new();
        for (r, c) in &self.cells {
            cells.push((self.width - 1 - c, *r));
        }

        // Normalize
        if !cells.is_empty() {
            let min_r =
                cells.iter().map(|(r, _)| r).min().copied().unwrap_or(0);
            let min_c =
                cells.iter().map(|(_, c)| c).min().copied().unwrap_or(0);
            cells =
                cells.iter().map(|(r, c)| (r - min_r, c - min_c)).collect();
        }

        let new_width = self.height;
        let new_height = self.width;

        Shape {
            cells,
            width: new_width,
            height: new_height,
        }
    }

    fn get_rotations(&self) -> Vec<Shape> {
        let mut rotations = vec![self.clone()];
        let mut current = self.clone();

        for _ in 0..3 {
            current = current.rotate_90();
            if !rotations.iter().any(|s| {
                s.width == current.width
                    && s.height == current.height
                    && s.cells == current.cells
            }) {
                rotations.push(current.clone());
            }
        }

        rotations
    }
}

fn can_place(
    grid: &mut Vec<Vec<bool>>,
    shape: &Shape,
    start_r: usize,
    start_c: usize,
) -> bool {
    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else { 0 };

    for (dr, dc) in &shape.cells {
        let r = start_r + dr;
        let c = start_c + dc;

        if r >= height || c >= width || grid[r][c] {
            return false;
        }
    }

    true
}

fn place_shape(
    grid: &mut Vec<Vec<bool>>,
    shape: &Shape,
    start_r: usize,
    start_c: usize,
) {
    for (dr, dc) in &shape.cells {
        grid[start_r + dr][start_c + dc] = true;
    }
}

fn remove_shape(
    grid: &mut Vec<Vec<bool>>,
    shape: &Shape,
    start_r: usize,
    start_c: usize,
) {
    for (dr, dc) in &shape.cells {
        grid[start_r + dr][start_c + dc] = false;
    }
}

fn try_pack(
    grid: &mut Vec<Vec<bool>>,
    _shape_idx: usize,
    counts: &mut [usize],
    all_shapes: &[Vec<Shape>],
) -> bool {
    // Find next unfilled cell
    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else { 0 };

    let mut found = false;
    let mut start_r = 0;
    let mut start_c = 0;

    for r in 0..height {
        for c in 0..width {
            if !grid[r][c] {
                start_r = r;
                start_c = c;
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    if !found {
        // All filled, check if all shapes used
        return counts.iter().all(|&c| c == 0);
    }

    // Try to place a shape at this position
    for shape_type in 0..counts.len() {
        if counts[shape_type] == 0 {
            continue;
        }

        for variant in &all_shapes[shape_type] {
            if can_place(grid, variant, start_r, start_c) {
                counts[shape_type] -= 1;
                place_shape(grid, variant, start_r, start_c);

                if try_pack(grid, shape_type, counts, all_shapes) {
                    return true;
                }

                remove_shape(grid, variant, start_r, start_c);
                counts[shape_type] += 1;
            }
        }
    }

    false
}

fn parse_input(
    input: &str,
) -> (Vec<Vec<Shape>>, Vec<(usize, usize, Vec<usize>)>) {
    let mut shapes = Vec::new();
    let mut regions = Vec::new();
    let mut lines = input.lines();

    // Parse shapes
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        if line.ends_with(':') {
            let mut shape_lines = Vec::new();
            while let Some(next_line) = lines.next() {
                if next_line.is_empty() || next_line.ends_with(':') {
                    // Put it back
                    if next_line.ends_with(':') {
                        // Handle this by looping
                    }
                    break;
                }
                shape_lines.push(next_line);
            }

            let shape = Shape::new(
                &shape_lines.iter().map(|s| *s).collect::<Vec<_>>(),
            );
            let rotations = shape.get_rotations();
            shapes.push(rotations);
        }
    }

    // Parse regions
    for line in lines {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            let dims: Vec<&str> = parts[0].split('x').collect();
            if dims.len() == 2 {
                let width = dims[0].parse::<usize>().unwrap_or(0);
                let height = dims[1].parse::<usize>().unwrap_or(0);
                let counts: Vec<usize> = parts[1]
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                regions.push((width, height, counts));
            }
        }
    }

    (shapes, regions)
}

pub fn part_one(input: &str) -> u64 {
    let (shapes, regions) = parse_input(input);

    let mut total = 0u64;

    for (width, height, mut counts) in regions {
        let mut grid = vec![vec![false; width]; height];
        if try_pack(&mut grid, 0, &mut counts, &shapes) {
            total += 1;
        }
    }

    total
}

pub fn part_two(input: &str) -> u64 {
    part_one(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_part_one() {
        let input = read_example(12);
        assert_eq!(part_one(&input), 2);
    }
}
