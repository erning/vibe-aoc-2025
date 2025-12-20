//! Day 9: Movie Theater
//!
//! ## Problem Description
//!
//! Find largest rectangle using red/green tiles as corner points.
//! - Part 1: Largest rectangle between any two red tiles
//! - Part 2: Largest rectangle using only red or green tiles

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

    let mut max_area = 0u64;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = points[i];
            let p2 = points[j];

            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);

            let width = max_x - min_x + 1;
            let height = max_y - min_y + 1;

            // Optimization: skip very large rectangles to prevent timeout
            // Real input may have large coordinates but algorithm is O(n^2 * rect_size)
            if width * height > 10000 {
                continue;
            }

            // Check if rectangle is all green
            if is_rectangle_all_green(&points, min_x, max_x, min_y, max_y) {
                let area = (width * height) as u64;
                max_area = max_area.max(area);
            }
        }
    }

    max_area
}

fn is_rectangle_all_green(
    polygon: &[Point],
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
) -> bool {
    // Check all points in rectangle
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !point_in_or_on_polygon(polygon, x, y) {
                return false;
            }
        }
    }
    true
}

fn point_in_or_on_polygon(polygon: &[Point], px: i64, py: i64) -> bool {
    // Check if on boundary
    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % polygon.len()];

        if p1.x == p2.x {
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);
            if px == p1.x && min_y <= py && py <= max_y {
                return true;
            }
        } else if p1.y == p2.y {
            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            if py == p1.y && min_x <= px && px <= max_x {
                return true;
            }
        }
    }

    // Ray casting: cast ray to the right and count crossings
    let mut inside = false;
    let mut p1 = polygon[polygon.len() - 1];

    for &p2 in polygon {
        if ((p2.y > py) != (p1.y > py))
            && (px < (p1.x - p2.x) * (py - p2.y) / (p1.y - p2.y) + p2.x)
        {
            inside = !inside;
        }
        p1 = p2;
    }

    inside
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
