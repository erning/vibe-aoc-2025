//! Day 9: Movie Theater
//!
//! ## Problem Description
//!
//! Find largest rectangle using red/green tiles as corner points.
//! - Part 1: Largest rectangle between any two red tiles
//! - Part 2: Largest rectangle using only red or green tiles

use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 2 {
                let x = parts[0].trim().parse().ok()?;
                let y = parts[1].trim().parse().ok()?;
                Some(Point { x, y })
            } else {
                None
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> u64 {
    let points = parse_input(input);

    let mut max_area = 0u64;

    // Check all pairs of points as opposite corners
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = points[i];
            let p2 = points[j];

            let width = ((p1.x - p2.x).abs() + 1) as u64;
            let height = ((p1.y - p2.y).abs() + 1) as u64;
            let area = width * height;

            max_area = max_area.max(area);
        }
    }

    max_area
}

pub fn part_two(input: &str) -> u64 {
    let points = parse_input(input);

    // Build the green region: includes red points and all green connecting tiles
    let mut green_tiles = HashSet::new();

    // Add all red tiles
    for &p in &points {
        green_tiles.insert(p);
    }

    // Add green tiles connecting consecutive red tiles
    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];

        // Connect p1 to p2 with green tiles
        if p1.x == p2.x {
            // Vertical line
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);
            for y in min_y..=max_y {
                green_tiles.insert(Point { x: p1.x, y });
            }
        } else if p1.y == p2.y {
            // Horizontal line
            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            for x in min_x..=max_x {
                green_tiles.insert(Point { x, y: p1.y });
            }
        }
    }

    // Also fill the interior of the polygon (using a flood fill or scanline approach)
    // For simplicity, we'll use a scanline approach
    // Find bounding box
    let min_x = points.iter().map(|p| p.x).min().unwrap_or(0);
    let _max_x = points.iter().map(|p| p.x).max().unwrap_or(0);
    let min_y = points.iter().map(|p| p.y).min().unwrap_or(0);
    let max_y = points.iter().map(|p| p.y).max().unwrap_or(0);

    // For each row, find intersections with the polygon and mark interior
    for y in min_y..=max_y {
        let mut crossings = Vec::new();

        // Find all x coordinates where the boundary crosses this row
        for i in 0..points.len() {
            let p1 = points[i];
            let p2 = points[(i + 1) % points.len()];

            if (p1.y <= y && y <= p2.y) || (p2.y <= y && y <= p1.y) {
                if p1.y != p2.y {
                    // This edge crosses the horizontal line at y
                    crossings.push((p1.x.min(p2.x), p1.x.max(p2.x)));
                }
            }
        }

        // Sort crossings and fill interior segments
        crossings.sort();
        let mut inside = false;
        let mut last_x = min_x;

        for (x1, x2) in crossings {
            if inside {
                // Fill from last_x to x1
                for x in last_x..=x1 {
                    green_tiles.insert(Point { x, y });
                }
            }
            inside = !inside;
            last_x = x2;
        }
    }

    // Check all pairs of red points for largest rectangle using only green/red tiles
    let mut max_area = 0u64;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = points[i];
            let p2 = points[j];

            // Check if all corners and edges of this rectangle are green/red
            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);

            let mut all_green = true;

            // Check all points in the rectangle
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    if !green_tiles.contains(&Point { x, y }) {
                        all_green = false;
                        break;
                    }
                }
                if !all_green {
                    break;
                }
            }

            if all_green {
                let width = (max_x - min_x) as u64;
                let height = (max_y - min_y) as u64;
                let area = (width + 1) * (height + 1);
                max_area = max_area.max(area);
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
    fn example_part_one() {
        let input = read_example(9);
        assert_eq!(part_one(&input), 50);
    }

    #[test]
    fn example_part_two() {
        let input = read_example(9);
        assert_eq!(part_two(&input), 24);
    }
}
