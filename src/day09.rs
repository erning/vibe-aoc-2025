//! Day 9: Movie Theater
//!
//! ## Problem Description
//!
//! Part 1: Find the largest rectangle that uses any two red tiles as opposite corners.
//! For each pair of red tiles at (x1, y1) and (x2, y2), the rectangle area is |x2-x1| * |y2-y1|.
//!
//! Part 2: Find the largest rectangle where two opposite corners are red tiles and ALL
//! tiles within the rectangle are either red or green. Green tiles are those on the polygon
//! edges (connecting consecutive red tiles) or inside the polygon.
//!
//! ## Solution Approach
//!
//! **Part 1**: O(n^2) brute force - check all pairs of red tiles and compute areas.
//!
//! **Part 2**: For each pair of red tiles that form opposite corners of a rectangle,
//! check if all 4 corners of the rectangle are inside or on the polygon boundary.
//! We use the ray-casting algorithm for point-in-polygon tests, adapted for axis-aligned
//! polygon edges.

type Point = (i64, i64);

fn parse_input(input: &str) -> Vec<Point> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

pub fn part_one(input: &str) -> i64 {
    let points = parse_input(input);
    let n = points.len();
    let mut max_area = 0i64;

    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            // Area includes all tiles from corner to corner (inclusive)
            let width = (x2 - x1).abs() + 1;
            let height = (y2 - y1).abs() + 1;
            let area = width * height;
            max_area = max_area.max(area);
        }
    }

    max_area
}

/// Check if a point is on a horizontal segment from (x1, y) to (x2, y)
fn on_horizontal_segment(px: i64, py: i64, x1: i64, x2: i64, y: i64) -> bool {
    py == y && px >= x1.min(x2) && px <= x1.max(x2)
}

/// Check if a point is on a vertical segment from (x, y1) to (x, y2)
fn on_vertical_segment(px: i64, py: i64, x: i64, y1: i64, y2: i64) -> bool {
    px == x && py >= y1.min(y2) && py <= y1.max(y2)
}

/// Check if point (px, py) is on any edge of the polygon
fn is_on_polygon_edge(px: i64, py: i64, polygon: &[Point]) -> bool {
    let n = polygon.len();
    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n];

        // Horizontal edge
        if y1 == y2 && on_horizontal_segment(px, py, x1, x2, y1) {
            return true;
        }
        // Vertical edge
        if x1 == x2 && on_vertical_segment(px, py, x1, y1, y2) {
            return true;
        }
    }
    false
}

/// Ray casting algorithm for point-in-polygon test
/// Counts how many times a ray from (px, py) going right crosses the polygon boundary
fn is_inside_polygon(px: i64, py: i64, polygon: &[Point]) -> bool {
    let n = polygon.len();
    let mut crossings = 0;

    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n];

        // Only consider vertical edges that cross the horizontal ray from (px, py) to the right
        if x1 == x2 {
            // Vertical edge from (x1, y1) to (x1, y2)
            let y_min = y1.min(y2);
            let y_max = y1.max(y2);

            // The edge must be to the right of px and the ray must cross the edge
            if x1 > px && py >= y_min && py < y_max {
                crossings += 1;
            }
        }
    }

    crossings % 2 == 1
}

/// Check if a point is inside or on the polygon
fn is_inside_or_on_polygon(px: i64, py: i64, polygon: &[Point]) -> bool {
    is_on_polygon_edge(px, py, polygon) || is_inside_polygon(px, py, polygon)
}

/// Check if a polygon edge crosses through the interior of the rectangle
/// (not on the boundary, but strictly through the inside)
fn edge_crosses_rect_interior(
    edge_start: Point,
    edge_end: Point,
    rect_x_min: i64,
    rect_x_max: i64,
    rect_y_min: i64,
    rect_y_max: i64,
) -> bool {
    let (x1, y1) = edge_start;
    let (x2, y2) = edge_end;

    // Horizontal edge
    if y1 == y2 {
        let y = y1;
        // Check if the edge is strictly inside the rectangle vertically
        if y > rect_y_min && y < rect_y_max {
            let edge_x_min = x1.min(x2);
            let edge_x_max = x1.max(x2);
            // Check if the edge crosses through the interior horizontally
            // The edge must start outside (or on boundary) and end outside (or on boundary)
            // but pass through the interior
            if edge_x_min < rect_x_max && edge_x_max > rect_x_min {
                // Edge overlaps with rectangle's x-range
                // Check if the edge has part strictly inside the rectangle
                if edge_x_min < rect_x_min && edge_x_max > rect_x_min {
                    // Edge enters from left
                    return true;
                }
                if edge_x_max > rect_x_max && edge_x_min < rect_x_max {
                    // Edge exits from right
                    return true;
                }
                if edge_x_min >= rect_x_min && edge_x_max <= rect_x_max {
                    // Edge is entirely within the rectangle's x-range
                    // This is also crossing the interior
                    return true;
                }
            }
        }
    }

    // Vertical edge
    if x1 == x2 {
        let x = x1;
        // Check if the edge is strictly inside the rectangle horizontally
        if x > rect_x_min && x < rect_x_max {
            let edge_y_min = y1.min(y2);
            let edge_y_max = y1.max(y2);
            // Check if the edge crosses through the interior vertically
            if edge_y_min < rect_y_max && edge_y_max > rect_y_min {
                // Edge overlaps with rectangle's y-range
                if edge_y_min < rect_y_min && edge_y_max > rect_y_min {
                    return true;
                }
                if edge_y_max > rect_y_max && edge_y_min < rect_y_max {
                    return true;
                }
                if edge_y_min >= rect_y_min && edge_y_max <= rect_y_max {
                    return true;
                }
            }
        }
    }

    false
}

/// Check if the rectangle is entirely contained within the polygon
/// (all interior points are inside or on the polygon)
fn rectangle_in_polygon(
    rect_x_min: i64,
    rect_x_max: i64,
    rect_y_min: i64,
    rect_y_max: i64,
    polygon: &[Point],
) -> bool {
    // First check all 4 corners are inside or on polygon
    let corners = [
        (rect_x_min, rect_y_min),
        (rect_x_min, rect_y_max),
        (rect_x_max, rect_y_min),
        (rect_x_max, rect_y_max),
    ];

    for &(px, py) in &corners {
        if !is_inside_or_on_polygon(px, py, polygon) {
            return false;
        }
    }

    // Then check no polygon edge crosses through the interior of the rectangle
    let n = polygon.len();
    for i in 0..n {
        let edge_start = polygon[i];
        let edge_end = polygon[(i + 1) % n];
        if edge_crosses_rect_interior(
            edge_start, edge_end, rect_x_min, rect_x_max, rect_y_min,
            rect_y_max,
        ) {
            return false;
        }
    }

    true
}

pub fn part_two(input: &str) -> i64 {
    let points = parse_input(input);
    let n = points.len();

    let mut max_area = 0i64;

    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            // Skip if they form a degenerate rectangle (same x or same y)
            if x1 == x2 || y1 == y2 {
                continue;
            }

            // Area includes all tiles from corner to corner (inclusive)
            let width = (x2 - x1).abs() + 1;
            let height = (y2 - y1).abs() + 1;
            let area = width * height;

            // Skip if this can't beat current max
            if area <= max_area {
                continue;
            }

            // Get rectangle bounds
            let rect_x_min = x1.min(x2);
            let rect_x_max = x1.max(x2);
            let rect_y_min = y1.min(y2);
            let rect_y_max = y1.max(y2);

            if rectangle_in_polygon(
                rect_x_min, rect_x_max, rect_y_min, rect_y_max, &points,
            ) {
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
    fn example() {
        let input = read_example(9);
        assert_eq!(part_one(&input), 50);
        assert_eq!(part_two(&input), 24);
    }
}
