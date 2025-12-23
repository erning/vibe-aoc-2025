//! Day 9: Movie Theater
//!
//! Find the largest rectangle using red tiles as opposite corners.
//!
//! Part 1: Find largest rectangle with any two red tiles as corners.
//! Part 2: Find largest rectangle where all tiles inside are red or green.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse coordinates (x,y) from comma-separated lines.
//!
//! **Part 1 Strategy**: Brute force through all pairs of red tiles:
//! - For each pair of tiles, compute rectangle area
//! - Area = (|x1 - x2| + 1) * (|y1 - y2| + 1)
//! - Track maximum area found
//!
//! **Part 2 Strategy**: Check if rectangle contains only red/green tiles:
//! - Green tiles = perimeter (connecting red tiles) + interior
//! - Use ray casting for point-in-polygon test
//! - For each rectangle, sample points to check validity
//! - Sampling strategy: check all points for small rectangles,
//!   strategic sampling for large ones
//!
//! **Complexity**: O(n²) for part 1, O(n² * k) for part 2 where n is
//! the number of red tiles and k is the sampling cost per rectangle.

use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            let x = parts[0].parse().unwrap();
            let y = parts[1].parse().unwrap();
            (x, y)
        })
        .collect()
}

/// Get perimeter points (green tiles on the boundary)
fn get_perimeter_points(red_tiles: &[(i64, i64)]) -> HashSet<(i64, i64)> {
    let mut perimeter = HashSet::new();
    let n = red_tiles.len();

    for i in 0..n {
        let (x1, y1) = red_tiles[i];
        let (x2, y2) = red_tiles[(i + 1) % n];

        // Add all points on the line segment from (x1, y1) to (x2, y2)
        if x1 == x2 {
            // Vertical line
            let (start, end) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
            for y in start..=end {
                perimeter.insert((x1, y));
            }
        } else {
            // Horizontal line
            let (start, end) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
            for x in start..=end {
                perimeter.insert((x, y1));
            }
        }
    }

    perimeter
}

/// Check if point q is on segment pr
fn on_segment(q: (i64, i64), p: (i64, i64), r: (i64, i64)) -> bool {
    let (qx, qy) = q;
    let (px, py) = p;
    let (rx, ry) = r;

    // Check if q is collinear with p and r
    let cross = (qx - px) * (ry - py) - (qy - py) * (rx - px);
    if cross != 0 {
        return false;
    }

    let min_x = px.min(rx);
    let max_x = px.max(rx);
    let min_y = py.min(ry);
    let max_y = py.max(ry);

    qx >= min_x && qx <= max_x && qy >= min_y && qy <= max_y
}

/// Check if a point is inside the polygon using ray casting
fn is_inside_polygon(point: (i64, i64), polygon: &[(i64, i64)]) -> bool {
    let (x, y) = point;
    let mut inside = false;
    let n = polygon.len();

    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n];

        // Check if point is on an edge
        if on_segment(point, (x1, y1), (x2, y2)) {
            return true;
        }

        // Ray casting algorithm
        if (y1 > y) != (y2 > y) {
            let x_intersect = x2 as f64
                - (y2 as f64 - y as f64) * (x2 as f64 - x1 as f64)
                    / (y2 as f64 - y1 as f64);
            if (x as f64) < x_intersect {
                inside = !inside;
            }
        }
    }

    inside
}

pub fn part_one(input: &str) -> i64 {
    let red_tiles = parse_input(input);
    let mut max_area = 0i64;

    // Brute force all pairs of red tiles
    for i in 0..red_tiles.len() {
        for j in (i + 1)..red_tiles.len() {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];

            let width = (x1 - x2).abs() + 1;
            let height = (y1 - y2).abs() + 1;
            let area = width * height;

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

/// Check if a rectangle contains only red or green tiles
fn is_valid_rectangle(
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    red_tiles: &HashSet<(i64, i64)>,
    perimeter: &HashSet<(i64, i64)>,
    polygon: &[(i64, i64)],
) -> bool {
    let (min_x, max_x) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
    let (min_y, max_y) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

    // Check all points for rectangles up to 10,000 area
    let area = (max_x - min_x + 1) * (max_y - min_y + 1);
    if area <= 10000 {
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let tile = (x, y);
                if !red_tiles.contains(&tile)
                    && !perimeter.contains(&tile)
                    && !is_inside_polygon(tile, polygon)
                {
                    return false;
                }
            }
        }
        return true;
    }

    // For larger rectangles, use dense sampling
    // We sample enough points to detect any significant region of invalid tiles
    let step = ((area as f64).sqrt() / 20.0).ceil() as usize;

    // Sample interior
    for x in (min_x..=max_x).step_by(step.max(1)) {
        for y in (min_y..=max_y).step_by(step.max(1)) {
            let tile = (x, y);
            if !red_tiles.contains(&tile)
                && !perimeter.contains(&tile)
                && !is_inside_polygon(tile, polygon)
            {
                return false;
            }
        }
    }

    // Check boundary points more carefully (every 100 points)
    let boundary_step = 100;
    for x in (min_x..=max_x).step_by(boundary_step) {
        for y in [min_y, max_y].iter() {
            let tile = (x, *y);
            if !red_tiles.contains(&tile)
                && !perimeter.contains(&tile)
                && !is_inside_polygon(tile, polygon)
            {
                return false;
            }
        }
    }
    for y in ((min_y + boundary_step as i64)..max_y).step_by(boundary_step) {
        for x in [min_x, max_x].iter() {
            let tile = (*x, y);
            if !red_tiles.contains(&tile)
                && !perimeter.contains(&tile)
                && !is_inside_polygon(tile, polygon)
            {
                return false;
            }
        }
    }

    true
}

pub fn part_two(input: &str) -> i64 {
    let red_tiles = parse_input(input);
    let red_set: HashSet<(i64, i64)> = red_tiles.iter().cloned().collect();
    let perimeter = get_perimeter_points(&red_tiles);
    let mut max_area = 0i64;

    // Brute force all pairs of red tiles
    for i in 0..red_tiles.len() {
        for j in (i + 1)..red_tiles.len() {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];

            let width = (x1 - x2).abs() + 1;
            let height = (y1 - y2).abs() + 1;
            let area = width * height;

            if area > max_area
                && is_valid_rectangle(
                    x1, y1, x2, y2, &red_set, &perimeter, &red_tiles,
                )
            {
                max_area = area;
            }
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_part1() {
        let input = read_example(9);
        assert_eq!(part_one(&input), 50);
    }

    #[test]
    fn example_part2() {
        let input = read_example(9);
        assert_eq!(part_two(&input), 24);
    }
}
