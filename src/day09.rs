//! Day 9: Movie Theater
//!
//! Find largest rectangle with red tiles at opposite corners.
//!
//! Part 1: Any two red tiles can be corners.
//! Part 2: Rectangle must contain only red/green tiles (inside/on polygon).

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let parts: Vec<i64> =
                line.split(',').map(|s| s.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect()
}

pub fn part_one(input: &str) -> i64 {
    let points = parse_input(input);
    let n = points.len();
    let mut max_area = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            // Area includes both endpoints: (|dx| + 1) * (|dy| + 1)
            let area = ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1);
            max_area = max_area.max(area);
        }
    }

    max_area
}

pub fn part_two(input: &str) -> i64 {
    let points = parse_input(input);
    let n = points.len();

    // Build polygon edges (vertical segments only for ray casting)
    let mut vertical_edges: Vec<(i64, i64, i64)> = Vec::new(); // (x, y_min, y_max)

    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];

        if x1 == x2 {
            let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            vertical_edges.push((x1, min_y, max_y));
        }
    }

    // Function to check if point is inside or on polygon boundary
    let is_inside_or_boundary = |px: i64, py: i64| -> bool {
        // Check if on boundary
        for i in 0..n {
            let (x1, y1) = points[i];
            let (x2, y2) = points[(i + 1) % n];

            if x1 == x2 {
                let (min_y, max_y) =
                    if y1 < y2 { (y1, y2) } else { (y2, y1) };
                if px == x1 && py >= min_y && py <= max_y {
                    return true;
                }
            } else {
                let (min_x, max_x) =
                    if x1 < x2 { (x1, x2) } else { (x2, x1) };
                if py == y1 && px >= min_x && px <= max_x {
                    return true;
                }
            }
        }

        // Ray casting to check if inside
        let mut crossings = 0;
        for &(ex, ey_min, ey_max) in &vertical_edges {
            if ex > px && py > ey_min && py < ey_max {
                crossings += 1;
            }
        }
        crossings % 2 == 1
    };

    // Check if rectangle is valid (all corners and edges inside polygon)
    let is_rect_valid = |x1: i64, y1: i64, x2: i64, y2: i64| -> bool {
        let (min_x, max_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

        // Check all 4 corners
        if !is_inside_or_boundary(min_x, min_y) {
            return false;
        }
        if !is_inside_or_boundary(max_x, min_y) {
            return false;
        }
        if !is_inside_or_boundary(min_x, max_y) {
            return false;
        }
        if !is_inside_or_boundary(max_x, max_y) {
            return false;
        }

        // Check if any polygon edge crosses through the rectangle interior
        for i in 0..n {
            let (ex1, ey1) = points[i];
            let (ex2, ey2) = points[(i + 1) % n];

            if ex1 == ex2 {
                // Vertical edge
                let (ey_min, ey_max) =
                    if ey1 < ey2 { (ey1, ey2) } else { (ey2, ey1) };
                // Check if edge crosses rectangle interior
                if ex1 > min_x
                    && ex1 < max_x
                    && ey_min < max_y
                    && ey_max > min_y
                {
                    return false;
                }
            } else {
                // Horizontal edge
                let (ex_min, ex_max) =
                    if ex1 < ex2 { (ex1, ex2) } else { (ex2, ex1) };
                // Check if edge crosses rectangle interior
                if ey1 > min_y
                    && ey1 < max_y
                    && ex_min < max_x
                    && ex_max > min_x
                {
                    return false;
                }
            }
        }

        true
    };

    let mut max_area = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            if is_rect_valid(x1, y1, x2, y2) {
                let area = ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1);
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
    fn example() {
        let input = read_example(9);
        assert_eq!(part_one(&input), 50);
        assert_eq!(part_two(&input), 24);
    }
}
